//! Async external interrupts (PORT `EIRQ` channels) for HC32F460.
//!
//! HC32 routes every port's pin *n* onto the same external-interrupt channel
//! *n*; the channel is then selected onto an NVIC line through `INTC.SELn`
//! (see [`crate::intc`]). This module provides the async half: the application's
//! interrupt handler calls [`on_interrupt`], which clears the `EIFR` flag and
//! wakes the task waiting on that channel.
//!
//! Enable order follows Errata §2.4.2 (peripheral side first, then
//! claim the line, clear pending, enable NVIC); [`ExtiInput::new`] only does the
//! peripheral side, so call [`crate::intc::register`] afterwards.
//!
//! ### Two-flag design (and the bug it fixes)
//!
//! `EIFR` is the only hardware record of an edge and it must be cleared in the
//! handler, or the level-asserted channel re-enters the ISR forever. The first
//! version cleared `EIFR` in the ISR and had the waiter test `EIFR` as well, so
//! the waiter *never* saw a pending edge: the task was woken, found the flag
//! clear and went back to sleep. Every async-tilt image therefore counted edges
//! but never flipped anything.
//!
//! Now the ISR clears `EIFR` **and sets a software latch** (`EDGE`); the waiter
//! consumes the latch. The latch is set before the wake and read after the
//! waker is registered, closing the lost-wakeup race. Multiple edges before
//! consumption coalesce into one pending event; this is not an edge counter.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use embassy_sync::waitqueue::AtomicWaker;

/// RM 10.5.5: external-interrupt channels EIRQ0..EIRQ15 match pin positions.
pub const MAX_CHANNEL: u8 = 15;

// One entry for each documented EIRQ channel, including channel zero.
const CHANNEL_COUNT: usize = MAX_CHANNEL as usize + 1;

/// One waker per channel; the ISR wakes, the task registers here.
static WAKERS: [AtomicWaker; CHANNEL_COUNT] = [const { AtomicWaker::new() }; CHANNEL_COUNT];

/// Software latch: bit `n` means "channel `n` has seen an edge that no waiter
/// has consumed yet". Set by the ISR, consumed by the waiter.
static EDGE: core::sync::atomic::AtomicU16 = core::sync::atomic::AtomicU16::new(0);

/// Wake a task waiting for `pin`'s external interrupt.
pub fn wake(pin: u8) {
    assert!(pin <= MAX_CHANNEL);
    WAKERS[pin as usize].wake();
}

/// Record an edge for `pin` and wake its waiter. Order matters: latch, then wake.
pub fn note_edge(pin: u8) {
    assert!(pin <= MAX_CHANNEL);
    EDGE.fetch_or(1 << pin, core::sync::atomic::Ordering::Release);
    wake(pin);
}

/// Take the latched edge for `pin`, if any.
pub fn take_edge(pin: u8) -> bool {
    assert!(pin <= MAX_CHANNEL);
    EDGE.fetch_and(!(1 << pin), core::sync::atomic::Ordering::AcqRel) & (1 << pin) != 0
}

/// Hardware side of an incoming external interrupt.
///
/// Clears the hardware flag (it must be cleared or the level-asserted channel
/// re-enters the ISR), latches the edge in software and wakes the waiter.
/// Returns `true` when a real edge was handled.
pub fn on_interrupt(pin: u8) -> bool {
    if crate::intc::eirq_pending(pin) {
        crate::intc::eirq_clear(pin);
        note_edge(pin);
        true
    } else {
        false
    }
}

/// An input pin routed to its external-interrupt channel.
pub struct ExtiInput {
    port: crate::gpio::Port,
    pin: u8,
}

impl ExtiInput {
    /// Configure `pin` on `port` as an external-interrupt input with a pull-up.
    ///
    /// # Safety
    /// The caller must own the pin and its channel across all competing ports,
    /// with the channel's NVIC routes disabled before changing PCR.INTE
    /// (Errata 2.4.2). XTAL32 must already be stopped for PC14/PC15; PC15 input
    /// requires the reset-time workaround in Errata 2.6.1.
    /// Register the channel afterwards:
    /// `intc::register(pin as u16, Line::new(<line>)…, prio)`.
    pub unsafe fn new(port: crate::gpio::Port, pin: u8, trigger: crate::intc::EirqTrigger) -> Self {
        // SAFETY: The caller exclusively owns this pin and its external-interrupt channel.
        unsafe { crate::gpio::configure_exti_input(port, pin) };
        critical_section::with(|_| {
            crate::intc::configure_eirq(pin, trigger);
            crate::intc::eirq_clear(pin);
            take_edge(pin);
        });
        Self { port, pin }
    }

