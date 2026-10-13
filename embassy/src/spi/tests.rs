//! Register-boundary checks prevent byte writes under incompatible raw modes.
extern crate std;
use super::*;
use std::vec::Vec;

fn config() -> Config {
    Config {
        cfg1: CFG1_RESERVED_ONE,
        cfg2: DATA_SIZE_8 | 3,
        control: REQUIRED_CONTROL,
    }
}

#[test]
fn typed_master_config_never_exceeds_requested_clock() {
    use embedded_hal::spi::{MODE_0, MODE_1, MODE_2, MODE_3};
    for mode in [MODE_0, MODE_1, MODE_2, MODE_3] {
        let config = Config::master_tx(100_000_001, 10_000_000, mode).unwrap();
        assert!(check_config(config).is_ok());
        let divider = 2u64 << ((config.cfg2 >> 2) & 7);
        assert!(100_000_001 <= 10_000_000 * divider);
    }
    assert!(Config::master_tx(100_000_000, 1, MODE_0).is_err());
    assert!(Config::master_tx(0, 10_000_000, MODE_0).is_err());
}

#[test]
fn incompatible_configs_are_rejected() {
    assert!(check_config(config()).is_ok());
    for invalid in [
        Config {
            control: REQUIRED_CONTROL & !(1 << 3),
            ..config()
        },
        Config {
            control: REQUIRED_CONTROL | MODE_FAULT_DETECTION,
            ..config()
        },
        Config {
            cfg1: CFG1_RESERVED_ONE | 1,
            ..config()
        },
        Config {
            cfg1: CFG1_RESERVED_ONE | (1 << 31),
            ..config()
        },
        Config {
            cfg2: 0xf << 8,
            ..config()
        },
        Config {
            cfg2: DATA_SIZE_8 | PROHIBITED_SS,
            ..config()
        },
    ] {
        assert_eq!(check_config(invalid), Err(Error::Config));
    }
}

#[test]
fn init_sets_reserved_one_and_clears_stale_errors_before_enabling() {
    struct Io(Vec<(usize, u32)>);
    impl Registers for Io {
        fn read(&mut self, offset: usize) -> u32 {
            assert_eq!(offset, SR);
            ERRORS
        }
        fn write(&mut self, offset: usize, value: u32) {
            self.0.push((offset, value));
        }
    }
    let mut io = Io(Vec::new());
    initialize(
        &mut io,
        Config {
            cfg1: 0,
            ..config()
        },
    );
    assert_eq!(io.0[1], (CFG1, CFG1_RESERVED_ONE));
    assert_eq!(io.0[3], (SR, STATUS_READ_ONLY_ONES));
    assert_eq!(io.0[4], (CR1, REQUIRED_CONTROL));
}
