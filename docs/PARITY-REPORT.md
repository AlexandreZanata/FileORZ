# Parity report — behavior catalog ↔ tests

Phase 19 gate: every catalog `B-*` id maps to automated evidence. Catalog source:
`.local/BEHAVIOR-CATALOG.md` (working copy). Product path: **Rust Linux binary**
(ADR-0001). Python UI is **legacy / upstream reference only** — not launched by
release installs (`.deb`, `fileorz` binary).

Commands:

```bash
cargo test --workspace
./scripts/e2e-linux.sh
npm run verify
python3 scripts/check_parity_report.py
cargo test -p fileorz-parity -- --include-ignored   # optional matrix lock
```

---

## Decision — Python out of the release path

| Item | Choice |
|------|--------|
| Linux product | `fileorz` Rust binary + `.deb` ([INSTALL-LINUX.md](INSTALL-LINUX.md)) |
| Python `ui/index.py` / `FileORZ.py` | Legacy oracle + GPL attribution; **not** started by packaging or docs “install” |
| ADR | [0001](adr/0001-linux-rust-rewrite.md) (phase 19 closes the retire-from-ship-path item) |

---

## Intentional differences (not bugs)

| Area | Upstream (Windows/Python) | Linux Rust | ADR / notes |
|------|---------------------------|------------|-------------|
| Config / keywords paths | `dist/` or AppData near exe | `$XDG_CONFIG_HOME/fileorz/{config,keywords}.json` | [ADR-0004](adr/0004-xdg-config-layout.md) |
| Autostart | Registry / AppData copy | XDG `autostart/fileorz.desktop` (`fileorz --tray`) | ADR-0004; [LINUX-DESKTOP.md](LINUX-DESKTOP.md) |
| Path separators | `\` string concat | `std::path::Path` | ADR-0004 |
| Organizer stop | `taskkill`-style | Cooperative `OrganizerHandle::stop` | ADR-0001; [SCHEDULER.md](SCHEDULER.md) |
| Trash | `send2trash` | FreeDesktop trash | ADR-0004 trash spec |
| Birthtime | `st_birthtime` always | `statx` crtime when available; never fake with `ctime` | [ADR-0005](adr/0005-linux-file-times.md) |
| PDF extract | `pypdf` | `lopdf` (last-page quirk preserved) | [ADR-0006](adr/0006-pdf-text-extract.md) |
| UI toolkit | CustomTkinter | iced | [ADR-0002](adr/0002-ui-toolkit-iced.md) |
| Empty-folder cleanup (B-13) | Module exists; **not** in tick | Still **out of tick** (optional later) | Catalog B-13 |
| Locales | PT-BR-oriented UI strings | Fluent `en` + `pt-BR` | [ADR-0003](adr/0003-i18n-fluent.md) |

---

## Parity matrix

| ID | Behavior (short) | Evidence |
|----|------------------|----------|
| B-01 | GUI default unless `--tray` | `fileorz` default → `ui_cmd`; e2e UI smoke in `e2e_01_organize` |
| B-02 | `--tray` starts organizer; window hidden | `tray_cmd` + `fileorz_ui::run_tray`; **E2E-04** `e2e_04_tray` |
| B-03 | Close hides to tray (no quit) | **`--tray` / start_hidden only**; normal GUI close quits (GNOME tray gap) |
| B-04 | Tray Open / Quit | `fileorz_linux::tray::menu_order_open_then_quit`; tray labels i18n |
| B-05 | Interval loop from config minutes | `scheduler::tests::wait_interruptible_*`; `OrganizerHandle` |
| B-10 | Tick: validate → autodelete → PDF → extension | `scheduler::tests::run_tick_once_moves_extensions`; [SCHEDULER.md](SCHEDULER.md) |
| B-11 | Extension map (enabled only, capitalize) | `organize::map::map_tests::*`; [ORGANIZE.md](ORGANIZE.md) |
| B-12 | OUTROS / collide `_N` / skip dotfiles / top-level | `organize::tests::parity_tiny_mixed_*`, `parity_collision_*`; golden `tiny-mixed`, `collision`; **REG-001** |
| B-13 | Empty-folder delete unused in tick | Documented intentional; no tick call (see differences) |
| B-14 | PDF keywords: first match, skip if dest exists | `advanced_pdf::tests::parity_pdf_keywords_pipeline`, `plan_skips_when_destination_exists`; golden `pdf-keywords` |
| B-20 | Auto-delete master gate | `autodelete::mode::disabled_yields_none`; golden `aged-files` |
| B-21 | Age `>` days (mtime / birthtime) | `autodelete::age::exceeds_uses_strict_greater_than`; ADR-0005 |
| B-22 | Trash XOR permanent (trash wins if both) | `autodelete::mode::trash_wins_when_both_true`; UI mutex tests |
| B-23 | Scan category/EXT only (not root loose) | `autodelete::tests::root_loose_files_not_scanned`; **E2E-02** |
| B-30 | Folder path persisted | `fileorz_ui::persist::roundtrip_folder_and_interval` |
| B-31 | Interval minutes persisted | same + `clamp_unknown_interval_to_five` |
| B-32 | Autostart flag + OS hook | `fileorz_linux::autostart::enable_disable_under_temp_xdg` |
| B-33 | Extension toggles | `settings::ext_logic::*`; **E2E-05** |
| B-34 | Keywords file separate | `xdg::keywords_and_autostart_paths`; keywords roundtrip tests |

### E2E cross-links

| Scenario | IDs touched |
|----------|-------------|
| E2E-01 organize golden | B-01, B-10, B-11, B-12 |
| E2E-02 autodelete | B-20, B-21, B-23 |
| E2E-03 locale probe | i18n (ADR-0003); labels for UI parity |
| E2E-04 tray worker | B-02, B-05 |
| E2E-05 extension off → OUTROS | B-12, B-33 |

Characterization oracle (Python, non-product): `npm run characterize` — see
[CHARACTERIZATION.md](CHARACTERIZATION.md).

---

## Gaps (accepted for v1)

| Item | Status |
|------|--------|
| Multi-page PDF concatenate-all vs last-page quirk | Quirk preserved; future ADR if product changes |
| Birthtime create-date fixture on all CI hosts | mtime goldens cover B-21; btime when `statx` available |
| B-03 automated hide-on-close | Covered by tray UI design + docs; no headless window-manager probe yet |

---

## Related docs

- [REGRESSION-LOG.md](REGRESSION-LOG.md)
- [E2E-LINUX.md](E2E-LINUX.md)
- [INSTALL-LINUX.md](INSTALL-LINUX.md)
- [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md)
