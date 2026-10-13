//! GPIO configuration/access for HC32F460. Pin constructors are unsafe because
//! legacy allocation is caller-enforced. Prefer [`split`] and the unique [`Pin`]
//! tokens for ownership-checked construction. No Clone or drop-time pin changes. Configuration unlock/RMW/relock is synchronous and guarded
//! by a critical section; output set/reset/toggle use 16-bit write-one aliases.
//!
//! All digital-pin constructors require the caller to stop XTAL32 before using
//! PC14/PC15 (RM-ZH 9.4.12). For PC15 input/open-drain use, Errata 2.6.1 requires
//! this at the start of Reset_Handler, not merely when constructing a pin.
//! This module does not own the shared oscillator and cannot stop it implicitly.
//! Debug functions must also be released before using their pins as GPIO or
//! alternate functions (RM 9.4.12). Legacy `InputPin::new`/`OutputPin::new` only
//! change direction/level: the caller must already have selected GPIO and
//! enabled digital functions; they do not override the inherited mux/DDIS.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
    H,
}
impl Port {
    fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
            Self::E => 4,
            Self::H => 5,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

// Vendor SVD: GPIO peripheral base address, in bytes.
const BASE: usize = 0x4005_3800;
// RM 9.4.10 / SVD: port write-protection register at byte offset 0x3FC.
const PWPR: usize = BASE + 0x3fc;
// RM 9.4.7 / SVD: debug function-selection register at byte offset 0x3F4.
const PSPCR: usize = BASE + 0x3f4;
// Vendor SVD: input-data register offset within each port, in bytes.
const PIDR: usize = 0x00;
// Vendor SVD: the output-data latch, not the pin's electrical input level.
const PODR: usize = 0x02;
// Vendor SVD: output-enable register offset within each port, in bytes.
const POER: usize = 0x06;
// Vendor SVD: write-one output-set register offset within each port, in bytes.
const POSR: usize = 0x08;
// Vendor SVD: write-one output-reset register offset within each port, in bytes.
const PORR: usize = 0x0a;
// Vendor SVD: write-one output-toggle register offset within each port, in bytes.
const POTR: usize = 0x0c;
// Vendor SVD: port data-register banks are spaced 16 bytes apart.
const PORT_STRIDE: usize = 0x10;
// Vendor SVD: PCRA0 starts at byte offset 0x400 within GPIO.
const PCR_BASE: usize = 0x400;
// Vendor SVD: PCR/PFSR banks for successive ports are spaced 64 bytes apart.
const CONFIG_PORT_STRIDE: usize = 0x40;
// Vendor SVD: successive pin PCR/PFSR pairs are spaced four bytes apart.
const CONFIG_PIN_STRIDE: usize = 4;
// Vendor SVD: each pin's PFSR follows its PCR by two bytes.
const PFSR_OFFSET: usize = 2;
// Vendor SVD: each A..E port data register has 16 pin bits.
const PINS_PER_PORT: u8 = 16;
// Vendor SVD: port H exposes PH0, PH1, and PH2.
const H_PIN_COUNT: u8 = 3;
// The public EXTI API selects PC13..15 on JEUA; cleanup still covers all ports.
const C_EXTI_FIRST_PIN: u8 = 13;
// RM 9.4.10: WP=0xA5 in bits 15:8 supplies the key, WE=0 disables writes.
const PWPR_LOCK: u16 = 0xa500;
// RM 9.4.10: the same key with WE (bit 0) set enables protected writes.
const PWPR_UNLOCK: u16 = PWPR_LOCK | 1;
// RM 9.4.7: SPFE bits 2..4 retain JTAG functions; bits 0..1 retain SWD.
const JTAG_FUNCTIONS: u16 = 0b111 << 2;
// RM 9.4.12: FSEL bits 5:0 select one of 64 pin functions.
const PFSR_FSEL: u16 = 0x3f;
// RM 9.4.12: BFE (bit 8) enables the second pin function.
const PFSR_BFE: u16 = 1 << 8;
// RM 9.4.12: FSEL=0 selects GPIO rather than a peripheral function.
const GPIO_FUNCTION: u8 = 0;
// RM 9.4.11: POUT (bit 0) is the output-data latch.
const PCR_POUT: u16 = 1 << 0;
// RM 9.4.11: POUTE (bit 1) enables the output driver.
const PCR_POUTE: u16 = 1 << 1;
// RM 9.4.11: NOD (bit 2) selects open-drain rather than CMOS output.
const PCR_NOD: u16 = 1 << 2;
// RM 9.4.11: DRV bits 5:4 select the pin's drive strength.
const PCR_DRV: u16 = 0b11 << 4;
// RM 9.4.11: DRV=0b10 selects high driving force, as in the original driver.
const PCR_DRV_HIGH: u16 = 0b10 << 4;
// RM 9.4.11: PUU (bit 6) enables the internal pull-up resistor.
const PCR_PUU: u16 = 1 << 6;
// RM 9.4.11: PIN (bit 8) is read-only input status, not an enable.
const PCR_PIN: u16 = 1 << 8;
// RM 9.4.11: INVE (bit 9) inverts input/output data.
const PCR_INVE: u16 = 1 << 9;
/// RM 9.4.11: INTE (bit 12) enables external-interrupt input.
pub const PCR_INTE: u16 = 1 << 12;
// RM 9.4.11: LTE (bit 14) locks the output state during function changes.
const PCR_LTE: u16 = 1 << 14;
// RM 9.4.11: DDIS (bit 15) disables all digital pin functions.
const PCR_DDIS: u16 = 1 << 15;
// Original high-drive profile resets these PCR fields (formerly mask 0xD275).
const OUTPUT_PROFILE_CLEAR: u16 =
    PCR_DDIS | PCR_LTE | PCR_INTE | PCR_INVE | PCR_PUU | PCR_DRV | PCR_NOD | PCR_POUT;

trait Registers {
    fn read(&mut self, address: usize) -> u16;
    fn write(&mut self, address: usize, value: u16);
}
struct Mmio;
impl Registers for Mmio {
    fn read(&mut self, address: usize) -> u16 {
        // SAFETY: Internal helpers supply documented, halfword-aligned GPIO register addresses.
        unsafe { (address as *const u16).read_volatile() }
    }
    fn write(&mut self, address: usize, value: u16) {
        // SAFETY: Internal helpers supply documented, halfword-aligned GPIO register addresses.
        unsafe { (address as *mut u16).write_volatile(value) }
    }
}
fn validate(port: Port, pin: u8) {
    assert!(pin < PINS_PER_PORT && (port != Port::H || pin < H_PIN_COUNT));
}
fn pcr(port: Port, pin: u8) -> usize {
    BASE + PCR_BASE + port.index() * CONFIG_PORT_STRIDE + usize::from(pin) * CONFIG_PIN_STRIDE
}
fn port_register(port: Port, offset: usize) -> usize {
    BASE + port.index() * PORT_STRIDE + offset
}
fn modify(bus: &mut impl Registers, address: usize, clear: u16, set: u16) {
    let old = bus.read(address);
    bus.write(address, (old & !clear) | set);
}
fn protected<R: Registers, T>(bus: &mut R, f: impl FnOnce(&mut R) -> T) -> T {
    struct Relock<'a, R: Registers>(&'a mut R);
    impl<R: Registers> Drop for Relock<'_, R> {
        fn drop(&mut self) {
            self.0.write(PWPR, PWPR_LOCK);
        }
    }
    critical_section::with(|_| {
        bus.write(PWPR, PWPR_UNLOCK);
        let guard = Relock(bus);
        f(guard.0)
    })
}
fn function(bus: &mut impl Registers, port: Port, pin: u8, code: u8) {
    assert!(u16::from(code) <= PFSR_FSEL);
    // RM 9.4.11 / RM-ZH 9.5: latch output across function selection, then
    // restore the caller's latch state. Do not expose the intermediate mux.
    let control = pcr(port, pin);
    let latch = bus.read(control) & PCR_LTE;
    modify(bus, control, 0, PCR_LTE);
    modify(
        bus,
        pcr(port, pin) + PFSR_OFFSET,
        PFSR_FSEL | PFSR_BFE,
        u16::from(code),
    );
    modify(bus, control, PCR_LTE, latch);
}
fn pull_up(bus: &mut impl Registers, port: Port, pin: u8, exti: bool) {
    function(bus, port, pin, GPIO_FUNCTION);
    // PIN bit8 is read-only status, not an enable. Preserve the legacy EXTI
    // write's bit8 literal; hardware ignores it. DDIS/output cleared, PUU set.
    modify(
        bus,
        pcr(port, pin),
        PCR_DDIS | PCR_POUTE | PCR_POUT,
        PCR_PUU | if exti { PCR_PIN } else { 0 },
    );
}
fn high_drive(bus: &mut impl Registers, port: Port, pin: u8, level: Level, inte: bool) {
    // Exact existing display profile: clear DDIS/LTE/INTE/INVE/PUU/DRV/NOD/POUT.
    modify(
        bus,
        pcr(port, pin),
        OUTPUT_PROFILE_CLEAR,
        PCR_POUTE
            | PCR_DRV_HIGH
            | u16::from(level == Level::High)
            | if inte { PCR_INTE } else { 0 },
    );
}
fn configured_output(bus: &mut impl Registers, port: Port, pin: u8, level: Level) {
    function(bus, port, pin, GPIO_FUNCTION);
    modify(bus, pcr(port, pin), PCR_DDIS, 0);
    high_drive(bus, port, pin, level, false);
}
fn direction(bus: &mut impl Registers, port: Port, pin: u8, output: bool) {
    let mask = 1 << pin;
    modify(
        bus,
        port_register(port, POER),
        if output { 0 } else { mask },
        if output { mask } else { 0 },
    );
}
fn write_level(bus: &mut impl Registers, port: Port, pin: u8, level: Level) {
    bus.write(
        port_register(port, if level == Level::High { POSR } else { PORR }),
        1 << pin,
    );
}

