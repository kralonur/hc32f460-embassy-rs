//! Protect register validity and exact-ratio decoding at the public API boundary.
use super::*;

#[test]
fn prohibited_mpll_input_dividers_are_rejected() {
    assert!(pll_from_pllcfgr((1 << PLL_P_SHIFT) | 23).is_ok());
    for encoding in 24..=31 {
        assert_eq!(
            pll_from_pllcfgr((1 << PLL_P_SHIFT) | encoding),
            Err(ClockError::ProhibitedPllInputDivider)
        );
    }
}

#[test]
fn fractional_pfd_is_not_truncated_before_multiplication() {
    // HRC / 3 * 36 / 2 = exactly 120 MHz, with a non-integer PFD.
    let pll = PLL_SOURCE_HRC | (1 << PLL_P_SHIFT) | (35 << PLL_N_SHIFT) | 2;
    let state = state_from_registers(5, pll, 0, HRC_20MHZ, 0).unwrap();
    assert_eq!(state.system_hz, 120_000_000);
    assert!(state.pll_plausible);
}

#[test]
fn fractional_vco_above_limit_is_not_rounded_into_range() {
    // Input * 72 / 3 = 480,000,024 Hz, just above the VCO limit.
    let pll = PLL_SOURCE_HRC | (1 << PLL_P_SHIFT) | (71 << PLL_N_SHIFT) | 2;
    let state = state_from_registers(5, pll, 0, HRC_20MHZ + 1, 0).unwrap();
    assert!(!state.pll_plausible);
    assert_eq!(state.system_hz, 240_000_012);
}

#[test]
fn unrepresentable_clock_is_rejected() {
    // A prohibited-frequency profile can exceed u32 even with valid field encodings.
    let pll = PLL_SOURCE_HRC | (1 << PLL_P_SHIFT) | (479 << PLL_N_SHIFT);
    assert_eq!(
        state_from_registers(5, pll, 0, HRC_20MHZ, 0),
        Err(ClockError::FrequencyOverflow)
    );
}
