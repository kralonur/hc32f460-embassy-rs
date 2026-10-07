//! Async external interrupts (PORT `EIRQ` channels) for HC32F460.
//!
//! HC32 routes every port's pin *n* onto the same external-interrupt channel
//! *n*; the channel is then selected onto an NVIC line through `INTC.SELn`
//! (see [`crate::intc`]). This module provides the async half: the application's
//! interrupt handler calls [`on_interrupt`], which clears the `EIFR` flag and
//! wakes the task waiting on that channel.
//!
//! Enable order follows Errata Rev 1.41 §2.4.2 (peripheral side first, then
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
//! waker is registered, so no edge can be lost in either direction.

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
    EDGE.fetch_or(1 << pin, core::sync::atomic::Ordering::Release);
    wake(pin);
}

/// Take the latched edge for `pin`, if any.
pub fn take_edge(pin: u8) -> bool {
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
    /// The caller must own the pin and must register the channel afterwards:
    /// `intc::register(pin as u16, Line::new(<line>)…, prio)`.
    pub unsafe fn new(port: crate::gpio::Port, pin: u8, trigger: crate::intc::EirqTrigger) -> Self {
        // SAFETY: The caller exclusively owns this pin and its external-interrupt channel.
        unsafe { crate::gpio::configure_exti_input(port, pin) };
        crate::intc::configure_eirq(pin, trigger);
        Self { port, pin }
    }

    /// Consume an already-owned input pin and configure its EXTI channel.
    /// # Safety
    /// Caller owns the channel and must register INTC after this call.
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
    /// Regististers the waker *before* checking the flag, so an interrupt that
    /// arrives between two polls cannot be lost.
    pub async fn wait_for_edge(&mut self) {
        WaitEdge { pin: self.pin }.await
    }

    /// Whether an edge is already latched and not yet consumed.
    pub fn is_pending(&self) -> bool {
        EDGE.load(core::sync::atomic::Ordering::Relaxed) & (1 << self.pin) != 0
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
