# i18n runtime (Fluent)

`fileorz-i18n` loads embedded Fluent catalogs (ADR-0003).

## Layout

```text
crates/fileorz-i18n/locales/{en,pt-BR}/{main,settings,errors,tray}.ftl
```

## API

| Item | Role |
|------|------|
| `resolve_locale` / `resolve_locale_from_env` | CLI → config → `en` (ignores `LANG`) |
| `normalize_locale` | Map `pt_BR.UTF-8` → `pt-BR`, `en_US` → `en` |
| `Localization::embed` | Compile-time catalogs |
| `Localization::from_dir` | Filesystem load (tests / tools) |
| `Localization::message` / `t!` | primary → `en` → message-id literal |

## CI

```bash
npm run check:i18n   # scripts/check-i18n.sh → ID set equality
cargo test -p fileorz-i18n
cargo run -p fileorz -- --demo-i18n --locale pt-BR
```

Catalog inventory: `docs/I18N-CATALOG.md`.
