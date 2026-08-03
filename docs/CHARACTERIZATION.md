# Characterization fixtures

How to refresh and interpret golden filesystem manifests for organize /
auto-delete / advanced PDF parity (phase 03).

Catalog: `.local/BEHAVIOR-CATALOG.md` (B-10..B-23). Strategy: `.local/TEST-STRATEGY.md`.

## Layout

```text
tests/fixtures/
  trees/
    tiny-mixed/       # txt png mp3 zip + unknown .xyz + .hidden
    collision/        # notes.txt + pre-seeded Documentos/TXT/notes.txt
    pdf-keywords/     # synthetic PDFs + Nota Fiscal/dup.pdf collision
    aged-files/       # Documentos/TXT/{old,fresh}.txt (mtimes set at runtime)
  configs/
    organize-basic.json
    organize-advanced.json
    autodelete-mtime.json
    keywords-fixture.json
  golden/
    tiny-mixed.json
    collision.json
    pdf-keywords.json
    aged-files.json
```

## Golden schema

| Field | Meaning |
|-------|---------|
| `tree` | Source tree under `tests/fixtures/` |
| `config` / `keywords` | Config JSON paths |
| `pipeline` | Ordered stages: `auto_delete`, `advanced_pdf`, `extension` |
| `mtime` | Optional `{relpath: {days_ago: N}}` applied after copy |
| `expected_actions` | `{action, from, to?}` — `move` \| `skip` \| `delete` |
| `expected_present` / `expected_absent` | Final relative paths |
| `expected_root_remaining` | Filenames left at Folder root |

## Oracle

Catalog-faithful Python replay (not the full GUI app):

```bash
./scripts/characterize-python.sh
# or
npm run characterize
python3 scripts/run_characterization.py --selftest-corrupt
```

Rules encoded:

- Extension: enabled map → `Category/EXT/`; else `OUTROS`; collide with `_N`; skip dotfiles.
- PDF: case-insensitive contains; first group/phrase; skip if dest exists; then extension for leftovers.
- Auto-delete: permanent + mtime `>` days; scans capitalized category folders (Linux-normalized Windows case-fold).

Oracle: `scripts/characterization/` (needs `pypdf` from `requirements-dev.txt`).


## Refreshing goldens

1. Change fixture tree or config intentionally.
2. Run oracle; if behavior matches catalog, update `expected_*` in the golden JSON.
3. Do **not** change goldens to hide product bugs — file REG entry instead.
4. Re-run `./scripts/characterize-python.sh` and `npm run verify`.

## Corrupt self-test

`--selftest-corrupt` mutates `tiny-mixed` expected `to` and asserts the runner
raises `AssertionError` (REG-001 seed).
