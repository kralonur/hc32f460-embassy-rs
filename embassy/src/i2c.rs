//! Async I2C3 master for HC32F460.
//!
//! Receive ordering follows DDL
//! `I2C_MasterReceiveDataAndStop` with fast ACK disabled. Instead of spinning
//! on `SR`, every step awaits a wakeup
//! from the I2C3 interrupt handlers, which the application routes through
//! [`crate::intc`] (events 428 `I2C3_RXI`, 429 `TXI`, 430 `TEI`, 431 `EEI`;
//! group 13 → lines 110..=115).
//!
//! The transaction sequence is separate from the hardware register access.
//!
//! Register facts (RM §26.5, vendor CMSIS header):
//! `CR1` `PE=0x1`, **`RESTART=0x80`**, `START=0x100`, `STOP=0x200`,
//! `ACK=0x400` (writing it sends NACK);
//! status flags and their interrupt enables share bit positions — `STARTF/E` 0,
//! `TENDF/E` 3, `STOPF/E` 4, `RFULLF/E` 6, `TEMPTYF/E` 7, `ARLOF/E` 9,
//! `ACKRF` 10, `NACKF/E` 12, `TMOUTF/E` 14, `BUSY` 17, `TRA` 18. (`TENDIE` is
//! bit 3, not bit 8: an early version armed bit 8, so the transfer-end wake
//! never fired and every async transaction ran into its timeout.)
//!
//! Errata §4.1.1 is implemented as both of its steps: `CR4.BUSWAIT` is
//! set, **and** the last received byte is preceded by `CR1.STOP` (see
//! [`I2c3::read_register`]). An earlier revision of this driver used the first step
//! only, reasoning that the owner-confirmed polled sequence does the same — the
//! vendor text requires both steps. Read-to-write repeated starts are rejected
//! before I/O because the documented workaround requires STOP after a read.
//! STOP ordering alone did not resolve the
//! owner-observed second-byte timeout; its cause is not yet hardware-proven.
//!
//! `i2c-rx-poll-diagnostic` temporarily replaces only the two RFULL waits with
//! bounded polling (4096 SR reads per byte). It is not full async support;
//! default builds retain interrupt-driven receive waits.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, with_timeout};

/// I2C3 register block base.
pub const I2C3_BASE: usize = 0x4004_E800;

// Vendor SVD: I2C control register 1 byte offset.
const CR1: usize = 0x00;
// Vendor SVD: I2C interrupt-enable register byte offset.
const CR2: usize = 0x04;
// Vendor SVD: I2C control register 3 byte offset.
const CR3: usize = 0x08;
// Vendor SVD: I2C control register 4 byte offset.
const CR4: usize = 0x0c;
// Vendor SVD: I2C status register byte offset.
const SR: usize = 0x1c;
// Vendor SVD: I2C status-clear register byte offset.
const CLR: usize = 0x20;
// Vendor SVD: I2C transmit-data register byte offset.
const DTR: usize = 0x24;
// Vendor SVD: I2C receive-data register byte offset.
const DRR: usize = 0x28;
// Vendor SVD: I2C clock-control register byte offset.
const CCR: usize = 0x2c;
// RM 26.5.5–26.5.6: slave address registers; this driver is master-only.
const SLR0: usize = 0x10;
const SLR1: usize = 0x14;
// RM 26.5.14: digital/analog filter configuration register byte offset.
const FLTR: usize = 0x30;
// RM 26.5.5–26.5.6: bit 12 enables the corresponding slave address match.
const SLAVE_ADDRESS_ENABLE: u32 = 1 << 12;
// RM 26.5.14: bits 1:0 encode filter capacity minus one; bit 4 enables it.
const DIGITAL_FILTER_WIDTH: u32 = 0b11;
const DIGITAL_FILTER_ENABLE: u32 = 1 << 4;

