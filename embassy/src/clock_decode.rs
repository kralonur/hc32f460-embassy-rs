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
pub const XTAL32_HZ: u32 = 32_768;

/// Documented PLL input (PFD) range in Hz (RM section 4.11.15).
pub const PLL_PFD_MIN_HZ: u32 = 1_000_000;
pub const PLL_PFD_MAX_HZ: u32 = 25_000_000;
/// Documented MPLL VCO range in Hz (RM section 4.11.15).
pub const PLL_VCO_MIN_HZ: u32 = 240_000_000;
pub const PLL_VCO_MAX_HZ: u32 = 480_000_000;

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
    /// Compact code for the on-target status record (0 means no error).
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
    match cksw & 0x7 {
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
    match field & 0x7 {
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
    let p_field = (raw >> 28) & 0xf;
    if p_field == 0 {
        return Err(ClockError::ProhibitedPllDivider);
    }
    Ok(Pll {
        source_hrc: (raw >> 7) & 1 != 0,
        m: (raw & 0x1f) + 1,
        n: ((raw >> 8) & 0x1ff) + 1,
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
    let hclk_hz = system_hz / divide_from_scfgr((scfgr_raw >> 24) & 0x7)?;
    let mut pclk_hz = [0u32; 5];
    for (index, shift) in [0u32, 4, 8, 12, 16].into_iter().enumerate() {
        pclk_hz[index] = system_hz / divide_from_scfgr((scfgr_raw >> shift) & 0x7)?;
    }
    Ok(ClockState {
        source,
        system_hz,
        hclk_hz,
        pclk_hz,
        pll_plausible,
    })
}

/// HRC frequency from the readback bit the stock application uses at `0x24648`.
///
/// The reference manual documents `ICG1.HRCFREQSEL` (`0` = 20 MHz, `1` = 16 MHz) but
/// not this readback address, so the mapping is inferred from stock behaviour and
/// recorded as an assumption rather than a documented fact.
pub fn hrc_hz_from_readback_bit(bit: bool) -> u32 {
    if bit { HRC_16MHZ } else { HRC_20MHZ }
}
