# hc32f460-embassy-rs

Rust support for the HC32F460 microcontroller: a peripheral access crate
(`hc32f460-pac`, in `pac/`) generated from the vendor SVD, and an Embassy HAL
(`embassy-hc32`, in `embassy/`) on top of it.

> **Warning:** this is not a complete Embassy HAL implementation. It contains
> only the functionality I used in my own project. Peripheral support and APIs
> are incomplete; review the implementation and validate it on your hardware
> before relying on it.

## Features

- **Clocks:** HRC and MPLL startup with checked dividers and bus limits.
- **Interrupts:** INTC routing of peripheral events to NVIC vectors.
- **GPIO** and **async EXTI** external interrupts.
- **Time:** an Embassy time driver on SysTick.
- **TimerA4 PWM.**
- **Async I²C3** master.
- **Transmit-only SPI3** master.
- **PAC:** every register of the vendor SVD, plus the 144 INTC vectors and
  the interrupt-event identifiers the SVD lacks.

## Limitations

- Only the peripherals above are implemented; the rest exist only as PAC
  registers, unaudited.
- I²C: a read followed by a repeated-start write is refused
  (`Error::ReadRestart`); write-then-read is supported.
- SPI: one-frame, 8-bit master transmit only; no receive, no slave mode, and
  gaps between frames are unavoidable.
