//! HRC/MPLL startup and nominal clock frequencies for peripheral drivers.
//!
//! Register definitions follow RM 4.7, 4.8, 4.11 and 7.4; clock constraints
//! follow RM-ZH 4.4, and reserved SCFGR bits follow Errata 2.1.4.
//! HRC detection and clock switching are cross-checked against DDL
//! `system_hc32f460.c` and `hc32_ll_clk.c`. No vendor code is embedded here.

use crate::clock_decode as decode;

mod startup;

/// Clock configuration/readback failures. Failure does not retry or restore hardware.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("PLL factors are outside their register ranges")]
    PllFactors,
    #[error("PLL input frequency is outside the documented PFD range")]
    PllInput,
    #[error("PLL VCO frequency is outside the documented range")]
    PllVco,
    #[error("system clock exceeds 200 MHz")]
    SystemFrequency,
    #[error("bus frequencies or divider relationships violate the clock specification")]
    BusClocks,
    #[error("entry clock source {0} is prohibited")]
    EntrySource(u8),
    #[error("UPLL is active or stopping; its shared source cannot be changed")]
    SharedPllSource,
    #[error("clock readiness timeout: {0:?}")]
    Timeout(Ready),
    #[error("clock register readback mismatch: {0:?}")]
    Readback(Register),
}

/// Oscillator state awaited during startup.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ready {
    HrcStable,
    MpllStopped,
    MpllStable,
}

/// Configuration register that failed readback.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    AdTrng,
    FlashReadMode,
    Source,
    Dividers,
    Pll,
}

/// SCFGR selector encodings: RM 4.11.21 maps 0..=6 to /1 through /64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Divider {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
    Div16 = 4,
    Div32 = 5,
    Div64 = 6,
}

/// Bus dividers relative to SYSCLK, not relative to HCLK.
#[derive(Debug, Clone, Copy)]
pub struct Dividers {
    pub hclk: Divider,
    pub exclk: Divider,
    /// Peripheral-clock domains PCLK0 through PCLK4, in that order.
    pub pclk: [Divider; 5],
}

/// Validated MPLL coefficient encodings; frequency limits depend on detected HRC.
#[derive(Debug, Clone, Copy)]
pub struct Mpll {
    input_divider: u8,
    multiplier: u16,
    output_dividers: [u8; 3],
}

impl Mpll {
    /// True factors, not register encodings. Output dividers are [P, Q, R].
    pub fn new(
        input_divider: u8,
        multiplier: u16,
        output_dividers: [u8; 3],
    ) -> Result<Self, Error> {
        // RM 4.11.15: M=1..24, N=20..480 and P/Q/R=2..16, stored minus one.
        // RM 4.11.15: minimum permitted VCO multiplication factor.
        const MULTIPLIER_MIN: u16 = 20;
        // RM 4.11.15: maximum permitted VCO multiplication factor.
        const MULTIPLIER_MAX: u16 = 480;
        // RM 4.11.15 prohibits output-divider encoding zero (true factor one).
        const OUTPUT_MIN: u8 = 2;
        // RM 4.11.15: four-bit output encoding permits a true factor of sixteen.
        const OUTPUT_MAX: u8 = 16;
        if !(1..=decode::PLL_INPUT_MAX_FACTOR).contains(&input_divider)
            || !(MULTIPLIER_MIN..=MULTIPLIER_MAX).contains(&multiplier)
            || output_dividers
                .iter()
                .any(|v| !(OUTPUT_MIN..=OUTPUT_MAX).contains(v))
        {
            return Err(Error::PllFactors);
        }
        Ok(Self {
            input_divider,
            multiplier,
            output_dividers,
        })
    }

    fn bits(self) -> u32 {
        let [p, q, r] = self.output_dividers.map(|v| u32::from(v - 1));
        (p << decode::PLL_P_SHIFT)
            | (q << decode::PLL_Q_SHIFT)
            | (r << decode::PLL_R_SHIFT)
            | (u32::from(self.multiplier - 1) << decode::PLL_N_SHIFT)
            | u32::from(self.input_divider - 1)
            | decode::PLL_SOURCE_HRC
    }
}

/// Requested HRC-backed MPLL profile. AD/TRNG select their bus clocks.
/// Frequency limits are checked against detected HRC before any register write.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub mpll: Mpll,
    pub dividers: Dividers,
}

