# Organize core (extensions)

`fileorz-core::organize` implements B-11/B-12 from the behavior catalog.

## API

| Function | Role |
|----------|------|
| `plan_moves(root, &AppConfig)` | Read-only plan → `Vec<PlannedMove>` |
| `apply_moves(root, &moves)` | `create_dir_all` + `rename` |
| `build_extension_map` | Enabled `.ext` → capitalized category |

Collision: destination exists → `stem_N.ext` (never overwrite).  
Unknown/disabled ext → `OUTROS/{EXT}/`. Dotfiles skipped. Top-level only.

## Parity goldens

- `tests/fixtures/golden/tiny-mixed.json`
- `tests/fixtures/golden/collision.json`

```bash
cargo test -p fileorz-core organize
```
