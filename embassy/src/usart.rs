//! Interrupt-driven USART1..4 in asynchronous 8-bit UART mode.
//!
//! Own the PAC peripheral and configure TX/RX pins before construction. Route
//! the unit's receive, receive-error and transmit-complete events through INTC;
//! all three handlers call [`on_interrupt`]. The ISR never accesses a borrowed
//! buffer. [`Uart::split`] permits concurrent receive and transmit without a
//! shared-bus mutex or allocation.
//!
//! Register definitions and baud arithmetic follow RM 25.5 / Tables 25-9..10.
//! Errata 4.5.1 prevents reliably rearming TX-empty after TE is enabled. This
//! driver waits for transmit-complete instead, without toggling TE or TX pins
//! between bytes. There can be executor-dependent gaps; no DMA, software RX
//! ring, hardware flow control, 9-bit data, synchronous or smart-card mode.
//! The hardware receive register holds one byte: sustained RX requires prompt
//! polling. Wrap waits in `embassy_time::with_timeout` when a bound is needed.
//!
//! `read` is cancellation-safe: a pending future has not consumed bytes. A
//! canceled `write` may have sent a prefix; `flush` fences any final frame.
//!
//! ```no_run
//! use embassy_hc32::usart::{Error, Instance, Uart};
//! use embedded_io_async::{Read, Write};
//!
//! async fn exchange<T: Instance>(uart: &mut Uart<T>) -> Result<u8, Error> {
//!     let (mut tx, mut rx) = uart.split();
//!     tx.write_all(b"hello").await?;
//!     tx.flush().await?;
//!     let mut byte = [0];
//!     rx.read(&mut byte).await?;
//!     Ok(byte[0])
//! }
//! ```

use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;

use crate::pac::{self, events::EventSource};

// RM 25.5: word-wide registers except the split halfword data register.
const SR: usize = 0x00;
const TDR: usize = 0x04;
const RDR: usize = 0x06;
const BRR: usize = 0x08;
const CR1: usize = 0x0c;
const CR2: usize = 0x10;
const CR3: usize = 0x14;
const PR: usize = 0x18;
// RM 25.5.1: receive errors and ready flags.
const PE: u32 = 1 << 0;
const FE: u32 = 1 << 1;
const ORE: u32 = 1 << 3;
const RXNE: u32 = 1 << 5;
const TC: u32 = 1 << 6;
const RECEIVE_ERRORS: u32 = PE | FE | ORE;
// RM 25.5.4: receiver/transmitter enables and interrupt masks.
const RE: u32 = 1 << 2;
const TE: u32 = 1 << 3;
const RIE: u32 = 1 << 5;
const TCIE: u32 = 1 << 6;
// RM 25.5.4: write-one error-clear commands (not persistent configuration).
const CLEAR_ERRORS: u32 = (1 << 16) | (1 << 17) | (1 << 19);
// RM 25.5.4: falling-edge start detection and fractional baud enable.
const SBS: u32 = 1 << 31;
const FBME: u32 = 1 << 29;
// RM 25.5.4 / 25.5.5: optional parity and two stop bits.
const PCE: u32 = 1 << 10;
const PS: u32 = 1 << 9;
const STOP_TWO: u32 = 1 << 13;
// RM Table 25-9: fixed 16x oversampling for this driver's UART mode.
const OVERSAMPLE: u64 = 16;
// RM Table 25-10: fractional multiplier is (128 + fraction) / 256.
const FRACTION_SCALE: u64 = 256;
const MIN_FRACTION_FACTOR: u64 = 128;
// BRR.DIV_INTEGER is an eight-bit encoding of the divisor minus one.
const MAX_DIVISOR: u64 = 256;
// A chosen nominal baud-error bound of 2%; clock tolerance is a board obligation.
const MAX_ERROR_PPM: u64 = 20_000;
// Parts per million express the error bound without floating point.
const PPM: u64 = 1_000_000;
// Bound ready-path work per poll so a fast UART cannot monopolize the executor.
const YIELD_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Usart1 = 0,
    Usart2 = 1,
    Usart3 = 2,
    Usart4 = 3,
}

