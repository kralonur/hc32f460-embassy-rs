//! Regressions for baud-rate rounding and documented master initialization/exit.
extern crate std;
use super::*;
use core::task::{Context, Waker};
use std::{
    collections::{BTreeMap, VecDeque},
    vec::Vec,
};

#[derive(Default)]
struct Io {
    registers: BTreeMap<usize, u32>,
    writes: Vec<(usize, u32)>,
    status: VecDeque<u32>,
    transmitted: Vec<u8>,
    stop_before_receive: Vec<bool>,
}
impl Registers for Io {
    fn read(&mut self, offset: usize) -> u32 {
        if offset == SR
            && let Some(status) = self.status.pop_front()
        {
            return status;
        }
        self.registers.get(&offset).copied().unwrap_or(0)
    }
    fn write(&mut self, offset: usize, value: u32) {
        self.registers.insert(offset, value);
        self.writes.push((offset, value));
    }
    fn receive(&mut self) -> u8 {
        let stopped = self.read(CR1) & STOP != 0;
        self.stop_before_receive.push(stopped);
        u8::try_from(self.stop_before_receive.len()).unwrap()
    }
    fn transmit(&mut self, byte: u8) {
        self.transmitted.push(byte);
    }
}

#[test]
fn baud_rate_never_rounds_above_requested_rate() {
    for (pclk, frequency) in [(20_000_001, 400_000), (50_000_000, 400_000)] {
        let ccr = timing(pclk, frequency).unwrap();
        let divider = (ccr >> CCR_FREQ_SHIFT) & 0b111;
        let overhead = 2 * CLOCK_DIVIDERS[divider as usize].1;
        let period =
            ((ccr & CCR_WIDTH_MASK) + ((ccr >> CCR_HIGH_SHIFT) & CCR_WIDTH_MASK) + overhead)
                << divider;
        assert!(u64::from(pclk) <= u64::from(frequency) * u64::from(period));
    }
    assert_eq!(timing(20_000_000, 0), None);
}

#[test]
fn scl_phases_meet_datasheet_minima() {
    for (pclk, frequency) in [
        (20_000_000, FAST_MAX_HZ),
        (50_000_000, FAST_MAX_HZ),
        (6_400_000, STANDARD_MAX_HZ),
        (9_600_000, STANDARD_MAX_HZ),
    ] {
        let ccr = timing(pclk, frequency).unwrap();
        let encoding = ((ccr >> CCR_FREQ_SHIFT) & 0b111) as usize;
        let (divisor, overhead) = CLOCK_DIVIDERS[encoding];
        let (low_ns, high_ns) = if frequency <= STANDARD_MAX_HZ {
            STANDARD_PHASE_NS
        } else {
            FAST_PHASE_NS
        };
        let low = u64::from((ccr & CCR_WIDTH_MASK) + overhead) * u64::from(divisor);
        let high =
            u64::from(((ccr >> CCR_HIGH_SHIFT) & CCR_WIDTH_MASK) + overhead) * u64::from(divisor);
        assert!(low * NANOSECONDS_PER_SECOND >= u64::from(pclk) * low_ns);
        assert!(high * NANOSECONDS_PER_SECOND >= u64::from(pclk) * high_ns);
    }
    assert_eq!(timing(20_000_000, FAST_MAX_HZ + 1), None);
}

#[test]
fn master_init_sets_filter_disables_slave_matches_and_preserves_reserved_bits() {
    let mut io = Io::default();
    // Seed nonzero reserved defaults and a different filter/slave configuration.
    let reserved = 1 << 22;
    io.registers.insert(CCR, reserved);
    io.registers.insert(FLTR, DIGITAL_FILTER_WIDTH | (1 << 5));
    io.registers.insert(SLR0, SLAVE_ADDRESS_ENABLE | 0x42);
    io.registers.insert(SLR1, SLAVE_ADDRESS_ENABLE | 0x44);
    initialize(&mut io, timing(20_000_000, 400_000).unwrap());
    assert_eq!(io.read(CCR) & reserved, reserved);
    assert_eq!(io.read(FLTR), DIGITAL_FILTER_ENABLE | (1 << 5));
    assert_eq!(io.read(SLR0), 0x42);
    assert_eq!(io.read(SLR1), 0x44);
    assert_ne!(io.read(CR4) & BUSWAIT, 0);
    assert_eq!(io.read(CR1), PE);
    assert_eq!(io.read(CR2), 0);
}

#[test]
fn stop_clears_stale_flag_before_requesting_stop() {
    let mut io = Io::default();
    io.registers.insert(CR1, PE);
    request_stop(&mut io);
    assert_eq!(io.writes, [(CLR, STOPFCLR), (CR1, PE | STOP)]);
}

#[test]
fn begin_clears_commands_left_by_cancelled_transaction() {
    let mut io = Io::default();
    io.registers.insert(CR1, PE | START | STOP | RESTART | NACK);
    io.registers.insert(SR, BUSY);
    {
        let mut future = core::pin::pin!(begin(&mut io));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        );
    }
    assert_eq!(io.writes[1], (CR1, PE));
}

#[test]
fn write_read_keeps_repeated_start_and_stops_before_final_receive() {
    let mut io = Io::default();
    io.registers.insert(CR1, PE);
    // Script the documented ready states: idle, master TX/address/restart,
    // master RX with each byte full, then STOP after the final DRR read.
    let tx = TEMPTYF | TENDF | BUSY | STARTF | TRA;
    let rx = TEMPTYF | RFULLF | BUSY | STARTF;
    io.status.push_back(TEMPTYF);
    io.status.extend(core::iter::repeat_n(tx, 9));
    io.status.extend(core::iter::repeat_n(rx, 4));
    io.status.push_back(TEMPTYF | STOPF);
    let mut bytes = [0; 2];
    let mut ops = [Operation::Write(&[0x10]), Operation::Read(&mut bytes)];
    {
        let mut future = core::pin::pin!(transfer(&mut io, 0x40, &mut ops));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Ok(()))
        );
    }
    assert_eq!(bytes, [1, 2]);
    assert_eq!(io.transmitted, [0x80, 0x10, 0x81]);
    assert_eq!(io.stop_before_receive, [false, true]);
    assert!(
        io.writes
            .iter()
            .any(|&(offset, value)| offset == CR1 && value & RESTART != 0)
    );
    assert!(io.status.is_empty());
}

#[test]
fn read_to_write_restart_is_rejected_before_any_io() {
    let mut io = Io::default();
    let mut byte = [0];
    let mut ops = [Operation::Read(&mut byte), Operation::Write(&[1])];
    let result = {
        let mut future = core::pin::pin!(transfer(&mut io, 0x40, &mut ops));
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
    };
    assert_eq!(result, Poll::Ready(Err(Error::ReadRestart)));
    assert!(io.writes.is_empty());
}
