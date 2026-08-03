#!/usr/bin/env python3
"""Characterization oracle CLI — run goldens or corrupt-self-test."""

from __future__ import annotations

import argparse
import json
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

from characterization.runner import all_goldens, run_case  # noqa: E402


def run_all() -> int:
    goldens = all_goldens()
    if not goldens:
        print("FAIL: no golden manifests", file=sys.stderr)
        return 1
    for path in goldens:
        run_case(path)
        print(f"OK  {path.name}")
    print(f"All {len(goldens)} characterization case(s) passed")
    return 0


def run_corrupt_selftest() -> int:
    """Prove the runner fails when a golden is intentionally wrong."""
    src = next(p for p in all_goldens() if p.name == "tiny-mixed.json")
    data = json.loads(src.read_text(encoding="utf-8"))
    data["expected_actions"][0]["to"] = "WRONG/PATH/notes.txt"
    with tempfile.TemporaryDirectory() as tmp:
        bad = Path(tmp) / "corrupt.json"
        bad.write_text(json.dumps(data), encoding="utf-8")
        try:
            run_case(bad)
        except AssertionError as err:
            print(f"OK  corrupt golden correctly failed: {err}")
            return 0
    print("FAIL: corrupt golden did not fail", file=sys.stderr)
    return 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--selftest-corrupt",
        action="store_true",
        help="assert runner fails on corrupted golden",
    )
    args = parser.parse_args()
    if args.selftest_corrupt:
        return run_corrupt_selftest()
    return run_all()


if __name__ == "__main__":
    raise SystemExit(main())
