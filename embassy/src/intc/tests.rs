//! A rejected route must not disable the interrupt already owned by another driver.
use super::*;

struct Io {
    owner: u32,
    disabled: bool,
    enabled: bool,
    pending_cleared: bool,
    priority: u8,
}
impl Registers for Io {
    fn read_sel(&mut self, _: Line) -> u32 {
        self.owner
    }
    fn write_sel(&mut self, _: Line, value: u32) {
        self.owner = value;
    }
    fn nvic_disable(&mut self, _: Line) {
        self.disabled = true;
    }
    fn nvic_clear_pending(&mut self, _: Line) {
        self.pending_cleared = true;
    }
    fn nvic_set_priority(&mut self, _: Line, priority: u8) {
        self.priority = priority;
    }
    fn nvic_enable(&mut self, _: Line) {
        self.enabled = true;
    }
}

#[test]
fn failed_claim_leaves_existing_owner_enabled() {
    let mut io = Io {
        owner: 1,
        disabled: false,
        enabled: true,
        pending_cleared: false,
        priority: 8,
    };
    let line = Line::new(0).unwrap();
    assert_eq!(register_with(&mut io, 2, line, 8), Err(IrqError::LineTaken));
    assert_eq!(io.owner, 1);
    assert!(!io.disabled);
    assert!(io.enabled);
    assert!(!io.pending_cleared);
    assert_eq!(io.priority, 8);
}
