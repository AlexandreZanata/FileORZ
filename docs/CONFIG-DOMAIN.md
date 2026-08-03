# Config domain (Rust)

Stable schema + legacy migration live in `fileorz-core::config`.
XDG paths live in `fileorz-linux::xdg` (ADR-0004).

## Stable keys

See [`CONFIG-KEY-MAP.md`](CONFIG-KEY-MAP.md). Defaults:
`crates/fileorz-core/defaults/config.v1.json`.

## Load behavior

1. Read JSON from path (typically `$XDG_CONFIG_HOME/fileorz/config.json`).
2. If legacy PT keys → migrate, write `config.json.bak-legacy` once, atomic-write stable JSON.
3. Second load is idempotent (no re-migrate).

```bash
cargo test -p fileorz-core config
cargo test -p fileorz-linux xdg
```
