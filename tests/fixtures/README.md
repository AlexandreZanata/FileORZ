# Test fixtures

Synthetic trees + golden manifests for characterization (phase 03+) and later
Rust parity. Prefer synthetic files; never commit secrets.

See [`docs/CHARACTERIZATION.md`](../../docs/CHARACTERIZATION.md).

```text
tests/fixtures/
  trees/
    tiny-mixed/
    collision/
    pdf-keywords/
    aged-files/
  configs/
    organize-basic.json
    organize-advanced.json
    autodelete-mtime.json
    keywords-fixture.json
    legacy-pt.json
    stable-v1.json
  golden/
    tiny-mixed.json
    collision.json
    pdf-keywords.json
    aged-files.json
```

## Status

Phase 03 goldens committed; oracle: `npm run characterize`.