// RM 26.5.1: CR1.PE (bit 0) enables the I2C peripheral.
const PE: u32 = 1 << 0;
// RM 26.5.1: CR1.ENGC (bit 6) enables acknowledgement of general-call addresses.
const GENERAL_CALL: u32 = 1 << 6;
/// `CR1.RESTART` — "generate repeat start condition" (RM 26.5.1 b7). A repeated
/// start is *not* a second `START`: the proven polled driver writes `0x80` here
/// (`0x17f90`), and using `START` instead is what made the first async read fail
/// on hardware while the write path worked.
const RESTART: u32 = 1 << 7;
// RM 26.5.1: CR1.START (bit 8) requests a start condition.
const START: u32 = 1 << 8;
// RM 26.5.1: CR1.STOP (bit 9) requests a stop condition.
const STOP: u32 = 1 << 9;
/// `CR1.ACK`: 0 sends ACK, 1 sends NACK (RM 26.5.1 b10).
const NACK: u32 = 1 << 10;
/// `CR3.FACKEN`: early RFULL and software-controlled ACK clock (RM §26.5.3).
const FACKEN: u32 = 1 << 7;
/// `CR4.BUSWAIT` (same bit number as `NACK`, different register).
const BUSWAIT: u32 = 1 << 10;
// RM 26.5.1: CR1.SWRST (bit 15) resets the peripheral's communication state.
const RESET_STATE: u32 = 1 << 15;
// Original startup sequence asserts SWRST and ENGC together (formerly 0x8040),
// then clears both. ENGC is a general-call bit, not another reset bit.
const STARTUP_RESET: u32 = RESET_STATE | GENERAL_CALL;
// Vendor SVD: CLR.STARTFCLR (bit 0) clears the accepted-start status flag.
const STARTFCLR: u32 = 1 << 0;
// RM 26.5.8: CLR.STOPFCLR (bit 4) clears the previous stop condition.
const STOPFCLR: u32 = 1 << 4;

// Status flags and their interrupt enables share bit positions (vendor CMSIS
// header): STARTF/E 0, TENDF/E 3, STOPF/E 4, RFULLF/E 6, TEMPTYF/E 7,
// ARLOF/E 9, ACKRF 10, NACKF/E 12, TMOUTF/E 14, BUSY 17, TRA 18.
// Vendor SVD: SR.STARTF indicates an accepted start condition.
const STARTF: u32 = 1 << 0;
// Vendor SVD: SR.TENDF indicates the current transmit byte has completed.
const TENDF: u32 = 1 << 3;
// Vendor SVD: SR.STOPF indicates a completed stop condition.
const STOPF: u32 = 1 << 4;
// Vendor SVD: SR.RFULLF indicates the receive buffer is full.
const RFULLF: u32 = 1 << 6;
// Vendor SVD: SR.TEMPTYF indicates the transmit buffer is empty.
const TEMPTYF: u32 = 1 << 7;
// Vendor SVD: SR.ARLOF indicates arbitration loss.
const ARLOF: u32 = 1 << 9;
// Vendor SVD: SR.ACKRF records a NACK received for the transmitted byte.
const ACKRF: u32 = 1 << 10;
// Vendor SVD: SR.NACKF indicates a NACK error.
const NACKF: u32 = 1 << 12;
// Vendor SVD: SR.TMOUTF indicates a peripheral timeout.
const TMOUTF: u32 = 1 << 14;
// Vendor SVD: SR.BUSY indicates the bus is in use.
const BUSY: u32 = 1 << 17;
// Vendor SVD: SR.TRA is set for transmit mode and clear for receive mode.
const TRA: u32 = 1 << 18;

const STARTIE: u32 = STARTF;
const TENDIE: u32 = TENDF;
const STOPIE: u32 = STOPF;
const RFULLIE: u32 = RFULLF;
const TEMPTYIE: u32 = TEMPTYF;

const ERROR: u32 = ARLOF | NACKF | TMOUTF;
const ERROR_INTERRUPTS: u32 = ERROR;

