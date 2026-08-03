# Rust crates (FileORZ Linux rewrite)

Workspace members match `.local/ARCHITECTURE-LINUX-RUST.md`.

| Crate | Role |
|-------|------|
| `fileorz` | Binary (`--help`, `--version`, reserved `--tray` / `--locale`) |
| `fileorz-core` | Domain: config, organize, autodelete, advanced PDF |
| `fileorz-i18n` | Fluent runtime + embedded locales |
| `fileorz-linux` | XDG paths (phase 06); desktop later |
| `fileorz-ui` | iced UI (phase 13+) |
| `fileorz-parity` | Golden parity |
| `fileorz-e2e` | Linux e2e helpers |

```bash
cargo test --workspace
cargo run -p fileorz -- --help
bash scripts/check-rust.sh
```
