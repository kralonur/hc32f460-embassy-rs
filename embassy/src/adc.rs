//! Async ADC2 scans and amplified ADC1 DMA captures at 12 bits.
//! ADC2 performs one optionally averaged sequence A scan on its end-of-scan
//! interrupt. ADC1 captures a single continuous input; see [`Adc1`].
//! No sequence B, external conversion triggers or analog watchdog.
//!
//! Ownership makes a scan valid by construction: [`Adc2::new`] takes the
//! [`AdcPin`]s it reads (one per ADC12 input pin, so none twice and at most
//! eight) and, for channel 8, the [`InternalReference`] proving the reference
//! is routed there.
//!
//! The application routes `pac::events::ADC2_EOCA` to an NVIC line inside its
//! group window (see [`crate::intc`]) and calls [`on_interrupt`] from that
//! line's handler, before the first [`Adc2::read`].
//!
//! Registers are reached through the PAC (vendor SVD `HC32F460JEUA.svd`);
//! values and sequences follow the vendor DDL (`hc32f460_adc.c/.h`,
//! `hc32f460_pwc.c`, `hc32f460_cmp.c`), unless cited otherwise.

use core::future::poll_fn;
use core::task::Poll;

use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, Timer, with_timeout};

use crate::gpio::{AnalogPin, Port};
use crate::pac;

mod capture;
pub use capture::{Adc1, AmplifiedPin, CaptureError};

/// Largest result: 2^12 − 1 at the 12-bit resolution this driver sets.
pub const FULL_SCALE: u16 = (1 << 12) - 1;

// DDL `AdcMode_SAOnce`: sequence A converts once per start (CR0.MS).
const SEQUENCE_A_ONCE: u8 = 0;
// DDL `AdcResolution_12Bit` (CR0.ACCSEL).
const TWELVE_BITS: u8 = 0;
// CHMUXR0/1 (SVD): four 4-bit multiplexer fields each, for channels 0..=7.
const MUX_FIELD_BITS: usize = 4;
const MUX_FIELDS_PER_REGISTER: usize = 4;
const MUX_FIELD: u16 = 0xf;
// DDL: ADC2 channels 0..=7 select ADC12_IN4..ADC12_IN11 (`ADC2_CH0`..`CH7`),
// channel 8 is internal (`ADC2_CH_INTERNAL`), chosen by CMP RVADC.
const FIRST_INPUT: u8 = 4;
const INTERNAL_CHANNEL: u8 = 8;
/// The ADC12_IN4..ADC12_IN11 pins, in input order (datasheet Table 2-1).
const INPUT_PINS: [(Port, u8); 8] = [
    (Port::A, 4),
    (Port::A, 5),
    (Port::A, 6),
    (Port::A, 7),
    (Port::B, 0),
    (Port::B, 1),
    (Port::C, 0),
    (Port::C, 1),
];

// PWC FPRC: DDL `ENABLE_PWR_REG_WRITE` writes key 0xA5 (FPRCWE) with FPRCB1,
// which unlocks PWCMR; `DISABLE_PWR_REG_WRITE` writes the key with it clear.
const FPRC_KEY: u8 = 0xa5;
// CMP RVADC: DDL `RVADC_WRITE_PROT_KEY` (WPRT) releases one write.
const RVADC_KEY: u8 = 0x55;
/// Settling time after buffering the reference, before it may be measured,
/// in µs (RM 5.3.8, step 3; the datasheet gives none and the manual was not
/// re-checked). The 1 kHz time driver rounds the wait up to a whole tick.
const REFERENCE_SETTLE_US: u64 = 50;
/// STR reads before a stop counts as failed: DDL `ADC_StopConvert` writes
/// STR = 0 until it reads 0, giving up after this many tries.
const STOP_ATTEMPTS: u32 = 10_000;

static WAKER: AtomicWaker = AtomicWaker::new();

/// Clear FCG3 gates (enabling their clocks; gates are active low), keeping
/// the others.
fn ungate(gates: impl FnOnce(&mut pac::pwc::fcg3::W) -> &mut pac::pwc::fcg3::W) {
    // SAFETY: only FCG3 is touched, and the critical section serializes this
    // read-modify-write with the other gate owners'.
    let pwc = unsafe { pac::Pwc::steal() };
    critical_section::with(|_| {
        pwc.fcg3().modify(|_, w| gates(w));
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("pin is not one of the ADC12_IN4..ADC12_IN11 inputs")]
    NotAnInput,
    #[error("pin is not one of the PA0..PA7 amplified ADC1 inputs")]
    NotAmplifiedInput,
    #[error("an ADC2 scan must contain between one and nine inputs")]
    InvalidScan,
    #[error("scan did not end within its bound; it was stopped")]
    Timeout,
    #[error("a running scan did not stop")]
    NotStopped,
}

/// Sampling time of one input, in ADC clock cycles: 11..=255 (datasheet
/// Rev1.3 Table 3-36, tS; the DDL accepts down to 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SampleTime(u8);

impl SampleTime {
    const MIN_CYCLES: u8 = 11;

    /// `cycles` of the ADC clock, or `None` below the datasheet minimum.
    /// Usable in a `const`, so a bad constant fails to compile.
    pub const fn new(cycles: u8) -> Option<Self> {
        if cycles >= Self::MIN_CYCLES {
            Some(Self(cycles))
        } else {
            None
        }
    }
}

/// Conversions averaged into each result (CR0.AVCNT; DDL `en_adc_avcnt_t`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Average {
    Two = 0,
    Four = 1,
    Eight = 2,
    Sixteen = 3,
    ThirtyTwo = 4,
    SixtyFour = 5,
    OneTwentyEight = 6,
    TwoFiftySix = 7,
}

