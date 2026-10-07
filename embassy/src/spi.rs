//! Interrupt-driven, transmit-only SPI3 for the existing display setup.
//!
//! 8-bit mode 3, MSB first, PCLK1/2, existing frame spacing and reserved CFG1
//! bit 4 retained. TX-empty (310), idle (311), error (312) are CPU IRQ sources;
//! SPTEND (313) is only an event trigger (RM Table 10-2 / §27.10).
//! No DMA or borrowed buffer pointers are accessed by the ISR.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_time::{Duration, with_timeout};

// Original driver's NVIC assignments within SPI3's group-9 routing window (86..=91).
const IRQ_ROUTES: [(u16, u8); 3] = [(310, 86), (311, 87), (312, 88)];
// Original driver's NVIC priority on the HC32F460's four-bit priority scale.
const IRQ_PRIORITY: u8 = 8;
// Original driver's whole-operation bound, in milliseconds.
const WRITE_TIMEOUT: Duration = Duration::from_millis(250);
// Original driver's cooperative scheduling interval, in transmitted bytes.
const YIELD_BYTES: usize = 64;
// Vendor SVD / RM 27.12.1: SPI data register byte offset.
const DR: usize = 0x00;
// Vendor SVD / RM 27.12.2: SPI control register byte offset.
const CR1: usize = 0x04;
// Vendor SVD / RM 27.12.3: first SPI configuration register byte offset.
const CFG1: usize = 0x0c;
// Vendor SVD / RM 27.12.4: SPI status register byte offset.
const SR: usize = 0x14;
// Vendor SVD / RM 27.12.5: second SPI configuration register byte offset.
const CFG2: usize = 0x18;
// Vendor SVD: SR.OVRERF (bit 0) reports receive overrun.
const OVERRUN: u32 = 1 << 0;
// Vendor SVD: SR.MODFERF (bit 2) reports a mode fault.
const MODE_FAULT: u32 = 1 << 2;
// Vendor SVD: SR.PERF (bit 3) reports a parity error.
const PARITY_ERROR: u32 = 1 << 3;
// Vendor SVD: SR.UDRERF (bit 4) reports transmit underrun.
const UNDERRUN: u32 = 1 << 4;
// Original driver's error set, composed from the SVD's status flags (formerly 0x1D).
const ERRORS: u32 = OVERRUN | MODE_FAULT | PARITY_ERROR | UNDERRUN;
// Vendor SVD: SR.IDLNF (bit 1) is set while SPI is not idle.
const BUSY: u32 = 1 << 1;
// Vendor SVD: SR.TDEF (bit 5) indicates the transmit buffer is empty.
const EMPTY: u32 = 1 << 5;
// Vendor SVD: CR1.EIE (bit 8) enables error interrupts.
const EIE: u32 = 1 << 8;
// Vendor SVD: CR1.TXIE (bit 9) enables transmit-empty interrupts.
const TXIE: u32 = 1 << 9;
// Vendor SVD: CR1.RXIE (bit 10) enables receive-full interrupts.
const RXIE: u32 = 1 << 10;
// Vendor SVD: CR1.IDIE (bit 11) enables idle interrupts.
const IDIE: u32 = 1 << 11;
// Mask of all SPI interrupt enables owned by this driver.
const INTERRUPTS: u32 = EIE | TXIE | RXIE | IDIE;
static WAKER: AtomicWaker = AtomicWaker::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Bus(u32),
    Timeout,
    Faulted,
    InterruptRouting(crate::intc::IrqError),
}

trait Registers {
    fn read(&mut self, offset: usize) -> u32;
    fn write(&mut self, offset: usize, value: u32);
}

struct Mmio;
impl Registers for Mmio {
    fn read(&mut self, offset: usize) -> u32 {
        // SAFETY: Internal helpers supply documented word-aligned SPI3 register offsets.
        unsafe { (crate::pac::Spi3::ptr().cast::<u8>().add(offset) as *const u32).read_volatile() }
    }
    fn write(&mut self, offset: usize, value: u32) {
        // SAFETY: Internal helpers supply writable SPI3 offsets; construction
        // requires exclusive peripheral ownership.
        unsafe {
            (crate::pac::Spi3::ptr().cast::<u8>().add(offset) as *mut u32).write_volatile(value)
        }
    }
}

/// Caller-supplied HC32 SPI register configuration; board/device mode and rate
/// choices are not inferred by the driver. Interrupt sources are driver-owned.
#[derive(Clone, Copy)]
pub struct Config {
    pub cfg1: u32,
    pub cfg2: u32,
    pub control: u32,
}
fn initialize(bus: &mut impl Registers, config: Config) {
    bus.write(CR1, 0);
    bus.write(CFG1, config.cfg1);
    bus.write(CFG2, config.cfg2);
    bus.write(CR1, config.control & !INTERRUPTS);
}

fn arm(bus: &mut impl Registers, enable: u32) {
    critical_section::with(|_| {
        let control = bus.read(CR1);
        bus.write(CR1, (control & !INTERRUPTS) | enable);
    });
}

fn handle_interrupt(bus: &mut impl Registers) {
    arm(bus, 0);
    WAKER.wake();
}

/// Mask level sources before waking; the future rechecks SR and rearms.
pub fn on_interrupt() {
    handle_interrupt(&mut Mmio);
}

