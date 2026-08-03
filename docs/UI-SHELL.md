# UI main shell (FileORZ Linux)

iced main window for phase 14 — folder, interval, Start/Stop, header controls.

## Layout

| Region | Jobs |
|--------|------|
| Header | Brand + tagline, autostart toggle, GitHub / Changelog / About |
| Body | Folder picker (rfd / portal), interval 1–10 min, Settings stub, Start/Stop |
| Feedback | i18n success / error line under actions |

Window size stays **700×420** (tokens). Theme: `docs/UI-TOKENS.md`.

## Behavior

| Action | Result |
|--------|--------|
| Choose folder / change interval | Persist to `$XDG_CONFIG_HOME/fileorz/config.json` |
| Start | `validate_root` then `OrganizerHandle::start`; errors via Fluent |
| Stop | Cooperative `OrganizerHandle::stop` |
| Autostart | Sync XDG `.desktop` + `AppConfig.autostart` |
| Close | Hide window when tray is available (else quit) |
| Tray Open | Show + focus window |
| Tray Quit | Stop organizer, shutdown tray, exit |

`--tray` launches the same shell hidden with optional organizer autostart
(`fileorz_ui::run_tray`). `FILEORZ_TRAY_SMOKE=1` keeps the headless tray smoke path.

## State machine

`RunPhase`: **Idle** → **Running** | **Error**; Running → Idle on Stop; Error → Idle on clear.

```bash
cargo test -p fileorz-ui
cargo run -p fileorz -- --locale en
FILEORZ_UI_SMOKE=.local/tmp/ui-reference/phase-14-shell-en.ppm \
  FILEORZ_UI_NO_TRAY=1 FILEORZ_UI_POS=1 cargo run -p fileorz -- --locale en
```

`FILEORZ_UI_SMOKE=<ppm-path>` captures the iced window then exits (manual gate).