/// An ADC12_IN4..ADC12_IN11 pin, made analog.
pub struct AdcPin {
    input: u8,
    _pin: AnalogPin,
}

impl AdcPin {
    /// Consume a unique GPIO token and configure it as an ADC2 input.
    pub fn from_pin(pin: crate::gpio::Pin) -> Result<Self, Error> {
        let index = INPUT_PINS
            .iter()
            .position(|&input| input == (pin.port(), pin.number()))
            .ok_or(Error::NotAnInput)?;
        Ok(Self {
            input: FIRST_INPUT + index as u8,
            _pin: pin.into_analog(),
        })
    }

    /// Make `pin` on `port` analog for ADC2, or refuse it untouched when it
    /// is not an ADC2 input.
    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    pub unsafe fn new(port: Port, pin: u8) -> Result<Self, Error> {
        let index = INPUT_PINS
            .iter()
            .position(|&input| input == (port, pin))
            .ok_or(Error::NotAnInput)?;
        Ok(Self {
            input: FIRST_INPUT + index as u8,
            // SAFETY: forwarded from this function's contract.
            _pin: unsafe { AnalogPin::new(port, pin) },
        })
    }
}

/// Proof the internal reference is buffered into the ADC and selected as
/// ADC2 channel 8's source.
pub struct InternalReference(());

impl InternalReference {
    /// Clock the comparator block, buffer the reference (about 1.15 V,
    /// RM 5.3.8) into the ADC, select it for channel 8 and wait for it to
    /// settle.
    /// # Safety
    /// Call once. The caller gives PWC FCG3.CMP, PWCMR.ADBUFE and CMP RVADC
    /// to this owner; no other code may configure them.
    pub async unsafe fn new() -> Self {
        ungate(|w| w.cmp().clear_bit());
        // SAFETY: the caller hands over the registers named above; the other
        // fields of FPRC and PWCMR are preserved.
        let (pwc, cmp) = unsafe { (pac::Pwc::steal(), pac::Cmpcr::steal()) };
        // The clock code changes FPRC under a critical section too, so no
        // unlock window interleaves with another.
        critical_section::with(|_| {
            // SAFETY (key writes): each key fits its 8-bit field.
            pwc.fprc()
                .modify(|_, w| unsafe { w.fprcwe().bits(FPRC_KEY) }.fprcb1().set_bit());
            pwc.pwcmr().modify(|_, w| w.adbufe().set_bit());
            pwc.fprc()
                .modify(|_, w| unsafe { w.fprcwe().bits(FPRC_KEY) }.fprcb1().clear_bit());
            cmp.rvadc().write(|w| unsafe { w.wprt().bits(RVADC_KEY) });
            cmp.rvadc().write(|w| w.vrefsw().set_bit());
        });
        Timer::after_micros(REFERENCE_SETTLE_US).await;
        Self(())
    }
}

/// What a scanned input reads.
pub enum Source {
    Pin(AdcPin),
    Internal(InternalReference),
}

/// One scanned input and its sampling time.
pub struct Input {
    pub source: Source,
    pub sample_time: SampleTime,
}

/// ADC2 set up for one scan of `N` inputs, read back in their order.
pub struct Adc2<const N: usize> {
    adc: pac::Adc2,
    _inputs: [Input; N],
    channels: [u8; N],
    timeout: Duration,
}

