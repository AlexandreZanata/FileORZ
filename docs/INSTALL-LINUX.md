# Install FileORZ on Linux

Linux-native Rust binary (iced GUI). Primary package: **`.deb`** for Ubuntu LTS
(x86_64). AppImage is optional/future — this fork ships `.deb` first.

Upstream attribution and GPL notices: [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).
Rust crate licenses: [THIRD_PARTY_RUST.md](THIRD_PARTY_RUST.md).

## Release build (from source)

Requirements: Rust **1.85+** (`rust-toolchain.toml`), Linux desktop libs for iced
(typically already present on Ubuntu desktop).

```bash
# GUI is always linked (fileorz → fileorz-ui / iced). No Cargo feature gate.
# Strip policy: [profile.release] strip = "symbols" in workspace Cargo.toml.
# Override or reinforce with RUSTFLAGS if needed:
#   RUSTFLAGS="-C strip=symbols" cargo build -p fileorz --release

cargo build -p fileorz --release
./target/release/fileorz --version   # → fileorz 0.1.0
./target/release/fileorz --help
```

| Item | Value |
|------|--------|
| Artifact | `target/release/fileorz` |
| Default features | GUI included (no `--features gui`) |
| Strip | `strip = "symbols"` on release profile |
| Optional RUSTFLAGS | `-C strip=symbols` (redundant if profile applies) |

Do **not** distribute unstripped debug binaries as “release” builds.

## Package (`.deb`)

Idempotent CI script (wipes `dist/linux` each run):

```bash
bash scripts/package-linux.sh
# or reuse an existing release binary:
bash scripts/package-linux.sh --skip-build

sudo apt install ./dist/linux/fileorz_*_amd64.deb
fileorz --version
fileorz            # iced main shell
```

The package installs:

- `/usr/bin/fileorz`
- `/usr/share/applications/fileorz.desktop`
- `/usr/share/icons/hicolor/{48,128,256}x{…}/apps/fileorz.png`
- `/usr/share/doc/fileorz/` (LICENSE, notices, this guide)

Validate assets without building:

```bash
python3 scripts/check_package_linux.py
```

## GPL-3.0 obligations when distributing

FileORZ is licensed under **GNU GPL v3** ([LICENSE](../LICENSE)). If you
redistribute binaries (`.deb`, tarball, or other):

1. **Provide Corresponding Source** — the complete preferred form for modifying
   the work (this repository at the matching tag/commit), or a written offer
   valid ≥3 years as allowed by GPL §6.
2. **Preserve license texts** — ship a copy of the GPL and keep copyright /
   modification notices intact (the `.deb` includes `copyright` under
   `/usr/share/doc/fileorz/`).
3. **State modifications** — this fork must not be presented as the original
   upstream project; keep [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md)
   and About-dialog credits.
4. **Dependency licenses** — retain [THIRD_PARTY_RUST.md](THIRD_PARTY_RUST.md)
   (or regenerate with `python3 scripts/gen_third_party_rust.py` / `cargo about`).
5. **No further restrictions** — do not add terms that restrict GPL freedoms
   on the combined work.

Source for this fork: https://github.com/AlexandreZanata/FileORZ  
Upstream project: https://github.com/ThainanViniciusKatchan/FileORZ

## Legacy Python UI

Python CustomTkinter sources are **not** the Linux product path (phase 19 /
ADR-0001). Use them only as a characterization oracle. Parity matrix:
[`PARITY-REPORT.md`](PARITY-REPORT.md).

## Desktop / tray

See [LINUX-DESKTOP.md](LINUX-DESKTOP.md) for XDG autostart and StatusNotifier tray.
