#!/usr/bin/env bash
# Linux E2E harness — Xvfb + isolated scenarios (phase 18).
# Usage: ./scripts/e2e-linux.sh
# Artifacts on failure: .local/tmp/e2e/
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Drop dead local SOCKS proxies that break crates.io.
case "${ALL_PROXY:-}${all_proxy:-}${HTTPS_PROXY:-}" in
  *127.0.0.1:11080*|*localhost:11080*)
    unset http_proxy https_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY all_proxy
    ;;
esac

if ! command -v xvfb-run >/dev/null; then
  echo "[e2e] FAIL — xvfb-run not found (install xvfb)" >&2
  exit 1
fi
if ! command -v timeout >/dev/null; then
  echo "[e2e] FAIL — timeout not found (coreutils)" >&2
  exit 1
fi
# iced/winit loads this at runtime under X11 (CI must apt-install libxkbcommon-x11-0).
if ! ldconfig -p 2>/dev/null | grep -q 'libxkbcommon-x11\.so'; then
  if [[ ! -e /usr/lib/x86_64-linux-gnu/libxkbcommon-x11.so.0 ]] \
    && [[ ! -e /usr/lib/libxkbcommon-x11.so.0 ]]; then
    echo "[e2e] FAIL — libxkbcommon-x11.so missing (apt install libxkbcommon-x11-0)" >&2
    exit 1
  fi
fi

ARTIFACT_DIR="${FILEORZ_E2E_ARTIFACT_DIR:-$ROOT/.local/tmp/e2e}"
rm -rf "$ARTIFACT_DIR"
mkdir -p "$ARTIFACT_DIR"
export FILEORZ_E2E_ARTIFACT_DIR="$ARTIFACT_DIR"
export FILEORZ_E2E=1

echo "[e2e] building fileorz + fileorz-e2e"
cargo build -p fileorz
cargo test -p fileorz-e2e --no-run

BIN="$ROOT/target/debug/fileorz"
export FILEORZ_BIN="$BIN"
"$BIN" --version

echo "[e2e] running under Xvfb (artifacts → $ARTIFACT_DIR)"
# Serial threads: scenarios share DISPLAY; avoid cross-talk.
set +e
xvfb-run -a -s "-screen 0 1280x800x24" \
  env FILEORZ_E2E=1 \
      FILEORZ_BIN="$BIN" \
      FILEORZ_E2E_ARTIFACT_DIR="$ARTIFACT_DIR" \
      cargo test -p fileorz-e2e -- --nocapture --test-threads=1
STATUS=$?
set -e

if [[ "$STATUS" -ne 0 ]]; then
  echo "[e2e] FAILED (exit=$STATUS) — artifacts:"
  ls -la "$ARTIFACT_DIR" || true
  exit "$STATUS"
fi

echo "[e2e] OK — all scenarios passed"
exit 0
