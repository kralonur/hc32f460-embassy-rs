//! Protect exact time units and independent timer deadlines when caching alarms.
extern crate std;
use super::*;
use embassy_time_driver::Driver;
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Wake, Waker},
};

#[derive(Default)]
struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn reload_refuses_silent_clock_rounding() {
    assert_eq!(reload(200_000_000), Ok(199_999));
    assert_eq!(reload(2_000), Ok(1));
    for hclk in [0, 999, 1_000, 1_999, 200_000_001, u32::MAX] {
        assert_eq!(reload(hclk), Err(InitError::Clock));
    }
}

#[test]
fn separate_deadlines_wake_once_and_idle_ticks_leave_the_queue_alone() {
    let driver = SystickTimeDriver::new();
    let first = Arc::new(Counter::default());
    let second = Arc::new(Counter::default());
    driver.schedule_wake(2, &Waker::from(first.clone()));
    driver.schedule_wake(4, &Waker::from(second.clone()));
    for _ in 0..3 {
        driver.on_systick();
    }
    assert_eq!(first.0.load(Ordering::Relaxed), 1);
    assert_eq!(second.0.load(Ordering::Relaxed), 0);
    driver.on_systick();
    assert_eq!(second.0.load(Ordering::Relaxed), 1);
    for _ in 0..10 {
        driver.on_systick();
    }
    assert_eq!(driver.now(), 14);
    assert_eq!(first.0.load(Ordering::Relaxed), 1);
    assert_eq!(second.0.load(Ordering::Relaxed), 1);
    driver
        .state
        .lock(|cell| assert_eq!(cell.borrow().next_alarm, u64::MAX));
}

#[test]
fn earlier_rescheduling_and_expired_deadlines_update_cached_alarm() {
    let driver = SystickTimeDriver::new();
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    driver.schedule_wake(10, &waker);
    driver.schedule_wake(2, &waker);
    driver.on_systick();
    driver.on_systick();
    assert_eq!(counter.0.load(Ordering::Relaxed), 1);
    driver.schedule_wake(1, &waker);
    assert_eq!(counter.0.load(Ordering::Relaxed), 2);
}
