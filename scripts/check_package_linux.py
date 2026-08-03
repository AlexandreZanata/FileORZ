#!/usr/bin/env python3
"""Validate Linux packaging sources (desktop entry + icons + script)."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PKG = ROOT / "packaging" / "linux"
DESKTOP = PKG / "fileorz.desktop"
SCRIPT = ROOT / "scripts" / "package-linux.sh"
REQUIRED_DESKTOP_KEYS = (
    "Type=Application",
    "Name=FileORZ",
    "Exec=fileorz",
    "Icon=fileorz",
    "Categories=Utility;Filesystem;",
)
ICON_SIZES = (48, 128, 256)


def fail(msg: str) -> None:
    print(f"[check-package-linux] FAIL — {msg}", file=sys.stderr)
    raise SystemExit(1)


def check_desktop() -> None:
    if not DESKTOP.is_file():
        fail(f"missing {DESKTOP.relative_to(ROOT)}")
    body = DESKTOP.read_text(encoding="utf-8")
    if not body.startswith("[Desktop Entry]"):
        fail("desktop file missing [Desktop Entry] header")
    for key in REQUIRED_DESKTOP_KEYS:
        if key not in body:
            fail(f"desktop file missing `{key}`")


def check_icons() -> None:
    for size in ICON_SIZES:
        path = PKG / "icons" / "hicolor" / f"{size}x{size}" / "apps" / "fileorz.png"
        if not path.is_file():
            fail(f"missing icon {path.relative_to(ROOT)}")
        if path.stat().st_size < 64:
            fail(f"icon too small: {path.relative_to(ROOT)}")
        magic = path.read_bytes()[:8]
        if magic != b"\x89PNG\r\n\x1a\n":
            fail(f"not a PNG: {path.relative_to(ROOT)}")


def check_script() -> None:
    if not SCRIPT.is_file():
        fail(f"missing {SCRIPT.relative_to(ROOT)}")
    text = SCRIPT.read_text(encoding="utf-8")
    if "rm -rf" not in text or "dist/linux" not in text:
        fail("package-linux.sh must wipe dist/linux (idempotent)")
    if "dpkg-deb" not in text:
        fail("package-linux.sh must invoke dpkg-deb")
    if "fileorz.desktop" not in text:
        fail("package-linux.sh must install desktop file")
    if "SHA256SUMS" not in text:
        fail("package-linux.sh must write SHA256SUMS")


def check_docs() -> None:
    for rel in (
        "docs/INSTALL-LINUX.md",
        "docs/THIRD_PARTY_RUST.md",
        "docs/RELEASE-NOTES-LINUX-v1.md",
        "docs/HANDOVER-LINUX.md",
        "LICENSE",
    ):
        path = ROOT / rel
        if not path.is_file():
            fail(f"missing {rel}")
    install = (ROOT / "docs" / "INSTALL-LINUX.md").read_text(encoding="utf-8")
    for needle in ("GPL", "Corresponding Source", "RUSTFLAGS", "strip"):
        if needle not in install:
            fail(f"INSTALL-LINUX.md missing `{needle}` section/term")


def main() -> int:
    check_desktop()
    check_icons()
    check_script()
    check_docs()
    print("[check-package-linux] OK — desktop, icons, script, docs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
