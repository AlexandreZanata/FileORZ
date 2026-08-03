#!/usr/bin/env bash
# Lint gate: 0 errors and 0 warnings (ruff).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if ! command -v ruff >/dev/null 2>&1; then
  echo "[lint] FAIL: ruff is not on PATH."
  echo "[lint] Install: pip install -r requirements-dev.txt"
  exit 1
fi

echo "[lint] Running ruff check (0 warnings)..."
ruff check \
  AdvancedAlg \
  AutoBuild \
  ui \
  utils \
  FileORZ.py \
  run.py \
  scripts

echo "[lint] OK — 0 errors, 0 warnings"
