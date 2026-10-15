//! Async DMA1/DMA2: event-paced byte writes to a peripheral or halfword
//! reads into memory, awaited on transfer completion. No repeat,
//! non-sequential, linked-list or reconfiguration modes.
//!
//! [`split`] clocks a unit, stops any channel left running and hands out
//! its four [`Channel`]s. [`Channel::write`] and [`Channel::read_u16`] arm
//! one transfer: every trigger event moves the next item, and the returned
//! [`Transfer`] resolves after the last one. Dropping it early stops the
//! channel, so the transfer never outlives the borrowed buffer.
//!
//! The application routes each used channel's [`Unit::complete_event`]
//! and its unit's [`Unit::error_event`] to NVIC lines inside their group
//! window (see [`crate::intc`]), and calls [`on_interrupt`] from those
//! lines' handlers, before the first transfer.
//!
//! Only one transfer per unit may run: another channel returns [`Error::Busy`].
//! This avoids enabling a channel while another nears its end, which needs the
//! wait the DDL's `DMA_ChannelCmd` performs. Cancellation waits for quiescence;
//! a wedged bus can therefore stall cancellation rather than release live memory.
//!
//! Registers are reached through the PAC (vendor SVD `HC32F460JEUA.svd`);
//! values and sequences follow the vendor DDL (`hc32f460_dmac.c/.h`,
//! `hc32f460_pwc.c`), unless cited otherwise.

use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::{Context, Poll};

use embassy_sync::waitqueue::AtomicWaker;

use crate::pac;
use crate::pac::events::EventSource;

/// Channels per unit (DDL `DmaChMax`).
pub const CHANNELS: usize = 4;
/// Most items one transfer moves: DTCTL.CNT is a 16-bit count of single-item
/// blocks (SVD; DDL `IS_VALID_TRNCNT`).
pub const MAX_TRANSFER: usize = u16::MAX as usize;

// SVD: each channel's registers repeat every 0x40 bytes (SAR0 at 0x40, SAR1
// at 0x80; DDL `_DMA_CH_REG_OFFSET`).
const CHANNEL_STRIDE: usize = 0x40;
// DDL `AddressFix` and `AddressIncrease` (CHCTL.SINC/DINC).
const ADDRESS_FIXED: u8 = 0;
const ADDRESS_INCREMENT: u8 = 1;
// DDL `Dma8Bit` (CHCTL.HSIZE).
const WIDTH_8: u8 = 0;
// DDL `Dma16Bit` (CHCTL.HSIZE): one aligned halfword per item.
const WIDTH_16: u8 = 1;
// One item per trigger event (DTCTL.BLKSIZE; 0 would mean 1024, DDL
// `stc_dma_config_t`).
const BLOCK_SIZE: u16 = 1;

/// The supported transfer shapes: which side advances and the item width.
#[derive(Clone, Copy)]
enum Kind {
    /// Memory bytes into a fixed peripheral register.
    WriteBytes,
    /// A fixed peripheral halfword register into memory.
    ReadHalfwords,
}

impl Kind {
    /// CHCTL (SINC, DINC, HSIZE) for this shape.
    const fn fields(self) -> (u8, u8, u8) {
        match self {
            Self::WriteBytes => (ADDRESS_INCREMENT, ADDRESS_FIXED, WIDTH_8),
            Self::ReadHalfwords => (ADDRESS_FIXED, ADDRESS_INCREMENT, WIDTH_16),
        }
    }
}
// DDL `EVT_MAX`: a trigger selection no event drives, set while unused.
const NO_TRIGGER: u16 = 0x1ff;
// FCG0PC: DDL `ENABLE_FCG0_REG_WRITE`/`DISABLE_FCG0_REG_WRITE` write key
// 0xA5A5 (FCG0PCWE) with PRT0 set to unlock FCG0, clear to lock it again.
const FCG0_KEY: u16 = 0xa5a5;
// INTSTAT0/INTMASK0/INTCLR0 (SVD): channel n's TRNERR flag is bit n and its
// REQERR flag bit 16 + n.
const REQUEST_ERROR_SHIFT: usize = 16;
// The per-channel 4-bit flag fields (INTSTAT1.TC/BTC, INTCLR1, INTMASK1,
// CHEN, CHSTAT.CHACT; SVD) with every channel's bit set.
const ALL_CHANNELS: u8 = (1 << CHANNELS) - 1;
/// CHSTAT reads before a stopping channel counts as stuck. The channel
/// stops after its current read or write (RM 14.3.11), so this is a generous
/// chosen bound, not a measured or specified time.
const STOP_POLLS: u32 = 200_000;

