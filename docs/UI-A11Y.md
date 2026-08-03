# UI accessibility & polish (FileORZ Linux)

Checklist results for phase 16 (`UI-LINUX.md` extras). Toolkit: iced 0.13.

## Keyboard

| Action | Behavior |
|--------|----------|
| **Tab** | Native iced focus order: header controls → folder → interval → Settings → Start/Stop; in settings: Back → cards/fields → Save |
| **Enter** | On **main** screen: Start/Stop organizer (same as primary CTA) |
| **Esc** | Pop settings editor → hub → main; closes **About** |

Documented order (main): Language → Autostart → GitHub → Changelog → About → Choose folder → Interval → Settings → Start.

## Focus visible

Buttons use a **2px teal accent border** on `Hovered` / `Pressed` (`style::focus_border`) as the visible focus/hover ring. Text inputs use iced’s built-in `Focused` status in advanced editor.

## Contrast (automated)

`fileorz_ui::contrast` WCAG-style ratios against tokens:

| Pair | Gate |
|------|------|
| `TEXT` on `BG` | ≥ 4.5 (AA normal) |
| `TEXT_MUTED` on `SURFACE` | ≥ 3.0 (AA UI) |
| `ACCENT` / `DANGER` on `BG`; white on `ACCENT` | ≥ 3.0 |

```bash
cargo test -p fileorz-ui contrast
```

## Language picker

Header / Settings language pick list `en` / `pt-BR` → `ShellApp::apply_locale`
→ persists `config.locale` to XDG `config.json` and reloads Fluent strings
without restart. Product default is **en** (Settings hub picker; not `LANG`).

## About

Header **About** opens an in-app dialog: version (`CARGO_PKG_VERSION`), GPL-3.0 label, upstream + fork URLs, third-party notices link (opens fork), and last measured **HiDPI scale**.

## HiDPI / Wayland smoke

On boot the shell samples `window::get_scale_factor` and logs `ui scale_factor=…`. About shows the value. Manual gate:

```bash
FILEORZ_UI_NO_TRAY=1 FILEORZ_UI_OPEN_ABOUT=1 FILEORZ_UI_SMOKE=.local/tmp/ui-reference/phase-16-about-en.ppm \
  cargo run -p fileorz -- --locale en
# Confirm log line and About “HiDPI scale · …” match the session scale (1.0, 1.25, 1.5, 2.0, …)
```

## Motion (≤3, non-blocking)

1. Feedback line fade-in (`motion.feedback_t`)
2. Settings/About enter fade (`motion.screen_t` + canvas alpha)
3. Button hover/press accent ring (style cue)

No modal blocking animations.

## Manual locales

```bash
FILEORZ_UI_SMOKE=.local/tmp/ui-reference/phase-16-about-en.ppm FILEORZ_UI_OPEN_ABOUT=1 \
  FILEORZ_UI_NO_TRAY=1 cargo run -p fileorz -- --locale en
FILEORZ_UI_SMOKE=.local/tmp/ui-reference/phase-16-about-pt.ppm FILEORZ_UI_OPEN_ABOUT=1 \
  FILEORZ_UI_NO_TRAY=1 cargo run -p fileorz -- --locale pt-BR
```

```bash
npm run verify
```