impl<const N: usize> Adc2<N> {
    /// Clock ADC2 and set up sequence A for `inputs`: each pin on the next
    /// multiplexed channel, the reference on channel 8. `average` averages
    /// every input. A scan not finished after `timeout` is stopped and
    /// fails; bound it from the ADC clock and the sampling times. Empty scans
    /// are rejected before hardware access; inherited conversions must stop
    /// successfully before any configuration is replaced.
    /// # Safety
    /// Call once. The caller gives ADC2 and PWC FCG3.ADC2 to this owner.
    pub unsafe fn new(
        adc: pac::Adc2,
        inputs: [Input; N],
        average: Option<Average>,
        timeout: Duration,
    ) -> Result<Self, Error> {
        if N == 0 || N > usize::from(INTERNAL_CHANNEL) + 1 {
            return Err(Error::InvalidScan);
        }
        // The pins are distinct ADC12 inputs, so at most eight take channels
        // 0..=7 in turn; the one reference takes channel 8.
        let mut next = 0;
        let channels = inputs.each_ref().map(|input| match input.source {
            Source::Pin(_) => {
                next += 1;
                next - 1
            }
            Source::Internal(_) => INTERNAL_CHANNEL,
        });
        ungate(|w| w.adc2().clear_bit());
        // DDL `ADC_Init`: stop before configuring, EOC interrupts off.
        if !stop(&adc) {
            return Err(Error::NotStopped);
        }
        adc.icr()
            .write(|w| w.eocaien().clear_bit().eocbien().clear_bit());
        // SAFETY (field writes below): each value fits its field.
        adc.cr0().write(|w| unsafe {
            w.ms()
                .bits(SEQUENCE_A_ONCE)
                .accsel()
                .bits(TWELVE_BITS)
                .avcnt()
                .bits(average.map_or(0, |average| average as u8))
        });
        let mask = channels
            .iter()
            .fold(0u16, |mask, &channel| mask | 1 << channel);
        adc.chselra().write(|w| unsafe { w.chsela().bits(mask) });
        let averaged = if average.is_some() { mask } else { 0 };
        adc.avchselr()
            .write(|w| unsafe { w.avchsel().bits(averaged) });
        for (input, &channel) in inputs.iter().zip(&channels) {
            let channel = usize::from(channel);
            // SAFETY: SSTR0..=SSTR8 are consecutive bytes (SVD), and the
            // channel is at most 8.
            unsafe {
                adc.sstr0()
                    .as_ptr()
                    .add(channel)
                    .write_volatile(input.sample_time.0)
            };
            // DDL `ADC_ChannelRemap`.
            if let Source::Pin(pin) = &input.source {
                let shift = channel % MUX_FIELDS_PER_REGISTER * MUX_FIELD_BITS;
                let mux = u16::from(pin.input - FIRST_INPUT);
                // SAFETY: CHMUXR0 and CHMUXR1 are consecutive halfwords (SVD)
                // covering channels 0..=7, which pins take.
                unsafe {
                    let register = adc
                        .chmuxr0()
                        .as_ptr()
                        .add(channel / MUX_FIELDS_PER_REGISTER);
                    let value = register.read_volatile() & !(MUX_FIELD << shift);
                    register.write_volatile(value | mux << shift);
                }
            }
        }
        Ok(Self {
            adc,
            _inputs: inputs,
            channels,
            timeout,
        })
    }

    /// Run one scan and return each input's count, `0..=FULL_SCALE`.
    pub async fn read(&mut self) -> Result<[u16; N], Error> {
        let adc = &self.adc;
        if !stop(adc) {
            return Err(Error::NotStopped);
        }
        // DDL `ADC_ClrEocFlag`: the flag clears by writing it 0.
        adc.isr().modify(|_, w| w.eocaf().clear_bit());
        adc.str().write(|w| w.strt().set_bit());
        // The scan never outlives this read: on every exit, cancellation
        // included, its interrupt goes off and it is stopped.
        struct Abort<'a>(&'a pac::Adc2);
        impl Drop for Abort<'_> {
            fn drop(&mut self) {
                self.0.icr().write(|w| w.eocaien().clear_bit());
                // A finished scan has already stopped; one that will not stop
                // is reported by the next read.
                stop(self.0);
            }
        }
        let _abort = Abort(adc);
        with_timeout(
            self.timeout,
            poll_fn(|cx| {
                // Register before reading the flag, and arm only while it is
                // clear, so an end between the two is never lost.
                WAKER.register(cx.waker());
                if adc.isr().read().eocaf().bit_is_set() {
                    Poll::Ready(())
                } else {
                    adc.icr().write(|w| w.eocaien().set_bit());
                    Poll::Pending
                }
            }),
        )
        .await
        .map_err(|_| Error::Timeout)?;
        adc.isr().modify(|_, w| w.eocaf().clear_bit());
        Ok(self.channels.map(|channel| {
            // SAFETY: DR0..=DR8 are consecutive halfwords (SVD), and the
            // channel is at most 8.
            unsafe { adc.dr0().as_ptr().add(usize::from(channel)).read_volatile() }
        }))
    }
}

/// Stop any scan as DDL `ADC_StopConvert` does; `false` if it keeps running.
fn stop(adc: &pac::Adc2) -> bool {
    for _ in 0..STOP_ATTEMPTS {
        if adc.str().read().strt().bit_is_clear() {
            return true;
        }
        adc.str().write(|w| w.strt().clear_bit());
    }
    false
}

/// Call from the handler of the line `ADC2_EOCA` is routed to. The flag stays
/// set until the next scan, so the handler masks the source before waking
/// the reader; left armed, it would re-enter the handler at once.
pub fn on_interrupt() {
    // SAFETY: only ICR, which the reader re-arms on its next poll, is
    // written.
    let adc = unsafe { pac::Adc2::steal() };
    adc.icr().write(|w| w.eocaien().clear_bit());
    WAKER.wake();
}

#[cfg(test)]
mod tests;
