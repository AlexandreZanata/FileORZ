# Advanced PDF keywords (Rust)

`fileorz-core::advanced_pdf` implements B-14 (ADR-0006: `lopdf` extract).

## Rules

| Concern | Behavior |
|---------|----------|
| Scope | Top-level files ending in `.pdf` only |
| Haystack | Last successfully extracted page, uppercased |
| Match | Case-insensitive substring; JSON group order then phrase order |
| First hit | Move or skip, then stop further groups for that file |
| Destination | `{group}/{filename}`; `mkdir -p` |
| Collision | Destination exists → skip (no `_N` rename) |
| Non-match | Left for extension organize |

## API

```text
load_keywords(path) -> KeywordGroups
last_page_haystack(path) -> String
plan_pdf_actions(root, &keywords) -> Vec<PdfAction>
apply_pdf_actions(root, &actions)
find_first_group(haystack, &keywords) -> Option<&str>
```

Keywords path (Linux): `$XDG_CONFIG_HOME/fileorz/keywords.json` (ADR-0004).

Golden: `tests/fixtures/golden/pdf-keywords.json`.

```bash
cargo test -p fileorz-core advanced_pdf
```