/// Read-only whole-port diagnostic snapshot; does not claim/reconfigure pins.
/// D/E registers can be read even on packages without those pins bonded out.
pub fn read_port(port: Port) -> u16 {
    Mmio.read(port_register(port, PIDR))
}

/// Release JTAG-only functions (PSPCR bits2..4), retaining SWD bits0..1.
/// # Safety
/// Caller must own the debug-pin configuration; this does not disable SWD.
pub unsafe fn release_jtag() {
    protected(&mut Mmio, |bus| modify(bus, PSPCR, JTAG_FUNCTIONS, 0));
}

/// Internal pull resistor selection. HC32F460 provides a pull-up, not a pull-down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pull {
    None,
    Up,
}

/// Exclusive GPIO allocator. Taking a pin permanently removes it from the bank;
/// dropping a pin does not silently allow another owner to acquire it.
pub struct Pins {
    _gpio: crate::pac::Gpio,
    available: [u16; 6],
}

/// Transfer GPIO ownership to a pin allocator, without changing any registers.
///
/// # Safety
/// The board must have clocks ready and disable inherited GPIO/EXTI routes.
/// Stop XTAL32 (including the PC15 reset-time workaround) before digital use of
/// PC14/PC15, and release debug functions before taking their pins. Do not use
/// legacy constructors or raw access to reconfigure pins owned by this bank.
pub unsafe fn split(gpio: crate::pac::Gpio) -> Pins {
    Pins {
        _gpio: gpio,
        available: [
            u16::MAX,
            u16::MAX,
            u16::MAX,
            u16::MAX,
            u16::MAX,
            (1 << H_PIN_COUNT) - 1,
        ],
    }
}