/// Wake for "transmit buffer empty" waits.
///
/// The peripheral's interrupt sources are *level* based: enabling, say,
/// `TEMPTYIE` while idle would retrigger the interrupt as fast as the CPU could
/// service it. So each wait arms only the source that can change the flag it is
/// waiting for, and `CR2` is cleared again as soon as that wait ends (and when an
/// operation ends, including on timeout).
const WAKE_TX_EMPTY: u32 = TEMPTYIE | ERROR_INTERRUPTS;
/// Wake for receive mode entry and data waits. TENDF is invalid after a read
/// address (Errata §4.1.2); the official IRQ example enables RXI instead.
const WAKE_RX_FULL: u32 = RFULLIE | ERROR_INTERRUPTS;
/// Wake for a byte transfer ending.
const WAKE_TEND: u32 = TENDIE | ERROR_INTERRUPTS;
/// Wake for a stop condition completing.
const WAKE_STOP: u32 = STOPIE | ERROR_INTERRUPTS;
/// Wake for bus-free/start waits. Do not use for master receive-mode entry:
/// TENDF is invalid there and the already-set STARTF cannot provide a new wake.
const WAKE_TRANSFER: u32 = STARTIE | TENDIE | STOPIE | ERROR_INTERRUPTS;

/// Whole-operation timeout: a few bytes at 400 kHz take well under 200 us, so
/// 50 ms is generous without letting a stuck bus hang the caller.
const TIMEOUT: Duration = Duration::from_millis(50);
// Original driver's cooperative scheduling interval, in transferred bytes.
const YIELD_BYTES: usize = 64;
// I2C seven-bit addresses occupy the upper seven bits of the wire address byte.
const MAX_ADDRESS: u8 = 0x7f;
// I2C wire-address bit 0 selects read (1) rather than write (0).
const READ_DIRECTION: u8 = 1;
// Vendor SVD / RM 26.5.13: CCR.SLOWW and SHIGHW are each five bits wide.
const CCR_WIDTH_MASK: u32 = 0x1f;
// Original timing model gives each high/low width at least one reference-clock tick.
const MIN_WIDTH_SUM: u32 = 2;
// Two five-bit widths can represent at most 31 + 31 reference-clock ticks.
const MAX_WIDTH_SUM: u32 = 2 * CCR_WIDTH_MASK;
// RM-ZH 26.5.13 with one-cycle filtering: each SCL phase adds 3+1 ticks
// at CKDIV=0, or 2+1 ticks at CKDIV=1. Tuple fields are (divider, phase overhead).
const CLOCK_DIVIDERS: [(u32, u32); 2] = [(1, 4), (2, 3)];
// DS Table 3-24: standard/fast-mode maximum bus frequencies, in Hz.
const STANDARD_MAX_HZ: u32 = 100_000;
const FAST_MAX_HZ: u32 = 400_000;
// DS Table 3-24: minimum SCL low/high periods, in nanoseconds.
const STANDARD_PHASE_NS: (u64, u64) = (4_700, 4_000);
const FAST_PHASE_NS: (u64, u64) = (1_300, 600);
// SI conversion for frequency (Hz) times phase duration (ns).
const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
// Vendor SVD: CCR.SHIGHW occupies bits 12:8.
const CCR_HIGH_SHIFT: u32 = 8;
// SVD: CCR.FREQ occupies bits 18:16; corrected name is CKDIV in RM-ZH 26.5.13.
const CCR_FREQ_SHIFT: u32 = 16;

// Legacy bring-up marker values; keep their numbering for application status records.
// The last wait entered is not necessarily where time was spent.
// Embassy's timeout polls the transaction first, so a deadline wake can advance
// a missing-IRQ wait and expire at a later step in the same poll.
/// `begin`: waiting for the bus to be free (`BUSY == 0`).
pub const STEP_BUS_FREE: u32 = 1;
/// `begin`: waiting for the start condition to be accepted.
pub const STEP_START: u32 = 2;
/// `send`: waiting for the transmit buffer to be empty.
pub const STEP_TX_EMPTY: u32 = 3;
/// `send`: waiting for the byte transfer to end.
pub const STEP_TX_DONE: u32 = 4;
/// `finish`: waiting for the stop condition to complete.
pub const STEP_STOP: u32 = 5;
/// read: waiting for the repeated start to be accepted.
pub const STEP_RESTART: u32 = 6;
/// read: waiting for the transmit buffer before the read address.
pub const STEP_RX_ADDR_READY: u32 = 7;
/// read: waiting to enter receive mode (`TRA == 0`).
pub const STEP_RX_MODE: u32 = 8;
/// read: waiting for the first received byte.
pub const STEP_RX_FIRST: u32 = 9;
/// read: waiting for the second received byte.
pub const STEP_RX_SECOND: u32 = 10;

