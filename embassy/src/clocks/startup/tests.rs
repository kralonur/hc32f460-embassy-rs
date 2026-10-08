//! MMIO-sequence regressions: clock failures must not start drivers or bypass
//! shared-source/protection gates. A simulated boundary is necessary because
//! host tests cannot safely exercise the actual startup registers.
extern crate std;

use super::*;
use crate::clocks::{Divider, Dividers, Mpll};
use std::{collections::BTreeMap, vec::Vec};

#[derive(Debug, PartialEq, Eq)]
enum Event {
    Write(usize, u32),
    Settle,
}

struct Io {
    registers: BTreeMap<usize, u32>,
    events: Vec<Event>,
    hrc_ready: bool,
    stop_stuck: bool,
    mpll_ready: bool,
    ignored_write: Option<usize>,
    polls: usize,
}

impl Io {
    fn new(hrc_16mhz: bool) -> Self {
        Self {
            registers: BTreeMap::from([
                (HRC_FREQUENCY_READBACK, u32::from(hrc_16mhz)),
                (CKSWR, 1), // RM 4.11.20: inherited MRC clock.
                (PLLCFGR, decode::PLL_SOURCE_HRC),
                (UPLLCR, u32::from(PLL_STOP)),
                (HRCCR, u32::from(RC_STOP)),
                (PERICKSEL, 8), // RM 4.11.23: inherited MPLL/P selection.
                (
                    FRMC,
                    (7 << FLASH_WAIT_SHIFT) | FLASH_LOW_POWER_MASK | (1 << 16),
                ),
                (PWC, 0x11111111),
                (PWC + FCG_STRIDE, 0x22222222),
                (PWC + 2 * FCG_STRIDE, 0x33333333),
                (PWC + 3 * FCG_STRIDE, 0x44444444),
            ]),
            events: Vec::new(),
            hrc_ready: true,
            stop_stuck: false,
            mpll_ready: true,
            ignored_write: None,
            polls: 0,
        }
    }

    fn get(&self, address: usize) -> u32 {
        self.registers.get(&address).copied().unwrap_or(0)
    }

    fn put(&mut self, address: usize, value: u32) {
        self.events.push(Event::Write(address, value));
        if self.ignored_write == Some(address) {
            return;
        }
        if [HRCCR, PLLCR, LRCCR, PLLCFGR, SCFGR, CKSWR, PERICKSEL].contains(&address) {
            assert_ne!(
                self.get(FPRC) & u32::from(CMU_WRITE_ENABLE),
                0,
                "protected clock write"
            );
        }
        let value = if address == EFM_FAPRT && value == EFM_UNLOCK_SECOND {
            if self.get(EFM_FAPRT) == EFM_UNLOCK_FIRST {
                EFM_UNPROTECTED
            } else {
                0
            }
        } else {
            value
        };
        self.registers.insert(address, value);
    }
}

impl Registers for Io {
    fn read8(&mut self, address: usize) -> u8 {
        if address == OSCSTBSR {
            self.polls += 1;
            let mut status = self.get(OSCSTBSR) as u8;
            if self.hrc_ready && self.get(HRCCR) & u32::from(RC_STOP) == 0 {
                status |= HRC_STABLE;
            }
            if (self.get(PLLCR) & u32::from(PLL_STOP) != 0 && self.stop_stuck)
                || (self.get(PLLCR) & u32::from(PLL_STOP) == 0 && self.mpll_ready)
            {
                status |= MPLL_STABLE;
            }
            status
        } else {
            self.get(address) as u8
        }
    }
    fn read16(&mut self, address: usize) -> u16 {
        self.get(address) as u16
    }
    fn read32(&mut self, address: usize) -> u32 {
        self.get(address)
    }
    fn write8(&mut self, address: usize, value: u8) {
        self.put(address, u32::from(value));
    }
    fn write16(&mut self, address: usize, value: u16) {
        self.put(address, u32::from(value));
    }
    fn write32(&mut self, address: usize, value: u32) {
        self.put(address, value);
    }
    fn settle(&mut self) {
        self.events.push(Event::Settle);
    }
}

fn config() -> Config {
    Config {
        mpll: Mpll::new(2, 30, [2; 3]).expect("valid test PLL coefficients"),
        dividers: Dividers {
            hclk: Divider::Div1,
            exclk: Divider::Div2,
            pclk: [
                Divider::Div1,
                Divider::Div2,
                Divider::Div4,
                Divider::Div4,
                Divider::Div2,
            ],
        },
    }
}

#[test]
fn both_hrc_rates_produce_checked_clocks_and_preserve_flash_latency() {
    for (hrc_16mhz, system_hz) in [(true, 120_000_000), (false, 150_000_000)] {
        let mut io = Io::new(hrc_16mhz);
        let clocks = setup(&mut io, config()).expect("simulated startup succeeds");
        assert_eq!(clocks.sys_clk, system_hz);
        assert_eq!(clocks.hclk, system_hz);
        assert_eq!(
            clocks.pclk,
            [
                system_hz,
                system_hz / 2,
                system_hz / 4,
                system_hz / 4,
                system_hz / 2
            ]
        );
        assert_eq!(io.get(FRMC), (7 << FLASH_WAIT_SHIFT) | (1 << 16));
        assert_eq!(io.get(EFM_FAPRT), 0);
        assert_eq!(io.get(SCFGR), 0x00112210);
        assert_eq!(io.get(PLLCFGR) & decode::PLL_CONFIG_MASK, 0x11101d81);
    }
}

