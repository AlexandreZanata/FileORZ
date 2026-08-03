# ADR-0005: Linux file times for auto-delete

**Status:** Accepted  
**Date:** 2026-08-03  
**Deciders:** Fork maintainers (AlexandreZanata/FileORZ)

## Context

Upstream auto-delete uses `st_birthtime` (creation) and `st_mtime` (modified)
via `DirEntry.stat()` (`utils/AutoDelete.py`). On Linux, birth/creation time is
**not** universally available: some filesystems expose `statx` birth time
(`btime` / crtime); others do not. `st_ctime` is inode change time, **not**
creation time — never treat it as birthtime.

Official references:

- https://man7.org/linux/man-pages/man2/statx.2.html
- https://docs.python.org/3/library/os.html#os.stat_result.st_birthtime
- Behavior catalog: `.local/BEHAVIOR-CATALOG.md` (B-20..B-23)

## Decision

Map upstream “created” / “modified” filters as follows:

| Upstream intent | Linux implementation |
|-----------------|----------------------|
| Por Data de Modificação | Always `mtime` |
| Por Data de Criação | Prefer filesystem birth/crtime when `statx` (or equivalent) reports it; if unavailable, **do not invent** via `ctime` — skip the create-age check for that file and log once per scan that birthtime is unsupported |

Age rule remains: delete when `(now - timestamp).days > max_age_days` (strict
greater-than), matching upstream.

Document in UI (i18n) that “created” filtering requires filesystem support.

## Consequences

### Positive

- Correct Linux semantics; no silent wrong deletes via `ctime`.
- Parity with upstream when birthtime exists (ext4 with birth, btrfs, etc.).

### Negative

- Create-date auto-delete may no-op on some mounts — operators must use mtime.
- Characterization fixtures must control mtime (and birthtime where possible).

### Revisit trigger

If a distro matrix shows birthtime always available for supported FS list, the
“skip when missing” branch can tighten to hard-require birthtime for that filter.

## Alternatives considered

| Option | Rejected because |
|--------|------------------|
| mtime-only always | Breaks create-date feature when FS supports birthtime |
| Use `st_ctime` as creation | Incorrect on Linux (metadata change time) |
| crtime when available, else skip | **Chosen** |
