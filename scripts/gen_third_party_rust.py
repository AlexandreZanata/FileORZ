#!/usr/bin/env python3
"""Regenerate docs/THIRD_PARTY_RUST.md from Cargo.lock + local registry."""

from __future__ import annotations

import re
from collections import defaultdict
from pathlib import Path

import tomllib

ROOT = Path(__file__).resolve().parents[1]
LOCK = ROOT / "Cargo.lock"
OUT = ROOT / "docs" / "THIRD_PARTY_RUST.md"
REG = Path.home() / ".cargo" / "registry" / "src"


def parse_lock_packages() -> list[tuple[str, str]]:
    text = LOCK.read_text(encoding="utf-8")
    rows: list[tuple[str, str]] = []
    for block in re.split(r"\n\[\[package\]\]\n", text)[1:]:
        name_m = re.search(r'^name = "([^"]+)"', block, re.M)
        ver_m = re.search(r'^version = "([^"]+)"', block, re.M)
        src_m = re.search(r'^source = "([^"]+)"', block, re.M)
        if name_m and ver_m and src_m:
            rows.append((name_m.group(1), ver_m.group(1)))
    return rows


def license_for(name: str, version: str) -> str:
    for idx in REG.glob("*"):
        path = idx / f"{name}-{version}" / "Cargo.toml"
        if path.is_file():
            data = tomllib.loads(path.read_text(encoding="utf-8"))
            pkg = data.get("package", {})
            return str(pkg.get("license") or pkg.get("license-file") or "UNKNOWN")
    return "MISSING-FROM-REGISTRY"


def render(rows: list[tuple[str, str, str]]) -> str:
    by_lic: dict[str, list[tuple[str, str]]] = defaultdict(list)
    missing = 0
    for name, ver, lic in rows:
        if lic == "MISSING-FROM-REGISTRY":
            missing += 1
        by_lic[lic].append((name, ver))

    lines = [
        "# Third-party Rust crates",
        "",
        "License inventory for the FileORZ Cargo workspace (transitive deps).",
        "FileORZ itself is **GPL-3.0-or-later** — see [LICENSE](../LICENSE) and",
        "[THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).",
        "",
        "Regenerate:",
        "",
        "```bash",
        "python3 scripts/gen_third_party_rust.py",
        "# or: cargo about generate  (if cargo-about is installed)",
        "```",
        "",
        f"Lockfile packages with `source`: **{len(rows)}**",
        f"(registry lookup misses, often macOS-only: **{missing}**).",
        "",
        "## Summary by license expression",
        "",
        "| Count | License |",
        "|------:|---------|",
    ]
    for lic, crates in sorted(by_lic.items(), key=lambda kv: (-len(kv[1]), kv[0])):
        lines.append(f"| {len(crates)} | `{lic}` |")

    lines.extend(
        [
            "",
            "## Full list",
            "",
            "| Crate | Version | License |",
            "|-------|---------|---------|",
        ]
    )
    for name, ver, lic in sorted(rows, key=lambda r: (r[0].lower(), r[1])):
        lines.append(f"| `{name}` | {ver} | `{lic}` |")

    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- Linux release links only crates needed for `x86_64-unknown-linux-gnu`;",
            "  some lockfile entries (e.g. `objc2-*`) are other-target transitive noise.",
            "- Prefer permissive dual-licensed crates; review any copyleft-only additions",
            "  before shipping a release tarball.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    pkgs = parse_lock_packages()
    rows = [(n, v, license_for(n, v)) for n, v in pkgs]
    OUT.write_text(render(rows), encoding="utf-8")
    print(f"[gen_third_party_rust] wrote {OUT.relative_to(ROOT)} ({len(rows)} crates)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
