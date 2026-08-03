# Temporary Fluent locale root (phase 04)

Files move to `crates/fileorz-i18n/locales/` in phase 05/10.

```text
i18n/
  en/{main,settings,errors,tray}.ftl
  pt-BR/{main,settings,errors,tray}.ftl
```

Validate ID parity:

```bash
python3 scripts/check_i18n_ids.py
```

Catalog: `docs/I18N-CATALOG.md`.
