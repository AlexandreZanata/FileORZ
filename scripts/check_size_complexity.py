#!/usr/bin/env python3
"""CLI: enforce file ≤200, function ≤80, cyclomatic ≤10.

Optional legacy exemptions (size/complexity only) live in
scripts/quality/legacy_exemptions.txt — remove paths as files are fixed.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

_SCRIPT_DIR = Path(__file__).resolve().parent
if str(_SCRIPT_DIR) not in sys.path:
    sys.path.insert(0, str(_SCRIPT_DIR))

from quality.limits import MAX_CYCLOMATIC, MAX_FILE_LINES, MAX_FUNCTION_LINES
from quality.scan import analyze_file, iter_source_files


def load_exemptions(root: Path) -> set[Path]:
    path = root / "scripts" / "quality" / "legacy_exemptions.txt"
    if not path.is_file():
        return set()
    exempt: set[Path] = set()
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        exempt.add((root / line).resolve())
    return exempt


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="*", type=Path)
    parser.add_argument("--root", type=Path, default=Path.cwd())
    parser.add_argument(
        "--no-exemptions",
        action="store_true",
        help="Ignore legacy_exemptions.txt (strict mode).",
    )
    args = parser.parse_args()
    root = args.root.resolve()
    files = iter_source_files(root, args.paths or None)
    exempt = set() if args.no_exemptions else load_exemptions(root)

    if not files:
        print("[size-complexity] No source files to check — OK")
        return 0

    findings = []
    skipped = 0
    for path in files:
        if path.resolve() in exempt:
            skipped += 1
            continue
        findings.extend(analyze_file(path))

    if findings:
        print("[size-complexity] FAILED — harness hard caps exceeded:")
        for item in findings:
            rel = (
                item.path.relative_to(root)
                if item.path.is_relative_to(root)
                else item.path
            )
            print(f"  - [{item.kind}] {rel}: {item.detail}")
        print(
            f"\nCaps: file≤{MAX_FILE_LINES}, function≤{MAX_FUNCTION_LINES}, "
            f"cyclomatic≤{MAX_CYCLOMATIC}"
        )
        return 1

    extra = f", {skipped} legacy-exempt" if skipped else ""
    print(
        f"[size-complexity] OK — {len(files)} file(s) within "
        f"file≤{MAX_FILE_LINES}, function≤{MAX_FUNCTION_LINES}, "
        f"cyclomatic≤{MAX_CYCLOMATIC}{extra}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
