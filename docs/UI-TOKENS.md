# UI design tokens (FileORZ Linux)

Dark-first shell tokens for `fileorz-ui` (phase 13). Avoid purple-on-white and
cream/serif “AI default” looks — cool charcoal + teal accent.

## Colors

| Token | Hex | Role |
|-------|-----|------|
| `BG` | `#1A1D23` | Window / canvas |
| `SURFACE` | `#242830` | Main panel |
| `SURFACE_RAISED` | `#2E3340` | Hover / raised |
| `ACCENT` | `#3D9B8F` | Primary CTA (Start) |
| `ACCENT_STRONG` | `#2E8278` | Pressed CTA |
| `DANGER` | `#E85D4C` | Destructive |
| `SUCCESS` | `#4AB07A` | Success feedback |
| `TEXT` | `#E8EAED` | Primary text |
| `TEXT_MUTED` | `#9AA0A6` | Secondary text |
| `BORDER` | `#3A3F4B` | Hairlines |

Rust: `fileorz_ui::tokens`. iced theme: `fileorz_ui::fileorz_theme()` (`FileORZ Dark`).

## Spacing & type

- Grid: 8 / 16 / 24 / 32 (`SPACE_1`…`SPACE_4`)
- Body: 12 / 14 / 16; title: 22
- Window start size: **700×420**

## Fonts

iced default sans (Fira Sans via iced feature). Prefer system Inter / IBM Plex /
Cantarell when bundling fonts in later polish (UI-LINUX.md).

## Screenshots

Gitignored captures: `.local/tmp/ui-reference/` (see phase 13 validation).

```bash
cargo test -p fileorz-ui
cargo run -p fileorz -- --locale en
```

Main shell behavior (folder / interval / Start / tray hide): [`UI-SHELL.md`](UI-SHELL.md).
Settings hub + editors: [`UI-SETTINGS.md`](UI-SETTINGS.md).