/// A DMA controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Dma1 = 0,
    Dma2 = 1,
}

impl Unit {
    /// The transfer-complete interrupt event of channel `channel`.
    ///
    /// # Panics
    /// Panics if `channel` is not below [`CHANNELS`]; in a `const`, that
    /// fails the build instead.
    pub const fn complete_event(self, channel: usize) -> EventSource {
        assert!(channel < CHANNELS, "DMA channel out of range");
        let first = match self {
            Self::Dma1 => EventSource::DMA1_TC0,
            Self::Dma2 => EventSource::DMA2_TC0,
        };
        // TC0..TC3 are consecutive events (vendor header `INT_SRC_DMAn_TCx`).
        EventSource(first.0 + channel as u16)
    }

    /// The unit's error interrupt event, raised for any of its channels.
    pub const fn error_event(self) -> EventSource {
        match self {
            Self::Dma1 => EventSource::DMA1_ERR,
            Self::Dma2 => EventSource::DMA2_ERR,
        }
    }

    fn registers(self) -> &'static pac::dma1::RegisterBlock {
        // SAFETY: the PAC's fixed register blocks, valid for the program's
        // life; channel ownership decides who writes which fields.
        unsafe {
            match self {
                Self::Dma1 => &*pac::Dma1::ptr(),
                Self::Dma2 => &*pac::Dma2::ptr(),
            }
        }
    }
}

/// What ended a transfer early.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("more than MAX_TRANSFER items")]
    TooLong,
    #[error("another channel of this DMA unit is running")]
    Busy,
    #[error("the channel hit a bus error (INTSTAT0.TRNERR)")]
    Transfer,
    #[error("a trigger came before the previous one was served (INTSTAT0.REQERR)")]
    Request,
    #[error("the channel did not stop")]
    NotStopped,
}

/// Per unit, the INTSTAT0 error bits and INTSTAT1 completion bits the
/// handler took, until the channel's transfer reads them.
static ERRORS: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
static COMPLETE: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
// A nonzero channel bit reserves the unit until its borrowed memory is quiescent.
static RUNNING: [AtomicU32; 2] = [AtomicU32::new(0), AtomicU32::new(0)];
static WAKERS: [[AtomicWaker; CHANNELS]; 2] = [
    [const { AtomicWaker::new() }; CHANNELS],
    [const { AtomicWaker::new() }; CHANNELS],
];

/// Channel `index`'s TRNERR and REQERR bits in INTSTAT0.
const fn error_bits(index: usize) -> u32 {
    (1 << index) | (1 << (REQUEST_ERROR_SHIFT + index))
}