- No stop or power-down mode support; SysTick time stops while its clock does.
- The board owns power mode, SRAM wait states, memory placement, the shared
  32 kHz crystal and the linker/startup code (see the obligations under
  [Hardware reference documents](#hardware-reference-documents)).
- Host tests simulate the registers; they do not prove timing or behaviour on
  silicon.

## Hardware reference documents

Document names, revisions, languages, and links are defined here only. Source
comments use the following aliases followed by a section, table, or symbol.
The revisions below are the newest listed on the
[official HC32F460 product page](https://www.xhsc.com.cn/product/1246.html)
at this review.

Prefer English when the cited content agrees with the latest Chinese edition;
use the Chinese edition for changed or added content. Section numbers belong to
the cited edition: they must not be carried across revisions without checking.

- **RM** — **Reference Manual**, [English Rev1.5](https://oss-nc-beijing-2.cecloudcs.com/doc-rm/RM_HC32F460_F45x_A460SeriesReferenceManual_Rev1.5.pdf):
  register definitions, clocking, and peripheral behavior unchanged in the newer
  Chinese edition.
- **RM-ZH** — the latest **Reference Manual**, [Chinese Rev1.71](https://oss-nc-beijing-2.cecloudcs.com/doc-rm/RM_HC32F460_F45x_A460%E7%B3%BB%E5%88%97%E5%8F%82%E8%80%83%E6%89%8B%E5%86%8C_Rev1.71.pdf):
  authority for additions and corrections since English Rev1.5.
- **Errata** — [Chinese Rev1.41](https://oss-nc-beijing-2.cecloudcs.com/doc-es/ES_HC32F460_F451_F452_A460%E7%B3%BB%E5%88%97%E5%8B%98%E8%AF%AF%E8%A1%A8_Rev1.41.pdf):
  hardware defects and required workarounds; no English edition is listed.
- **DS** — **Datasheet**, [English Rev1.61](https://oss-nc-beijing-2.cecloudcs.com/doc-ds/DS_HC32F460SeriesDatasheet_Rev1.61.pdf):
  device capabilities, pin functions, and electrical limits. The listed
  [Chinese Rev1.61](https://oss-nc-beijing-2.cecloudcs.com/doc-ds/DS_HC32F460%E7%B3%BB%E5%88%97%E6%95%B0%E6%8D%AE%E6%89%8B%E5%86%8C_Rev1.61.pdf)
  has the same revision and revision date; the existing clock-limit citations
  use the English edition.
- **DDL** — **Device Driver Library**, [Rev3.3.0](https://oss-nc-beijing-2.cecloudcs.com/doc-hc/HC32F460_DDL_Rev3.3.0.zip):
  vendor C drivers and examples used to cross-check sequences; not included here.
- **SVD** — **System View Description**, [`svd/HC32F460JEUA.svd`](svd/HC32F460JEUA.svd):
  the committed machine-readable register description used to generate the PAC.
  The accompanying [CMSIS header](svd/HC32F460JEUA.h) supplies register masks and
  interrupt-event identifiers. These are pinned generation inputs, not a claim
  that they are the newest vendor files; see PAC generation and licensing below.

For example, `RM 9.4.10` cites the English GPIO write-protection register;
`RM-ZH 4.4` cites the newer Chinese clock constraints. Do not repeat document
URLs, revision numbers, or alias definitions in source files.

Changes relevant to existing HAL citations, checked against the manuals and
Chinese revision history:

- **Clocks:** `RM-ZH 4.4` includes the PCLK2:PCLK4 relationship and requires
  PCLK3 > SWDTCLK when SWDT is used. The clock API does not configure SWDT.
- **GPIO:** `RM-ZH 9.4.12` adds the requirement to stop the external low-speed
  oscillator before using PC14/PC15 as digital pins.
- **INTC:** `RM-ZH 10.5.7` corrects the external-interrupt flag-clear register
  name to `EIFCR`; the register behavior is unchanged.
- **TimerA:** `RM-ZH 21.5.4–21.5.5` splits BCSTR into low/high byte registers;
  the HAL now accesses them separately, matching the current DDL/header.
- **I²C:** `RM-ZH 26.5.13` corrects CCR bits 18:16 from `FREQ` to `CKDIV`;
  `RM-ZH 26.5.4` documents `SDADLY`. Older SVD/PAC field names are unchanged.

If a download link fails, use the official product page above.

### Implementation review against these documents

The handwritten HAL was reviewed against the latest Chinese manual, errata,
English datasheet, and current DDL sources/header. Relevant unchanged English
sections remain the preferred citations. This is a source/register-sequence
review, not a claim of validation on silicon or of correctness of every register
in the generated PAC.

Corrections made during the review:

- **Clocks / clock decoding:** accept MPLL input factors only in `1..=24`
  (`RM 4.11.15`), retain exact ratios before rounding frequencies, and wait for
  an inherited enabled MPLL to stabilize before stopping it (`RM 4.11.16`).
  Existing bus limits/divider constraints and SCFGR reserved-bit mirroring were
  checked. The minimum supported final PCLK3 is 234,375 Hz, so supported
  profiles satisfy the added SWDT clock constraint without an extra check.
- **GPIO / EXTI:** latch output across pin-function selection (`RM 9.4.11`,
  `RM-ZH 9.5`); clear competing INTE bits on all ports (`Errata 2.4.1`), not only
  the original board's pin subset. Halfword accesses, pin-control fields and
  EXTI trigger/flag handling were checked against the current DDL/header.
- **INTC:** failed IRQ claims no longer disable the existing owner's line.
  `IER` documentation now correctly identifies vector numbers 0..31, rather
  than peripheral event IDs (`RM 10.5.14`). Routing windows, priorities and
  the documented enable/disable order were checked.
- **PWM:** stop with START and CKDIV both cleared before resetting CNTER
  (`Errata 3.3.5`), including on restart; use the latest byte-wide control
  layout. Endpoint rejection and the divided-clock initial-level limitation
  remain explicit (`Errata 3.3.2–3.3.3`).
- **I²C:** round clock periods upward and enforce standard/fast-mode minimum
  SCL low/high times (`DS Table 3-24`), rather than assuming symmetric phases.
  Explicitly configure the one-cycle digital filter used by the calculation,
  disable slave address matches,
  preserve CCR reserved defaults, clear stale STOPF before STOP, and discard
  stale command bits after internal reset. Master initialization, normal-ACK
  receive ordering, and TRA-based receive-mode entry were checked against DDL
  and `Errata 4.1.1–4.1.2`. Read-to-write repeated starts now return
  `Error::ReadRestart` before any I/O: the documented receive workaround
  requires STOP and does not establish a safe repeated-start alternative.
  Write-to-read repeated starts remain supported.
- **SPI:** validate one-frame, 8-bit master TX configuration, enforce CFG1's
  reserved-one bit, and clear stale errors before enabling. Status/IRQ masks,
  idle fencing and event IDs were checked. Frame gaps are unavoidable
  (`Errata 4.3.1`); this driver does not provide slave-mode operation.
- **SysTick / Embassy time:** CPU-clock selection, reload bounds, millisecond
  tick accounting and multi-deadline wake queue were reviewed. No new
  chip-document discrepancy was identified in this review.

Narrow host regressions protect the changed register sequences and numeric
boundaries. They simulate MMIO; they do not prove electrical timing, interrupt
routing on silicon, or resolution of the previously observed I²C receive timeout.

Board/application obligations still apply:

- Stop XTAL32 before digital use of PC14/PC15. For PC15 input/open-drain use,
  `Errata 2.6.1` requires doing this at the beginning of Reset_Handler. The HAL
  does not own the shared crystal/RTC clock and must not silently stop it.
- Disable the channel's NVIC routes before changing EXTI pin enables
  (`Errata 2.4.2`); enforce exclusive ownership of pins, peripherals and vectors.
- Configure power mode, SRAM wait states/ECC and supply-dependent ADC limits
  before changing clocks. `Errata 2.2.1–2.2.2` also constrains memory placement
  and SRAM3 access; this workspace has no board linker/startup implementation.
- Stop/power-down entry/wakeup workarounds are not implemented by this HAL;
  SysTick time does not keep advancing while its clock is stopped. Unimplemented
  peripherals were not audited, and the PAC was not regenerated.

## PAC generation

The PAC was regenerated using **svd2rust 0.37.1**, then split into
peripheral and register modules with **form 0.13.0** and formatted with rustfmt.

`svd/HC32F460JEUA.svd` is the original vendor device description, copied
unchanged from `kws-x1-custom-rs` at revision
`6c3cd7f8ee7fc126ac6509f414f86270ba70db4e`. Its SHA-256 is
`fa52ed35f8d15eff5d6c7e5d3826715fb823896565d21b38f7baf555ac37a118`.

The standalone `tools/generate_pac.py` pipeline:

1. Adds the missing 144 INTC interrupt vectors to a temporary copy of the SVD.
2. Runs `svd2rust -i <augmented-svd> --target cortex-m --edition 2024`.
3. Runs `form -i <generated-lib.rs> -o <pac-src>`.
4. Copies generated `build.rs` and `device.x`.
5. Generates `events.rs` from `INT_SRC_*` identifiers in the vendor header
   `HC32F460JEUA.h` and adds the runtime interrupt re-export.

With svd2rust, form, and rustfmt on PATH, run:

```sh
python3 tools/generate_pac.py
```

Running svd2rust on the unmodified SVD alone will not reproduce the additional
vectors and event identifiers. The script uses the committed SVD and header,
formats the generated Rust, and preserves the workspace manifests.

## Build and validation

- Current stable Rust toolchain, Rust 2024 edition, Cargo resolver 3.
- Target `thumbv7em-none-eabihf` (`rustup target add thumbv7em-none-eabihf`).

```sh
cargo fmt --all -- --check
cargo check --workspace --locked --target thumbv7em-none-eabihf
cargo clippy --workspace --locked --target thumbv7em-none-eabihf -- -D warnings
```

## License

This project's original code is dual-licensed under either
[MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option
(`MIT OR Apache-2.0`). Third-party material is subject to its own terms.

### Vendor licensing caveat

- [`svd/HC32F460JEUA.h`](svd/HC32F460JEUA.h) declares **BSD-3-Clause**,
  copyright 2022–2023 Xiaohua Semiconductor Co., Ltd. Its notice is preserved;
  the full terms are in [`svd/LICENSE-BSD-3-Clause`](svd/LICENSE-BSD-3-Clause).
  The interrupt-event definitions in `pac/src/events.rs` are derived from this
  header; retain the vendor attribution and BSD terms when redistributing them.
- [`svd/HC32F460JEUA.svd`](svd/HC32F460JEUA.svd) has no embedded license
  notice. It matches the SVD in the vendor's
  [HDSC HC32F460 device pack 1.0.11](https://raw.githubusercontent.com/hdscmcu/pack/master/HDSC.HC32F460.1.0.11.pack).
  The pack contains a BSD license under `Examples/EmptyMain/`, but no clear
  package-wide grant covering this SVD was found. The header's BSD license
  must not be assumed to cover the SVD.
- The project's MIT/Apache choice does **not** establish redistribution
  rights for the vendor SVD or the PAC generated from it. Confirm the applicable
  vendor terms or obtain permission before redistributing those materials.
  Both crates retain `publish = false`; that blocks Cargo publication, not
  Git redistribution.
