# Architecture and migration

## Boundaries

- `pac/` is generated register access, not a qualified HAL. Its generation
  inputs and vendor licensing caveat remain unchanged. Runtime macro exports
  are reproduced by `tools/generate_pac.py`.
- `embassy/` owns peripheral state machines, interrupt wake protocols and
  standard embedded traits. It stays `no_std`, with no allocator.
- The board owns reset/linker code, pin-function validity, actual clock rates,
  power/SRAM configuration and the shared crystal. The HAL does not infer them.
- The application owns its Embassy executor, task priorities, shared-bus mutexes
  and physical-device recovery. `embassy-sync` already supplies synchronization;
  this repository does not duplicate it.

## Ownership patterns

Use `pac::Peripherals::take()` rather than duplicating tokens with `steal`.
The HAL enables the PAC's critical-section-based singleton acquisition.

`gpio::split` consumes the GPIO token. After its board startup contract is met,
`Pins::take(port, number)` yields a unique unconfigured pin. Conversion consumes
that pin into input/output/analog/alternate ownership. Pins are neither cloned
nor silently reallocated on drop. Alternate-function validity remains an
explicit unsafe board obligation. The runtime pin representation matches the
existing HAL; it does not generate a large package-specific type matrix.

`AdcPin::from_pin` checks the pin's ADC2 identity before making it analog.
`I2c3::from_peripheral` consumes the I²C token and its alternate pins. SPI3 and
UART consume their PAC tokens; UART callers must retain ownership of configured
pins (the integration template does this). Legacy raw constructors remain
unsafe, allowing incremental migration without duplicating a second HAL.

`intc::Route` owns a routed vector. Its destructor disables NVIC and releases
SEL. Stop/mask the peripheral source before dropping its route. SPI3 retains
three routes; partial initialization automatically rolls earlier routes back.
UART leaves line choices to the board. The template uses unrestricted lines
0–2; these are choices, not fixed UART assignments.

## Async contract

Register a waker before checking readiness, and either recheck after arming or
use persistent hardware flags. Mask level-triggered sources in the handler
before waking, preventing ISR tail-chaining from starving the executor.

No ISR in this HAL dereferences an application's borrowed buffer. EXTI records
an atomic software edge latch because clearing EIFR alone would erase the event
before the task runs. Standard `Wait` methods discard old events, select the
requested edge, and preserve short pulses even if the level changes again
before the executor polls. The legacy `wait_for_edge` consumes already-latched
configured events. Standard waits leave their selected edge configured; they do
not restore an earlier trigger after completion or cancellation.

| Operation | Cancellation behavior |
| --- | --- |
| GPIO EXTI | A pending wait consumes no event; its channel stays configured. A new standard edge wait clears old events. |
| UART read | Pending means no bytes consumed; RX interrupt source is masked on drop. |
| UART write | May have sent a prefix; masks its wait source on drop. `flush` fences the last frame. |
| I²C transaction | Masks sources, but does not guarantee STOP. `recover` resets peripheral state, not physical SDA/SCL wiring. |
| ADC read | Masks end-of-scan and requests scan stop on every exit; the next read reports a failed stop. |
| SPI write | Masks wait sources; already-copied DR data can finish. Fence with `flush` before changing control pins. |
| DMA | Disables the channel and waits for quiescence before releasing a borrowed source. A wedged bus can stall. |

### DMA lifetime safety

Destructors are not guaranteed to run: `core::mem::forget` is safe Rust. A future
that starts DMA against a borrowed slice cannot be made safe just by stopping
the channel in `Drop`. Forgetting that future can end the borrow while DMA keeps
reading the original storage.

Consequently:

- `Channel::write` is unsafe, with an explicit source-lifetime/immutability
  contract until the channel is quiescent.
- `Spi3::write_dma` is unsafe for the same reason. Do not forget a polled future;
  await or drop it before reusing/freeing the source.
- `Spi3::write_dma_static` safely accepts immutable program-lifetime bytes.
  Forgetting its future can leak resources, but cannot expire the DMA source.
- Channel stop timeout is diagnostic, not permission to release live memory.
  Borrowed transfer cleanup keeps waiting if necessary. Recovery from a
  permanently wedged DMA bus requires a system-level reset/watchdog policy.
- Memory fences order buffer publication before channel enable, channel disable
  before inactivity checks, and quiescence before CPU reuse. HC32F460 Cortex-M4
  has no data cache requiring cache maintenance.
- A per-unit atomic reservation enforces one running channel. Starting another
  returns `Busy`; this avoids the unimplemented DDL simultaneous-channel wait.

## Time and performance

The default SysTick driver uses millisecond Embassy ticks. Owned-token startup
checks exact divisibility, rejects zero reload, stops inherited SysTick, clears
its pending exception and permits initialization only once. The legacy unsafe
root initializer delegates to it and retains startup panic semantics.

The time queue still supports 16 independent wakers. Its next expiration is
cached: ticks increment time but only scan the queue when an alarm is due.
Scheduling a changed deadline recomputes the cache and immediately wakes expired
alarms. Disabling `time-driver` removes the global HAL driver and fixed tick-rate
feature so a board can supply a different Embassy driver.

I²C searches all eight documented clock prescalers, retaining exact integer
ratios and respecting minimum low/high times. This makes standard-mode rates
representable at normal high PCLK3 values without overclocking the bus.

UART baud setup uses integer arithmetic and the documented fractional formula,
rejecting nominal errors above 2%. It uses transmit-complete pacing because
Errata 4.5.1 prevents arbitrary TX-empty rearming with TE already enabled. This
is a correctness tradeoff, not a claim of gapless maximum-rate transmission.
There is only one byte of hardware RX storage; executor/interrupt latency can
cause overrun. A buffered or DMA UART needs a separately qualified design.

SPI has a typed master-TX mode/rate constructor, but remains TX-only. It does not
implement a fake full-duplex `SpiBus` that fabricates received data. PWM likewise
still avoids claiming standard endpoint semantics forbidden by its errata.

Release builds use thin LTO and one codegen unit. GPIO write-one aliases, SPI DMA,
and cooperative ready-path work bounds are retained. No timing/throughput claim
is made without a board measurement.

## Migration

1. Handle `Adc2::new(...)` as a `Result`, normally with `?` in board setup.
2. Replace safe-looking borrowed SPI DMA calls with explicitly justified unsafe
   calls whose futures are never leaked, or use immutable static sources.
3. Update exhaustive I²C/DMA error matches for specific causes and `Busy`.
4. Prefer owned GPIO/I²C/SysTick APIs and retain route owners alongside drivers.
5. Enable `rt` for PAC interrupt attributes. Select exactly one critical-section
   implementation and one Embassy time driver in the application.

Host tests target risky boundaries: unique pin acquisition, event latching,
exact tick/baud arithmetic, deadline caching, I²C clock/error/cancellation
behavior, empty ADC rejection and UART interrupt races/cancellation. They
simulate registers; the board acceptance checklist remains mandatory.
