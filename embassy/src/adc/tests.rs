//! An empty scan must fail before clocking/configuring ADC, rather than hang.
use super::*;

#[test]
fn empty_scan_is_rejected_before_hardware_access() {
    // SAFETY: no device exists on this host and the invalid empty configuration
    // is rejected before MMIO. The unused timeout never schedules a timer.
    let result = unsafe { Adc2::new(pac::Adc2::steal(), [], None, Duration::from_ticks(0)) };
    assert!(matches!(result, Err(Error::InvalidScan)));
}
