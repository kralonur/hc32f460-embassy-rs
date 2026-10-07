# HC32F460 Embassy support

> **Warning: this is not a complete Embassy HAL implementation.** It contains
> only the functionality I used in my own project. Peripheral support and APIs
> are incomplete; review the implementation and validate it on your hardware
> before relying on it.

A standalone Rust workspace for the HC32F460 peripheral access crate (`pac/`)
and Embassy HAL (`embassy/`). The HAL includes clocks, INTC routing, GPIO,
async EXTI, SysTick time support, TimerA4 PWM, async I²C3, and transmit-only SPI3.

Both crates use the Rust 2024 edition; the workspace uses Cargo resolver 3.

## Hardware reference documents

HAL comments cite these Xiaohua/HDSC documents. Section numbers refer to the
listed revisions, not necessarily the latest available edition:

- **RM** means **Reference Manual**: [HC32F460/F45x/A460 Reference Manual,
  Rev1.5](https://oss-nc-beijing-2.cecloudcs.com/doc-rm/RM_HC32F460_F45x_A460SeriesReferenceManual_Rev1.5.pdf).
  It describes registers, bit fields, clocking, and peripheral behavior.
- **Errata**: [HC32F460/F451/F452/A460 Errata,
  Rev1.41](https://oss-nc-beijing-2.cecloudcs.com/doc-es/ES_HC32F460_F451_F452_A460%E7%B3%BB%E5%88%97%E5%8B%98%E8%AF%AF%E8%A1%A8_Rev1.41.pdf).
  It documents hardware defects and required workarounds.
- **Datasheet**: [HC32F460 Datasheet,
  Rev1.61](https://oss-nc-beijing-2.cecloudcs.com/doc-ds/DS_HC32F460SeriesDatasheet_Rev1.61.pdf).
  It covers device capabilities and electrical limits.
- **SVD**: [`svd/HC32F460JEUA.svd`](svd/HC32F460JEUA.svd), the machine-readable
  register description. The accompanying [CMSIS header](svd/HC32F460JEUA.h)
  supplies register masks and interrupt-event identifiers.
- **DDL** means **Device Driver Library**. References to Rev3.3.0 identify the
  vendor's C implementation used to cross-check driver sequences; the DDL
  itself is not included here.

For example, `RM 9.4.10` means section **9.4.10** of the reference manual
above, which documents the GPIO write-protection register and its unlock key.

The versioned PDF URLs come from the originating project's download metadata;
they are not a claim that those revisions are current or that the links remain
available. If a link fails, use the HC32F460 downloads on the
[official Xiaohua website](https://www.xhsc.com.cn/), including the DDL downloads.

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

## License

This project's original Rust code and tooling are dual-licensed under either
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