/// Clock `unit` and the event router, enable the unit, stop any channel left
/// running (an earlier image may have left one copying) and hand out the
/// channels.
/// # Safety
/// Call once per unit. The caller gives this owner the unit's registers, its
/// four AOS `DMAn_TRGSELx` registers and PWC FCG0's `DMAn` gate; the AOS
/// gate is shared and only ever cleared.
pub unsafe fn split(unit: Unit) -> Result<[Channel; CHANNELS], Error> {
    // SAFETY: only FCG0 and its write protection are touched, under a
    // critical section, so no other gate owner interleaves.
    let pwc = unsafe { pac::Pwc::steal() };
    critical_section::with(|_| {
        // SAFETY (key writes): the key fits its 16-bit field.
        pwc.fcg0pc()
            .write(|w| unsafe { w.fcg0pcwe().bits(FCG0_KEY) }.prt0().set_bit());
        // Gates are active low: clearing a bit clocks the block.
        pwc.fcg0().modify(|_, w| {
            match unit {
                Unit::Dma1 => w.dma1().clear_bit(),
                Unit::Dma2 => w.dma2().clear_bit(),
            };
            w.aos().clear_bit()
        });
        pwc.fcg0pc()
            .write(|w| unsafe { w.fcg0pcwe().bits(FCG0_KEY) }.prt0().clear_bit());
    });
    let channels = core::array::from_fn(|index| Channel { unit, index });
    for channel in &channels {
        channel.stop()?;
    }
    // DDL `DMA_Cmd`. Stop inherited channels before enabling the unit, so a
    // stale enabled channel cannot restart against an earlier image's memory.
    unit.registers().en().write(|w| w.en().set_bit());
    RUNNING[unit as usize].store(0, Ordering::Release);
    Ok(channels)
}

/// One channel of a DMA unit.
pub struct Channel {
    unit: Unit,
    index: usize,
}

