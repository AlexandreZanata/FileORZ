# AGENTS.md — FileORZ (fork)

> **Read this first** in any new agent session (Cursor, Claude Code, Codex, etc.).

**Language:** 100% English — code, comments, docs, commits, and all agent output.

---

## What this repo is

**Fork** of [ThainanViniciusKatchan/FileORZ](https://github.com/ThainanViniciusKatchan/FileORZ)
(see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)). Work happens **only** on
the fork remote (`origin` → `AlexandreZanata/FileORZ`). Upstream is `upstream`.

| Is | Is not |
|----|--------|
| Agent Harness rules + Cursor entry points | Permission to skip Lefthook |
| Lefthook quality gates (local commit block) | Place to commit `.local/` plans |
| Gitignored `.local/tasks/` workspace | The original upstream repo |

When rules conflict with existing code, **rules prevail** — unless the user
explicitly overrides for a task.

---

## Rules path (resolve first)

```bash
pip install -r agent-harness/requirements.txt   # once per machine
./agent-harness/rules-path.sh                   # → agent-rules/
./agent-harness/resolve-rules.sh <keywords>
```

Config: `agent-harness/harness.config.yaml` → `rules_dir: agent-rules`.

### Always load

1. `agent-rules/AGENT-CORE-PRINCIPLES.md`
2. `agent-rules/09-ai-agent-specific/token-economy.md`
3. `agent-rules/09-ai-agent-specific/anti-hallucination.md`

Cursor: `.cursor/rules/*.mdc` applies automatically.

### Conditional load

```bash
./agent-harness/resolve-rules.sh <task keywords>
./agent-harness/generate-task-rules.sh <keywords>   # optional Cursor _task-active.mdc
./agent-harness/generate-task-rules.sh --clean        # when task done
```

Load **2–6** rule files only.

---

## Quality gates (Lefthook)

Hard caps before every commit:

| Metric | Cap |
|--------|-----|
| File | ≤ **200** lines |
| Function / method | ≤ **80** lines |
| Cyclomatic complexity | ≤ **10** |
| Lint (ruff) | **0** errors, **0** warnings |
| Compile (`compileall`) | **0** errors |

```bash
npm install                  # installs lefthook + git hooks
npm run verify               # full gate
bash scripts/install-hooks.sh
```

Brownfield size debt (temporary): `scripts/quality/legacy_exemptions.txt`.
Clear via `.local/tasks/01-baseline-quality-debt/`. Never expand exemptions.

**Never** commit with `--no-verify` unless the user explicitly requests it.

---

## Architecture decisions

Accepted ADRs: [`docs/adr/`](docs/adr/README.md) (Rust, iced, Fluent, XDG, file times).  
Regression registry: [`docs/REGRESSION-LOG.md`](docs/REGRESSION-LOG.md).  
Characterization: [`docs/CHARACTERIZATION.md`](docs/CHARACTERIZATION.md), `tests/fixtures/`.  
i18n catalog: [`docs/I18N-CATALOG.md`](docs/I18N-CATALOG.md), locales in [`i18n/`](i18n/) (move to crates in phase 05/10).  
Config keys: [`docs/CONFIG-KEY-MAP.md`](docs/CONFIG-KEY-MAP.md), domain notes [`docs/CONFIG-DOMAIN.md`](docs/CONFIG-DOMAIN.md).  
Gates: `check:adr`, `characterize`, `check:i18n`, `check:config-keys`, `check:rust` (also in `verify`).  
Rust workspace: [`crates/`](crates/README.md) (`cargo test --workspace`).

---

## Local workspace (gitignored)

```text
.local/tasks/<task-slug>/
  OFFICIAL-REFERENCE.md
  README.md
  TASK.md

.local/phases/<NN-slug>/
  OFFICIAL-REFERENCE.md
  README.md
  TASKS.md
```

See `.local/README.md` and `.local/IMPLEMENTATION-PLAN.md` for the **i18n + Rust Linux**
program (phases 02–20). **Never commit `.local/`.**

---

## Remotes

| Remote | URL |
|--------|-----|
| `origin` | `git@github.com:AlexandreZanata/FileORZ.git` (this fork — **only** push target) |
| `upstream` | `git@github.com:ThainanViniciusKatchan/FileORZ.git` (original) |
