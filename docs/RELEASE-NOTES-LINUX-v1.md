# Release notes — Linux v1.0.0 (`linux-v1.0.0`)

**Tag:** `linux-v1.0.0`  
**Version:** `1.0.0` (Cargo workspace)  
**Date:** 2026-08-03  
**Remote:** [AlexandreZanata/FileORZ](https://github.com/AlexandreZanata/FileORZ) (**origin only**)

First Linux-native Rust release of this FileORZ fork. Upstream Windows/Python
project remains attributable under GPL-3.0 — see
[THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md) and [LICENSE](../LICENSE).

## Highlights

- **Linux desktop app** (iced) with organize, auto-delete, PDF keywords, tray,
  and XDG config ([INSTALL-LINUX.md](INSTALL-LINUX.md)).
- **i18n:** Fluent catalogs for **en** and **pt-BR** (`crates/fileorz-i18n`).
- **Packaging:** stripped `fileorz` binary + `.deb` for Ubuntu LTS x86_64.
- **Quality:** Lefthook/`npm run verify`, characterization goldens, Xvfb e2e,
  parity matrix ([PARITY-REPORT.md](PARITY-REPORT.md)).

## Artifacts

Published on the GitHub release for tag `linux-v1.0.0`:

| Asset | Notes |
|-------|--------|
| `fileorz` | Stripped x86_64 ELF (`fileorz --version` → `fileorz 1.0.0`) |
| `fileorz_1.0.0_amd64.deb` | Desktop entry + icons + docs |
| `SHA256SUMS` | Checksums for the above |

Verify:

```bash
sha256sum -c SHA256SUMS
sudo apt install ./fileorz_1.0.0_amd64.deb
fileorz --version
```

## Upstream attribution

- Original project: [ThainanViniciusKatchan/FileORZ](https://github.com/ThainanViniciusKatchan/FileORZ)
- This fork must not be presented as the original upstream product.
- Python CustomTkinter sources are **legacy / oracle only** — not the Linux
  ship path (ADR-0001, phase 19).

## Support matrix

| Item | Supported |
|------|-----------|
| OS | Ubuntu 24.04 LTS (and glibc-compatible x86_64) |
| Arch | `x86_64` / `amd64` |
| Display | X11 / Wayland via iced/winit |
| Locales | `en`, `pt-BR` |
| Windows | Out of scope for this release |

## Known limits

- AppImage not shipped (`.deb` + bare binary first).
- Tray needs a StatusNotifier watcher (KDE native; GNOME needs extension).
- Standing task `01-baseline-quality-debt` (Python size exemptions) remains
  open — does not block the Rust Linux product.

## Docs

- Install: [INSTALL-LINUX.md](INSTALL-LINUX.md)
- Handover: [HANDOVER-LINUX.md](HANDOVER-LINUX.md)
- E2E: [E2E-LINUX.md](E2E-LINUX.md)
- Parity: [PARITY-REPORT.md](PARITY-REPORT.md)
