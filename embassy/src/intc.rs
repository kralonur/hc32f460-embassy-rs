//! INTC (interrupt controller) routing for HC32F460.
//!
//! Semantics verified against three vendor sources, not against secondary code:
//!
//! * **RM Rev 1.5** §10.4.4, §10.5.5–10.5.12 and Table 10-2 (event/line routing);
//! * **vendor SVD** `HC32F460JEUA.svd` (`SEL0` at offset `0x5C`, reset `0x1FF`;
//!   `VSSEL128` at `0x25C`, reset `0x0`; `EIRQCRn` at `0x10 + 4n`; `EIFR` `0x54`,
//!   `EIFCR` `0x58`, `SWIER` `0x29C`, `EVTER` `0x2A0`, `IER` `0x2A4`);
//! * **DDL Rev 3.3.0** (`hc32_ll_interrupts.c`): `INTSEL_RST_VALUE = 0x1FF`,
//!   the `IRQ_GRP_*` window rule, `IRQn_MAX = INT127_IRQn`, and the `VSSEL` bitmask.
//!
//! Facts that shape the API:
//!
//! 1. `INT_SEL0..=127` hold an event number; `0x1FF` means *unmapped* and is the
//!    reset value. A line may be claimed only while unmapped or already holding
//!    the same event (`INTSEL_RST_VALUE` check in the DDL).
//! 2. Lines `0..=31` accept any event; lines `32..=127` accept only the six-line
//!    window of the event's 32-event group (`group * 6 + 32 ..= group * 6 + 37`).
//!    Routing an event outside its window silently never fires.
//! 3. Lines `128..=143` are **vector-sharing** lines: `VSSEL128..=143` are
//!    bitmasks (`bit = event % 32`, `register = event / 32`) and the handler must
//!    poll peripheral status to find the source. That needs a dispatcher this HAL
//!    does not provide, so such lines are rejected instead of mis-programmed.
//! 4. Completion order used by the vendor: disable, claim, clear pending, set
//!    priority, enable — all inside one critical section.

/// `SEL` value that means "no event mapped to this line".
pub const SEL_UNMAPPED: u32 = 0x1FF;
/// First shared-vector line (`VSSEL128`).
pub const SHARED_LINE_BASE: u8 = 128;
/// Highest NVIC line on this part.
pub const MAX_LINE: u8 = 143;
/// Largest valid event source number (0x1FF is the unmapped marker, not an event).
pub const MAX_EVENT: u16 = 510;

/// Trigger selection for an external interrupt line (RM §10.5.5 `EIRQTRG`).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EirqTrigger {
    Falling = 0b00,
    Rising = 0b01,
    Both = 0b10,
    LowLevel = 0b11,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IrqError {
    /// Another event already owns this line.
    LineTaken,
    /// Lines 128..=143 are vector-sharing lines; see the module docs.
    SharedLineUnsupported,
    /// Event number is out of range (0x1FF is the unmapped marker).
    SourceInvalid,
    /// Event cannot reach this line: lines 32..=127 only serve their group window.
    OutsideGroupWindow,
    /// NVIC priority above 15.
    PriorityInvalid,
}

/// A routable NVIC line, `INT000..=INT143`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line(u8);

impl Line {
    pub const fn new(n: u8) -> Option<Self> {
        if n <= MAX_LINE { Some(Self(n)) } else { None }
    }
    #[inline]
    pub const fn n(self) -> u8 {
        self.0
    }
    #[inline]
    pub const fn is_shared(self) -> bool {
        self.0 >= SHARED_LINE_BASE
    }
}

// SAFETY: Line can only represent the HC32F460's NVIC interrupt numbers 0..=143.
unsafe impl cortex_m::interrupt::InterruptNumber for Line {
    #[inline]
    fn number(self) -> u16 {
        self.0 as u16
    }
}

/// Six-line window assigned to the event's 32-event group, when it fits in
/// `0..=127`. Groups whose window starts at or beyond 128 can only use the
/// shared VSSEL lines, which this HAL does not support.
pub fn group_window(event: u16) -> Option<(u8, u8)> {
    let group = (event / 32) as u32;
    let low = 32 + group * 6;
    let high = low + 5;
    if high <= 127 {
        Some((low as u8, high as u8))
    } else {
        None
    }
}

/// Whether `event` can drive `line` (DDL `INTC_IrqSignIn` rule, RM Table 10-2).
pub fn can_route(event: u16, line: Line) -> Result<(), IrqError> {
    if event > MAX_EVENT {
        return Err(IrqError::SourceInvalid);
    }
    if line.is_shared() {
        return Err(IrqError::SharedLineUnsupported);
    }
    if line.n() < 32 {
        return Ok(());
    }
    match group_window(event) {
        Some((low, high)) if line.n() >= low && line.n() <= high => Ok(()),
        _ => Err(IrqError::OutsideGroupWindow),
    }
}

