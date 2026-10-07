#!/usr/bin/env python3
"""Regenerate the PAC with svd2rust 0.37.1, form 0.13.0, and rustfmt on PATH."""

from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent
PAC = ROOT / "pac"
SVD = ROOT / "svd/HC32F460JEUA.svd"
HEADER = ROOT / "svd/HC32F460JEUA.h"
# The vendor header defines 144 HC32F460 external vectors, INT000 through INT143.
INTERRUPT_COUNT = 144


def generate_events() -> str:
    matches = re.findall(
        r"^\s*(INT_SRC_[A-Za-z0-9_]+)\s*=\s*([0-9]+)U?,",
        HEADER.read_text(),
        re.MULTILINE,
    )
    lines = [
        "//! Peripheral event sources for INTC routing on HC32F460.",
        "#![allow(non_upper_case_globals)]",
        "",
        "#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]",
        "#[repr(transparent)]",
        "pub struct EventSource(pub u16);",
        "",
        "impl EventSource {",
    ]
    seen = set()
    for name, value in matches:
        if name in seen:
            continue
        seen.add(name)
        short_name = name.removeprefix("INT_SRC_")
        lines.extend([
            f"    /// {name} = {value}",
            f"    pub const {short_name}: EventSource = EventSource({value});",
        ])
    lines.extend([
        "}",
        "",
        "impl EventSource {",
        "    #[inline]",
        "    pub const fn id(self) -> u16 { self.0 }",
        "}",
        "",
        "impl From<u16> for EventSource {",
        "    #[inline]",
        "    fn from(val: u16) -> Self { EventSource(val) }",
        "}",
        "",
        "impl From<EventSource> for u16 {",
        "    #[inline]",
        "    fn from(evt: EventSource) -> Self { evt.0 }",
        "}",
    ])
    return "\n".join(lines) + "\n"


def main() -> None:
    for tool in ("svd2rust", "form", "rustfmt"):
        subprocess.run([tool, "--version"], check=True)

    with tempfile.TemporaryDirectory(prefix="hc32-pac-") as directory:
        work = Path(directory)
        tree = ET.parse(SVD)
        intc = tree.getroot().find("./peripherals/peripheral[name='INTC']")
        if intc is None:
            raise ValueError("vendor SVD does not contain INTC")
        for number in range(INTERRUPT_COUNT):
            interrupt = ET.SubElement(intc, "interrupt")
            ET.SubElement(interrupt, "name").text = f"INT{number:03d}"
            ET.SubElement(interrupt, "value").text = str(number)
            ET.SubElement(interrupt, "description").text = (
                f"External interrupt {number:03d} (routed dynamically via INTC matrix)"
            )
        augmented = work / SVD.name
        tree.write(augmented)
        subprocess.run(
            ["svd2rust", "-i", str(augmented), "--target", "cortex-m", "--edition", "2024"],
            cwd=work,
            check=True,
        )
        src = work / "src"
        subprocess.run(["form", "-i", str(work / "lib.rs"), "-o", str(src)], check=True)
        lib = src / "lib.rs"
        lib.write_text(
            "#![allow(warnings)]\n" + lib.read_text()
            + '\npub mod events;\n\n#[cfg(feature = "rt")]\n'
            + "pub use self::Interrupt as interrupt;\n"
        )
        (src / "events.rs").write_text(generate_events())
        subprocess.run(
            ["rustfmt", "--edition", "2024", str(lib), str(work / "build.rs")],
            check=True,
        )
        device = work / "device.x"
        device.write_text(device.read_text().rstrip() + "\n")
        # Replace generated files only after every tool succeeds; preserve Cargo.toml.
        shutil.rmtree(PAC / "src")
        shutil.move(str(src), PAC / "src")
        for name in ("build.rs", "device.x"):
            shutil.copyfile(work / name, PAC / name)
    print(f"Generated {PAC}")


if __name__ == "__main__":
    main()
