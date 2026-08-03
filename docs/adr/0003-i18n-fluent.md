# ADR-0003: i18n — Fluent catalogs

**Status:** Accepted  
**Date:** 2026-08-03  
**Deciders:** Fork maintainers (AlexandreZanata/FileORZ)

## Context

Upstream UI strings are hardcoded Brazilian Portuguese. The fork ships **en** and
**pt-BR** now, with add-only locales later. Message IDs must be stable English
identifiers (`area.component.element`).

Official references:

- https://projectfluent.org/
- https://github.com/projectfluent/fluent-rs
- https://www.rfc-editor.org/rfc/rfc5646

## Decision

Use **Fluent (`.ftl`)** via `fluent-rs` in `fileorz-i18n`. Layout:

```text
crates/fileorz-i18n/locales/{en,pt-BR}/*.ftl
```

**Fallback chain:** requested locale → `en` → message-id literal (debug builds
only; release logs missing IDs).

**Locale selection order:** `--locale` → config `locale` → default `en`.
System `LANG` / `LC_MESSAGES` do **not** auto-select the UI (Linux product
starts in English; user changes language in Settings).

**CI ID check (phase 10+):** every ID referenced in Rust UI/core errors must
exist in **both** `en` and `pt-BR` catalogs; deleting an ID fails CI.

## Consequences

### Positive

- Plurals/selectors without reinventing gettext plural rules.
- Catalogs reviewable as text; clear extraction in phase 04.

### Negative

- Translators less familiar with Fluent than PO in some communities.
- Need custom CI checker (not gettext `msgfmt` alone).

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Fluent | **Chosen** — matches I18N-STRATEGY |
| gettext (`.po`) | Mature tooling, weaker first-class Rust story for our crates |
| `rust-i18n` JSON only | Weaker plural/select support than Fluent |