/// The wait the driver last entered (`0` before the first one).
static LAST_STEP: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// Bring-up trace of the last wait entered, or 0 before the first transaction.
pub fn last_step() -> u32 {
    LAST_STEP.load(core::sync::atomic::Ordering::Relaxed)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// NACK, arbitration loss, or peripheral timeout.
    Bus,
    /// Operation did not complete inside its bound. The payload is the last
    /// wait entered (the `STEP_*` constants), which may differ from where time was spent.
    Timeout(u32),
    /// The bus clock cannot be represented for the current PCLK3.
    Clock,
    /// Reserved for callers reporting an unavailable bus master.
    NotReady,
    /// Address is outside the supported seven-bit range; no I/O performed.
    Address,
    /// A read followed by a write needs a repeated start, but Errata 4.1.1
    /// requires STOP at the end of master reception. No I/O is performed.
    ReadRestart,
}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        // Bus currently groups NACK/arbitration/hardware timeout; do not invent
        // a more specific cause than the driver recorded.
        embedded_hal::i2c::ErrorKind::Other
    }
}

/// Register-file access used by the transaction state machine.
trait Registers {
    fn read(&mut self, offset: usize) -> u32;
    fn write(&mut self, offset: usize, value: u32);
    fn receive(&mut self) -> u8;
    fn transmit(&mut self, byte: u8);
}

static WAKER: AtomicWaker = AtomicWaker::new();

/// Mask every wake source (write `CR2 = 0`).
fn mask_sources() {
    // SAFETY: CR2 is the word-aligned I2C3 interrupt-enable register.
    unsafe { ((I2C3_BASE + CR2) as *mut u32).write_volatile(0) }
}

/// Called from every I2C3 interrupt handler (RXI/TXI/TEI/EEI).
///
/// The handler **masks the wake sources** and the waiter re-arms them when it
/// runs. That is not tidiness, it is the difference between working and a black
/// screen:
///
/// * these sources are level-based, so a source that is *already asserted* when
///   it is armed re-enters this handler immediately;
/// * on Cortex-M an interrupt that is still asserted tail-chains straight back
///   into the handler, so the task that would disarm the source never runs;
/// * the executor is starved, the first awaited transaction never completes, and
///   startup stops before the display is initialized — backlight on, screen
///   black. That is exactly what the first version of this driver did on
///   hardware.
///
/// Masking here can never lose a wake: the waiter reads `SR` before arming, so a
/// flag that sets while the source is masked is seen by the next poll.
pub fn on_interrupt() {
    mask_sources();
    WAKER.wake();
}

/// Wait until `sr & mask == expected`, or until an error bit appears.
///
/// The waker is registered *before* `SR` is read, so a status change between the
/// check and the sleep cannot be lost. The sources are armed only *after* that
/// read, and only the ones whose flags are still clear.
struct WaitSr<'a, R: Registers> {
    bus: &'a mut R,
    mask: u32,
    expected: u32,
    enable: u32,
}

impl<R: Registers> Future for WaitSr<'_, R> {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        WAKER.register(cx.waker());
        let this = self.get_mut();
        let sr = this.bus.read(SR);
        if sr & ERROR != 0 || sr & this.mask == this.expected {
            Poll::Ready(sr)
        } else {
            // Status flags and interrupt enables share bit positions, so `!sr`
            // removes exactly the sources whose flags are already asserted —
            // arming one of those would spin a handler entry per poll (and, with
            // the handler masking, would starve the executor on hardware).
            this.bus.write(CR2, this.enable & !sr);
            Poll::Pending
        }
    }
}

async fn wait(
    bus: &mut impl Registers,
    mask: u32,
    expected: u32,
    enable: u32,
    step: u32,
) -> Result<(), Error> {
    LAST_STEP.store(step, core::sync::atomic::Ordering::Relaxed);
    let sr = WaitSr {
        bus,
        mask,
        expected,
        enable,
    }
    .await;
    bus.write(CR2, 0); // never leave a level source armed
    if sr & ERROR != 0 {
        Err(Error::Bus)
    } else {
        Ok(())
    }
}

