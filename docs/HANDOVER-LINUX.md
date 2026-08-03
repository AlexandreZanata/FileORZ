# Handover — Linux FileORZ (maintainers)

Ops notes after `linux-v1.0.0`. Product is the **Rust** binary on **origin**
(`AlexandreZanata/FileORZ`). Do not push releases to `upstream`.

## Daily commands

```bash
npm run verify                 # harness + rust + packaging + parity gates
cargo test --workspace
./scripts/e2e-linux.sh         # needs xvfb
npm run characterize           # Python oracle goldens
bash scripts/package-linux.sh  # → dist/linux/{fileorz,*.deb,SHA256SUMS}
```

## Add a language

1. Copy `crates/fileorz-i18n/locales/en/` → `locales/<tag>/` (BCP 47, e.g. `es`).
2. Translate all `.ftl` files; keep **identical message IDs**.
3. Register the locale in `fileorz-i18n` embed/load code and UI locale picker.
4. Run `npm run check:i18n` (ID sets must match across locales).
5. Smoke: `cargo run -p fileorz -- --demo-i18n --locale <tag>` and UI switch.
6. Update [I18N-CATALOG.md](I18N-CATALOG.md) / [I18N-RUNTIME.md](I18N-RUNTIME.md).

Details: ADR-0003, [I18N-RUNTIME.md](I18N-RUNTIME.md).

## Run tests (pyramid)

| Layer | Command |
|-------|---------|
| Harness | `npm run verify` |
| Unit / integration | `cargo test --workspace` |
| Goldens (oracle) | `npm run characterize` |
| Parity fixtures | `cargo test -p fileorz-parity` |
| Optional matrix lock | `cargo test -p fileorz-parity -- --include-ignored` |
| E2E | `./scripts/e2e-linux.sh` (`FILEORZ_E2E=1` under Xvfb) |

Behavior matrix: [PARITY-REPORT.md](PARITY-REPORT.md).  
Regressions: [REGRESSION-LOG.md](REGRESSION-LOG.md).

## Cut a patch release

1. Bump `[workspace.package] version` in root `Cargo.toml`.
2. Update release notes (new file or amend).
3. `npm run verify` && `./scripts/e2e-linux.sh`.
4. `bash scripts/package-linux.sh` → attach `fileorz`, `.deb`, `SHA256SUMS`.
5. Commit, tag `linux-vX.Y.Z`, push tag to **origin only**.
6. `gh release create linux-vX.Y.Z … --repo AlexandreZanata/FileORZ`.

## Rollback

1. Revert users to previous `.deb` / binary from the prior GitHub release.
2. Point docs/install at the previous tag.
3. Hotfix on a new patch tag — do not rewrite published checksums in place.

## Known limits

- Primary package is `.deb` (no AppImage yet).
- GNOME tray needs AppIndicator / SNI extension ([LINUX-DESKTOP.md](LINUX-DESKTOP.md)).
- Python size-debt task (`01-baseline-quality-debt`) is deferred; never expand
  `scripts/quality/legacy_exemptions.txt`.
- CI workflow: `.github/workflows/e2e-linux.yml` — enable required checks on
  `main` in GitHub branch protection.

## Attribution

Keep [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md) and About-dialog credits
when distributing. GPL obligations: [INSTALL-LINUX.md](INSTALL-LINUX.md).