    /// Consume an already-owned input pin and configure its EXTI channel.
    /// # Safety
    /// Caller must meet the channel, NVIC and oscillator requirements of
    /// [`Self::new`] and register INTC after this call.
    pub unsafe fn from_input(
        input: crate::gpio::InputPin,
        trigger: crate::intc::EirqTrigger,
    ) -> Self {
        let (port, pin) = input.into_parts();
        // SAFETY: Consuming the input transfers pin ownership; the caller owns the channel.
        unsafe { Self::new(port, pin, trigger) }
    }

    /// Current logic level of the pin.
    ///
    /// An external-interrupt event is a pulse; a driver that must act once per
    /// real event should confirm the level after the edge and then require the
    /// pin to be idle before re-arming (a sensor or switch can produce a burst
    /// of edges for one physical movement).
    pub fn is_high(&self) -> bool {
        let value = crate::gpio::read_port(self.port);
        value & (1 << self.pin) != 0
    }

    /// Wait until the pin produces a configured edge (or level) event.
    ///
    /// Registers the waker *before* checking the flag, so an interrupt that
    /// arrives between two polls cannot be lost.
    pub async fn wait_for_edge(&mut self) {
        WaitEdge { pin: self.pin }.await
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    fn arm_edge(&mut self, trigger: crate::intc::EirqTrigger) {
        // Change only EIRQCR, not GPIO.INTE (the latter requires disabled NVIC
        // routes per Errata 2.4.2). Clear inherited events before this new wait.
        critical_section::with(|_| {
            crate::intc::configure_eirq(self.pin, trigger);
            crate::intc::eirq_clear(self.pin);
            take_edge(self.pin);
        });
    }

    /// Wait for a new rising edge, ignoring previously latched events.
    pub async fn wait_for_rising_edge(&mut self) {
        self.arm_edge(crate::intc::EirqTrigger::Rising);
        self.wait_for_edge().await;
    }

    /// Wait for a new falling edge, ignoring previously latched events.
    pub async fn wait_for_falling_edge(&mut self) {
        self.arm_edge(crate::intc::EirqTrigger::Falling);
        self.wait_for_edge().await;
    }

    pub async fn wait_for_any_edge(&mut self) {
        self.arm_edge(crate::intc::EirqTrigger::Both);
        self.wait_for_edge().await;
    }

    /// Return immediately if high, otherwise latch the next rising edge even
    /// if the signal falls again before the executor polls this task.
    pub async fn wait_for_high(&mut self) {
        self.arm_edge(crate::intc::EirqTrigger::Rising);
        if !self.is_high() {
            self.wait_for_edge().await;
        }
    }

    pub async fn wait_for_low(&mut self) {
        self.arm_edge(crate::intc::EirqTrigger::Falling);
        if !self.is_low() {
            self.wait_for_edge().await;
        }
    }

    /// Whether an edge is already latched and not yet consumed.
    pub fn is_pending(&self) -> bool {
        EDGE.load(core::sync::atomic::Ordering::Relaxed) & (1 << self.pin) != 0
    }
}

impl embedded_hal::digital::ErrorType for ExtiInput {
    type Error = core::convert::Infallible;
}

impl embedded_hal::digital::InputPin for ExtiInput {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(ExtiInput::is_high(self))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(ExtiInput::is_low(self))
    }
}

impl embedded_hal_async::digital::Wait for ExtiInput {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        ExtiInput::wait_for_high(self).await;
        Ok(())
    }
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        ExtiInput::wait_for_low(self).await;
        Ok(())
    }
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        ExtiInput::wait_for_rising_edge(self).await;
        Ok(())
    }
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        ExtiInput::wait_for_falling_edge(self).await;
        Ok(())
    }
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        ExtiInput::wait_for_any_edge(self).await;
        Ok(())
    }
}

struct WaitEdge {
    pin: u8,
}

impl Future for WaitEdge {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Register first, then consume: an ISR between the two only sets the
        // latch, which the next poll picks up.
        WAKERS[self.pin as usize].register(cx.waker());
        if take_edge(self.pin) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests;
