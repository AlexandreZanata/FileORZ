# ADR-0002: UI toolkit — iced

**Status:** Accepted  
**Date:** 2026-08-03  
**Deciders:** Fork maintainers (AlexandreZanata/FileORZ)

## Context

Upstream UI is CustomTkinter (dark professional shell). Linux v1 must preserve
flows (main → start, settings hub, tray hide/quit) with a retained-mode desktop
UI that works on **Wayland and X11**.

Official references:

- https://iced.rs/
- https://book.iced.rs/
- https://docs.rs/iced/

## Decision

Use **iced 0.13+** for `fileorz-ui`. Dark-first theming to approach the current
CTk look. Tray/StatusNotifier and XDG portals may use thin Linux helpers
(`fileorz-linux`) rather than forcing every OS API through iced widgets.

## Consequences

### Positive

- Pure Rust UI; good custom theming for product polish.
- Cross-compositor targets (Wayland/X11) without shipping GTK CSS stacks.

### Negative

- Less “GNOME-native” than libadwaita; a11y work is explicit (phase 16).
- iced API churn historically — pin versions in workspace.

### Wayland notes

- Prefer portal file dialogs when available; fall back to iced/rfd pickers.
- Smoke HiDPI / fractional scaling on at least one Wayland session (phase 18).
- Tray: FreeDesktop StatusNotifier; do not assume XEmbed-only trays.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| iced | **Chosen** — retained-mode + theming fit |
| relm4 + libadwaita | Most native GTK look; slower visual parity with CTk layout; heavier GTK dep story |
| egui | Fine for tools; weaker “product app” polish for this brief |
| Keep CustomTkinter | Blocks Rust single-binary goal |
