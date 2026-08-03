# aged-files — placeholder

Files under `Category/EXT/` with controlled ages for auto-delete.

Suggested (phase 03):

- `old.txt` — mtime older than `max_age_days` → delete when modify filter on
- `fresh.txt` — mtime within window → keep
- birthtime cases only where the test harness can set crtime (see ADR-0005)

Scan target: category subfolders only, not loose files at Folder root.