// Original diagnostic cap: 4,096 SR reads per byte, not a calibrated time bound.
// The executor cannot poll the outer Embassy timeout during this loop.
#[cfg(feature = "i2c-rx-poll-diagnostic")]
const RX_POLL_LIMIT: usize = 4096;

#[cfg(feature = "i2c-rx-poll-diagnostic")]
fn poll_receive(bus: &mut impl Registers, step: u32) -> Result<(), Error> {
    LAST_STEP.store(step, core::sync::atomic::Ordering::Relaxed);
    bus.write(CR2, 0); // no RX wake dependency; CPU interrupts remain enabled
    for _ in 0..RX_POLL_LIMIT {
        let sr = bus.read(SR);
        if sr & ERROR != 0 {
            return Err(Error::Bus);
        }
        if sr & RFULLF != 0 {
            return Ok(());
        }
    }
    Err(Error::Timeout(step))
}

async fn wait_receive(bus: &mut impl Registers, step: u32) -> Result<(), Error> {
    #[cfg(feature = "i2c-rx-poll-diagnostic")]
    {
        poll_receive(bus, step)
    }
    #[cfg(not(feature = "i2c-rx-poll-diagnostic"))]
    {
        wait(bus, RFULLF, RFULLF, WAKE_RX_FULL, step).await
    }
}

fn control(bus: &mut impl Registers, clear: u32, set: u32) {
    let value = bus.read(CR1);
    bus.write(CR1, (value & !clear) | set);
}

async fn begin(bus: &mut impl Registers) -> Result<(), Error> {
    // Same steps and same flags as the owner-confirmed polled driver: internal
    // state reset with PE untouched, wait for BUSY to clear, start, then wait
    // for BUSY|STARTF.
    control(bus, 0, RESET_STATE);
    // RM 26.5.1: internal-state reset does not reset the CR1 configuration.
    // Discard commands left pending by a cancelled/failed previous transaction.
    control(bus, RESET_STATE | NACK | START | STOP | RESTART, 0);
    wait(bus, BUSY, 0, WAKE_TRANSFER, STEP_BUS_FREE).await?;
    control(bus, 0, START);
    wait(bus, BUSY | STARTF, BUSY | STARTF, WAKE_TRANSFER, STEP_START).await
}

async fn send(bus: &mut impl Registers, byte: u8) -> Result<(), Error> {
    wait(bus, TEMPTYF, TEMPTYF, WAKE_TX_EMPTY, STEP_TX_EMPTY).await?;
    bus.transmit(byte);
    wait(bus, TENDF, TENDF, WAKE_TEND, STEP_TX_DONE).await?;
    // ACKRF set means a NACK came back for this byte.
    if bus.read(SR) & ACKRF != 0 {
        Err(Error::Bus)
    } else {
        Ok(())
    }
}

fn request_stop(bus: &mut impl Registers) {
    // DDL I2C_Stop / I2C_MasterReceiveDataAndStop clear stale STOPF first.
    bus.write(CLR, STOPFCLR);
    control(bus, 0, STOP);
}

async fn finish(bus: &mut impl Registers) -> Result<(), Error> {
    request_stop(bus);
    wait(bus, STOPF, STOPF, WAKE_STOP, STEP_STOP).await
}

use embedded_hal::i2c::Operation;

fn is_read(op: &Operation<'_>) -> bool {
    matches!(op, Operation::Read(_))
}
fn empty_read(op: &Operation<'_>) -> bool {
    matches!(op, Operation::Read(bytes) if bytes.is_empty())
}
async fn restarted(bus: &mut impl Registers) -> Result<(), Error> {
    wait(
        bus,
        BUSY | STARTF,
        BUSY | STARTF,
        WAKE_TRANSFER,
        STEP_RESTART,
    )
    .await
}
fn request_restart(bus: &mut impl Registers) {
    bus.write(CLR, STARTFCLR); // Clear STARTF before requesting the next start.
    control(bus, 0, RESTART);
}

