# Linux packaging assets

Sources for `scripts/package-linux.sh` (phase 17).

| Path | Purpose |
|------|---------|
| `fileorz.desktop` | FreeDesktop application entry |
| `icons/hicolor/*/apps/fileorz.png` | App icons (48 / 128 / 256) |

Icons are derived from `assets/File_ORZ.png` (letterboxed). Rebuild:

```bash
for s in 48 128 256; do
  convert assets/File_ORZ.png -resize "${s}x${s}" -background '#0d1117' \
    -gravity center -extent "${s}x${s}" \
    "packaging/linux/icons/hicolor/${s}x${s}/apps/fileorz.png"
done
```

User install: [`docs/INSTALL-LINUX.md`](../../docs/INSTALL-LINUX.md).