impl Pins {
    /// Take one unique pin token. Invalid or already-taken pins return `None`
    /// without register access. The board must select pins bonded on its package.
    pub fn take(&mut self, port: Port, pin: u8) -> Option<Pin> {
        if pin >= PINS_PER_PORT {
            return None;
        }
        let bit = 1 << pin;
        let available = &mut self.available[port.index()];
        if *available & bit == 0 {
            return None;
        }
        *available &= !bit;
        Some(Pin { port, pin })
    }
}

/// An unconfigured, uniquely owned GPIO pin. Conversions consume the token.
pub struct Pin {
    port: Port,
    pin: u8,
}

impl Pin {
    pub fn port(&self) -> Port {
        self.port
    }

    pub fn number(&self) -> u8 {
        self.pin
    }

    /// Select digital GPIO input and configure the internal pull-up.
    pub fn into_input(self, pull: Pull) -> InputPin {
        protected(&mut Mmio, |bus| {
            pull_up(bus, self.port, self.pin, false);
            modify(
                bus,
                pcr(self.port, self.pin),
                PCR_PUU,
                if pull == Pull::Up { PCR_PUU } else { 0 },
            );
        });
        InputPin {
            port: self.port,
            pin: self.pin,
        }
    }

    /// Select digital push-pull GPIO with high drive at `level`.
    pub fn into_output(self, level: Level) -> OutputPin {
        protected(&mut Mmio, |bus| {
            configured_output(bus, self.port, self.pin, level)
        });
        OutputPin {
            port: self.port,
            pin: self.pin,
        }
    }

