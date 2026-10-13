//! Protect baud arithmetic, interrupt races and cancellation without touching MMIO.
extern crate std;
use super::*;
use core::task::Waker;
use std::{
    collections::{BTreeMap, VecDeque},
    vec::Vec,
};

#[derive(Default)]
struct Io {
    registers: BTreeMap<usize, u32>,
    statuses: VecDeque<u32>,
    received: VecDeque<u8>,
    transmitted: Vec<u8>,
    writes: Vec<(usize, u32)>,
}
impl Registers for Io {
    fn read(&mut self, offset: usize) -> u32 {
        if offset == SR {
            if let Some(status) = self.statuses.pop_front() {
                return status;
            }
            return if self.received.is_empty() {
                TC
            } else {
                TC | RXNE
            };
        }
        self.registers.get(&offset).copied().unwrap_or(0)
    }
    fn write(&mut self, offset: usize, value: u32) {
        self.registers.insert(offset, value);
        self.writes.push((offset, value));
    }
    fn receive(&mut self) -> u8 {
        self.received.pop_front().unwrap_or(0)
    }
    fn transmit(&mut self, byte: u8) {
        self.transmitted.push(byte);
    }
}

#[test]
fn baudrate_uses_exact_integer_math_across_prescalers() {
    for clock in [234_375, 8_000_000, 50_000_000, 100_000_000] {
        for baud in [300, 1_200, 9_600, 115_200, 1_000_000] {
            let Ok(timing) = timing(clock, baud) else {
                continue;
            };
            let divisor = u64::from((timing.brr >> 8) + 1);
            let factor = if timing.fractional {
                MIN_FRACTION_FACTOR + u64::from(timing.brr & 0x7f)
            } else {
                FRACTION_SCALE
            };
            let numerator = u64::from(clock) * factor;
            let denominator = OVERSAMPLE * (1 << (2 * timing.prescaler)) * divisor * FRACTION_SCALE;
            let target = u64::from(baud) * denominator;
            assert!(numerator.abs_diff(target) * PPM <= target * MAX_ERROR_PPM);
            assert_eq!(timing.brr & !0xff7f, 0);
        }
    }
    // /4 still exceeds BRR's eight-bit integer divider; /16 is required.
    assert_eq!(timing(50_000_000, 1_200).unwrap().prescaler, 2);
    assert_eq!(timing(16_000_000, 1_000_000).unwrap().brr, 0);
    assert!(!timing(16_000_000, 1_000_000).unwrap().fractional);
    for (clock, baud) in [
        (0, 115_200),
        (50_000_000, 0),
        (50_000_000, u32::MAX),
        (50_000_000, 1),
    ] {
        assert!(matches!(timing(clock, baud), Err(Error::Baudrate)));
    }
}

#[test]
fn configuration_precedes_enabling_and_preserves_data_bits_with_parity() {
    let mut io = Io::default();
    io.registers.insert(CR1, RE | TE);
    let config = Config {
        parity: Parity::Odd,
        stop_bits: StopBits::Two,
        ..Config::default()
    };
    let timing = timing(50_000_000, config.baudrate).unwrap();
    initialize(&mut io, config, &timing);
    assert_eq!(io.writes[0], (CR1, 0));
    assert_eq!(io.writes[1], (CR1, SBS | CLEAR_ERRORS));
    let enabled = io.writes.len() - 1;
    assert_eq!(io.writes[enabled].0, CR1);
    assert_eq!(io.writes[enabled - 1].0, CR1);
    assert_eq!(io.writes[enabled - 1].1 & (RE | TE), 0);
    assert_eq!(io.writes[enabled].1, io.writes[enabled - 1].1 | RE | TE);
    assert_eq!(io.read(CR2), STOP_TWO);
    assert_eq!(io.read(BRR), timing.brr);
    assert_eq!(io.read(CR1) & (RE | TE | PCE | PS), RE | TE | PCE | PS);
    assert_eq!(io.read(CR1) & (RIE | TCIE | CLEAR_ERRORS | (1 << 12)), 0);
}

#[test]
fn read_returns_partial_data_and_errors_are_reported_and_cleared() {
    let waker = AtomicWaker::new();
    let mut io = Io::default();
    io.received.extend([0x12, 0x34]);
    let mut bytes = [0; 8];
    {
        let mut future = core::pin::pin!(read(&mut io, &waker, &mut bytes));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Ok(2))
        );
    }
    assert_eq!(&bytes[..2], &[0x12, 0x34]);
    for (flag, error) in [
        (ORE, Error::Overrun),
        (FE, Error::Framing),
        (PE, Error::Parity),
    ] {
        io.statuses.push_back(flag);
        let mut future = core::pin::pin!(read(&mut io, &waker, &mut bytes));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Err(error))
        );
    }
    assert!(
        io.writes
            .iter()
            .any(|&(offset, value)| offset == CR1 && value & CLEAR_ERRORS == CLEAR_ERRORS)
    );
}

#[test]
fn pending_read_cancellation_masks_rx_but_keeps_tx_wait_armed() {
    let mut io = Io::default();
    io.registers.insert(CR1, RE | TE | TCIE);
    let waker = AtomicWaker::new();
    let mut bytes = [0; 1];
    {
        let mut future = core::pin::pin!(read(&mut io, &waker, &mut bytes));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Pending
        );
    }
    assert_eq!(io.read(CR1), RE | TE | TCIE);
    assert!(io.received.is_empty());
}

#[test]
fn wait_rechecks_completion_after_arming_and_never_uses_tx_empty() {
    let mut io = Io::default();
    io.registers.insert(CR1, RE | TE | RIE);
    io.statuses.extend([0, TC]);
    let waker = AtomicWaker::new();
    {
        let mut future = core::pin::pin!(Wait {
            bus: &mut io,
            waker: &waker,
            receive: false,
            armed: false,
        });
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(TC)
        );
    }
    assert_eq!(io.read(CR1), RE | TE | RIE);
    assert!(
        io.writes
            .iter()
            .all(|&(offset, value)| offset != CR1 || value & (1 << 7) == 0)
    );
}

#[test]
fn transmit_preserves_byte_order_and_empty_io_is_a_noop() {
    let mut io = Io::default();
    let waker = AtomicWaker::new();
    {
        let mut future = core::pin::pin!(write(&mut io, &waker, &[1, 2, 3]));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Ok(3))
        );
    }
    assert_eq!(io.transmitted, [1, 2, 3]);
    // Ready transmit paths must not touch CR1.
    assert!(io.writes.is_empty());
    let writes = io.writes.len();
    {
        let mut future = core::pin::pin!(read(&mut io, &waker, &mut []));
        assert_eq!(
            future
                .as_mut()
                .poll(&mut Context::from_waker(Waker::noop())),
            Poll::Ready(Ok(0))
        );
    }
    assert_eq!(io.writes.len(), writes);
}
