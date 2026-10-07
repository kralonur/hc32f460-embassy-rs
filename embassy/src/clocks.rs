//! Clock frequencies supplied by board startup after configuring the HC32 CMU.
//! No bootloader-owned RAM locations or board clock profile are assumed here.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Clocks {
    /// System clock frequency, in Hz.
    pub sys_clk: u32,
    /// AHB clock frequency, in Hz.
    pub hclk: u32,
    /// Frequencies in Hz for CMU peripheral-clock domains PCLK0 through PCLK4.
    pub pclk: [u32; 5],
    /// External bus clock frequency, in Hz.
    pub exclk: u32,
}
