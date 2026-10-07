//! Single-channel TimerA4 PWM, using the backlight's proven sawtooth setup.
//!
//! The driver owns the whole timer (all channels share its counter/period).
//! Pin muxing remains the caller's responsibility. Counting uses PCLK1 / 256;
//! no interrupt, async wait, or software refresh is required.
//!
//! Only interior compare values are supported. This deliberately does not
//! implement `embedded_hal::pwm::SetDutyCycle`: 0%/100% require separate output
//! handling (errata Rev 1.41 §3.3.3). Runtime compare updates are unbuffered and
//! are not guaranteed glitch-free. §3.3.2 also prevents guaranteeing the initial
//! output level with a divided clock; normal output settles at the period match.

use crate::pac;

// HC32F460 PWC FCG2 register controlling the TimerA clocks.
const CLOCK_GATE: usize = 0x4004_8008;
// Vendor SVD / RM 21.5.1: TimerA counter register byte offset.
const CNTER: usize = 0x00;
// Vendor SVD / RM 21.5.2: TimerA period register byte offset.
const PERAR: usize = 0x04;
// Vendor SVD / RM 21.5.3: first channel's comparison register byte offset.
const CMPAR: usize = 0x40;
// Vendor SVD / RM 21.5.4: TimerA control/status register byte offset.
const BCSTR: usize = 0x80;
// Vendor SVD: first channel's output-control register byte offset.
const PCONR: usize = 0x140;
// Vendor SVD: PWC FCG2.TIMERA_4 (bit 5) disables TimerA4's clock when set.
const CLOCK_BIT: u32 = 1 << 5;
// Vendor SVD: TimerA provides eight comparison/output channels, numbered 1..=8.
const CHANNEL_COUNT: u8 = 8;
// Vendor SVD: successive CMPAR/PCONR channel registers are spaced four bytes apart.
const CHANNEL_STRIDE: usize = 4;
// Interior duty cycles require 0 < compare < period; the smallest valid period is 2.
const MIN_PERIOD: u16 = 2;
// RM 21.5 PCONR: OUTEN=1, CMPC=0b11 (invert), PERC=0b01 (high),
// STAC=0b01 (high at start), STPC=0b00, FORC=0b00. Preserve the original
// setup; the start-level request is ineffective with /256 per errata §3.3.2.
const OUTPUT: u16 = 0x1071;
// RM 21.5.4 BCSTR: CKDIV=8 (/256), DIR=1 (up), MODE=0 (sawtooth), START=1.
const RUN: u16 = 0x0083;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidChannel,
    InvalidPeriod,
    CompareOutOfRange,
}

trait Registers {
    fn read_gate(&mut self) -> u32;
    fn write_gate(&mut self, value: u32);
    fn write_timer(&mut self, offset: usize, value: u16);
}

struct Mmio;

impl Registers for Mmio {
    fn read_gate(&mut self) -> u32 {
        // SAFETY: CLOCK_GATE is the word-aligned, readable PWC FCG2 register.
        unsafe { (CLOCK_GATE as *const u32).read_volatile() }
    }

    fn write_gate(&mut self, value: u32) {
        // SAFETY: CLOCK_GATE is writable; enable_clock serializes its read-modify-write.
        unsafe { (CLOCK_GATE as *mut u32).write_volatile(value) }
    }

    fn write_timer(&mut self, offset: usize, value: u16) {
        // SAFETY: Internal helpers use documented halfword-aligned TimerA4 offsets,
        // and construction requires exclusive ownership of the timer.
        unsafe { (pac::Tmra4::ptr().cast::<u8>().add(offset) as *mut u16).write_volatile(value) }
    }
}

fn enable_clock(bus: &mut impl Registers) {
    critical_section::with(|_| {
        let gate = bus.read_gate();
        bus.write_gate(gate & !CLOCK_BIT);
    });
}

fn check_config(channel: u8, period: u16) -> Result<(), Error> {
    if !(1..=CHANNEL_COUNT).contains(&channel) {
        Err(Error::InvalidChannel)
    } else if period < MIN_PERIOD {
        Err(Error::InvalidPeriod)
    } else {
        Ok(())
    }
}

fn check_compare(period: u16, compare: u16) -> Result<(), Error> {
    if compare == 0 || compare >= period {
        Err(Error::CompareOutOfRange)
    } else {
        Ok(())
    }
}

fn set_compare(
    bus: &mut impl Registers,
    channel: u8,
    period: u16,
    compare: u16,
) -> Result<(), Error> {
    check_compare(period, compare)?;
    bus.write_timer(CMPAR + CHANNEL_STRIDE * usize::from(channel - 1), compare);
    Ok(())
}

fn start(bus: &mut impl Registers, channel: u8, period: u16, compare: u16) -> Result<(), Error> {
    check_compare(period, compare)?;
    let index = usize::from(channel - 1);
    bus.write_timer(CNTER, 0);
    bus.write_timer(PERAR, period);
    bus.write_timer(PCONR + CHANNEL_STRIDE * index, OUTPUT);
    bus.write_timer(CMPAR + CHANNEL_STRIDE * index, compare);
    bus.write_timer(BCSTR, RUN);
    Ok(())
}

/// One PWM output on TimerA4. Dropping the handle does not stop the timer.
pub struct TimerA4Pwm {
    _timer: pac::Tmra4,
    channel: u8,
    period: u16,
}

impl TimerA4Pwm {
    /// Claim TimerA4 and enable its clock, without starting/configuring output.
    /// `channel` is numbered 1..=8; `period` is the PERAR reference (at least 2).
    /// Route the desired pin before calling [`Self::start`].
    ///
    /// # Safety
    /// Caller must own the entire TimerA4 unit, with no conflicting raw access
    /// or other channel users. System clocks and PWC write access must be ready.
    /// The selected channel must already be in compare mode with buffering
    /// disabled, and external timer triggers disabled (reset-compatible state,
    /// as in the proven backlight path). This wrapper does not reset the unit.
    pub unsafe fn new(timer: pac::Tmra4, channel: u8, period: u16) -> Result<Self, Error> {
        check_config(channel, period)?;
        enable_clock(&mut Mmio);
        Ok(Self {
            _timer: timer,
            channel,
            period,
        })
    }

    /// Configure and start the counter. Use once after pin routing; calling
    /// again resets the counter. The compare value must be in `1..period`.
    pub fn start(&mut self, compare: u16) -> Result<(), Error> {
        start(&mut Mmio, self.channel, self.period, compare)
    }

    /// Update only this channel's compare register. No counter restart or
    /// period change occurs. Endpoints are rejected without register writes.
    pub fn set_compare(&mut self, compare: u16) -> Result<(), Error> {
        set_compare(&mut Mmio, self.channel, self.period, compare)
    }
}