impl Unit {
    pub const fn receive_event(self) -> EventSource {
        match self {
            Self::Usart1 => EventSource::USART1_RI,
            Self::Usart2 => EventSource::USART2_RI,
            Self::Usart3 => EventSource::USART3_RI,
            Self::Usart4 => EventSource::USART4_RI,
        }
    }
    // Vendor header: each unit's EI, RI, TI and TCI event IDs are consecutive.
    pub const fn error_event(self) -> EventSource {
        EventSource(self.receive_event().0 - 1)
    }
    pub const fn complete_event(self) -> EventSource {
        EventSource(self.receive_event().0 + 2)
    }
    fn base(self) -> *const u8 {
        match self {
            Self::Usart1 => pac::Usart1::ptr().cast(),
            Self::Usart2 => pac::Usart2::ptr().cast(),
            Self::Usart3 => pac::Usart3::ptr().cast(),
            Self::Usart4 => pac::Usart4::ptr().cast(),
        }
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A supported PAC USART token; sealed to preserve peripheral identity.
pub trait Instance: sealed::Sealed {
    const UNIT: Unit;
}

macro_rules! instances {
    ($($name:ident),+ $(,)?) => {$(
        impl sealed::Sealed for pac::$name {}
        impl Instance for pac::$name {
            const UNIT: Unit = Unit::$name;
        }
    )+};
}
instances!(Usart1, Usart2, Usart3, Usart4);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Parity {
    #[default]
    None,
    Even,
    Odd,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StopBits {
    #[default]
    One,
    Two,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub baudrate: u32,
    pub parity: Parity,
    pub stop_bits: StopBits,
}

impl Default for Config {
    fn default() -> Self {
        // Conventional console baud, in bits/s; board/device must agree.
        Self {
            baudrate: 115_200,
            parity: Parity::None,
            stop_bits: StopBits::One,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("baud rate cannot be represented within 2% of the requested rate")]
    Baudrate,
    #[error("receive register overrun; data was lost")]
    Overrun,
    #[error("received an invalid stop bit")]
    Framing,
    #[error("received invalid parity")]
    Parity,
}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::Baudrate => embedded_io::ErrorKind::InvalidInput,
            Self::Framing | Self::Parity => embedded_io::ErrorKind::InvalidData,
            Self::Overrun => embedded_io::ErrorKind::Other,
        }
    }
}

struct Timing {
    prescaler: u32,
    brr: u32,
    fractional: bool,
    actual_baudrate: u32,
}

fn timing(pclk1: u32, baudrate: u32) -> Result<Timing, Error> {
    if pclk1 == 0 || baudrate == 0 {
        return Err(Error::Baudrate);
    }
    let clock = u64::from(pclk1);
    let baudrate = u64::from(baudrate);
    // PR.PSC=0..3 encodes /1, /4, /16, /64 (RM 25.5.7).
    for prescaler in 0..=3 {
        let sample_clock_divisor = OVERSAMPLE * (1 << (2 * prescaler));
        let divisor = clock / (baudrate * sample_clock_divisor);
        if !(1..=MAX_DIVISOR).contains(&divisor) {
            continue;
        }
        let denominator = sample_clock_divisor * divisor * FRACTION_SCALE;
        let factor = (baudrate * denominator + clock / 2) / clock;
        if !(MIN_FRACTION_FACTOR..=FRACTION_SCALE).contains(&factor) {
            continue;
        }
        let numerator = clock * factor;
        let target = baudrate * denominator;
        if numerator.abs_diff(target) * PPM > target * MAX_ERROR_PPM {
            continue;
        }
        let fractional = factor != FRACTION_SCALE;
        let fraction = if fractional {
            factor - MIN_FRACTION_FACTOR
        } else {
            0
        };
        return Ok(Timing {
            prescaler,
            brr: (((divisor - 1) << 8) | fraction) as u32,
            fractional,
            actual_baudrate: ((numerator + denominator / 2) / denominator) as u32,
        });
    }
    Err(Error::Baudrate)
}

trait Registers {
    fn read(&mut self, offset: usize) -> u32;
    fn write(&mut self, offset: usize, value: u32);
    fn receive(&mut self) -> u8;
    fn transmit(&mut self, byte: u8);
}

struct Hardware(Unit);
impl Registers for Hardware {
    fn read(&mut self, offset: usize) -> u32 {
        // SAFETY: internal offsets select word-aligned readable USART registers.
        unsafe { self.0.base().add(offset).cast::<u32>().read_volatile() }
    }
    fn write(&mut self, offset: usize, value: u32) {
        // SAFETY: the USART token owns these writable registers; shared CR1
        // updates are serialized by a critical section.
        unsafe {
            self.0
                .base()
                .add(offset)
                .cast::<u32>()
                .cast_mut()
                .write_volatile(value)
        }
    }
    fn receive(&mut self) -> u8 {
        // SAFETY: RDR is the readable upper halfword of DR (SVD and RM 25.5.2).
        unsafe { self.0.base().add(RDR).cast::<u16>().read_volatile() as u8 }
    }
    fn transmit(&mut self, byte: u8) {
        // SAFETY: TDR is the writable lower halfword of DR; eight-bit mode is set.
        unsafe {
            self.0
                .base()
                .add(TDR)
                .cast::<u16>()
                .cast_mut()
                .write_volatile(u16::from(byte))
        }
    }
}

fn control(bus: &mut impl Registers, clear: u32, set: u32) {
    critical_section::with(|_| {
        let current = bus.read(CR1) & !CLEAR_ERRORS;
        bus.write(CR1, (current & !clear) | set);
    });
}

fn initialize(bus: &mut impl Registers, config: Config, timing: &Timing) {
    // RM 25.3: reset CR1 first; protected format/baud fields are configured
    // only while TE and RE are clear, before enabling either direction.
    bus.write(CR1, 0);
    bus.write(CR1, SBS | CLEAR_ERRORS);
    // Discard inherited receive data while RE/TE are disabled.
    let _ = bus.receive();
    bus.write(
        CR2,
        if config.stop_bits == StopBits::Two {
            STOP_TWO
        } else {
            0
        },
    );
    bus.write(CR3, 0);
    bus.write(PR, timing.prescaler);
    bus.write(BRR, timing.brr);
    let parity = match config.parity {
        Parity::None => 0,
        Parity::Even => PCE,
        Parity::Odd => PCE | PS,
    };
    let format = SBS | parity | if timing.fractional { FBME } else { 0 };
    bus.write(CR1, format);
    bus.write(CR1, format | RE | TE);
}

static RX_WAKERS: [AtomicWaker; 4] = [const { AtomicWaker::new() }; 4];
static TX_WAKERS: [AtomicWaker; 4] = [const { AtomicWaker::new() }; 4];

/// Mask the asserted level sources before waking their independent waiters.
/// Route RI, EI and TCI to handlers calling this function for their unit.
pub fn on_interrupt(unit: Unit) {
    let mut bus = Hardware(unit);
    let status = bus.read(SR);
    if status & (RXNE | RECEIVE_ERRORS) != 0 {
        control(&mut bus, RIE, 0);
        RX_WAKERS[unit as usize].wake();
    }
    if status & TC != 0 {
        control(&mut bus, TCIE, 0);
        TX_WAKERS[unit as usize].wake();
    }
}

struct Wait<'a, R: Registers> {
    bus: &'a mut R,
    waker: &'a AtomicWaker,
    receive: bool,
    armed: bool,
}
impl<R: Registers + Unpin> Future for Wait<'_, R> {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        let this = self.get_mut();
        let ready = if this.receive {
            RXNE | RECEIVE_ERRORS
        } else {
            TC
        };
        let enable = if this.receive { RIE } else { TCIE };
        let status = this.bus.read(SR);
        if status & ready != 0 {
            return Poll::Ready(status);
        }
        // Immediately-ready paths need no waker lock or CR1 writes. Otherwise
        // register before arming and recheck persistent status to close the race.
        this.waker.register(cx.waker());
        this.armed = true;
        control(this.bus, 0, enable);
        let status = this.bus.read(SR);
        if status & ready != 0 {
            Poll::Ready(status)
        } else {
            Poll::Pending
        }
    }
}
impl<R: Registers> Drop for Wait<'_, R> {
    fn drop(&mut self) {
        if self.armed {
            control(self.bus, if self.receive { RIE } else { TCIE }, 0);
        }
    }
}

