# Scheduler / organize loop

`fileorz-core::scheduler` runs the B-10 tick pipeline on an interval with
cooperative start/stop (no `kill -9` / `taskkill`).

## Tick order (B-10)

1. Validate organize root (exists, directory, writable)
2. Auto-delete (if enabled in config)
3. Advanced PDF keywords (if `advanced_organize` and keywords non-empty)
4. Extension organize

## API

| Item | Role |
|------|------|
| `run_tick(...)` | Single pipeline pass → `TickReport` |
| `OrganizerHandle::start(OrganizerOptions)` | Spawn worker thread (tick, then sleep) |
| `OrganizerHandle::stop(timeout)` | Set stop flag + join within timeout |
| `wait_interruptible` | Interval sleep that returns early on stop |

`OrganizerOptions.interval` overrides `config.interval_minutes` for tests/UI.

## CLI

```bash
fileorz organize --once --config <path> --folder <path> [--keywords <path>]
```

### Exit codes

| Code | Meaning |
|------|---------|
| 0 | ok |
| 1 | organize / runtime error |
| 2 | config or keywords error |
| 3 | folder missing / not writable |
| 4 | usage error |

```bash
cargo test -p fileorz-core scheduler
cargo run -p fileorz -- organize --once --config … --folder …
```
