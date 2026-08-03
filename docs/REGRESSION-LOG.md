# Regression log

Registry of bugs that must not return. Every fix that closes a user-visible or
parity defect adds a row **and** an automated test when feasible.

Linked strategy: `.local/TEST-STRATEGY.md`. Commands: `npm run verify`, later
`cargo test --workspace`.

## Entry format

Copy this block for each regression:

```markdown
### REG-NNN — short title

| Field | Value |
|-------|-------|
| Date | YYYY-MM-DD |
| Status | open \| fixed \| verified |
| Symptom | What the user / parity suite saw |
| Root cause | One sentence |
| Fix | PR / commit / module |
| Test | `path::test_name` or fixture id |
| ADR / catalog | Optional link (e.g. B-12, ADR-0005) |
```

## Entries

_None yet — phase 02 scaffolding._
