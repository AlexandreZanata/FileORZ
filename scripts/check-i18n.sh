#!/usr/bin/env bash
# Fail if en / pt-BR Fluent message ID sets drift (ADR-0003 / phase 10).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec python3 "$ROOT/scripts/check_i18n_ids.py"
