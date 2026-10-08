//! Errata 3.3.5 regression: restart must stop undivided before resetting CNTER.
extern crate std;
use super::*;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
enum Write {
    Control(usize, u8),
    Timer(usize, u16),
}
#[derive(Default)]
struct Io(Vec<Write>);
impl Registers for Io {
    fn read_gate(&mut self) -> u32 {
        unreachable!("start does not access gates")
    }
    fn write_gate(&mut self, _: u32) {
        unreachable!("start does not access gates")
    }
    fn write_timer(&mut self, offset: usize, value: u16) {
        self.0.push(Write::Timer(offset, value));
    }
    fn write_control(&mut self, offset: usize, value: u8) {
        self.0.push(Write::Control(offset, value));
    }
}

#[test]
fn restart_stops_undivided_before_counter_write() {
    let mut io = Io::default();
    for _ in 0..2 {
        let first = io.0.len();
        start(&mut io, 1, 100, 50).unwrap();
        assert_eq!(io.0[first], Write::Control(BCSTRL, STOP_UNDIVIDED));
        assert_eq!(
            io.0[first + 1],
            Write::Control(BCSTRH, COUNTER_STATUS_RESET)
        );
        assert_eq!(io.0[first + 2], Write::Timer(CNTER, 0));
        assert_eq!(io.0.last(), Some(&Write::Control(BCSTRL, RUN)));
    }
}

#[test]
fn rejected_compare_does_not_stop_or_reconfigure_timer() {
    for compare in [0, 100] {
        let mut io = Io::default();
        assert_eq!(
            start(&mut io, 1, 100, compare),
            Err(Error::CompareOutOfRange)
        );
        assert!(io.0.is_empty());
    }
}
