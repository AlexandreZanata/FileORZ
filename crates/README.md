# Rust crates (FileORZ Linux rewrite)

Workspace members match `.local/ARCHITECTURE-LINUX-RUST.md`.

| Crate | Role |
|-------|------|
| `fileorz` | Binary (`--help`, `--version`, reserved `--tray` / `--locale`) |
| `fileorz-core` | Domain (phase 06+) |
| `fileorz-i18n` | Fluent runtime (phase 10); catalogs still in `/i18n` |
| `fileorz-linux` | XDG / desktop (phase 12) |
| `fileorz-ui` | iced UI (phase 13+) |
| `fileorz-parity` | Golden parity |
| `fileorz-e2e` | Linux e2e helpers |

```bash
cargo test --workspace
cargo run -p fileorz -- --help
bash scripts/check-rust.sh
```