#[derive(Clone, Copy)]
enum Condition {
    Empty,
    Idle,
}
impl Condition {
    fn ready(self, status: u32) -> bool {
        status & EMPTY != 0 && (matches!(self, Self::Empty) || status & BUSY == 0)
    }
    fn enable(self) -> u32 {
        EIE | match self {
            Self::Empty => TXIE,
            Self::Idle => IDIE,
        }
    }
}

struct Wait<'a, R: Registers> {
    bus: &'a mut R,
    condition: Condition,
    armed: bool,
}
impl<'a, R: Registers> Wait<'a, R> {
    fn new(bus: &'a mut R, condition: Condition) -> Self {
        Self {
            bus,
            condition,
            armed: false,
        }
    }
}
impl<R: Registers + Unpin> Future for Wait<'_, R> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        // An immediately-ready wait needs neither a waker nor CR1 writes.
        // If not ready, register before arming and recheck the flag/enable race.
        for pass in 0..2 {
            let status = this.bus.read(SR);
            if status & ERRORS != 0 {
                return Poll::Ready(Err(Error::Bus(status)));
            }
            if this.condition.ready(status) {
                return Poll::Ready(Ok(()));
            }
            if pass == 0 {
                WAKER.register(cx.waker());
                this.armed = true;
                arm(this.bus, this.condition.enable());
            }
        }
        Poll::Pending
    }
}
impl<R: Registers> Drop for Wait<'_, R> {
    fn drop(&mut self) {
        if self.armed {
            arm(self.bus, 0);
        }
    }
}

async fn write(bus: &mut (impl Registers + Unpin), bytes: &[u8]) -> Result<(), Error> {
    for (index, &byte) in bytes.iter().enumerate() {
        Wait::new(bus, Condition::Empty).await?;
        bus.write(DR, u32::from(byte));
        // Even if hardware is always ready, cap work before giving the executor
        // a turn. This is cooperative fairness, not a status-polling fallback.
        if (index + 1) % YIELD_BYTES == 0 {
            embassy_futures::yield_now().await;
        }
    }
    // Buffer empty is not wire idle. Fence the last physical byte too.
    Wait::new(bus, Condition::Idle).await
}

/// Exclusive SPI3 owner. Configure pins before construction; IRQ vectors must
/// call [`on_interrupt`] on INT086/087/088. Failures latch this driver off.
pub struct Spi3 {
    _peripheral: crate::pac::Spi3,
    bus: Mmio,
    failed: bool,
}
impl Spi3 {
    /// Initialize the proven display settings and register the three IRQ routes.
    ///
    /// # Safety
    /// Caller must exclusively own SPI3 and its pins; clocks/PWC write access
    /// must be ready, inherited display DMA stopped, and INTC routes available.
    pub unsafe fn new(peripheral: crate::pac::Spi3, config: Config) -> Result<Self, Error> {
        critical_section::with(|_| {
            // PWC FCG1 controls SPI3's clock with an active-low gate at bit 18.
            const CLOCK_GATE: usize = 0x4004_8004;
            // Vendor SVD: FCG1.SPI3 (bit 18) disables the SPI3 clock when set.
            const CLOCK_BIT: u32 = 1 << 18;
            // SAFETY: FCG1 is word-aligned and the caller guarantees PWC write
            // access. The critical section serializes this read-modify-write.
            let gate = CLOCK_GATE as *mut u32;
            unsafe { gate.write_volatile(gate.read_volatile() & !CLOCK_BIT) };
        });
        let mut bus = Mmio;
        initialize(&mut bus, config);
        for (index, &(event, line)) in IRQ_ROUTES.iter().enumerate() {
            // Invariant: IRQ_ROUTES contains only valid HC32F460 NVIC line numbers.
            let line = crate::intc::Line::new(line)
                .expect("fixed SPI3 interrupt route must use a valid NVIC line");
            // SAFETY: The caller owns SPI3 and the installed handlers; these are
            // its documented event sources and the original driver's routes.
            let result = unsafe { crate::intc::register(event, line, IRQ_PRIORITY) };
            if let Err(error) = result {
                for &(_, claimed) in &IRQ_ROUTES[..index] {
                    // Invariant: Every rollback line comes from the same fixed IRQ_ROUTES.
                    let line = crate::intc::Line::new(claimed)
                        .expect("fixed SPI3 rollback route must use a valid NVIC line");
                    crate::intc::unregister(line);
                }
                return Err(Error::InterruptRouting(error));
            }
        }
        Ok(Self {
            _peripheral: peripheral,
            bus,
            failed: false,
        })
    }

    /// Send bytes and wait for physical idle. Waiting uses IRQs, not busy loops.
    /// Each call is bounded to 250 ms; timeout/bus errors prevent further traffic.
    /// Cancellation masks interrupt sources; already-copied DR data may finish
    /// shifting, so a caller must still fence idle before changing D/C or CS.
    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if self.failed {
            return Err(Error::Faulted);
        }
        let result = with_timeout(WRITE_TIMEOUT, write(&mut self.bus, bytes))
            .await
            .unwrap_or(Err(Error::Timeout));
        if result.is_err() {
            self.failed = true;
        }
        result
    }

    /// Fence the wire before changing a control pin (also after cancellation).
    pub async fn flush(&mut self) -> Result<(), Error> {
        self.write(&[]).await
    }
}
impl Drop for Spi3 {
    fn drop(&mut self) {
        arm(&mut self.bus, 0);
    }
}
