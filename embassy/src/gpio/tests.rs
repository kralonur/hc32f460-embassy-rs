//! Pin-mux latch ordering, cross-port EXTI ownership and output-group
//! regressions.
extern crate std;
use super::*;
use std::{collections::BTreeMap, vec::Vec};

#[derive(Default)]
struct Io {
    registers: BTreeMap<usize, u16>,
    writes: Vec<(usize, u16)>,
}
impl Registers for Io {
    fn read(&mut self, address: usize) -> u16 {
        self.registers.get(&address).copied().unwrap_or(0)
    }
    fn write(&mut self, address: usize, value: u16) {
        self.registers.insert(address, value);
        self.writes.push((address, value));
    }
}

#[test]
fn pin_allocator_never_hands_out_the_same_pin_twice() {
    // SAFETY: this host-only allocator regression never accesses MMIO or
    // converts its tokens into configured pins; no hardware owner exists here.
    let mut pins = unsafe { split(crate::pac::Gpio::steal()) };
    let pin = pins.take(Port::A, 4).unwrap();
    assert_eq!((pin.port(), pin.number()), (Port::A, 4));
    assert!(pins.take(Port::A, 4).is_none());
    assert!(pins.take(Port::B, 4).is_some());
    assert!(pins.take(Port::H, 2).is_some());
    assert!(pins.take(Port::H, 3).is_none());
    assert!(pins.take(Port::A, 16).is_none());
}

#[test]
fn function_selection_is_latched_and_restores_prior_latch_state() {
    for latch in [0, PCR_LTE] {
        let mut io = Io::default();
        let control = pcr(Port::A, 4);
        io.registers.insert(control, PCR_POUTE | latch);
        function(&mut io, Port::A, 4, 1);
        assert_eq!(io.writes[0], (control, PCR_POUTE | PCR_LTE));
        assert_eq!(io.writes[1].0, control + PFSR_OFFSET);
        assert_eq!(io.writes[2], (control, PCR_POUTE | latch));
    }
}

#[test]
fn exti_claim_clears_competitors_outside_original_jeua_cleanup_subset() {
    let mut io = Io::default();
    for port in [Port::A, Port::B, Port::C, Port::D, Port::E] {
        io.registers.insert(pcr(port, 4), PCR_INTE | PCR_PUU);
    }
    claim(&mut io, Port::A, 4, true);
    for port in [Port::A, Port::B, Port::C, Port::D, Port::E] {
        assert_eq!(
            io.read(pcr(port, 4)),
            PCR_PUU | if port == Port::A { PCR_INTE } else { 0 }
        );
    }
}

#[test]
fn output_group_latches_levels_before_enabling_and_keeps_other_pins() {
    let mut io = Io::default();
    let enable = port_register(Port::B, POER);
    // Pins 12 and 14 belong to other owners and stay enabled.
    io.registers.insert(enable, 1 << 12 | 1 << 14 | 1 << 4);
    let pins = [8, 3, 9, 4];
    drive_group(
        &mut io,
        Port::B,
        &pins,
        &[Some(Level::High), Some(Level::Low), Some(Level::High), None],
    );
    assert_eq!(
        io.writes,
        [
            (port_register(Port::B, PORR), 1 << 3),
            (port_register(Port::B, POSR), 1 << 8 | 1 << 9),
            (enable, 1 << 12 | 1 << 14 | 1 << 8 | 1 << 3 | 1 << 9),
        ]
    );
}
