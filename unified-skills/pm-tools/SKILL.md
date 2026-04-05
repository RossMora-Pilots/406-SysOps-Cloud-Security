---
name: pm-tools
description: Orchestrate PM workflows for the pilot — parse ROADMAP.md, index sessions, commit artifacts.
---

# Skill: PM Tools

## When to Use

Load this skill when the user asks to:

- Update roadmap artifacts (`scripts/pm.sh run`)
- Sync roadmap items to GitHub Issues
- Index or regenerate the sessions list
- Commit PM evidence to the repo

## Tools

### `scripts/pm.sh`

The main PM orchestrator. Run at end of every session:

```bash
scripts/pm.sh run
```

Sub-commands: `parse`, `sync`, `index`, `all`.

### `scripts/roadmap/parse_roadmap.py`

Parses `ROADMAP.md` → `artifacts/roadmap.json`. Extracts section hierarchy and checkbox items with line numbers, checked status, and full text.

### `scripts/sessions/index_sessions.py`

Generates `docs/sessions.md` from `sessions/` directory with file sizes and sha256 hashes for traceability.

### `scripts/sessions/sanitize_session.py`

Redacts PII (emails, student IDs) and secrets (tokens, API keys) from session transcripts before they enter the repository.

## Environment

| Variable | Purpose | Default |
|---|---|---|
| `PM_COMMIT` | Auto-commit after run | `1` |
| `PM_PUSH` | Push after commit | `0` |
| `ROADMAP_PATH` | Override roadmap path | `ROADMAP.md` |
| `GH_REPO` | GitHub repo for sync | (none) |
| `GH_TOKEN` | GitHub token for sync | (none) |

## Safety Rules

- Never commit secrets.
- Use `umask 077` for all temp files.
- Do not echo credential values.
- Validate that gitleaks passes before push.
