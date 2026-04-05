# Contributing & PM Conventions

- **Roadmap-first:** Edit `ROADMAP.md` (Now/Next/Later, Milestones, Runbook).
- **Evidence:** `scripts/pm.sh run` → `artifacts/roadmap.json`, `docs/sessions.md`, `artifacts/state.json`.
- **Issues (optional):** `scripts/pm.sh sync` (requires `gh` + `GH_TOKEN`).
- **Labels:** `roadmap`, `lane:now|next|later|unspecified`, `pilot:406-sysops-cloud-security`.
- **Secrets:** never commit. Fetch via providers; scripts avoid `echo`; temp files use `umask 077`.
- **Commits:** Conventional Commits (`type(scope): subject`). See `.gitmessage.txt`.
- **Third-party content:** Reference only — do not redistribute vendor lab manuals or copyrighted documentation.
- **PII:** No other students' names, student IDs, or group-assignment rosters. Owner attribution is acceptable.

## Adding Weekly Content

1. Create `CC/Winter 2025/.../weekly/week-NN-<short-topic>.md` following the existing week files.
2. Add evidence screenshots to `CC/.../screenshots/` using `wkNN_topic_N.png` naming.
3. Update `CC/.../EVIDENCE_INDEX.md` with grouped references.
4. If adding student code: place in `CC/.../scripts/<project_name>/` with a README.
5. Run `scripts/pm.sh run` to regenerate artifacts.

## Adding Scripts

- **Student-authored** → `CC/.../scripts/` (with README.md documenting purpose and usage).
- **External/reference** → `CC/.../scripts-extra/` (with provenance note).
- Use executable shebangs (`#!/usr/bin/env bash`, `#!/usr/bin/env python3`).
- Set `set -euo pipefail` and `umask 077` in all shell scripts.
