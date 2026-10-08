//! Register sequencing for the public clock configuration API.
use super::{Clocks, Config, Error, MAX_SYSTEM_HZ, Ready, Register};
use crate::clock_decode as decode;

// RM 4.11 register table: CMU base and documented register offsets.
const CMU: usize = 0x40054000;
// RM 5.7 register table: function-clock gate register base.
const PWC: usize = 0x40048000;
// RM 7.9 register table: EFM base, also its 32-bit protection register.
const EFM_FAPRT: usize = 0x40010400;
// RM 4.11.23: 16-bit AD/TRNG clock selection register offset.
const PERICKSEL: usize = CMU + 0x10;
// RM 4.11.21: 32-bit bus-divider configuration offset.
const SCFGR: usize = CMU + 0x20;
// RM 4.11.20: 8-bit system-clock selection offset.
const CKSWR: usize = CMU + 0x26;
// RM 4.11.16: 8-bit MPLL control offset.
const PLLCR: usize = CMU + 0x2a;
// RM 4.11.18: 8-bit UPLL control offset.
const UPLLCR: usize = CMU + 0x2e;
// RM 4.11.10: 8-bit HRC control offset.
const HRCCR: usize = CMU + 0x36;
// RM 4.11.19: 8-bit oscillator status offset.
const OSCSTBSR: usize = CMU + 0x3c;
// RM 4.11.15: 32-bit shared PLL source/MPLL configuration offset.
const PLLCFGR: usize = CMU + 0x100;
// RM 5.7.18 / PAC layout: 16-bit function protection in the CMU address region.
const FPRC: usize = CMU + 0x3fe;
// RM 4.11.14: 8-bit LRC control offset.
const LRCCR: usize = CMU + 0x427;
// RM 5.7: four consecutive word-wide function-clock gate registers.
const FCG_COUNT: usize = 4;
// RM 5.7: gate-register spacing in bytes.
const FCG_STRIDE: usize = core::mem::size_of::<u32>();
// RM 5.7.17: 32-bit gate-write protection offset.
const FCG0PC: usize = PWC + 0x10;
// RM 7.9.3: 32-bit flash read-mode offset.
const FRMC: usize = EFM_FAPRT + 8;
// RM 5.7.18: FPRC write-enable key in the upper byte.
const CMU_WRITE_KEY: u16 = 0xa500;
// RM Table 5-9: FPRCB0 grants writes to clock configuration registers.
const CMU_WRITE_ENABLE: u16 = 1;
// RM 5.7.17: FCG0PC write-enable key in bits 31:16.
const FCG_WRITE_KEY: u32 = 0xa5a5_0000;
// RM 5.7.17: FCG0PC.PRT0 enables gate writes.
const FCG_WRITE_ENABLE: u32 = 1;
// RM 4.8 / DDL CLK_FCG0_DEFAULT: park gates when entering/leaving MPLL.
const PARKED_FCG0: u32 = 0xfffffaee;
// RM 7.9.1: first flash-register unlock key, not a programming command.
const EFM_UNLOCK_FIRST: u32 = 0x0123;
// RM 7.9.1: second unlock key; any write when unprotected relocks access.
const EFM_UNLOCK_SECOND: u32 = 0x3210;
// RM 7.9.1: protection register reads one when unprotected.
const EFM_UNPROTECTED: u32 = 1;
// RM 7.9.3: FRMC.FLWT begins at bit four.
const FLASH_WAIT_SHIFT: u32 = 4;
// RM 7.9.3: FLWT is four bits wide.
const FLASH_WAIT_MASK: u32 = 0xf;
// RM 7.9.3: low-power read modes SLPMD and LVM are bits zero and eight.
const FLASH_LOW_POWER_MASK: u32 = (1 << 0) | (1 << 8);
// RM Table 7-1: five waits cover the supported 200 MHz HCLK envelope.
const MIN_FLASH_WAITS: u32 = 5;
// Only latency/low-power fields may change; preserve cache-control bits.
const FLASH_READ_MODE_MASK: u32 = (FLASH_WAIT_MASK << FLASH_WAIT_SHIFT) | FLASH_LOW_POWER_MASK;
// RM 4.11.16, 4.11.18: both PLL stop controls are bit zero.
const PLL_STOP: u8 = 1;
// RM 4.11.10, 4.11.14: HRC/LRC stop controls are bit zero.
const RC_STOP: u8 = 1;
// RM 4.11.19: HRC stable status is bit zero.
const HRC_STABLE: u8 = 1;
// RM 4.11.19: MPLL stable status is bit five.
const MPLL_STABLE: u8 = 1 << 5;
// RM 4.11.19: UPLL stable status is bit six.
const UPLL_STABLE: u8 = 1 << 6;
// RM 4.11.23: selector zero uses the PCLK2/PCLK4 bus clocks.
const PERIPHERAL_CLOCK_MASK: u16 = 0xf;
// DDL Rev3.3.0 system_hc32f460.c, HRC_FREQ_MON(): frequency-selection readback.
const HRC_FREQUENCY_READBACK: usize = 0x40010684;
// DDL SystemCoreClockUpdate(): bit zero set means nominal 16 MHz, clear 20 MHz.
const HRC_16MHZ_SELECTED: u32 = 1;
// Existing finite software readiness bound, in polls, not calibrated milliseconds.
const POLL_BUDGET: usize = 100_000;
// RM 4.8 / DDL CLK_SYSCLK_SW_STB: at least 30 microseconds at each switch boundary.
const SETTLE_US: u32 = 30;
// SI conversion: frequency in Hz times microseconds divided by one million.
const MICROSECONDS_PER_SECOND: u32 = 1_000_000;
// cortex_m::asm::delay guarantees at least this many CPU cycles. Computing at
// the supported maximum HCLK also covers slower or unknown startup clocks.
const SETTLE_CYCLES: u32 = MAX_SYSTEM_HZ / MICROSECONDS_PER_SECOND * SETTLE_US;

