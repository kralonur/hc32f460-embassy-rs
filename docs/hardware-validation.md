# Hardware acceptance checklist

None of these checks have been performed by the coding environment. Host tests,
clippy and ARM compilation do not qualify electrical timing or silicon behavior.
Document aliases below refer to the [root README](../README.md#hardware-reference-documents).

## Startup and ownership

- Confirm device/package, supply, power mode, SRAM wait states/ECC and flash
  timing for the selected clocks. Review the existing RM/Errata startup obligations.
- Apply XTAL32/PC15 reset-time workarounds before digital pin use; retain SWD.
- Boot through the actual bootloader and from reset. Verify inherited routes,
  peripheral activity and SysTick pending state do not enter application drivers.
- Check every chosen INTC event/line against the group window and confirm that
  the expected handler runs. Keep shared vectors unsupported.
- Verify drop/failed-construction releases SPI routes without enabling any
  half-initialized source. Keep source shutdown before route release.

## SysTick and executor

- Measure the 1 kHz SysTick rate against a known time reference; confirm HCLK.
- Run multiple tasks with different deadlines and repeatedly reschedule earlier
  deadlines. Observe that each is woken and idle ticks do not cause spurious work.
- Measure worst-case interrupt masking and handler/executor latency. SysTick
  cannot recover a tick lost while interrupts are masked for a millisecond.
- Test the board-provided time driver with HAL defaults disabled if low-power or
  finer resolution is required. The SysTick implementation does not cover stop mode.

## GPIO and EXTI

- Confirm pin functions and output-latch readback for each bonded pin used.
- Test rising/falling/both edge waits and short pulses that return to their prior
  level before the waiting task runs. Test immediate high/low completion.
- Cancel and restart waits; old events must not satisfy a fresh standard edge wait.
- Verify competing port INTE cleanup and the disabled-NVIC sequence in Errata 2.4.
- The legacy low-level EXTI trigger needs source deassertion to avoid interrupt
  retriggering; it is not a latched/debounced interrupt service. Standard async
  high/low waits use edges plus a level check, not a continuously asserted source.

## UART

- Loop back each unit actually used. Validate data order at intended clock/baud,
  including low baud rates requiring PR prescaling and fractional baud settings.
- Measure realized bit period and oscillator tolerance; nominal arithmetic alone
  is insufficient. Exercise parity and both stop-bit configurations.
- Verify RI, EI and TCI routing. Confirm transmit-complete waits wake reliably
  without TX-empty rearming (Errata 4.5.1).
- Force framing/parity/overrun errors; check they are returned and subsequent
  receives resume. Measure sustained RX limits under the real executor load.
- Run TX and RX halves concurrently. Cancel pending reads/writes and fence TX
  using `flush` before changing pin configuration or dropping the driver.

## I²C, ADC and SPI/DMA

- Scope standard/fast I²C low/high periods and rise/fall times with actual pull-ups
  and capacitance. Test all prescalers used by board clock/rate combinations.
- Exercise NACK, arbitration loss, disconnected/stuck target and cancellation.
  Validate peripheral recovery separately from physical bus recovery. Reproduce
  the previously observed receive timeout; its silicon cause remains unresolved.
- Verify ADC scan ordering, sampling limits, averaging, reference settling and
  full-scale behavior against the current datasheet/supply conditions. Cancel
  conversion and confirm the next scan does not read stale values.
- Scope SPI mode, maximum clock, frame gaps and the final-byte idle fence.
- Exercise DMA empty/one-byte/chunk-boundary transfers, cancellation near an
  event, completion/error interrupts, concurrent-channel `Busy` and restart.
- Confirm CHACT clears only after source accesses stop; verify no stale completion
  satisfies the next transfer. Test memory publication/quiescence barriers under
  optimized release builds. Never release a borrowed source on stop timeout.
- Provide a watchdog/reset strategy for a permanently stuck DMA bus. The driver
  intentionally cannot return memory still being accessed by the bus master.

USB, RTC/low-power time, buffered/DMA UART, additional I²C/SPI/ADC instances,
full-duplex SPI and other peripherals are still outside this implementation.
Add them only with qualified register sequences, ownership, cancellation and
hardware validation rather than empty trait implementations.
