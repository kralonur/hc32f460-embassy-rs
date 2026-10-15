//! Single amplified ADC1 input, continuous sequence A, captured by DMA.
//! PGA configuration follows DDL `ADC_ConfigPga`, `ADC_PgaSelChannel` and
//! `ADC_PgaCmd`: gain two, internal analog ground, one external input.

use embassy_time::{Duration, Timer, with_timeout};

use super::{
    Error, MUX_FIELD, MUX_FIELD_BITS, MUX_FIELDS_PER_REGISTER, STOP_ATTEMPTS, SampleTime,
    TWELVE_BITS, ungate,
};
use crate::dma;
use crate::gpio::{AnalogPin, Pin, Port};
use crate::pac;

// DDL `AdcMode_SAContinuous`: repeat sequence A until software stops it.
const SEQUENCE_A_CONTINUOUS: u8 = 1;
// DDL `AdcPgaCtl_Amplify` and `AdcPgaFactor_2`.
const PGA_ENABLE: u8 = 0xe;
const PGA_GAIN_TWO: u8 = 0;
// PGA's external inputs are PA0..PA7 (datasheet pin table; DDL PGA_CH0..7).
const LAST_PGA_PIN: u8 = 7;
// Conservative startup wait in microseconds, not a measured settling time.
// The 1 kHz firmware time driver rounds this to a whole tick.
const PGA_SETTLE_US: u64 = 1000;

/// An exclusively owned external PGA input (PA0..PA7), configured analog.
pub struct AmplifiedPin {
    channel: u8,
    _pin: AnalogPin,
}

impl AmplifiedPin {
    /// Consume a unique GPIO token; only PA0..PA7 can use the amplifier.
    pub fn from_pin(pin: Pin) -> Result<Self, Error> {
        let channel = Self::channel(pin.port(), pin.number())?;
        Ok(Self {
            channel,
            _pin: pin.into_analog(),
        })
    }

    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    pub unsafe fn new(port: Port, pin: u8) -> Result<Self, Error> {
        let channel = Self::channel(port, pin)?;
        Ok(Self {
            channel,
            // SAFETY: forwarded from this function's contract.
            _pin: unsafe { AnalogPin::new(port, pin) },
        })
    }

