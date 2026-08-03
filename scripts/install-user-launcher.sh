#!/usr/bin/env bash
# Install FileORZ user launcher + icons (no root).
# Usage: scripts/install-user-launcher.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${FILEORZ_BIN:-$ROOT/target/release/fileorz}"
APP_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
ICON_BASE="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor"

if [[ ! -x "$BIN" ]]; then
  echo "[launcher] building release binary…"
  cargo build -p fileorz --release
fi
test -x "$BIN"

for s in 48 128 256 512; do
  src="$ROOT/packaging/linux/icons/hicolor/${s}x${s}/apps/fileorz.png"
  dest="$ICON_BASE/${s}x${s}/apps/fileorz.png"
  mkdir -p "$(dirname "$dest")"
  install -m 0644 "$src" "$dest"
done

mkdir -p "$APP_DIR"
cat >"$APP_DIR/fileorz.desktop" <<EOF
[Desktop Entry]
Type=Application
Version=1.5
Name=FileORZ
GenericName=File organizer
Comment=Automatic file organizer for Linux
Exec=$BIN
Icon=fileorz
Terminal=false
Categories=Utility;Filesystem;
Keywords=organize;files;folders;
StartupNotify=true
EOF

gtk-update-icon-cache -f -t "$ICON_BASE" 2>/dev/null || true
update-desktop-database "$APP_DIR" 2>/dev/null || true
echo "[launcher] OK — search for FileORZ in the app menu"
echo "  desktop: $APP_DIR/fileorz.desktop"
echo "  binary:  $BIN"
