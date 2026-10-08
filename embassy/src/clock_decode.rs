//! HC32F460 clock-state decoding from documented CMU registers.
//!
//! Decode caller-observed CMU registers without assuming an inherited clock.
//! Authority: RM Rev1.5 sections 4.11.15, 4.11.20, 4.11.21.
//!
//! Two inputs are assumptions, not measurements, and are passed in by the caller:
//! the board XTAL frequency and the HRC frequency (16 or 20 MHz,
//! selected by `ICG1.HRCFREQSEL` in flash sector 0, which the download does not
//! contain).

/// HRC selection: `ICG1.HRCFREQSEL = 0` (RM section 6.2.2).
pub const HRC_20MHZ: u32 = 20_000_000;
/// HRC selection: `ICG1.HRCFREQSEL = 1` (RM section 6.2.2).
pub const HRC_16MHZ: u32 = 16_000_000;
/// Internal medium-speed RC, typical value per datasheet Rev1.61.
pub const MRC_HZ: u32 = 8_000_000;
/// Internal low-speed RC and the 32.768 kHz crystal share a nominal rate.
pub const LRC_HZ: u32 = 32_768;
/// Nominal external low-speed crystal frequency in Hz (RM 4.11.20).
pub const XTAL32_HZ: u32 = 32_768;

/// Documented PLL input (PFD) range in Hz (RM section 4.11.15).
pub const PLL_PFD_MIN_HZ: u32 = 1_000_000;
/// Maximum documented PLL input (PFD) frequency in Hz (RM 4.11.15).
pub const PLL_PFD_MAX_HZ: u32 = 25_000_000;
/// Documented MPLL VCO range in Hz (RM section 4.11.15).
pub const PLL_VCO_MIN_HZ: u32 = 240_000_000;
/// Maximum documented MPLL VCO frequency in Hz (RM 4.11.15).
pub const PLL_VCO_MAX_HZ: u32 = 480_000_000;

// Vendor SVD: CKSW and each SCFGR divider selector occupy three bits.
pub(crate) const SELECTOR_MASK: u32 = 0b111;
// Vendor SVD: PLLCFGR.MPLLM occupies bits 4:0.
pub(crate) const PLL_M_MASK: u32 = 0x1f;
// Vendor SVD: PLLCFGR.PLLSRC (bit 7) selects HRC rather than XTAL.
pub(crate) const PLL_SOURCE_HRC: u32 = 1 << 7;
// Vendor SVD: PLLCFGR.MPLLN begins at bit 8.
pub(crate) const PLL_N_SHIFT: u32 = 8;
// Vendor SVD: PLLCFGR.MPLLN is nine bits wide.
pub(crate) const PLL_N_MASK: u32 = 0x1ff;
// Vendor SVD: PLLCFGR.MPLLP begins at bit 28.
pub(crate) const PLL_P_SHIFT: u32 = 28;
// Vendor SVD: PLLCFGR.MPLLP is four bits wide.
pub(crate) const PLL_P_MASK: u32 = 0xf;
// Vendor SVD: SCFGR.HCLKS occupies bits 26:24.
pub(crate) const HCLK_SHIFT: u32 = 24;
// Vendor SVD: SCFGR PCLK0S..PCLK4S start at these bit positions, in clock order.
pub(crate) const PCLK_SHIFTS: [u32; 5] = [0, 4, 8, 12, 16];
// RM 4.11.15: MPLLQ and MPLLR are four-bit output dividers like MPLLP.
pub(crate) const PLL_Q_SHIFT: u32 = 24;
// RM 4.11.15: MPLLR begins at bit 20.
pub(crate) const PLL_R_SHIFT: u32 = 20;
// RM 4.11.21: external-bus divider EXCKS starts at bit 20.
pub(crate) const EXCLK_SHIFT: u32 = 20;
// RM 4.11.15: writable MPLL factors and the shared input-source selector.
pub(crate) const PLL_CONFIG_MASK: u32 = (PLL_P_MASK << PLL_P_SHIFT)
    | (PLL_P_MASK << PLL_Q_SHIFT)
    | (PLL_P_MASK << PLL_R_SHIFT)
    | (PLL_N_MASK << PLL_N_SHIFT)
    | PLL_M_MASK
    | PLL_SOURCE_HRC;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockError {
    /// `CKSW[2:0]` was 6 or 7, which the manual marks as prohibited.
    ProhibitedSource(u32),
    /// A divider selector was 7, which the manual marks as prohibited.
    ProhibitedDivide,
    /// `MPLLP[3:0]` was 0, which the manual marks as prohibited.
    ProhibitedPllDivider,
}

impl ClockError {
    /// Legacy on-target status code: 1 = source, 2 = divider, 3 = PLL divider.
    /// Zero is reserved for no error; preserve these values for existing consumers.
    pub fn code(self) -> u8 {
        match self {
            ClockError::ProhibitedSource(_) => 1,
            ClockError::ProhibitedDivide => 2,
            ClockError::ProhibitedPllDivider => 3,
        }
    }
}