impl Channel {
    /// Copy `source` to the byte register at `destination`, one byte each
    /// time `trigger` fires. Up to [`MAX_TRANSFER`] bytes; none completes at
    /// once.
    ///
    /// The channel is armed on return: the first byte moves on the next
    /// trigger, so a trigger that marks "ready for data" (an empty transmit
    /// buffer) needs its peripheral to signal once more, for example by the
    /// caller writing the byte before `source` itself.
    /// # Safety
    /// `destination` must be a peripheral register that takes byte writes,
    /// and `trigger` an event that paces it. `source` must remain alive and
    /// immutable until the channel is quiescent, even if the transfer is
    /// forgotten. Await or drop the transfer to enforce this for a borrowed
    /// buffer; a program-lifetime immutable source also satisfies it.
    pub unsafe fn write<'a>(
        &'a mut self,
        source: &'a [u8],
        destination: *mut u8,
        trigger: EventSource,
    ) -> Result<Transfer<'a>, Error> {
        let count = u16::try_from(source.len()).map_err(|_| Error::TooLong)?;
        self.start(
            source.as_ptr(),
            destination,
            count,
            trigger,
            Kind::WriteBytes,
        )
    }

    /// Copy the fixed halfword register at `source` into `destination`, one
    /// sample per `trigger`. The channel is armed before returning.
    ///
    /// # Safety
    /// `source` must be an aligned peripheral register accepting halfword
    /// reads, paced by `trigger`. The destination must remain alive and
    /// exclusively DMA-owned until quiescence, even if the transfer is
    /// forgotten. Await or drop the transfer; never leak it with a borrowed
    /// buffer. Do not read samples before completion or cancellation.
    pub unsafe fn read_u16<'a>(
        &'a mut self,
        source: *const u16,
        destination: &'a mut [u16],
        trigger: EventSource,
    ) -> Result<Transfer<'a>, Error> {
        let count = u16::try_from(destination.len()).map_err(|_| Error::TooLong)?;
        self.start(
            source.cast(),
            destination.as_mut_ptr().cast(),
            count,
            trigger,
            Kind::ReadHalfwords,
        )
    }

    fn start(
        &mut self,
        source: *const u8,
        destination: *mut u8,
        count: u16,
        trigger: EventSource,
        kind: Kind,
    ) -> Result<Transfer<'_>, Error> {
        self.stop()?;
        if count == 0 {
            return Ok(Transfer {
                channel: self,
                running: false,
            });
        }
        RUNNING[self.unit as usize]
            .compare_exchange(0, 1 << self.index, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| Error::Busy)?;
        self.clear_flags();
        let dma = self.unit.registers();
        // SAFETY (register views below): each points into this channel's
        // own copy of the register, CHANNEL_STRIDE bytes per channel, which
        // has the same layout as channel 0's.
        let (sar, dar, dtctl, chctl) = unsafe {
            (
                &*self.channel_register(dma.sar0()),
                &*self.channel_register(dma.dar0()),
                &*self.channel_register(dma.dtctl0()),
                &*self.channel_register(dma.chctl0()),
            )
        };
        // DDL `DMA_InitChannel` order: interrupt off, addresses, size and
        // count, then the channel mode with its interrupt on.
        chctl.modify(|_, w| w.ie().clear_bit());
        // SAFETY (field writes below): each value fits its field; addresses
        // are 32-bit on this part.
        sar.write(|w| unsafe { w.bits(source as u32) });
        dar.write(|w| unsafe { w.bits(destination as u32) });
        dtctl.write(|w| unsafe { w.blksize().bits(BLOCK_SIZE).cnt().bits(count) });
        let (source_step, destination_step, width) = kind.fields();
        chctl.write(|w| unsafe {
            w.sinc()
                .bits(source_step)
                .dinc()
                .bits(destination_step)
                .hsize()
                .bits(width)
                .ie()
                .set_bit()
        });
        // Every block is one item, so the block-complete interrupt would
        // fire per sample: mask it, and unmask completion and errors (DDL
        // `DMA_EnableIrq`/`DMA_DisableIrq`).
        let bit = 1 << self.index;
        critical_section::with(|_| {
            dma.intmask0().modify(|r, w| unsafe {
                w.msktrnerr()
                    .bits(r.msktrnerr().bits() & !bit)
                    .mskreqerr()
                    .bits(r.mskreqerr().bits() & !bit)
            });
            dma.intmask1().modify(|r, w| unsafe {
                w.msktc()
                    .bits(r.msktc().bits() & !bit)
                    .mskbtc()
                    .bits(r.mskbtc().bits() | bit)
            });
        });
        self.select_trigger(trigger.0);
        // Publish source-buffer stores before DMA observes the channel enable.
        // Cortex-M4 has no data cache; the fence also orders compiler accesses.
        core::sync::atomic::fence(Ordering::SeqCst);
        // DDL `DMA_ChannelCmd`: CHEN is shared by the unit's channels.
        critical_section::with(|_| {
            dma.chen()
                .modify(|r, w| unsafe { w.chen().bits(r.chen().bits() | bit) });
        });
        Ok(Transfer {
            channel: self,
            running: true,
        })
    }

    /// Disable the channel and wait until it is no longer active.
    fn stop(&self) -> Result<(), Error> {
        let dma = self.unit.registers();
        let bit = 1 << self.index;
        critical_section::with(|_| {
            dma.chen()
                .modify(|r, w| unsafe { w.chen().bits(r.chen().bits() & !bit) });
        });
        // Ensure the disable reaches the controller before checking inactivity.
        core::sync::atomic::fence(Ordering::SeqCst);
        let stopped = (0..STOP_POLLS).any(|_| dma.chstat().read().chact().bits() & bit == 0);
        self.select_trigger(NO_TRIGGER);
        if stopped {
            Ok(())
        } else {
            Err(Error::NotStopped)
        }
    }

    /// A borrowed buffer cannot be released until the bus master is quiescent.
    /// A timeout is diagnostic, not permission to keep reading expired memory.
    fn stop_borrowed(&self) -> Result<(), Error> {
        let result = self.stop();
        if result.is_err() {
            let dma = self.unit.registers();
            let bit = 1 << self.index;
            while dma.chstat().read().chact().bits() & bit != 0 {
                core::hint::spin_loop();
            }
        }
        core::sync::atomic::fence(Ordering::SeqCst);
        RUNNING[self.unit as usize].store(0, Ordering::Release);
        result
    }

    /// Point this channel's AOS trigger at `event` (DDL `DMA_SetTriggerSrc`).
    fn select_trigger(&self, event: u16) {
        // SAFETY: the PAC's fixed AOS block; split's caller gave this
        // channel its TRGSEL register.
        let aos = unsafe { &*pac::Aos::ptr() };
        let first = match self.unit {
            Unit::Dma1 => aos.dma1_trgsel0().as_ptr(),
            Unit::Dma2 => aos.dma2_trgsel0().as_ptr(),
        };
        // SAFETY: DMAn_TRGSEL0..3 are consecutive words (SVD offsets
        // 0x14..0x30; DDL `WRITE_DMA_CH_TRGSEL`), and the index is below 4.
        unsafe { first.add(self.index).write_volatile(u32::from(event)) };
    }

    /// This channel's copy of a channel-0 register.
    fn channel_register<T>(&self, channel_0: &T) -> *const T {
        let address: *const T = channel_0;
        address.wrapping_byte_add(self.index * CHANNEL_STRIDE)
    }

    /// Clear this channel's flags, in hardware and as recorded, so a stale
    /// one cannot end the next transfer.
    fn clear_flags(&self) {
        critical_section::with(|_| {
            let dma = self.unit.registers();
            let bit = 1 << self.index;
            // SAFETY: INTCLR0/1 only clear the flags written as one.
            dma.intclr0()
                .write(|w| unsafe { w.clrtrnerr().bits(bit).clrreqerr().bits(bit) });
            dma.intclr1()
                .write(|w| unsafe { w.clrtc().bits(bit).clrbtc().bits(bit) });
            self.take_flags();
        });
    }

    /// Take and clear the flags the handler recorded for this channel.
    fn take_flags(&self) -> (bool, u32) {
        let unit = self.unit as usize;
        let bit = 1 << self.index;
        let complete = COMPLETE[unit].fetch_and(!bit, Ordering::AcqRel) & bit != 0;
        let errors = ERRORS[unit].fetch_and(!error_bits(self.index), Ordering::AcqRel)
            & error_bits(self.index);
        (complete, errors)
    }
}

