# Linux packaging assets

Sources for `scripts/package-linux.sh` (phase 17).

| Path | Purpose |
|------|---------|
| `fileorz.desktop` | FreeDesktop application entry |
| `icons/hicolor/*/apps/fileorz.png` | App icons (48 / 128 / 256 / 512) |

Master artwork: `assets/fileorz-icon.png` (1024×1024). Rebuild sizes:

```bash
for s in 48 128 256 512; do
  convert assets/fileorz-icon.png -resize "${s}x${s}" \
    "packaging/linux/icons/hicolor/${s}x${s}/apps/fileorz.png"
done
```

User menu (no root): `bash scripts/install-user-launcher.sh`  
System install: [`docs/INSTALL-LINUX.md`](../../docs/INSTALL-LINUX.md).
