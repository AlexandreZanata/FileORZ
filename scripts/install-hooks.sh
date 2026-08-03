#!/usr/bin/env bash
# Install Lefthook git hooks (blocks commits unless quality gate is green).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

export PATH="${PATH}:$(go env GOPATH 2>/dev/null)/bin:${HOME}/go/bin"

if ! command -v lefthook >/dev/null 2>&1; then
  if command -v npx >/dev/null 2>&1; then
    echo "lefthook not on PATH — using npx lefthook"
    npx lefthook install
  else
    echo "lefthook not found. Install with:" >&2
    echo "  go install github.com/evilmartians/lefthook@latest" >&2
    echo "  # or: npm install (uses local lefthook via prepare)" >&2
    exit 1
  fi
else
  lefthook install
fi

echo "Lefthook hooks installed. Commits require scripts/check-quality.sh to pass."
echo "Manual run: lefthook run pre-commit"
echo "Full gate:  npm run verify   # or: bash scripts/check-quality.sh"
