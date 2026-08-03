# ADR-0004: XDG config and data layout

**Status:** Accepted  
**Date:** 2026-08-03  
**Deciders:** Fork maintainers (AlexandreZanata/FileORZ)

## Context

Upstream stores `dist/config.json` and `dist/Key_Words.json` near the executable
(or under AppData after install). Linux must follow the FreeDesktop base-dir
spec and migrate legacy Portuguese JSON keys once.

Official references:

- https://specifications.freedesktop.org/basedir-spec/latest/
- https://specifications.freedesktop.org/desktop-entry-spec/latest/
- https://specifications.freedesktop.org/trash-spec/latest/

## Decision

Exact paths (expand `$HOME` / env as usual):

| Concern | Path |
|---------|------|
| Config | `$XDG_CONFIG_HOME/fileorz/config.json` (default `~/.config/fileorz/config.json`) |
| Keywords | `$XDG_CONFIG_HOME/fileorz/keywords.json` |
| Data/cache | `$XDG_DATA_HOME/fileorz/` (default `~/.local/share/fileorz/`) |
| Autostart | `$XDG_CONFIG_HOME/autostart/fileorz.desktop` |

**Writes:** serialize JSON to a temp file in the same directory, `fsync`, then
atomic `rename` onto the target. Never truncate the live file in place.

**Legacy migrate:** on first load of Portuguese/mixed keys, convert to stable
English schema (see I18N-STRATEGY), write new config, keep backup
`config.json.bak-legacy` beside the live file. Do not rename user category
folders on disk.

Paths always use `std::path::Path` — never invent Windows `\`.

## Consequences

### Positive

- Predictable Linux packaging and backups.
- Safe concurrent UI/worker config updates via atomic replace.

### Negative

- One-shot migration code until legacy files vanish.
- Autostart is FreeDesktop-only (no Windows registry in this binary).

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| Keep `dist/` beside binary | Breaks FHS/XDG expectations; writable install dirs |
| XDG layout + atomic write | **Chosen** |
| SQLite for config | Overkill for small JSON; harder to hand-edit |
