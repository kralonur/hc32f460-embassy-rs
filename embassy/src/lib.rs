//! Embassy HAL and runtime support for HC32F460 ARM Cortex-M4.
//!
//! Peripheral tokens come from [`pac::Peripherals::take`]. Board startup owns
//! clock/power configuration and pin mux qualification. Disable the default
//! `time-driver` feature to supply a different Embassy time driver.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub use hc32f460_pac as pac;

pub mod adc;
pub mod clock_decode;
pub mod clocks;
pub mod dma;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod intc;
pub mod pwm;
pub mod spi;
#[cfg(feature = "time-driver")]
pub mod time_driver;
pub mod usart;

/// Initialize SysTick after the board configures its clocks.
///
/// Prefer [`time_driver::init`] with an owned `SYST` token and checked errors.
/// The application must forward its SysTick exception to
/// [`time_driver::systick_tick`]. No other Embassy time driver may be linked.
///
/// # Panics
/// Panics if already initialized or if HCLK cannot produce an exact 1 kHz tick.
///
/// # Safety
/// Call once at startup, before other drivers run. The caller must exclusively
/// own SysTick and supply the configured CPU clock in `clocks.hclk`.
#[cfg(feature = "time-driver")]
pub unsafe fn init(clocks: clocks::Clocks) {
    // SAFETY: The caller transfers exclusive SysTick ownership to the driver.
    let systick = unsafe { cortex_m::Peripherals::steal() }.SYST;
    // Invariant: legacy startup must provide an exact supported clock and run once.
    time_driver::init(systick, clocks.hclk)
        .expect("SysTick initialization requires an exact 1 kHz clock and exclusive startup");
}