/// One transaction, coalescing adjacent same-direction buffers. Empty reads
/// are no-ops; an empty write still performs an address-only ACK probe.
/// Read-to-write transitions are rejected before I/O (Errata 4.1.1).
async fn transfer(
    bus: &mut impl Registers,
    addr7: u8,
    ops: &mut [Operation<'_>],
) -> Result<(), Error> {
    if addr7 > MAX_ADDRESS {
        return Err(Error::Address);
    }
    let mut received = false;
    for op in ops.iter().filter(|op| !empty_read(op)) {
        if is_read(op) {
            received = true;
        } else if received {
            return Err(Error::ReadRestart);
        }
    }
    let Some(mut index) = ops.iter().position(|op| !empty_read(op)) else {
        return Ok(());
    };
    // Always mask on completion/error/cancellation, including outer timeout.
    // This does NOT promise STOP/recovery on drop; a bus can remain busy.
    struct MaskOnDrop<'a, R: Registers>(&'a mut R);
    impl<R: Registers> Drop for MaskOnDrop<'_, R> {
        fn drop(&mut self) {
            self.0.write(CR2, 0);
        }
    }
    let guard = MaskOnDrop(bus);
    let bus = &mut *guard.0;
    if ops.iter().any(|op| is_read(op) && !empty_read(op)) {
        let cr3 = bus.read(CR3);
        bus.write(CR3, cr3 & !FACKEN);
    }
    begin(bus).await?;
    let mut progress = 0usize;
    loop {
        let read = is_read(&ops[index]);
        let mut end = index + 1;
        while end < ops.len() && (empty_read(&ops[end]) || is_read(&ops[end]) == read) {
            end += 1;
        }
        let last_group = end == ops.len();
        if read {
            let mut remaining: usize = ops[index..end]
                .iter()
                .map(|op| match op {
                    Operation::Read(bytes) => bytes.len(),
                    Operation::Write(_) => 0,
                })
                .sum();
            // Normal ACK flow pipelines one byte. A one-byte read needs NACK
            // selected BEFORE the read address can start reception (RM 26.5.3).
            if remaining == 1 {
                control(bus, 0, NACK);
            }
            wait(bus, TEMPTYF, TEMPTYF, WAKE_TX_EMPTY, STEP_RX_ADDR_READY).await?;
            bus.transmit((addr7 << 1) | READ_DIRECTION);
            // TENDF invalid in receive mode, Errata 4.1.2: retain RXI wake.
            wait(bus, TRA, 0, WAKE_RX_FULL, STEP_RX_MODE).await?;
            if bus.read(SR) & ACKRF != 0 {
                return Err(Error::Bus);
            }
            let mut first = true;
            for op in &mut ops[index..end] {
                if let Operation::Read(bytes) = op {
                    for byte in bytes.iter_mut() {
                        wait_receive(bus, if first { STEP_RX_FIRST } else { STEP_RX_SECOND })
                            .await?;
                        first = false;
                        // Penultimate across ALL adjacent read buffers, not
                        // each individual slice; set before DRR releases SCL.
                        if remaining == 2 {
                            control(bus, 0, NACK);
                        }
                        if remaining == 1 {
                            // Errata 4.1.1: STOP must precede the final DRR read.
                            // Validation above ensures a read is the final group.
                            request_stop(bus);
                        }
                        *byte = bus.receive();
                        remaining -= 1;
                        progress += 1;
                        if progress.is_multiple_of(YIELD_BYTES) {
                            embassy_futures::yield_now().await;
                        }
                    }
                }
            }
            return wait(bus, STOPF, STOPF, WAKE_STOP, STEP_STOP).await;
        } else {
            send(bus, addr7 << 1).await?;
            progress += 1;
            if progress.is_multiple_of(YIELD_BYTES) {
                embassy_futures::yield_now().await;
            }
            for op in &ops[index..end] {
                if let Operation::Write(bytes) = op {
                    for &byte in *bytes {
                        send(bus, byte).await?;
                        progress += 1;
                        if progress.is_multiple_of(YIELD_BYTES) {
                            embassy_futures::yield_now().await;
                        }
                    }
                }
            }
            if last_group {
                return finish(bus).await;
            }
            request_restart(bus);
            restarted(bus).await?;
        }
        index = end;
    }
}

