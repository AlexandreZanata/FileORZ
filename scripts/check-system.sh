#!/usr/bin/env bash
# System / compile gate: Python bytecode compile with 0 errors.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "[system] Running python -m compileall (app sources)..."
python3 -m compileall -q \
  AdvancedAlg \
  AutoBuild \
  ui \
  utils \
  FileORZ.py \
  run.py \
  scripts

echo "[system] OK — 0 errors"