/// System clock source selected by `CMU_CKSWR.CKSW[2:0]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Hrc,
    Mrc,
    Lrc,
    Xtal,
    Xtal32,
    Mpll,
}

/// Decoded `CMU_PLLCFGR` fields. `m`, `n`, and `p` are true division and
/// multiplication factors (the register stores them minus one).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pll {
    /// `PLLSRC`: false selects XTAL, true selects HRC.
    pub source_hrc: bool,
    pub m: u32,
    pub n: u32,
    pub p: u32,
}

/// Derived clock tree. `pll_plausible` is false when a computed MPLL PFD input or
/// VCO frequency falls outside the documented range; the frequencies are still
/// reported so a diagnostic can show the raw result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockState {
    pub source: Source,
    pub system_hz: u32,
    pub hclk_hz: u32,
    /// PCLK0 through PCLK4, in that order.
    pub pclk_hz: [u32; 5],
    pub pll_plausible: bool,
}

pub fn source_from_cksw(cksw: u32) -> Result<Source, ClockError> {
    match cksw & SELECTOR_MASK {
        0 => Ok(Source::Hrc),
        1 => Ok(Source::Mrc),
        2 => Ok(Source::Lrc),
        3 => Ok(Source::Xtal),
        4 => Ok(Source::Xtal32),
        5 => Ok(Source::Mpll),
        other => Err(ClockError::ProhibitedSource(other)),
    }
}

/// `CMU_SCFGR` divider selector: `000`..`110` mean divide by 1, 2, 4, 8, 16, 32, 64.
pub fn divide_from_scfgr(field: u32) -> Result<u32, ClockError> {
    match field & SELECTOR_MASK {
        0 => Ok(1),
        1 => Ok(2),
        2 => Ok(4),
        3 => Ok(8),
        4 => Ok(16),
        5 => Ok(32),
        6 => Ok(64),
        _ => Err(ClockError::ProhibitedDivide),
    }
}

/// Decode `CMU_PLLCFGR` (0x40054100) into true factors.
pub fn pll_from_pllcfgr(raw: u32) -> Result<Pll, ClockError> {
    let p_field = (raw >> PLL_P_SHIFT) & PLL_P_MASK;
    if p_field == 0 {
        return Err(ClockError::ProhibitedPllDivider);
    }
    Ok(Pll {
        source_hrc: raw & PLL_SOURCE_HRC != 0,
        m: (raw & PLL_M_MASK) + 1,
        n: ((raw >> PLL_N_SHIFT) & PLL_N_MASK) + 1,
        p: p_field + 1,
    })
}

/// Derive the clock tree from raw register values.
///
/// `cksw_raw` is `CMU_CKSWR` (0x40054026), `pllcfgr_raw` is `CMU_PLLCFGR`
/// (0x40054100), and `scfgr_raw` is `CMU_SCFGR` (0x40054020).
pub fn state_from_registers(
    cksw_raw: u32,
    pllcfgr_raw: u32,
    scfgr_raw: u32,
    hrc_hz: u32,
    xtal_hz: u32,
) -> Result<ClockState, ClockError> {
    let source = source_from_cksw(cksw_raw)?;
    let mut pll_plausible = true;
    let system_hz = match source {
        Source::Hrc => hrc_hz,
        Source::Mrc => MRC_HZ,
        Source::Lrc => LRC_HZ,
        Source::Xtal => xtal_hz,
        Source::Xtal32 => XTAL32_HZ,
        Source::Mpll => {
            let pll = pll_from_pllcfgr(pllcfgr_raw)?;
            let input = if pll.source_hrc { hrc_hz } else { xtal_hz };
            let pfd = input / pll.m;
            let vco = pfd * pll.n;
            pll_plausible = (PLL_PFD_MIN_HZ..=PLL_PFD_MAX_HZ).contains(&pfd)
                && (PLL_VCO_MIN_HZ..=PLL_VCO_MAX_HZ).contains(&vco);
            vco / pll.p
        }
    };
    let hclk_hz = system_hz / divide_from_scfgr((scfgr_raw >> HCLK_SHIFT) & SELECTOR_MASK)?;
    let mut pclk_hz = [0u32; PCLK_SHIFTS.len()];
    for (index, shift) in PCLK_SHIFTS.into_iter().enumerate() {
        pclk_hz[index] = system_hz / divide_from_scfgr((scfgr_raw >> shift) & SELECTOR_MASK)?;
    }
    Ok(ClockState {
        source,
        system_hz,
        hclk_hz,
        pclk_hz,
        pll_plausible,
    })
}

/// Nominal HRC frequency from the vendor DDL's frequency-monitor selector.
///
/// RM 6.2.2 documents `ICG1.HRCFREQSEL` (`0` = 20 MHz, `1` = 16 MHz).
/// DDL Rev3.3.0 `system_hc32f460.c::SystemCoreClockUpdate` uses the same
/// mapping for `HRC_FREQ_MON()`. Vendor reference links are in [`crate::clocks`].
pub fn hrc_hz_from_readback_bit(bit: bool) -> u32 {
    if bit { HRC_16MHZ } else { HRC_20MHZ }
}