async fn bounded_transfer(
    bus: &mut impl Registers,
    addr7: u8,
    ops: &mut [Operation<'_>],
) -> Result<(), Error> {
    with_timeout(TIMEOUT, transfer(bus, addr7, ops))
        .await
        .map_err(|_| Error::Timeout(last_step()))?
}

/// Baud-rate register value for the caller's requested bus speed.
/// Uses CCR.CKDIV=0 (PCLK3/1) when representable, otherwise CKDIV=1 (PCLK3/2)
/// with a rounded-up period. Returns None when neither setting can represent it.
///
/// RM 26.5.13: the timing model uses an enabled one-cycle digital filter,
/// configured by the constructor. DS Table 3-24 supplies minimum low/high times;
/// the requested frequency is an upper bound, not an exact-rate guarantee.
/// Physical rise/fall times and pull-ups still require board validation.
pub fn timing(pclk: u32, frequency: u32) -> Option<u32> {
    if frequency == 0 || frequency > FAST_MAX_HZ {
        return None;
    }
    let (low_ns, high_ns) = if frequency <= STANDARD_MAX_HZ {
        STANDARD_PHASE_NS
    } else {
        FAST_PHASE_NS
    };
    for (encoding, (divisor, overhead)) in CLOCK_DIVIDERS.into_iter().enumerate() {
        let denominator = NANOSECONDS_PER_SECOND * u64::from(divisor);
        let minimum_width = |ns: u64| {
            (u64::from(pclk) * ns)
                .div_ceil(denominator)
                .saturating_sub(u64::from(overhead))
                .max(1) as u32
        };
        let min_low = minimum_width(low_ns);
        let min_high = minimum_width(high_ns);
        let Some(count) = pclk.div_ceil(frequency * divisor).checked_sub(2 * overhead) else {
            continue;
        };
        let low = (count / 2).max(min_low);
        let high = count.saturating_sub(low).max(min_high);
        if !(MIN_WIDTH_SUM..=MAX_WIDTH_SUM).contains(&(low + high))
            || low > CCR_WIDTH_MASK
            || high > CCR_WIDTH_MASK
        {
            continue;
        }
        return Some(low | (high << CCR_HIGH_SHIFT) | ((encoding as u32) << CCR_FREQ_SHIFT));
    }
    None
}

fn initialize(bus: &mut impl Registers, ccr: u32) {
    bus.write(CR1, STARTUP_RESET);
    control(bus, 0, PE);
    // Keep reserved bits at their observed reset defaults (Errata 2.7.1).
    let ccr_mask = CCR_WIDTH_MASK | (CCR_WIDTH_MASK << CCR_HIGH_SHIFT) | (0b111 << CCR_FREQ_SHIFT);
    let value = bus.read(CCR);
    bus.write(CCR, (value & !ccr_mask) | ccr);
    let filter = bus.read(FLTR);
    bus.write(
        FLTR,
        (filter & !DIGITAL_FILTER_WIDTH) | DIGITAL_FILTER_ENABLE,
    );
    for address in [SLR0, SLR1] {
        let value = bus.read(address);
        bus.write(address, value & !SLAVE_ADDRESS_ENABLE);
    }
    let cr4 = bus.read(CR4);
    bus.write(CR4, cr4 | BUSWAIT);
    bus.write(CR2, 0);
    control(bus, STARTUP_RESET, 0);
}

struct Hardware;

impl Registers for Hardware {
    fn read(&mut self, offset: usize) -> u32 {
        // SAFETY: Internal helpers supply documented, word-aligned I2C3 register offsets.
        unsafe { ((I2C3_BASE + offset) as *const u32).read_volatile() }
    }
    fn write(&mut self, offset: usize, value: u32) {
        // SAFETY: Internal helpers supply documented, writable I2C3 register offsets.
        unsafe { ((I2C3_BASE + offset) as *mut u32).write_volatile(value) }
    }
    fn receive(&mut self) -> u8 {
        // SAFETY: DRR supports byte reads and construction requires exclusive I2C3 ownership.
        unsafe { ((I2C3_BASE + DRR) as *const u8).read_volatile() }
    }
    fn transmit(&mut self, byte: u8) {
        // SAFETY: DTR supports byte writes and construction requires exclusive I2C3 ownership.
        unsafe { ((I2C3_BASE + DTR) as *mut u8).write_volatile(byte) }
    }
}

/// The async I2C3 master. Its field is private, so only [`I2c3::new`] can
/// create one.
pub struct I2c3 {
    bus: Hardware,
    _pins: [crate::gpio::AlternatePin; 2],
}

impl I2c3 {
    /// Construct the exclusive I2C3 master with caller-configured pins and speed.
    /// The application owns any shared-bus mutex and the PCLK3 frequency.
    ///
    /// # Safety
    /// Called once with exclusive I2C3 ownership, configured I2C pins/pull-ups,
    /// PWC write access and a valid PCLK3. Route events 428..431 and install
    /// handlers that call [`on_interrupt`]. Slave operation is not supported.
    pub unsafe fn new(
        pclk3: u32,
        frequency: u32,
        pins: [crate::gpio::AlternatePin; 2],
    ) -> Result<Self, Error> {
        let ccr = timing(pclk3, frequency).ok_or(Error::Clock)?;

        // PWC FCG1 controls I2C3's clock with an active-low gate at bit 6.
        const CLOCK_GATE: usize = 0x4004_8004;
        // Vendor SVD: FCG1.I2C3 (bit 6) disables the I2C3 clock when set.
        const CLOCK_BIT: u32 = 1 << 6;
        // SAFETY: FCG1 is word-aligned; the caller guarantees exclusive startup
        // configuration and PWC write access before using I2C3.
        let gate = CLOCK_GATE as *mut u32;
        unsafe { gate.write_volatile(gate.read_volatile() & !CLOCK_BIT) };

        let mut bus = Hardware;
        initialize(&mut bus, ccr);

        Ok(Self { bus, _pins: pins })
    }

    /// Read one big-endian 16-bit register with a repeated start.
    pub async fn read_register(&mut self, addr7: u8, reg8: u8) -> Result<u16, Error> {
        let mut bytes = [0; 2];
        embedded_hal_async::i2c::I2c::write_read(self, addr7, &[reg8], &mut bytes).await?;
        Ok(u16::from_be_bytes(bytes))
    }

    /// Write one big-endian 16-bit register.
    pub async fn write_register(&mut self, addr7: u8, reg8: u8, value: u16) -> Result<(), Error> {
        let [high, low] = value.to_be_bytes();
        embedded_hal_async::i2c::I2c::write(self, addr7, &[reg8, high, low]).await
    }

    /// General seven-bit transaction; one 50ms bound for the whole operation.
    /// Empty reads are skipped; empty writes are address-only ACK probes.
    /// Read-to-write repeated starts return [`Error::ReadRestart`] before I/O.
    /// Cancellation masks IRQ sources but does not guarantee STOP/recovery.
    pub async fn transaction(
        &mut self,
        addr7: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        bounded_transfer(&mut self.bus, addr7, operations).await
    }

    /// Bring-up self-test for the wake path: arm the "transmit buffer empty"
    /// source, which is asserted while the bus is idle, and return.
    ///
    /// If the interrupt handler then runs, `SEL`/`NVIC` routing for the I2C3
    /// lines is proven *independently of any transaction* — which separates "the
    /// routing is wrong" from "the transaction is wrong". The handler masks the
    /// source, so this cannot storm.
    pub fn wake_self_test(&mut self) {
        self.bus.write(CR2, TEMPTYIE);
    }

    /// Current `CR1`, for diagnostics after a failure.
    pub fn control(&mut self) -> u32 {
        self.bus.read(CR1)
    }

    /// Current `CR4`, for diagnostics after a failure (it carries `BUSWAIT`).
    pub fn control4(&mut self) -> u32 {
        self.bus.read(CR4)
    }

    /// Current `SR`, for diagnostics after a failure (the caller still holds
    /// the bus lock, so nothing else can change it underneath).
    pub fn status(&mut self) -> u32 {
        self.bus.read(SR)
    }

    /// Write a single data byte (LIS3DH-style register write).
    pub async fn write_register8(&mut self, addr7: u8, reg8: u8, value: u8) -> Result<(), Error> {
        embedded_hal_async::i2c::I2c::write(self, addr7, &[reg8, value]).await
    }
}

impl embedded_hal::i2c::ErrorType for I2c3 {
    type Error = Error;
}

#[cfg(test)]
mod tests;

impl embedded_hal_async::i2c::I2c for I2c3 {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        I2c3::transaction(self, address, operations).await
    }
}
