# Architecture Decision Records

ADRs lock non-trivial choices for the Linux-first Rust rewrite of this FileORZ fork.

Template: `agent-rules/11-documentation-and-glossary/adr-template.md`.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-linux-rust-rewrite.md) | Linux Rust rewrite | Accepted |
| [0002](0002-ui-toolkit-iced.md) | UI toolkit: iced | Accepted |
| [0003](0003-i18n-fluent.md) | i18n: Fluent | Accepted |
| [0004](0004-xdg-config-layout.md) | XDG config layout | Accepted |
| [0005](0005-linux-file-times.md) | Linux file times for auto-delete | Accepted |

## Rules

- New stack or schema fork → draft ADR before coding.
- Status must be one of: Proposed, Accepted, Deprecated, Superseded by ADR-XXX.
- Every Accepted ADR needs Status, Context, Decision, Consequences, and ≥1 official URL.
- Revisit only when the ADR’s trigger is met; do not re-decide casually.
