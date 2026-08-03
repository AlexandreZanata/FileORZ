# Test fixtures (draft)

Placeholder trees and configs for characterization (phase 03+) and Rust
integration tests. Prefer synthetic files; never commit secrets.

Layout matches `.local/TEST-STRATEGY.md` and ADRs 0001–0005.

```text
tests/fixtures/
  trees/
    tiny-mixed/       # few extensions → Category/EXT + OUTROS
    pdf-keywords/     # PDFs for advanced organize
    aged-files/       # mtime (and birthtime when test harness can set)
  configs/
    legacy-pt.json    # Portuguese / mixed keys
    stable-v1.json    # post-migration English keys
  golden/
    organize-tiny.json  # expected move list (filled in phase 03)
```

## Status

Directories exist; payloads are placeholders until phase 03 records goldens.