    pub fn into_analog(self) -> AnalogPin {
        protected(&mut Mmio, |bus| analog(bus, self.port, self.pin));
        AnalogPin {
            _port: self.port,
            _pin: self.pin,
        }
    }

    /// Select a peripheral function, retaining unique pin ownership.
    /// # Safety
    /// The function and electrical mode must be valid for this pin/peripheral.
    pub unsafe fn into_alternate(self, code: u8, mode: AlternateMode) -> AlternatePin {
        protected(&mut Mmio, |bus| {
            alternate(bus, self.port, self.pin, code, mode)
        });
        AlternatePin {
            port: self.port,
            pin: self.pin,
        }
    }
}

pub struct InputPin {
    port: Port,
    pin: u8,
}
impl InputPin {
    /// Disable output through POER, preserving other configuration (legacy API).
    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    pub unsafe fn new(port: Port, pin: u8) -> Self {
        validate(port, pin);
        critical_section::with(|_| direction(&mut Mmio, port, pin, false));
        Self { port, pin }
    }
    /// Select GPIO, clear DDIS/output, set pull-up; preserve unrelated PCR bits.
    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    pub unsafe fn new_pull_up(port: Port, pin: u8) -> Self {
        validate(port, pin);
        protected(&mut Mmio, |bus| pull_up(bus, port, pin, false));
        Self { port, pin }
    }
    pub(crate) fn into_parts(self) -> (Port, u8) {
        (self.port, self.pin)
    }
    #[inline]
    pub fn is_high(&self) -> bool {
        read_port(self.port) & (1 << self.pin) != 0
    }
    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}

pub struct OutputPin {
    port: Port,
    pin: u8,
}
impl OutputPin {
    /// Preload level through POSR/PORR and enable POER (legacy API).
    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    pub unsafe fn new(port: Port, pin: u8, level: Level) -> Self {
        validate(port, pin);
        critical_section::with(|_| {
            write_level(&mut Mmio, port, pin, level);
            direction(&mut Mmio, port, pin, true);
        });
        Self { port, pin }
    }
    /// Existing display GPIO profile: select GPIO/digital, push-pull high drive,
    /// no latch/pull-up/inversion/interrupt, output enabled at the given level.
    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    /// Selection precedes PCR level setup, as in the original board driver;
    /// this is not a new glitch-free transition guarantee.
    pub unsafe fn new_high_drive(port: Port, pin: u8, level: Level) -> Self {
        validate(port, pin);
        protected(&mut Mmio, |bus| configured_output(bus, port, pin, level));
        Self { port, pin }
    }
    #[inline]
    pub fn set_high(&mut self) {
        write_level(&mut Mmio, self.port, self.pin, Level::High);
    }
    #[inline]
    pub fn set_low(&mut self) {
        write_level(&mut Mmio, self.port, self.pin, Level::Low);
    }
    #[inline]
    pub fn toggle(&mut self) {
        Mmio.write(port_register(self.port, POTR), 1 << self.pin);
    }
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        write_level(&mut Mmio, self.port, self.pin, level);
    }
    /// Read the output latch, not the electrical level on the pad.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        Mmio.read(port_register(self.port, PODR)) & (1 << self.pin) != 0
    }
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }
}

