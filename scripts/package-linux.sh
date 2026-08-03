#!/usr/bin/env bash
# Build a Debian package for FileORZ (Ubuntu LTS x86_64).
# Idempotent: wipes dist/linux and rebuilds cleanly each run.
# Usage: scripts/package-linux.sh [--skip-build]
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Drop dead local SOCKS proxies that break crates.io.
case "${ALL_PROXY:-}${all_proxy:-}${HTTPS_PROXY:-}" in
  *127.0.0.1:11080*|*localhost:11080*)
    unset http_proxy https_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY all_proxy
    ;;
esac

SKIP_BUILD=0
for arg in "$@"; do
  case "$arg" in
    --skip-build) SKIP_BUILD=1 ;;
    -h|--help)
      echo "Usage: $0 [--skip-build]"
      exit 0
      ;;
    *)
      echo "unknown arg: $arg" >&2
      exit 2
      ;;
  esac
done

VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -1)"
ARCH="$(dpkg --print-architecture 2>/dev/null || echo amd64)"
OUT="$ROOT/dist/linux"
STAGE="$OUT/deb-root"
PKG_NAME="fileorz_${VERSION}_${ARCH}"
BIN_SRC="$ROOT/target/release/fileorz"

echo "[package-linux] version=${VERSION} arch=${ARCH}"

rm -rf "$OUT"
mkdir -p "$OUT"

if [[ "$SKIP_BUILD" -eq 0 ]]; then
  echo "[package-linux] cargo build --release (strip via profile.release)"
  cargo build -p fileorz --release
fi

if [[ ! -x "$BIN_SRC" ]]; then
  echo "[package-linux] FAIL — missing $BIN_SRC (build first or drop --skip-build)" >&2
  exit 1
fi

"$BIN_SRC" --version | tee "$OUT/version.txt"

# Stage FHS tree for .deb
mkdir -p \
  "$STAGE/DEBIAN" \
  "$STAGE/usr/bin" \
  "$STAGE/usr/share/applications" \
  "$STAGE/usr/share/doc/fileorz" \
  "$STAGE/usr/share/icons/hicolor/48x48/apps" \
  "$STAGE/usr/share/icons/hicolor/128x128/apps" \
  "$STAGE/usr/share/icons/hicolor/256x256/apps"

install -m 0755 "$BIN_SRC" "$STAGE/usr/bin/fileorz"
install -m 0644 "$ROOT/packaging/linux/fileorz.desktop" \
  "$STAGE/usr/share/applications/fileorz.desktop"
install -m 0644 "$ROOT/packaging/linux/icons/hicolor/48x48/apps/fileorz.png" \
  "$STAGE/usr/share/icons/hicolor/48x48/apps/fileorz.png"
install -m 0644 "$ROOT/packaging/linux/icons/hicolor/128x128/apps/fileorz.png" \
  "$STAGE/usr/share/icons/hicolor/128x128/apps/fileorz.png"
install -m 0644 "$ROOT/packaging/linux/icons/hicolor/256x256/apps/fileorz.png" \
  "$STAGE/usr/share/icons/hicolor/256x256/apps/fileorz.png"
install -m 0644 "$ROOT/LICENSE" "$STAGE/usr/share/doc/fileorz/copyright"
install -m 0644 "$ROOT/THIRD_PARTY_NOTICES.md" \
  "$STAGE/usr/share/doc/fileorz/THIRD_PARTY_NOTICES.md"
install -m 0644 "$ROOT/docs/THIRD_PARTY_RUST.md" \
  "$STAGE/usr/share/doc/fileorz/THIRD_PARTY_RUST.md"
gzip -9 -n -c "$ROOT/docs/INSTALL-LINUX.md" \
  >"$STAGE/usr/share/doc/fileorz/INSTALL-LINUX.md.gz"

# Estimate installed size in KiB for control file.
INSTALLED_SIZE="$(du -sk "$STAGE" | awk '{print $1}')"

cat >"$STAGE/DEBIAN/control" <<EOF
Package: fileorz
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Installed-Size: ${INSTALLED_SIZE}
Maintainer: AlexandreZanata <https://github.com/AlexandreZanata/FileORZ>
Depends: libc6
Description: Automatic file organizer (Linux Rust rewrite)
 FileORZ organizes files by extension, keywords, and age rules.
 This package is a GPL-3.0-or-later fork; see /usr/share/doc/fileorz/.
Homepage: https://github.com/AlexandreZanata/FileORZ
EOF

dpkg-deb --root-owner-group --build "$STAGE" "$OUT/${PKG_NAME}.deb"
# Keep stage for inspection; also expose a stable symlink name for CI.
ln -sfn "${PKG_NAME}.deb" "$OUT/fileorz_latest.deb"

# Standalone stripped binary + checksums for GitHub release assets.
install -m 0755 "$BIN_SRC" "$OUT/fileorz"
(
  cd "$OUT"
  sha256sum "fileorz" "${PKG_NAME}.deb" >SHA256SUMS
)

echo "[package-linux] OK → $OUT/${PKG_NAME}.deb"
echo "[package-linux] binary → $OUT/fileorz"
echo "[package-linux] checksums → $OUT/SHA256SUMS"
ls -lh "$OUT/${PKG_NAME}.deb" "$OUT/fileorz" "$OUT/SHA256SUMS"
cat "$OUT/SHA256SUMS"