// RM 4.11.20: only these two stable sources are selected by this startup recipe.
#[derive(Clone, Copy)]
#[repr(u8)]
enum Source {
    Hrc = 0,
    Mpll = 5,
}

trait Registers {
    fn read8(&mut self, address: usize) -> u8;
    fn read16(&mut self, address: usize) -> u16;
    fn read32(&mut self, address: usize) -> u32;
    fn write8(&mut self, address: usize, value: u8);
    fn write16(&mut self, address: usize, value: u16);
    fn write32(&mut self, address: usize, value: u32);
    fn settle(&mut self);
}

fn unlock(io: &mut impl Registers) {
    let value = io.read16(FPRC);
    io.write16(FPRC, value | CMU_WRITE_KEY | CMU_WRITE_ENABLE);
}
fn lock(io: &mut impl Registers) {
    let value = io.read16(FPRC);
    io.write16(FPRC, (value & !CMU_WRITE_ENABLE) | CMU_WRITE_KEY);
}
fn wait(io: &mut impl Registers, mask: u8, expected: u8, phase: Ready) -> Result<(), Error> {
    for _ in 0..POLL_BUDGET {
        if io.read8(OSCSTBSR) & mask == expected {
            return Ok(());
        }
    }
    Err(Error::Timeout(phase))
}

fn transition(io: &mut impl Registers, source: Option<Source>, divider_bits: u32) {
    let gates = core::array::from_fn::<_, FCG_COUNT, _>(|n| io.read32(PWC + n * FCG_STRIDE));
    io.write32(FCG0PC, FCG_WRITE_KEY | FCG_WRITE_ENABLE);
    if io.read8(CKSWR) & decode::SELECTOR_MASK as u8 == Source::Mpll as u8
        || matches!(source, Some(Source::Mpll))
    {
        io.write32(PWC, PARKED_FCG0);
        for n in 1..FCG_COUNT {
            io.write32(PWC + n * FCG_STRIDE, u32::MAX);
        }
        io.settle();
    }
    unlock(io);
    if let Some(source) = source {
        let value = io.read8(CKSWR);
        io.write8(
            CKSWR,
            (value & !(decode::SELECTOR_MASK as u8)) | source as u8,
        );
    } else {
        io.write32(SCFGR, divider_bits);
    }
    lock(io);
    io.settle();
    for (n, value) in gates.into_iter().enumerate() {
        io.write32(PWC + n * FCG_STRIDE, value);
    }
    io.write32(FCG0PC, FCG_WRITE_KEY);
    io.settle();
}

