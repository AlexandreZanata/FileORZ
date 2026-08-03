#!/usr/bin/env bash
# Full quality gate used by Lefthook pre-commit and `npm run verify`.
# Caps: file ≤200, function ≤80, cyclomatic ≤10, lint 0/0, compile 0 errors.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

GATE_START=$(date +%s)

step() {
  local num="$1"
  local label="$2"
  shift 2
  local started ended elapsed
  started=$(date +%s)
  echo ""
  echo "${num}/8 ${label}"
  "$@"
  ended=$(date +%s)
  elapsed=$((ended - started))
  echo "  → ${elapsed}s"
}

echo "=== FileORZ quality gate ==="

step 1 "Size + complexity (file≤200, function≤80, cyclomatic≤10)" \
  python3 "$ROOT/scripts/check_size_complexity.py" --root "$ROOT" "$@"

step 2 "Lint (0 errors, 0 warnings)" \
  bash "$ROOT/scripts/check-lint.sh"

step 3 "System / compile (0 errors)" \
  bash "$ROOT/scripts/check-system.sh"

step 4 "ADR contracts (Status/Context/Decision/Consequences + URL)" \
  python3 "$ROOT/scripts/check_adr.py"

step 5 "Characterization goldens + corrupt self-test" \
  bash "$ROOT/scripts/characterize-python.sh"

step 6 "i18n Fluent ID parity (en ↔ pt-BR)" \
  bash "$ROOT/scripts/check-i18n.sh"

step 7 "Config key map vs utils/*.py" \
  python3 "$ROOT/scripts/check_config_key_map.py"

step 8 "Rust fmt + clippy + test" \
  bash "$ROOT/scripts/check-rust.sh"

GATE_END=$(date +%s)
GATE_ELAPSED=$((GATE_END - GATE_START))
echo ""
echo "=== All quality gates passed (${GATE_ELAPSED}s) ==="