// GPIO register accesses have no recoverable software error. Infallible does
// not assert that a pin is bonded, wired correctly or electrically qualified.
impl embedded_hal::digital::ErrorType for InputPin {
    type Error = core::convert::Infallible;
}
impl embedded_hal::digital::InputPin for InputPin {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(InputPin::is_high(self))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(InputPin::is_low(self))
    }
}
impl embedded_hal::digital::ErrorType for OutputPin {
    type Error = core::convert::Infallible;
}
impl embedded_hal::digital::OutputPin for OutputPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        OutputPin::set_low(self);
        Ok(())
    }
    fn set_high(&mut self) -> Result<(), Self::Error> {
        OutputPin::set_high(self);
        Ok(())
    }
}

impl embedded_hal::digital::StatefulOutputPin for OutputPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(OutputPin::is_set_high(self))
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(OutputPin::is_set_low(self))
    }
    fn toggle(&mut self) -> Result<(), Self::Error> {
        OutputPin::toggle(self);
        Ok(())
    }
}

/// A pin left to its analog function, e.g. an ADC input.
pub struct AnalogPin {
    _port: Port,
    _pin: u8,
}
impl AnalogPin {
    /// Select GPIO and disable every digital function (DDIS), the output
    /// driver, pull-up and interrupt input (RM 9.4.11, 9.4.12).
    /// # Safety
    /// Caller must exclusively own this bonded pin and have clocks ready.
    pub unsafe fn new(port: Port, pin: u8) -> Self {
        validate(port, pin);
        protected(&mut Mmio, |bus| analog(bus, port, pin));
        Self {
            _port: port,
            _pin: pin,
        }
    }
}
fn analog(bus: &mut impl Registers, port: Port, pin: u8) {
    function(bus, port, pin, GPIO_FUNCTION);
    modify(
        bus,
        pcr(port, pin),
        OUTPUT_PROFILE_CLEAR | PCR_POUTE,
        PCR_DDIS,
    );
}

/// Pins of one port driven together, each high, low or undriven: one write
/// each to PORR, POSR and POER changes the whole group.
pub struct OutputGroup<const N: usize> {
    port: Port,
    pins: [u8; N],
}
impl<const N: usize> OutputGroup<N> {
    /// Select GPIO with digital functions and high drive on `pins`, outputs
    /// disabled, so the pins stay undriven until [`Self::drive`].
    /// # Safety
    /// Caller must exclusively own these distinct bonded pins and have clocks
    /// ready; debug functions on them must already be released (RM 9.4.12).
    pub unsafe fn new_high_drive(port: Port, pins: [u8; N]) -> Self {
        let mut seen = 0u16;
        for pin in pins {
            validate(port, pin);
            let bit = 1 << pin;
            assert!(seen & bit == 0, "an output group requires distinct pins");
            seen |= bit;
        }
        protected(&mut Mmio, |bus| {
            for pin in pins {
                undriven_high_drive(bus, port, pin);
            }
        });
        Self { port, pins }
    }
    /// Drive each pin, in construction order, to its level, or leave it
    /// undriven (`None`). Levels latch before the outputs enable.
    pub fn drive(&mut self, levels: [Option<Level>; N]) {
        critical_section::with(|_| drive_group(&mut Mmio, self.port, &self.pins, &levels));
    }
}
fn undriven_high_drive(bus: &mut impl Registers, port: Port, pin: u8) {
    function(bus, port, pin, GPIO_FUNCTION);
    modify(
        bus,
        pcr(port, pin),
        OUTPUT_PROFILE_CLEAR | PCR_POUTE,
        PCR_DRV_HIGH,
    );
}
fn drive_group(bus: &mut impl Registers, port: Port, pins: &[u8], levels: &[Option<Level>]) {
    let (mut group, mut high, mut low) = (0, 0, 0);
    for (&pin, level) in pins.iter().zip(levels) {
        let bit = 1 << pin;
        group |= bit;
        match level {
            Some(Level::High) => high |= bit,
            Some(Level::Low) => low |= bit,
            None => {}
        }
    }
    // PORR and POSR are write-one (vendor SVD), so only the group's pins
    // change; POER keeps the port's other pins.
    bus.write(port_register(port, PORR), low);
    bus.write(port_register(port, POSR), high);
    modify(bus, port_register(port, POER), group, high | low);
}

