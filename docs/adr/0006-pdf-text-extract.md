# ADR-0006: PDF text extraction crate

**Status:** Accepted  
**Date:** 2026-08-03  
**Deciders:** Fork maintainers (AlexandreZanata/FileORZ)

## Context

Advanced PDF keyword organize (B-14) needs text from `*.pdf` files at the
organize root, matching `AdvancedAlg/Alg.py` / `pypdf` semantics closely enough
for characterization fixtures. Phase 09 named two candidates: `pdf-extract` and
`lopdf`. Extraction must support **per-page** reads because upstream overwrites
the haystack each page (effective match = last successfully read page).

Official references:

- https://docs.rs/lopdf/
- https://github.com/J-F-Liu/lopdf
- Behavior catalog: `.local/BEHAVIOR-CATALOG.md` (B-14)

## Decision

Use **`lopdf`** (`Document::load` + `extract_text(&[page])`) inside
`fileorz-core::advanced_pdf`. Do **not** add `pdf-extract` for v1.

## Consequences

### Positive

- Page-level extract matches the last-page parity quirk without custom parsing.
- Sufficient for synthetic fixture PDFs (Type1 Helvetica literal strings).
- One dependency surface (pdf-extract itself wraps older lopdf).

### Negative

- Complex real-world PDFs (CJK, tagged, unusual encodings) may extract poorly —
  same class of risk as upstream `pypdf` for this feature; NLP expansion is out
  of scope for phase 09.
- `lopdf` 0.39 declares `rust-version = "1.85"` (workspace MSRV stays documented
  separately; CI uses a newer toolchain).

### Revisit trigger

If fixture or user PDFs fail substring match due to extractor gaps, evaluate
`pdf-extract` or another extractor behind the same `last_page_haystack` API.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| `pdf-extract` | Higher-level whole-doc API; heavier stack; task allows either — lopdf fits page quirk directly |
| Hand-rolled content-stream scrape | Fragile; reinventing lopdf |
| `lopdf` page extract | **Chosen** |