    fn channel(port: Port, pin: u8) -> Result<u8, Error> {
        if port == Port::A && pin <= LAST_PGA_PIN {
            Ok(pin)
        } else {
            Err(Error::NotAmplifiedInput)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CaptureError {
    #[error(transparent)]
    Adc(#[from] Error),
    #[error(transparent)]
    Dma(#[from] dma::Error),
}

/// ADC1 and a DMA channel, capturing one amplified input at 12 bits.
/// Each capture starts conversion only after DMA is armed and stops it on
/// every exit. ADC2, including its reference routing, is independent.
pub struct Adc1 {
    adc: pac::Adc1,
    input: AmplifiedPin,
    dma: dma::Channel,
    timeout: Duration,
}

impl Adc1 {
    /// Configure continuous sequence A for one input, without averaging,
    /// with twofold PGA gain relative to internal analog ground.
    ///
    /// # Safety
    /// Call once; the caller gives ADC1 (including SYNC and PGA registers)
    /// and PWC FCG3.ADC1 to this owner. Route the DMA channel's completion
    /// and error interrupts before capturing. No ADC1 EOC interrupt is used.
    pub async unsafe fn new(
        adc: pac::Adc1,
        input: AmplifiedPin,
        dma: dma::Channel,
        sample_time: SampleTime,
        timeout: Duration,
    ) -> Result<Self, Error> {
        ungate(|w| w.adc1().clear_bit());
        if !stop(&adc) {
            return Err(Error::NotStopped);
        }
        adc.icr()
            .write(|w| w.eocaien().clear_bit().eocbien().clear_bit());
        // Disable inherited external triggers, ADC synchronization, averaging
        // and watchdogs; this owner starts sequence A in software only.
        adc.trgsr().reset();
        adc.synccr().reset();
        adc.chselrb().reset();
        adc.avchselr().reset();
        adc.awdcr().reset();
        adc.pgacr().reset();
        // SAFETY: DDL encodings and the owned pin's channel fit these fields.
        adc.cr0().write(|w| unsafe {
            w.ms()
                .bits(SEQUENCE_A_CONTINUOUS)
                .accsel()
                .bits(TWELVE_BITS)
        });
        adc.cr1().reset();
        let channel = input.channel;
        // SAFETY (field writes below): channel is at most 7, so each one-hot
        // mask fits its select field; gain and enable are DDL encodings.
        adc.chselra()
            .write(|w| unsafe { w.chsela().bits(1 << channel) });
        write_sample_time(&adc, channel, sample_time);
        // SAFETY: CHMUXR0..1 are consecutive halfwords, each containing four
        // 4-bit fields. channel is at most 7.
        unsafe {
            let shift = usize::from(channel) % MUX_FIELDS_PER_REGISTER * MUX_FIELD_BITS;
            let mux = adc
                .chmuxr0()
                .as_ptr()
                .add(usize::from(channel) / MUX_FIELDS_PER_REGISTER);
            mux.write_volatile(
                (mux.read_volatile() & !(MUX_FIELD << shift)) | u16::from(channel) << shift,
            );
        }
        adc.pgagsr()
            .write(|w| unsafe { w.gain().bits(PGA_GAIN_TWO) });
        // DDL `AdcPgaNegative_VSSA`: internal analog ground, no external pin.
        adc.pgainsr1().write(|w| w.pgavssen().set_bit());
        adc.pgainsr0()
            .write(|w| unsafe { w.pgainsel().bits(1 << channel) });
        adc.pgacr()
            .write(|w| unsafe { w.pgactl().bits(PGA_ENABLE) });
        Timer::after_micros(PGA_SETTLE_US).await;
        Ok(Self {
            adc,
            input,
            dma,
            timeout,
        })
    }

    /// Change the input's sampling time between captures, e.g. for a slower
    /// sample rate. Takes effect on the next [`Self::capture`].
    pub fn set_sample_time(&mut self, sample_time: SampleTime) -> Result<(), Error> {
        if !stop(&self.adc) {
            return Err(Error::NotStopped);
        }
        write_sample_time(&self.adc, self.input.channel, sample_time);
        Ok(())
    }

    /// Capture consecutive samples. A timeout, DMA error or cancellation
    /// stops DMA before returning the buffer. Empty buffers do not start ADC.
    ///
    /// # Safety
    /// Never forget the polled capture future: await or drop it before
    /// releasing or accessing `samples`. DMA owns that buffer while running.
    pub async unsafe fn capture(&mut self, samples: &mut [u16]) -> Result<(), CaptureError> {
        if !stop(&self.adc) {
            return Err(Error::NotStopped.into());
        }
        if samples.is_empty() {
            return Ok(());
        }
        self.adc.isr().modify(|_, w| w.eocaf().clear_bit());
        // SAFETY: DR0..7 are consecutive halfwords; this channel is at most 7.
        let source = unsafe { self.adc.dr0().as_ptr().add(usize::from(self.input.channel)) };
        // SAFETY: the source is our selected input's result register, EOCA
        // occurs after each single-input scan, and the caller forbids leaking
        // the future. Transfer's Drop quiesces DMA before releasing samples.
        let transfer = unsafe {
            self.dma
                .read_u16(source, samples, pac::events::EventSource::ADC1_EOCA)
        }?;
        struct Stop<'a>(&'a pac::Adc1);
        impl Drop for Stop<'_> {
            fn drop(&mut self) {
                stop(self.0);
            }
        }
        // Stop conversion on cancellation too. Transfer independently waits
        // for DMA quiescence before releasing the borrowed sample buffer.
        let guard = Stop(&self.adc);
        self.adc.str().write(|w| w.strt().set_bit());
        let result = with_timeout(self.timeout, transfer).await;
        drop(guard);
        if !stop(&self.adc) {
            return Err(Error::NotStopped.into());
        }
        result.map_err(|_| Error::Timeout)??;
        Ok(())
    }
}

impl Drop for Adc1 {
    fn drop(&mut self) {
        stop(&self.adc);
        self.adc.pgacr().reset();
    }
}

/// Write `channel`'s sampling time; the caller has stopped conversion.
fn write_sample_time(adc: &pac::Adc1, channel: u8, sample_time: SampleTime) {
    // SAFETY: SSTR0..7 are consecutive bytes; the owned PGA channel is at most 7.
    unsafe {
        adc.sstr0()
            .as_ptr()
            .add(usize::from(channel))
            .write_volatile(sample_time.0);
    }
}

fn stop(adc: &pac::Adc1) -> bool {
    for _ in 0..STOP_ATTEMPTS {
        if adc.str().read().strt().bit_is_clear() {
            return true;
        }
        adc.str().write(|w| w.strt().clear_bit());
    }
    false
}