fn setup(io: &mut impl Registers, config: Config) -> Result<Clocks, Error> {
    let source = io.read8(CKSWR) & decode::SELECTOR_MASK as u8;
    if source > Source::Mpll as u8 {
        return Err(Error::EntrySource(source));
    }
    // Never change the shared PLL source while UPLL is active.
    if io.read32(PLLCFGR) & decode::PLL_SOURCE_HRC == 0 && io.read8(UPLLCR) & PLL_STOP == 0 {
        return Err(Error::SharedPllSource);
    }
    let hrc_hz = decode::hrc_hz_from_readback_bit(
        io.read32(HRC_FREQUENCY_READBACK) & HRC_16MHZ_SELECTED != 0,
    );
    // Configuration validation precedes every write, including EFM unlocking.
    let prepared = config.prepare(hrc_hz)?;
    io.write32(EFM_FAPRT, EFM_UNLOCK_FIRST);
    io.write32(EFM_FAPRT, EFM_UNLOCK_SECOND);
    let value = io.read32(FRMC);
    let waits = ((value >> FLASH_WAIT_SHIFT) & FLASH_WAIT_MASK).max(MIN_FLASH_WAITS);
    io.write32(
        FRMC,
        (value & !FLASH_READ_MODE_MASK) | (waits << FLASH_WAIT_SHIFT),
    );
    if io.read32(EFM_FAPRT) == EFM_UNPROTECTED {
        io.write32(EFM_FAPRT, EFM_UNLOCK_SECOND);
        io.write32(EFM_FAPRT, EFM_UNLOCK_SECOND);
    }
    if io.read32(FRMC) & FLASH_READ_MODE_MASK != waits << FLASH_WAIT_SHIFT {
        return Err(Error::Readback(Register::FlashReadMode));
    }
    unlock(io);
    let value = io.read8(HRCCR);
    io.write8(HRCCR, value & !RC_STOP);
    let ready = wait(io, HRC_STABLE, HRC_STABLE, Ready::HrcStable);
    lock(io);
    ready?;
    transition(io, Some(Source::Hrc), prepared.divider_bits);
    if io.read8(CKSWR) & decode::SELECTOR_MASK as u8 != Source::Hrc as u8 {
        return Err(Error::Readback(Register::Source));
    }
    unlock(io);
    let value = io.read8(PLLCR);
    io.write8(PLLCR, value | PLL_STOP);
    let stopped = wait(io, MPLL_STABLE, 0, Ready::MpllStopped);
    lock(io);
    stopped?;
    transition(io, None, prepared.divider_bits);
    if io.read32(SCFGR) != prepared.divider_bits {
        return Err(Error::Readback(Register::Dividers));
    }
    unlock(io);
    let value = io.read8(LRCCR);
    io.write8(LRCCR, value & !RC_STOP);
    io.settle();
    lock(io);
    unlock(io);
    let value = io.read32(PLLCFGR);
    if value & decode::PLL_SOURCE_HRC == 0 && io.read8(OSCSTBSR) & UPLL_STABLE != 0 {
        lock(io);
        return Err(Error::SharedPllSource);
    }
    io.write32(
        PLLCFGR,
        (value & !decode::PLL_CONFIG_MASK) | prepared.pll_bits,
    );
    lock(io);
    if io.read32(PLLCFGR) & decode::PLL_CONFIG_MASK != prepared.pll_bits {
        return Err(Error::Readback(Register::Pll));
    }
    unlock(io);
    let value = io.read8(PLLCR);
    io.write8(PLLCR, value & !PLL_STOP);
    let ready = wait(io, MPLL_STABLE, MPLL_STABLE, Ready::MpllStable);
    lock(io);
    ready?;
    transition(io, Some(Source::Mpll), prepared.divider_bits);
    unlock(io);
    let value = io.read16(PERICKSEL);
    io.write16(PERICKSEL, value & !PERIPHERAL_CLOCK_MASK);
    lock(io);
    if io.read16(PERICKSEL) & PERIPHERAL_CLOCK_MASK != 0 {
        return Err(Error::Readback(Register::AdTrng));
    }
    if io.read8(CKSWR) & decode::SELECTOR_MASK as u8 != Source::Mpll as u8 {
        return Err(Error::Readback(Register::Source));
    }
    if io.read32(SCFGR) != prepared.divider_bits {
        return Err(Error::Readback(Register::Dividers));
    }
    if io.read32(PLLCFGR) & decode::PLL_CONFIG_MASK != prepared.pll_bits {
        return Err(Error::Readback(Register::Pll));
    }
    Ok(prepared.clocks)
}

struct Hardware;

// SAFETY: only exclusive startup uses this private backend. Addresses and access
// widths are from the RM/PAC, including the vendor DDL HRC monitor word.
impl Registers for Hardware {
    fn read8(&mut self, address: usize) -> u8 {
        unsafe { (address as *const u8).read_volatile() }
    }
    fn read16(&mut self, address: usize) -> u16 {
        unsafe { (address as *const u16).read_volatile() }
    }
    fn read32(&mut self, address: usize) -> u32 {
        unsafe { (address as *const u32).read_volatile() }
    }
    fn write8(&mut self, address: usize, value: u8) {
        unsafe { (address as *mut u8).write_volatile(value) }
    }
    fn write16(&mut self, address: usize, value: u16) {
        unsafe { (address as *mut u16).write_volatile(value) }
    }
    fn write32(&mut self, address: usize, value: u32) {
        unsafe { (address as *mut u32).write_volatile(value) }
    }
    fn settle(&mut self) {
        cortex_m::asm::delay(SETTLE_CYCLES);
    }
}

// Only the public unsafe entry can reach this private hardware backend.
pub(super) fn configure(config: Config) -> Result<Clocks, Error> {
    setup(&mut Hardware, config)
}

#[cfg(test)]
mod tests;