/// Register file operations needed by the routing logic.
pub trait Registers {
    fn read_sel(&mut self, line: Line) -> u32;
    fn write_sel(&mut self, line: Line, value: u32);
    fn nvic_disable(&mut self, line: Line);
    fn nvic_clear_pending(&mut self, line: Line);
    fn nvic_set_priority(&mut self, line: Line, priority: u8);
    fn nvic_enable(&mut self, line: Line);
}

/// Claim `line` for `event` if it is free or already ours.
fn claim(io: &mut impl Registers, event: u16, line: Line) -> Result<(), IrqError> {
    let current = io.read_sel(line) & SEL_UNMAPPED;
    if current == SEL_UNMAPPED || current == event as u32 {
        io.write_sel(line, event as u32);
        Ok(())
    } else {
        Err(IrqError::LineTaken)
    }
}

fn release(io: &mut impl Registers, line: Line) {
    io.write_sel(line, SEL_UNMAPPED);
}

/// Full vendor completion order: disable, route, clear pending, set priority,
/// enable. Leaves nothing half-registered on failure.
pub fn register_with(
    io: &mut impl Registers,
    event: u16,
    line: Line,
    priority: u8,
) -> Result<(), IrqError> {
    if priority > 15 {
        return Err(IrqError::PriorityInvalid);
    }
    can_route(event, line)?;
    io.nvic_disable(line);
    claim(io, event, line)?;
    io.nvic_clear_pending(line);
    io.nvic_set_priority(line, priority);
    io.nvic_enable(line);
    Ok(())
}

pub fn unregister_with(io: &mut impl Registers, line: Line) {
    if line.is_shared() {
        return;
    }
    io.nvic_disable(line);
    release(io, line);
}

/// Selectable lines that have their own `SEL` register (`SEL0..SEL127`).
pub const ROUTABLE_LINES: u8 = 128;

/// Reset every `SEL0..127` entry to "unmapped".
///
/// Jumping into the application does **not** reset the INTC, so entries left
/// mapped by a bootloader or an earlier image make [`register`] refuse the line
/// with [`IrqError::LineTaken`]. That is how the first async I2C3 read failed on
/// hardware: `SEL110` (`I2C3_RXI`) was still claimed, so receive interrupts never
/// reached the NVIC while transmit/end interrupts did — the driver then hung
/// waiting for the second received byte. This application owns the INTC, so it
/// starts from a clean slate. The shared `VSSEL128..143` registers are bitmasks
/// and are deliberately left alone.
pub fn clear_routes_with(io: &mut impl Registers) {
    for n in 0..ROUTABLE_LINES {
        // Selectable lines 0..128 are all valid NVIC interrupt numbers.
        io.write_sel(Line(n), SEL_UNMAPPED);
    }
}

/// A line may only be reused once it has been released; report the current owner.
pub fn owner(io: &mut impl Registers, line: Line) -> Option<u16> {
    let value = io.read_sel(line) & SEL_UNMAPPED;
    if value == SEL_UNMAPPED {
        None
    } else {
        Some(value as u16)
    }
}

// ---------------------------------------------------------------- hardware ---

const INTC: usize = 0x40051000;
const SEL0: usize = INTC + 0x5C;
const EIRQCR0: usize = INTC + 0x10;
const EIFR: usize = INTC + 0x54;
const EIFCR: usize = INTC + 0x58;
const SWIER: usize = INTC + 0x29C;
const IER: usize = INTC + 0x2A4;
const NVIC_ICER: usize = 0xE000_E180;
const NVIC_ICPR: usize = 0xE000_E280;
const NVIC_ISER: usize = 0xE000_E100;
const NVIC_IPR: usize = 0xE000_E400;

struct Hardware;

