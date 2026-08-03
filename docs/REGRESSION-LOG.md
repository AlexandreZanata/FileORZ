# Regression log

Registry of bugs that must not return. Every fix that closes a user-visible or
parity defect adds a row **and** an automated test when feasible.

Linked strategy: `.local/TEST-STRATEGY.md`. Matrix: [`PARITY-REPORT.md`](PARITY-REPORT.md).
Commands: `npm run verify`, `npm run characterize`, `cargo test --workspace`,
`./scripts/e2e-linux.sh`.

## Entry format

Copy this block for each regression:

```markdown
### REG-NNN — short title

| Field | Value |
|-------|-------|
| Date | YYYY-MM-DD |
| Status | open \| fixed \| verified |
| Symptom | What the user / parity suite saw |
| Root cause | One sentence |
| Fix | PR / commit / module |
| Test | `path::test_name` or fixture id |
| ADR / catalog | Optional link (e.g. B-12, ADR-0005) |
```

## Entries

### REG-001 — corrupt golden must fail characterization

| Field | Value |
|-------|-------|
| Date | 2026-08-03 |
| Status | verified |
| Symptom | Characterization could silently pass if expected paths were wrong |
| Root cause | Need a negative control that the oracle asserts FS/actions strictly |
| Fix | `scripts/run_characterization.py --selftest-corrupt` |
| Test | `npm run characterize` (corrupt self-test step) |
| ADR / catalog | B-12; [`CHARACTERIZATION.md`](CHARACTERIZATION.md) |

### REG-002 — PDF keyword collision must skip (no `_N` rename)

| Field | Value |
|-------|-------|
| Date | 2026-08-03 |
| Status | verified |
| Symptom | Extension organize uses `_N`; PDF path must **not** overwrite or rename |
| Root cause | Catalog B-14 collision rule differs from B-12 |
| Fix | `fileorz-core::advanced_pdf` plan skip when dest exists |
| Test | `advanced_pdf::tests::plan_skips_when_destination_exists`; golden `pdf-keywords` |
| ADR / catalog | B-14; ADR-0006 |

### REG-003 — disabled extension goes to OUTROS

| Field | Value |
|-------|-------|
| Date | 2026-08-03 |
| Status | verified |
| Symptom | Disabling `.txt` still filed under Documentos |
| Root cause | Enabled-map must omit disabled keys; unknown/disabled → OUTROS |
| Fix | `organize::map::build_extension_map` skips `enabled == false` |
| Test | `organize::map::map_tests::disabled_ext_not_in_map`; **E2E-05** |
| ADR / catalog | B-12, B-33 |

### REG-004 — trash wins when trash and permanent both true

| Field | Value |
|-------|-------|
| Date | 2026-08-03 |
| Status | verified |
| Symptom | Ambiguous dual flags could permanently delete when UX meant trash |
| Root cause | Upstream runtime: `if trash elif permanent`; both true → trash |
| Fix | `autodelete::mode` enum resolution; UI mutex radios |
| Test | `autodelete::mode::mode_tests::trash_wins_when_both_true`; settings mutex tests |
| ADR / catalog | B-22 |

### REG-005 — auto-delete must not scan Folder root loose files

| Field | Value |
|-------|-------|
| Date | 2026-08-03 |
| Status | verified |
| Symptom | Root-level aged files deleted though catalog only scans Category/EXT |
| Root cause | Scan scope is category subfolders only |
| Fix | `autodelete::plan` category walk |
| Test | `autodelete::tests::root_loose_files_not_scanned`; **E2E-02** |
| ADR / catalog | B-23 |

### REG-006 — age rule uses strict greater-than days

| Field | Value |
|-------|-------|
| Date | 2026-08-03 |
| Status | verified |
| Symptom | Files exactly `N` days old deleted when catalog requires `(now - ts).days > N` |
| Root cause | Off-by-one if using `>=` |
| Fix | `autodelete::age::exceeds_max_age` |
| Test | `autodelete::age::age_tests::exceeds_uses_strict_greater_than` |
| ADR / catalog | B-21; ADR-0005 |
