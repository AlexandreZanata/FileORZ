# Auto-delete (Rust)

`fileorz-core::autodelete` + `fileorz-linux::trash` (B-20..B-23, ADR-0005).

## Rules

| Concern | Behavior |
|---------|----------|
| Master | `auto_delete.enabled` |
| Destination | trash wins if both flags set; else permanent |
| Age | `(now - stamp).days > max_age_days` |
| Modified | always `mtime` |
| Created | `Metadata::created()` when available; else skip |
| Scan | `Category/SUB/` only (capitalized category ids) |

## API

```text
plan_deletes(root, &AppConfig, now) -> Vec<PlannedDelete>
apply_deletes(root, &plans, Option<&dyn TrashSink>)
fileorz_linux::trash::FreedesktopTrash
```

Golden: `tests/fixtures/golden/aged-files.json`.

```bash
cargo test -p fileorz-core autodelete
cargo test -p fileorz-linux trash
```