impl Registers for Hardware {
    fn read_sel(&mut self, line: Line) -> u32 {
        // SAFETY: Line is valid and SEL registers are word-aligned, readable INTC registers.
        unsafe { ((SEL0 + 4 * line.n() as usize) as *const u32).read_volatile() }
    }
    fn write_sel(&mut self, line: Line, value: u32) {
        // SAFETY: Line is valid and SEL registers are word-aligned, writable INTC registers.
        unsafe { ((SEL0 + 4 * line.n() as usize) as *mut u32).write_volatile(value) }
    }
    fn nvic_disable(&mut self, line: Line) {
        let n = line.n() as usize;
        // SAFETY: Line selects an implemented NVIC interrupt-clear-enable register bit.
        unsafe { ((NVIC_ICER + (n / 32) * 4) as *mut u32).write_volatile(1 << (n % 32)) }
    }
    fn nvic_clear_pending(&mut self, line: Line) {
        let n = line.n() as usize;
        // SAFETY: Line selects an implemented NVIC interrupt-clear-pending register bit.
        unsafe { ((NVIC_ICPR + (n / 32) * 4) as *mut u32).write_volatile(1 << (n % 32)) }
    }
    fn nvic_set_priority(&mut self, line: Line, priority: u8) {
        // Four implemented priority bits, left-aligned in the IPR byte.
        // SAFETY: Line selects an implemented byte-wide NVIC priority register.
        unsafe { ((NVIC_IPR + line.n() as usize) as *mut u8).write_volatile(priority << 4) }
    }
    fn nvic_enable(&mut self, line: Line) {
        let n = line.n() as usize;
        // SAFETY: Line selects an implemented NVIC interrupt-set-enable register bit.
        unsafe { ((NVIC_ISER + (n / 32) * 4) as *mut u32).write_volatile(1 << (n % 32)) }
    }
}

/// Route `event` to `line` and enable it at the NVIC.
///
/// # Safety
/// The caller must own the vector for `line` (install an interrupt handler
/// before enabling) and must not route an event that is already in use by a
/// peripheral driver.
pub unsafe fn register(event: u16, line: Line, priority: u8) -> Result<(), IrqError> {
    critical_section::with(|_| register_with(&mut Hardware, event, line, priority))
}

/// Reset every `SEL0..127` entry to "unmapped"; see [`clear_routes_with`].
///
/// # Safety
/// Must run before anything is routed, and never concurrently with [`register`]
/// (both write the same `SEL` registers).
pub unsafe fn clear_routes() {
    critical_section::with(|_| clear_routes_with(&mut Hardware));
}

/// Disable `line` and free its `SEL` entry.
pub fn unregister(line: Line) {
    critical_section::with(|_| unregister_with(&mut Hardware, line));
}

/// Current event mapped to `line`, if any.
pub fn route_owner(line: Line) -> Option<u16> {
    critical_section::with(|_| owner(&mut Hardware, line))
}

/// Configure the external-interrupt input for a pin position (`EIRQCRn`).
///
/// The pin itself must also have `PCR.INTE` set; see RM §10.4.2 for the full
/// sequence (pin EIRQ input, `EIRQCRn`, `SELn`, `IER`, NVIC).
pub fn configure_eirq(pin: u8, trigger: EirqTrigger) {
    assert!(pin < 16);
    let value = trigger as u32;
    // SAFETY: The checked pin selects one of the word-aligned EIRQ configuration registers.
    unsafe { ((EIRQCR0 + 4 * pin as usize) as *mut u32).write_volatile(value) }
}

/// Pending external-interrupt flag for a pin position.
pub fn eirq_pending(pin: u8) -> bool {
    assert!(pin < 16);
    // SAFETY: EIFR is a word-aligned, readable external-interrupt status register.
    unsafe { ((EIFR) as *const u32).read_volatile() & (1 << pin) != 0 }
}

/// Clear the external-interrupt flag for a pin position (write 1 clears).
pub fn eirq_clear(pin: u8) {
    assert!(pin < 16);
    // SAFETY: EIFCR is a word-aligned write-one-to-clear register; pin was checked above.
    unsafe { ((EIFCR) as *mut u32).write_volatile(1 << pin) }
}

/// Trigger software interrupt channel `channel` (sets its `SWIER` bit).
///
/// RM §10.5.12: the bit stays set until software writes 0, so the handler must
/// call [`software_interrupt_clear`] or the request re-fires.
pub fn software_interrupt_trigger(channel: u8) {
    assert!(channel < 32);
    let bit = 1u32 << channel;
    // SAFETY: SWIER is word-aligned and supports the checked software-interrupt channel.
    unsafe {
        let reg = SWIER as *mut u32;
        reg.write_volatile(reg.read_volatile() | bit);
    }
}

/// Clear a software interrupt channel after handling it.
pub fn software_interrupt_clear(channel: u8) {
    assert!(channel < 32);
    let bit = 1u32 << channel;
    // SAFETY: SWIER is word-aligned and supports the checked software-interrupt channel.
    unsafe {
        let reg = SWIER as *mut u32;
        reg.write_volatile(reg.read_volatile() & !bit);
    }
}

/// Enable or disable an interrupt event request in `IER`.
pub fn event_request_enable(event: u16, enable: bool) {
    assert!(event < 32, "IER only covers the first 32 event requests");
    let bit = 1u32 << event;
    // SAFETY: IER is word-aligned and event selects one of its documented request bits.
    unsafe {
        let reg = IER as *mut u32;
        if enable {
            reg.write_volatile(reg.read_volatile() | bit);
        } else {
            reg.write_volatile(reg.read_volatile() & !bit);
        }
    }
}
