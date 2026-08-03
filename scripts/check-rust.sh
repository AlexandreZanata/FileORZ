#!/usr/bin/env bash
# Rust quality gate: fmt, clippy (-D warnings), workspace tests.
# Fails if Cargo.toml exists but rustc/cargo are missing.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ ! -f "$ROOT/Cargo.toml" ]]; then
  echo "[rust] skip — no Cargo.toml"
  exit 0
fi

if ! command -v rustc >/dev/null || ! command -v cargo >/dev/null; then
  echo "[rust] FAIL — Cargo.toml present but rustc/cargo not found" >&2
  exit 1
fi

echo "[rust] cargo fmt --check"
cargo fmt --all -- --check

echo "[rust] cargo clippy --workspace -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings

echo "[rust] cargo test --workspace"
cargo test --workspace

echo "[rust] OK"
