//! Embassy HAL and runtime support for HC32F460 ARM Cortex-M4.

#![no_std]

pub use hc32f460_pac as pac;

pub mod clock_decode;
pub mod clocks;
pub mod exti;
pub mod gpio;
pub mod i2c;
pub mod intc;
pub mod pwm;
pub mod spi;
pub mod time_driver;

/// Initialize the 1 kHz SysTick after the board configures its clocks.
///
/// The application must forward its SysTick exception to
/// [`time_driver::systick_tick`]. No other Embassy time driver may be linked.
///
/// # Panics
/// Panics if the CPU clock cannot produce a reload value for a 1 kHz tick.
///
/// # Safety
/// Call once at startup, before other drivers run. The caller must exclusively
/// own SysTick and supply the configured CPU clock in `clocks.sys_clk`.
pub unsafe fn init(clocks: clocks::Clocks) {
    // Embassy time uses 1,000 ticks per second, selected in the HAL manifest.
    const TICK_HZ: u32 = 1_000;
    // ARM Cortex-M4 SysTick has a 24-bit reload register.
    const MAX_TICKS: u32 = 1 << 24;
    // ARM Cortex-M4 core peripheral map: reload, current value, and control registers.
    const RELOAD: usize = 0xE000_E014;
    const CURRENT: usize = 0xE000_E018;
    const CONTROL: usize = 0xE000_E010;
    // ARM SysTick CTRL: enable counter, enable interrupt, select CPU clock.
    const ENABLE: u32 = 0b111;

    assert!(
        clocks.sys_clk >= TICK_HZ && clocks.sys_clk / TICK_HZ <= MAX_TICKS,
        "CPU clock must support a 1 kHz SysTick within its 24-bit reload range"
    );
    let reload = clocks.sys_clk / TICK_HZ - 1;
    // SAFETY: These are word-aligned SysTick registers. The caller owns the timer,
    // and the checked reload fits its register. Setup precedes enabling interrupts.
    unsafe {
        (RELOAD as *mut u32).write_volatile(reload);
        (CURRENT as *mut u32).write_volatile(0);
        (CONTROL as *mut u32).write_volatile(ENABLE);
    }
}
