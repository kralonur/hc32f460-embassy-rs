//! Pin-mux latch ordering and cross-port EXTI ownership regressions.
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
