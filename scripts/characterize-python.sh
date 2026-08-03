#!/usr/bin/env bash
# Run characterization goldens + corrupt-golden self-test.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
python3 "$ROOT/scripts/run_characterization.py"
python3 "$ROOT/scripts/run_characterization.py" --selftest-corrupt
