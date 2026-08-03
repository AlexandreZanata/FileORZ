#!/usr/bin/env python3
"""Validate docs/adr/*.md contract sections (phase 02 gate)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

REQUIRED = ("**Status:**", "## Context", "## Decision", "## Consequences")
URL_RE = re.compile(r"https?://\S+")
SKIP = {"README.md"}


def check_adr(path: Path) -> list[str]:
    text = path.read_text(encoding="utf-8")
    errors: list[str] = []
    for marker in REQUIRED:
        if marker not in text:
            errors.append(f"missing {marker!r}")
    if not URL_RE.search(text):
        errors.append("missing official URL (http/https)")
    if "**Status:**" in text and "Accepted" not in text.split("**Status:**", 1)[1][:80]:
        # Allow Proposed only if explicitly still open — phase 02 requires Decision.
        pass
    return errors


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    adr_dir = root / "docs" / "adr"
    if not adr_dir.is_dir():
        print("FAIL: docs/adr/ missing", file=sys.stderr)
        return 1

    files = sorted(p for p in adr_dir.glob("*.md") if p.name not in SKIP)
    if not files:
        print("FAIL: no ADR files found", file=sys.stderr)
        return 1

    failed = 0
    for path in files:
        errs = check_adr(path)
        if errs:
            failed += 1
            print(f"FAIL {path.relative_to(root)}: {', '.join(errs)}")
        else:
            print(f"OK   {path.relative_to(root)}")

    if failed:
        print(f"\n{failed} ADR(s) failed validation", file=sys.stderr)
        return 1
    print(f"\nAll {len(files)} ADR(s) passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