async fn read(
    bus: &mut (impl Registers + Unpin),
    waker: &AtomicWaker,
    bytes: &mut [u8],
) -> Result<usize, Error> {
    if bytes.is_empty() {
        return Ok(0);
    }
    let status = Wait {
        bus,
        waker,
        receive: true,
        armed: false,
    }
    .await;
    if status & RECEIVE_ERRORS != 0 {
        // Consume the retained bad/stale byte and clear all receive errors so
        // the next call can receive again (RM 25.5.1 / 25.5.4).
        let _ = bus.receive();
        control(bus, 0, CLEAR_ERRORS);
        return Err(if status & ORE != 0 {
            Error::Overrun
        } else if status & FE != 0 {
            Error::Framing
        } else {
            Error::Parity
        });
    }
    bytes[0] = bus.receive();
    let mut count = 1;
    // Return partial progress instead of waiting for the entire buffer.
    // Leave a newly encountered error for the next read to report.
    while count < bytes.len().min(YIELD_BYTES) && bus.read(SR) & (RXNE | RECEIVE_ERRORS) == RXNE {
        bytes[count] = bus.receive();
        count += 1;
    }
    Ok(count)
}

async fn write(
    bus: &mut (impl Registers + Unpin),
    waker: &AtomicWaker,
    bytes: &[u8],
) -> Result<usize, Error> {
    for (index, &byte) in bytes.iter().enumerate() {
        Wait {
            bus,
            waker,
            receive: false,
            armed: false,
        }
        .await;
        bus.transmit(byte);
        if (index + 1).is_multiple_of(YIELD_BYTES) {
            embassy_futures::yield_now().await;
        }
    }
    // embedded-io writes may return before the last frame leaves the wire.
    Ok(bytes.len())
}

