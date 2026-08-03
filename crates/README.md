# Rust crates (FileORZ Linux rewrite)

Workspace members match `.local/ARCHITECTURE-LINUX-RUST.md`.

| Crate | Role |
|-------|------|
| `fileorz` | Binary (`--help`, `--tray`, `organize`, `autostart`) |
| `fileorz-core` | Domain: config, organize, autodelete, advanced PDF, scheduler |
| `fileorz-i18n` | Fluent runtime + embedded locales |
| `fileorz-linux` | XDG paths, autostart, tray (ksni), trash |
| `fileorz-ui` | iced main shell + settings hub/editors |
| `fileorz-parity` | Golden parity |
| `fileorz-e2e` | Linux e2e helpers |

```bash
cargo test --workspace
cargo run -p fileorz -- --help
cargo build -p fileorz --release   # → target/release/fileorz (strip=symbols)
bash scripts/check-rust.sh
bash scripts/package-linux.sh      # → dist/linux/*.deb (see docs/INSTALL-LINUX.md)
```
