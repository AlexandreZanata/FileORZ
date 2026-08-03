#!/usr/bin/env python3
"""Ensure docs/PARITY-REPORT.md lists every required behavior catalog ID."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REPORT = ROOT / "docs" / "PARITY-REPORT.md"

# Locked B-* ids from .local/BEHAVIOR-CATALOG.md (phase 19 matrix).
REQUIRED_IDS = (
    "B-01",
    "B-02",
    "B-03",
    "B-04",
    "B-05",
    "B-10",
    "B-11",
    "B-12",
    "B-13",
    "B-14",
    "B-20",
    "B-21",
    "B-22",
    "B-23",
    "B-30",
    "B-31",
    "B-32",
    "B-33",
    "B-34",
)

REQUIRED_SECTIONS = (
    "Intentional differences",
    "Parity matrix",
    "Python out of the release path",
)


def main() -> int:
    if not REPORT.is_file():
        print(f"[check-parity] FAIL — missing {REPORT.relative_to(ROOT)}", file=sys.stderr)
        return 1
    text = REPORT.read_text(encoding="utf-8")
    missing_ids = [bid for bid in REQUIRED_IDS if not re.search(rf"\b{bid}\b", text)]
    missing_sec = [s for s in REQUIRED_SECTIONS if s not in text]
    if missing_ids or missing_sec:
        if missing_ids:
            print(
                "[check-parity] FAIL — PARITY-REPORT missing IDs: "
                + ", ".join(missing_ids),
                file=sys.stderr,
            )
        if missing_sec:
            print(
                "[check-parity] FAIL — PARITY-REPORT missing sections: "
                + ", ".join(missing_sec),
                file=sys.stderr,
            )
        return 1
    print(
        f"[check-parity] OK — {len(REQUIRED_IDS)} behavior IDs + "
        f"{len(REQUIRED_SECTIONS)} sections in PARITY-REPORT"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