// RM 4.4, 4.7: maximum SYSCLK/HCLK and PCLK0 frequency, in Hz.
const MAX_SYSTEM_HZ: u32 = 200_000_000;
// RM Table 4-1: maximum PCLK0..4 frequencies in Hz. ADC analog supply
// constraints can impose a lower conversion-clock limit (DS Table 3-37).
const MAX_PCLK_HZ: [u32; 5] = [
    200_000_000,
    100_000_000,
    60_000_000,
    50_000_000,
    100_000_000,
];
// RM 4.4: external bus clock is limited to 100 MHz.
const MAX_EXCLK_HZ: u32 = 100_000_000;
// RM 4.4: HCLK:EXCLK must be 2, 4, 8, 16 or 32; compare selector differences.
const MAX_EXCLK_DIVIDER_DELTA: u8 = 5;
// RM-ZH 4.4: PCLK2:PCLK4 must be 1:4, 1:2, 1:1, 2:1, 4:1 or 8:1.
const MIN_ADC_DIVIDER_DELTA: i8 = -2;
// Largest allowed log2(PCLK2/PCLK4) from the same relationship.
const MAX_ADC_DIVIDER_DELTA: i8 = 3;
// Errata 2.1.4: SCFGR[31:28] must mirror HCLKS for power-down entry to work.
const HCLK_MIRROR_SHIFT: u32 = 28;

struct Prepared {
    pll_bits: u32,
    divider_bits: u32,
    clocks: Clocks,
}

impl Config {
    fn prepare(self, hrc_hz: u32) -> Result<Prepared, Error> {
        let m = u64::from(self.mpll.input_divider);
        let input = u64::from(hrc_hz);
        if input < u64::from(decode::PLL_PFD_MIN_HZ) * m
            || input > u64::from(decode::PLL_PFD_MAX_HZ) * m
        {
            return Err(Error::PllInput);
        }
        // Use the exact ratio for bounds, avoiding rounding an overclock down.
        let vco_numerator = input * u64::from(self.mpll.multiplier);
        if vco_numerator < u64::from(decode::PLL_VCO_MIN_HZ) * m
            || vco_numerator > u64::from(decode::PLL_VCO_MAX_HZ) * m
        {
            return Err(Error::PllVco);
        }
        let system_divisor = m * u64::from(self.mpll.output_dividers[0]);
        if vco_numerator > u64::from(MAX_SYSTEM_HZ) * system_divisor {
            return Err(Error::SystemFrequency);
        }
        let d = self.dividers;
        let hclk = d.hclk as u8;
        let exclk = d.exclk as u8;
        let pclk = d.pclk.map(|v| v as u8);
        let adc_delta = pclk[4] as i8 - pclk[2] as i8;
        if exclk <= hclk
            || exclk - hclk > MAX_EXCLK_DIVIDER_DELTA
            || [pclk[1], pclk[3], pclk[4]].iter().any(|&v| v < hclk)
            || pclk[0] > pclk[1]
            || pclk[0] > pclk[3]
            || !(MIN_ADC_DIVIDER_DELTA..=MAX_ADC_DIVIDER_DELTA).contains(&adc_delta)
            || vco_numerator > u64::from(MAX_EXCLK_HZ) * (system_divisor << exclk)
            || pclk
                .iter()
                .zip(MAX_PCLK_HZ)
                .any(|(&v, max)| vco_numerator > u64::from(max) * (system_divisor << v))
        {
            return Err(Error::BusClocks);
        }
        let sys_clk = (vco_numerator / system_divisor) as u32;
        let clocks = Clocks {
            sys_clk,
            hclk: sys_clk >> hclk,
            exclk: sys_clk >> exclk,
            pclk: pclk.map(|v| sys_clk >> v),
        };
        let divider_bits = (u32::from(hclk) << decode::HCLK_SHIFT)
            | (u32::from(hclk) << HCLK_MIRROR_SHIFT)
            | (u32::from(exclk) << decode::EXCLK_SHIFT)
            | pclk
                .iter()
                .zip(decode::PCLK_SHIFTS)
                .fold(0, |bits, (&v, shift)| bits | (u32::from(v) << shift));
        Ok(Prepared {
            pll_bits: self.mpll.bits(),
            divider_bits,
            clocks,
        })
    }
}

/// Configure HRC/MPLL, select bus clocks for AD/TRNG and return nominal rates.
/// Errors fail closed without retries/restoration; do not start dependent drivers.
///
/// # Safety
/// Call once during exclusive startup, with interrupts and dependent drivers
/// inactive. The inherited CPU clock must be within the supported 200 MHz limit.
/// SRAM timing/power mode must already support the requested HCLK. Analog supply
/// and peripheral-specific limits must also be respected by their drivers.
pub unsafe fn configure(config: Config) -> Result<Clocks, Error> {
    startup::configure(config)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Clocks {
    /// System clock frequency, in Hz.
    pub sys_clk: u32,
    /// AHB/CPU clock frequency, in Hz.
    pub hclk: u32,
    /// Frequencies in Hz for CMU peripheral-clock domains PCLK0 through PCLK4.
    pub pclk: [u32; 5],
    /// External bus clock frequency, in Hz.
    pub exclk: u32,
}
