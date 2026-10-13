//! SysTick-backed Embassy time driver for HC32F460.
//!
//! The first hardware build stored only a single pending alarm. With two tasks
//! sleeping on different deadlines, the later `schedule_wake` replaced the
//! earlier one, so the earlier task was never woken again (the meter screen
//! froze after the first frame). This version keeps a real timer queue
//! (`embassy-time-queue-utils`, 16 slots), which is the implementation the
//! `embassy-time-driver` documentation recommends.

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};

use cortex_m::peripheral::{SYST, syst::SystClkSource};
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_queue_utils::Queue;

// This driver's tick unit is milliseconds, matching embassy-time/tick-hz-1_000.
const TICK_HZ: u32 = 1_000;
// ARMv7-M SysTick reload is 24 bits and stores the cycle count minus one.
const MAX_CYCLES: u32 = 1 << 24;
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum InitError {
    #[error("HCLK cannot produce an exact 1 kHz tick within the SysTick reload range")]
    Clock,
    #[error("the SysTick time driver has already been initialized")]
    AlreadyInitialized,
}

fn reload(hclk: u32) -> Result<u32, InitError> {
    // LOAD=0 never transitions from one to zero and cannot generate a tick.
    if hclk / TICK_HZ < 2 || !hclk.is_multiple_of(TICK_HZ) || hclk / TICK_HZ > MAX_CYCLES {
        return Err(InitError::Clock);
    }
    Ok(hclk / TICK_HZ - 1)
}

/// Transfer SysTick to the time driver, using the board's actual HCLK in Hz.
/// Forward the SysTick exception to [`systick_tick`]. HCLK must remain unchanged,
/// and interrupts must not be masked for a whole millisecond: missed SysTick
/// interrupts cannot be reconstructed. No stop-mode timekeeping is provided.
pub fn init(mut systick: SYST, hclk: u32) -> Result<(), InitError> {
    let reload = reload(hclk)?;
    INITIALIZED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| InitError::AlreadyInitialized)?;
    critical_section::with(|_| {
        // Stop an inherited timer before replacing its reload/current state.
        systick.disable_counter();
        systick.disable_interrupt();
        cortex_m::peripheral::SCB::clear_pendst();
        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(reload);
        systick.clear_current();
        systick.enable_interrupt();
        systick.enable_counter();
    });
    Ok(())
}

struct State {
    ticks: u64,
    queue: Queue,
    next_alarm: u64,
}

pub struct SystickTimeDriver {
    state: Mutex<CriticalSectionRawMutex, RefCell<State>>,
}

impl Default for SystickTimeDriver {
    fn default() -> Self {
        Self::new()
    }
}
impl SystickTimeDriver {
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(RefCell::new(State {
                ticks: 0,
                queue: Queue::new(),
                next_alarm: u64::MAX,
            })),
        }
    }

    /// Advance time by one millisecond tick and wake every expired timer.
    ///
    /// Called from the SysTick interrupt; `next_expiration` wakes wakers, which
    /// only set an executor flag and `sev`, so it is safe inside the critical
    /// section used by the blocking mutex.
    pub fn on_systick(&self) {
        self.state.lock(|cell| {
            let mut state = cell.borrow_mut();
            state.ticks = state.ticks.wrapping_add(1);
            let now = state.ticks;
            if now >= state.next_alarm {
                state.next_alarm = state.queue.next_expiration(now);
            }
        });
    }
}

impl embassy_time_driver::Driver for SystickTimeDriver {
    fn now(&self) -> u64 {
        self.state.lock(|cell| cell.borrow().ticks)
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        self.state.lock(|cell| {
            let mut state = cell.borrow_mut();
            if state.queue.schedule_wake(at, waker) {
                // Expired deadlines wake immediately; otherwise cache the next
                // one so idle ticks do not scan the whole queue.
                let now = state.ticks;
                state.next_alarm = state.queue.next_expiration(now);
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: SystickTimeDriver = SystickTimeDriver::new());

/// Hook called from SysTick interrupt handler
#[inline]
pub fn systick_tick() {
    DRIVER.on_systick();
}

#[cfg(test)]
mod tests;