/// Exclusive UART owner. The nominal realized baud rate is available for
/// board diagnostics; oscillator tolerance is not included in it.
pub struct Uart<T: Instance> {
    _peripheral: T,
    actual_baudrate: u32,
}
impl<T: Instance> Uart<T> {
    /// Configure internal PCLK1 clocking and 8-bit LSB-first data.
    /// # Safety
    /// The board owns the configured TX/RX pins and supplies the actual PCLK1
    /// in Hz. PWC write access must be ready; no inherited UART/DMA access may
    /// remain. Install RI/EI/TCI handlers before awaiting any I/O.
    pub unsafe fn new(peripheral: T, pclk1: u32, config: Config) -> Result<Self, Error> {
        let timing = timing(pclk1, config.baudrate)?;
        // SAFETY: only the unit's FCG1 gate is modified, serialized with peers.
        let pwc = unsafe { &*pac::Pwc::ptr() };
        critical_section::with(|_| {
            // SVD: FCG1.USART1..4 are consecutive active-low gates at bits 24..27.
            const USART_GATE_FIRST: u32 = 24;
            let bit = 1 << (USART_GATE_FIRST + T::UNIT as u32);
            pwc.fcg1().modify(|r, w| unsafe { w.bits(r.bits() & !bit) });
        });
        initialize(&mut Hardware(T::UNIT), config, &timing);
        Ok(Self {
            _peripheral: peripheral,
            actual_baudrate: timing.actual_baudrate,
        })
    }

    pub fn baudrate(&self) -> u32 {
        self.actual_baudrate
    }

    /// Borrow independent directions. Neither half can outlive this owner.
    pub fn split(&mut self) -> (Tx<'_>, Rx<'_>) {
        (
            Tx {
                bus: Hardware(T::UNIT),
                _owner: PhantomData,
            },
            Rx {
                bus: Hardware(T::UNIT),
                _owner: PhantomData,
            },
        )
    }
}
impl<T: Instance> Drop for Uart<T> {
    fn drop(&mut self) {
        // Dropping may truncate the final TX frame; flush before dropping when
        // physical completion is required. No borrowed buffers are accessed.
        control(&mut Hardware(T::UNIT), RE | TE | RIE | TCIE, 0);
    }
}

/// Borrowed transmitter. Cancellation can send a prefix; flush to fence the wire.
pub struct Tx<'a> {
    bus: Hardware,
    _owner: PhantomData<&'a mut ()>,
}
/// Borrowed receiver. Reads return partial progress and are cancellation-safe.
pub struct Rx<'a> {
    bus: Hardware,
    _owner: PhantomData<&'a mut ()>,
}

impl<T: Instance> embedded_io::ErrorType for Uart<T> {
    type Error = Error;
}
impl embedded_io::ErrorType for Tx<'_> {
    type Error = Error;
}
impl embedded_io::ErrorType for Rx<'_> {
    type Error = Error;
}

impl embedded_io_async::Read for Rx<'_> {
    async fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        let waker = &RX_WAKERS[self.bus.0 as usize];
        read(&mut self.bus, waker, bytes).await
    }
}
impl embedded_io_async::Write for Tx<'_> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        let waker = &TX_WAKERS[self.bus.0 as usize];
        write(&mut self.bus, waker, bytes).await
    }
    async fn flush(&mut self) -> Result<(), Error> {
        Wait {
            waker: &TX_WAKERS[self.bus.0 as usize],
            bus: &mut self.bus,
            receive: false,
            armed: false,
        }
        .await;
        Ok(())
    }
}
impl<T: Instance> embedded_io_async::Read for Uart<T> {
    async fn read(&mut self, bytes: &mut [u8]) -> Result<usize, Error> {
        let (_, mut rx) = self.split();
        embedded_io_async::Read::read(&mut rx, bytes).await
    }
}
impl<T: Instance> embedded_io_async::Write for Uart<T> {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        let (mut tx, _) = self.split();
        embedded_io_async::Write::write(&mut tx, bytes).await
    }
    async fn flush(&mut self) -> Result<(), Error> {
        let (mut tx, _) = self.split();
        embedded_io_async::Write::flush(&mut tx).await
    }
}

#[cfg(test)]
mod tests;
