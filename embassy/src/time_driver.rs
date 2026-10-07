//! SysTick-backed Embassy time driver for HC32F460.
//!
//! The first hardware build stored only a single pending alarm. With two tasks
//! sleeping on different deadlines, the later `schedule_wake` replaced the
//! earlier one, so the earlier task was never woken again (the meter screen
//! froze after the first frame). This version keeps a real timer queue
//! (`embassy-time-queue-utils`, 16 slots), which is the implementation the
//! `embassy-time-driver` documentation recommends.

use core::cell::RefCell;

use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_queue_utils::Queue;

struct State {
    ticks: u64,
    queue: Queue,
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
            state.queue.next_expiration(now);
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
            state.queue.schedule_wake(at, waker);
            // A deadline that has already passed must wake immediately.
            let now = state.ticks;
            state.queue.next_expiration(now);
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: SystickTimeDriver = SystickTimeDriver::new());

/// Hook called from SysTick interrupt handler
#[inline]
pub fn systick_tick() {
    DRIVER.on_systick();
}
