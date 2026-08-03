# Linux E2E suite (Xvfb)

Phase 18 — headless end-to-end flows for organize, auto-delete, i18n, tray, and
extension settings. Spec: `.local/TEST-STRATEGY.md`. Official Xvfb man page:
https://www.x.org/releases/X11R7.7/doc/man/man1/Xvfb.1.xhtml

## Run locally

```bash
# Requires: xvfb (`xvfb-run`), coreutils `timeout`, optional ImageMagick `import`
# Ubuntu iced UI deps (CI installs these):
#   libxkbcommon0 libxkbcommon-x11-0 libxcb* libegl1 libgl1 libfontconfig1
./scripts/e2e-linux.sh
# or
npm run e2e:linux
```

The harness:

1. Builds `fileorz` + `fileorz-e2e`
2. Sets `FILEORZ_E2E=1` and a fresh `.local/tmp/e2e/` artifact dir
3. Runs `cargo test -p fileorz-e2e` under `xvfb-run` (serial)

Integration scenarios **no-op** unless `FILEORZ_E2E` is set, so
`cargo test --workspace` / `npm run verify` stay display-free.

## Scenarios

| ID | What |
|----|------|
| E2E-01 | UI smoke + `organize --once` on `tiny-mixed` → golden paths |
| E2E-02 | Auto-delete aged-files fixture → old gone, fresh kept |
| E2E-03 | `--demo-i18n` en → pt-BR key label probe |
| E2E-04 | `FILEORZ_TRAY_SMOKE=1 fileorz --tray` starts worker, no iced window |
| E2E-05 | Disable `.txt` → next tick sends `notes.txt` to `OUTROS/TXT/` |

Each test uses a temp `HOME` + `XDG_*`. On panic, `ArtifactGuard` tries
`import -window root` → `.local/tmp/e2e/fail-*.png`.

## CI

Workflow: [`.github/workflows/e2e-linux.yml`](../.github/workflows/e2e-linux.yml)
(Ubuntu 24.04 LTS). Jobs: `verify-harness`, `cargo-ubuntu`, `e2e-xvfb`,
`package-smoke`. Enable **required status checks** on `main` so failures block
merge.

```bash
cargo test -p fileorz-e2e
./scripts/e2e-linux.sh
npm run verify
```
