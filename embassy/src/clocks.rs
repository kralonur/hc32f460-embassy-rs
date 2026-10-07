//! Clock frequencies supplied by board startup after configuring the HC32 CMU.
//! No bootloader-owned RAM locations or board clock profile are assumed here.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Clocks {
    pub sys_clk: u32,
    pub hclk: u32,
    pub pclk: [u32; 5],
    pub exclk: u32,
}