#[test]
fn invalid_profiles_are_refused_before_register_writes() {
    assert_eq!(Mpll::new(0, 30, [2; 3]).unwrap_err(), Error::PllFactors);
    assert_eq!(Mpll::new(2, 19, [2; 3]).unwrap_err(), Error::PllFactors);
    assert_eq!(Mpll::new(2, 30, [1, 2, 2]).unwrap_err(), Error::PllFactors);
    let mut overclock = config();
    overclock.dividers.pclk[2] = Divider::Div2;
    let mut bad_ratio = config();
    bad_ratio.dividers.exclk = Divider::Div1;
    for (profile, error) in [
        (overclock, Error::BusClocks),
        (bad_ratio, Error::BusClocks),
        (
            Config {
                mpll: Mpll::new(32, 30, [2; 3]).unwrap(),
                ..config()
            },
            Error::PllInput,
        ),
        (
            Config {
                mpll: Mpll::new(1, 30, [2; 3]).unwrap(),
                ..config()
            },
            Error::PllVco,
        ),
        (
            Config {
                mpll: Mpll::new(1, 24, [2; 3]).unwrap(),
                ..config()
            },
            Error::SystemFrequency,
        ),
    ] {
        let mut io = Io::new(false);
        assert_eq!(setup(&mut io, profile), Err(error));
        assert!(io.events.is_empty());
    }
}

#[test]
fn prohibited_source_and_active_upll_are_refused_without_writes() {
    let mut io = Io::new(false);
    io.registers.insert(CKSWR, 7);
    assert_eq!(setup(&mut io, config()), Err(Error::EntrySource(7)));
    assert!(io.events.is_empty());
    let mut io = Io::new(false);
    io.registers.insert(PLLCFGR, 0);
    io.registers.insert(UPLLCR, 0);
    assert_eq!(setup(&mut io, config()), Err(Error::SharedPllSource));
    assert!(io.events.is_empty());
}

#[test]
fn stopping_upll_cannot_have_its_source_changed() {
    let mut io = Io::new(false);
    io.registers.insert(PLLCFGR, 0);
    io.registers.insert(OSCSTBSR, u32::from(UPLL_STABLE));
    assert_eq!(setup(&mut io, config()), Err(Error::SharedPllSource));
    assert!(
        !io.events
            .iter()
            .any(|event| matches!(event, Event::Write(PLLCFGR, _)))
    );
    assert_eq!(io.get(FPRC) & u32::from(CMU_WRITE_ENABLE), 0);
}

#[test]
fn readiness_failures_are_bounded_and_relock_configuration() {
    for phase in [Ready::HrcStable, Ready::MpllStopped, Ready::MpllStable] {
        let mut io = Io::new(false);
        io.hrc_ready = phase != Ready::HrcStable;
        io.stop_stuck = phase == Ready::MpllStopped;
        io.mpll_ready = phase != Ready::MpllStable;
        assert_eq!(setup(&mut io, config()), Err(Error::Timeout(phase)));
        assert!(io.polls <= POLL_BUDGET + 3);
        assert_eq!(io.get(FPRC) & u32::from(CMU_WRITE_ENABLE), 0);
    }
}

#[test]
fn ignored_writes_fail_closed_at_their_readback() {
    for (address, register) in [
        (FRMC, Register::FlashReadMode),
        (CKSWR, Register::Source),
        (SCFGR, Register::Dividers),
        (PLLCFGR, Register::Pll),
        (PERICKSEL, Register::AdTrng),
    ] {
        let mut io = Io::new(false);
        io.ignored_write = Some(address);
        assert_eq!(setup(&mut io, config()), Err(Error::Readback(register)));
        assert_eq!(io.get(FPRC) & u32::from(CMU_WRITE_ENABLE), 0);
    }
}

#[test]
fn transitions_restore_gates_settle_afterwards_and_apply_hclk_erratum() {
    let mut io = Io::new(false);
    io.registers.insert(CKSWR, Source::Mpll as u32);
    let initial_gates = core::array::from_fn::<_, FCG_COUNT, _>(|n| io.get(PWC + n * FCG_STRIDE));
    let mut profile = config();
    profile.dividers.hclk = Divider::Div2;
    profile.dividers.exclk = Divider::Div4;
    profile.dividers.pclk[0] = Divider::Div2;
    let clocks = setup(&mut io, profile).expect("valid divided-HCLK startup");
    assert_eq!(clocks.hclk, clocks.sys_clk / 2);
    assert_eq!(
        io.get(SCFGR) >> super::super::HCLK_MIRROR_SHIFT,
        Divider::Div2 as u32
    );
    for (n, expected) in initial_gates.into_iter().enumerate() {
        assert_eq!(io.get(PWC + n * FCG_STRIDE), expected);
    }
    for (index, event) in io.events.iter().enumerate() {
        if *event == Event::Write(FCG0PC, FCG_WRITE_KEY) {
            assert_eq!(io.events[index + 1], Event::Settle);
        }
    }
    let hrc_switch = io
        .events
        .iter()
        .position(|e| *e == Event::Write(CKSWR, Source::Hrc as u32))
        .unwrap();
    let pll_stop = io
        .events
        .iter()
        .position(|e| *e == Event::Write(PLLCR, u32::from(PLL_STOP)))
        .unwrap();
    assert!(hrc_switch < pll_stop, "depart MPLL before stopping it");
    assert!(
        io.events.iter().all(|e| match e {
            Event::Settle => true,
            Event::Write(a, _) => [
                FPRC,
                EFM_FAPRT,
                FRMC,
                FCG0PC,
                CKSWR,
                HRCCR,
                PLLCR,
                LRCCR,
                PLLCFGR,
                PERICKSEL,
                SCFGR,
                PWC,
                PWC + FCG_STRIDE,
                PWC + 2 * FCG_STRIDE,
                PWC + 3 * FCG_STRIDE
            ]
            .contains(a),
        }),
        "startup writes only documented clock/protection registers, not bookkeeping RAM"
    );
}
