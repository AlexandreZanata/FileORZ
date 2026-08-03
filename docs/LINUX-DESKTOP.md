# Linux desktop integration

XDG autostart + StatusNotifier tray (`fileorz-linux`, phase 12).

## Autostart

Path: `$XDG_CONFIG_HOME/autostart/fileorz.desktop` (default `~/.config/autostart/`).

```bash
fileorz autostart enable    # write Exec=fileorz --tray
fileorz autostart disable   # remove file
fileorz autostart status
fileorz autostart print      # dry-run: print .desktop body + path
```

API: `fileorz_linux::autostart::{enable, disable, is_enabled, desktop_entry}`.

## Tray (`--tray`)

StatusNotifierItem via **ksni** (menu: Open / Quit, i18n `tray-*`).

Prefers the iced main shell started **hidden** (`fileorz_ui::run_tray`). Close
hides the window in **`--tray` mode only**; a normal desktop launch **quits** on
close (tray icons are often invisible on GNOME without an extension). Tray
**Open** shows the window again; **Quit** stops the organizer and exits. If the
UI cannot start (no display), falls back to tray-only mode.

| Action | Behavior |
|--------|----------|
| Open | Show iced main window (or log stub in tray-only fallback) |
| Quit | Stops `OrganizerHandle` (if running) then shuts tray and exits |
| Left-click | Same as Open |

If config exists with a valid `folder`, the organizer loop starts in the background.

Smoke without blocking forever:

```bash
FILEORZ_TRAY_SMOKE=1 fileorz --tray
```

Main shell details: [`docs/UI-SHELL.md`](UI-SHELL.md).
## GNOME / desktop caveats

| Environment | Notes |
|-------------|--------|
| **KDE Plasma** | Native SNI — tray works out of the box |
| **GNOME Shell** | No built-in StatusNotifier; install an AppIndicator / tray extension (e.g. “AppIndicator and KStatusNotifierItem Support”) or the icon will not appear |
| **Cosmic / others** | Depends on SNI watcher availability |
| **Headless CI** | No watcher → spawn fails; unit tests cover autostart files and skip live tray |

Official specs:

- https://specifications.freedesktop.org/desktop-entry-spec/latest/
- https://specifications.freedesktop.org/basedir-spec/latest/
- https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/

```bash
cargo test -p fileorz-linux
npm run verify
```

Install / `.deb` packaging: [`INSTALL-LINUX.md`](INSTALL-LINUX.md).
E2E under Xvfb: [`E2E-LINUX.md`](E2E-LINUX.md).