/// The three configuration sequences already used by board peripheral setup.
#[derive(Clone, Copy)]
pub enum AlternateMode {
    /// PFSR only (existing I2C pins); do not alter PCR.
    FunctionOnly,
    /// Clear DDIS (existing SPI pins), preserve direction and level.
    Digital,
    /// Clear DDIS/POUT, enable output (initial TimerA4 pin setup).
    DigitalOutput,
}
pub struct AlternatePin {
    port: Port,
    pin: u8,
}
fn alternate(bus: &mut impl Registers, port: Port, pin: u8, code: u8, mode: AlternateMode) {
    function(bus, port, pin, code);
    match mode {
        AlternateMode::FunctionOnly => {}
        AlternateMode::Digital => modify(bus, pcr(port, pin), PCR_DDIS, 0),
        AlternateMode::DigitalOutput => modify(bus, pcr(port, pin), PCR_DDIS | PCR_POUT, PCR_POUTE),
    }
}
impl AlternatePin {
    /// Select an alternate function using the chosen existing register sequence.
    /// # Safety
    /// Caller owns this bonded pin; function/code and peripheral setup must be
    /// valid for the pin. No automatic function-routing/electrical validation.
    /// For PC14/PC15, stop the external low-speed oscillator before enabling
    /// digital functions (RM-ZH 9.4.12); this constructor does not stop it.
    pub unsafe fn new(port: Port, pin: u8, code: u8, mode: AlternateMode) -> Self {
        validate(port, pin);
        assert!(u16::from(code) <= PFSR_FSEL);
        protected(&mut Mmio, |bus| alternate(bus, port, pin, code, mode));
        Self { port, pin }
    }
    /// Apply the display's output profile without changing its alternate function.
    /// `inte` is the literal PCR interrupt-enable bit, NOT input inversion. The
    /// board preserves it on PH2 for compatibility; this does not claim/route an
    /// EXTI channel or qualify output+interrupt use.
    /// # Safety
    /// Caller must own any affected interrupt channel and verify peripheral
    /// output use; no other pin driver may reconfigure this pin.
    pub unsafe fn configure_high_drive_output(&mut self, level: Level, inte: bool) {
        protected(&mut Mmio, |bus| {
            high_drive(bus, self.port, self.pin, level, inte)
        });
    }
}

fn claim(bus: &mut impl Registers, port: Port, pin: u8, enable: bool) {
    // Errata 2.4.1 applies to every port sharing this channel, including pins
    // left configured by a bootloader outside the selected JEUA pin subset.
    for p in [Port::A, Port::B, Port::C, Port::D, Port::E, Port::H] {
        if p == Port::H && pin >= H_PIN_COUNT {
            continue;
        }
        let address = pcr(p, pin);
        let old = bus.read(address);
        let new = if enable && p == port {
            old | PCR_INTE
        } else {
            old & !PCR_INTE
        };
        if old != new {
            bus.write(address, new);
        }
    }
}
/// Errata §2.4.1: clear competing INTE flags, then claim the selected channel.
/// # Safety
/// Caller owns the channel across all competing ports and has PWPR unlocked.
/// Selected pins are limited to A/B, C13..15 and H0..2; cleanup covers all ports.
/// NVIC routes for this channel must be disabled while changing INTE (Errata
/// 2.4.2). Enable before INTC registration; unregister INTC before disabling.
pub unsafe fn claim_external_interrupt(port: Port, pin: u8, enable: bool) {
    validate(port, pin);
    assert!(
        matches!(port, Port::A | Port::B)
            || (port == Port::C && pin >= C_EXTI_FIRST_PIN)
            || port == Port::H
    );
    claim(&mut Mmio, port, pin, enable);
}
/// Existing EXTI pull-up sequence and competing-channel cleanup, protected.
/// # Safety
/// Caller exclusively owns the pin/channel across all competing ports, with
/// its NVIC routes disabled; register INTC afterwards. XTAL32 must already be
/// stopped for PC14/PC15.
pub(crate) unsafe fn configure_exti_input(port: Port, pin: u8) {
    validate(port, pin);
    assert!(
        matches!(port, Port::A | Port::B)
            || (port == Port::C && pin >= C_EXTI_FIRST_PIN)
            || port == Port::H
    );
    protected(&mut Mmio, |bus| {
        pull_up(bus, port, pin, true);
        claim(bus, port, pin, true);
    });
}

#[cfg(test)]
mod tests;
