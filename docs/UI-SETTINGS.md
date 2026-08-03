# UI settings (FileORZ Linux)

Settings hub + three editors (phase 15). Opened from the main shell **Settings**
button as in-app subviews (not separate OS windows).

## Save semantics

| Screen | Persist | Notes |
|--------|---------|-------|
| Hub | none | Navigation only |
| Extensions | **Apply** (`settings-ext-save`) | Writes `config.json` categories |
| Advanced enable | **Autosave** | `advanced_organize` → `config.json` |
| Keyword groups | **Apply** per card | Writes XDG `keywords.json` via `save_keywords` |
| Auto-delete | **Autosave** | Every toggle / day / destination |

Esc or **Back** pops editor → hub → main and restores the 720×460 window.
Hub/editors use **900×520**. The hub includes a **language** pick list
(`en` / `pt-BR`); default product locale is **English** (not `LANG`).

## Mutual exclusion (auto-delete)

UI radios clear the sibling flag (created↔modified, trash↔permanent). Runtime
still prefers trash if both destination flags are set (`resolve_delete_mode`).

## API / modules

- `fileorz_ui::settings` — screens, views, mutex helpers, keyword drafts
- `fileorz_core::advanced_pdf::save_keywords` — atomic keywords write
- Fluent: `settings-*`, `category-*`, `settings-back`, `settings-advanced-delete-group`

```bash
cargo test -p fileorz-ui
cargo run -p fileorz -- --locale en
# Settings → Configure cards; Esc closes
```
