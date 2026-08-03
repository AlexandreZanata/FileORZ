# ADR-0001: Linux-first Rust rewrite

**Status:** Accepted  
**Date:** 2026-08-03  
**Deciders:** Fork maintainers (AlexandreZanata/FileORZ)

## Context

Upstream FileORZ is a Windows-oriented Python + CustomTkinter app, shipped via
Nuitka / Inno (`FL_ORZ.exe`). This fork targets a **Linux desktop** product with
en + pt-BR UI, XDG paths, and a single shippable binary. Keeping the full Python
GUI stack conflicts with “compilable for all Linux” and professional desktop UX.

Official references:

- https://doc.rust-lang.org/book/
- https://doc.rust-lang.org/cargo/reference/workspaces.html
- Upstream: https://github.com/ThainanViniciusKatchan/FileORZ

## Decision

Rewrite the product as a **Cargo workspace** Rust binary for Linux
(`x86_64-unknown-linux-gnu` primary). Domain logic lives in `fileorz-core`; UI,
i18n, and Linux integration are separate crates. Python sources remain a
**parity oracle / historical reference** until phase 19, then retire from the
production ship path (not from attribution / GPL notices).

## Consequences

### Positive

- One native binary; no Nuitka/Windows installer as the primary path.
- Strong typing and harness-friendly module splits (file ≤200, fn ≤80).
- Clear Linux packaging story (AppImage / `.deb` later).

### Negative

- Full rewrite cost; temporary dual maintenance until golden fixtures lock.
- Team must learn Rust + chosen UI/i18n stack.

### Revisit trigger

Revisit only if: (a) iced + Fluent path is abandoned in favor of shipping a
Python Linux build, or (b) a second OS must share the same binary before Linux
v1 is stable.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Keep Python + Nuitka on Linux | Conflicts with single native Linux binary goal; poor Wayland/desktop story |
| Go rewrite | Viable, but weaker fit for retained-mode GUI + Fluent ecosystem chosen here |
| Rust rewrite | **Chosen** — matches locked `.local/ARCHITECTURE-LINUX-RUST.md` |