/// A running DMA write or read; resolves after the last item.
/// Dropping it first stops the channel and waits until it no longer accesses
/// the buffer. A permanently wedged DMA bus stalls instead of returning
/// borrowed memory to its owner while it is still being accessed.
#[must_use = "a DMA transfer must be awaited or dropped, never leaked"]
pub struct Transfer<'a> {
    channel: &'a mut Channel,
    running: bool,
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if !this.running {
            return Poll::Ready(Ok(()));
        }
        // Register before reading the flags, so a completion between the two
        // still wakes this task.
        WAKERS[this.channel.unit as usize][this.channel.index].register(cx.waker());
        let (complete, errors) = this.channel.take_flags();
        let result = if errors & (1 << this.channel.index) != 0 {
            Err(Error::Transfer)
        } else if errors != 0 {
            Err(Error::Request)
        } else if complete {
            Ok(())
        } else {
            return Poll::Pending;
        };
        // The channel disables itself once its count runs out; after an
        // error it may not, so stop it either way.
        let stopped = this.channel.stop_borrowed();
        this.running = false;
        Poll::Ready(result.and(stopped))
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        if self.running {
            let _ = self.channel.stop_borrowed();
        }
    }
}

/// Call from the handlers of the lines a unit's completion and error events
/// are routed to. The flags stay set until cleared, so the handler records
/// and clears them (DDL `DMA_ClearIrqFlag`), then wakes their channels.
pub fn on_interrupt(unit: Unit) {
    let dma = unit.registers();
    let errors = dma.intstat0().read().bits();
    let complete = u32::from(dma.intstat1().read().tc().bits());
    // SAFETY: INTCLR0/1 only clear the flags written as one; these are the
    // flags just read.
    dma.intclr0().write(|w| unsafe { w.bits(errors) });
    dma.intclr1()
        .write(|w| unsafe { w.clrtc().bits(complete as u8).clrbtc().bits(ALL_CHANNELS) });
    ERRORS[unit as usize].fetch_or(errors, Ordering::AcqRel);
    COMPLETE[unit as usize].fetch_or(complete, Ordering::AcqRel);
    for (index, waker) in WAKERS[unit as usize].iter().enumerate() {
        if (errors & error_bits(index)) | (complete & (1 << index)) != 0 {
            waker.wake();
        }
    }
}
