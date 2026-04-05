# AGENTS.md — 406-SysOps-Cloud-Security

Marker: PROJECT_OK

## Overview

**Purpose:** Public, employer-facing course portfolio for SysOps and Cloud Security (CSC-7308), Winter 2025, Cambrian College Postgraduate Cybersecurity Certificate. Instructor: Aditya Palshikar.

**Scope:** This directory and all subdirectories.

**Status:** Active portfolio — content populated from source course materials at `D:/CC/Winter2025/SysOps and Cloud Security - Aditya Palshikar - CSC-7308 - 11819 - 202501 - 002`.

---

## Pre-Flight Awareness Check

Before starting work on this pilot, start your session through the cross-pilot awareness wrapper:

```bash
/mnt/d/pilots/02001-Pilots-Aware-of-Other-Pilots/scripts/start_session.sh "$(basename $PWD)"
```

This runs the pre-flight awareness check, searches 300+ pilots for related work, flags potential duplicates, and writes a local awareness proof stamp used by supported workflows. For deeper searches, use:

```bash
/mnt/d/pilots/02001-Pilots-Aware-of-Other-Pilots/scripts/search_pilots.sh "topic or keyword"
```

---

## Quick Start

```bash
# 1. Check for related existing work
/mnt/d/pilots/02001-Pilots-Aware-of-Other-Pilots/scripts/start_session.sh "$(basename $PWD)"

# 2. Read ROADMAP.md for current tasks
cat ROADMAP.md

# 3. Run PM loop to refresh artifacts
scripts/pm.sh run
```

---

## Safety Rules

- **Never commit secrets.** No tokens, API keys, or credentials. Gitleaks is configured.
- **Respect third-party copyright.** Palo Alto Networks lab PDFs, Wazuh documentation, and other vendor materials are referenced but not redistributed.
- **Protect other students' PII.** The Build a SOC team roster contains other students' names and IDs — it must never appear in this repo. Only the repository owner's name/ID is retained as portfolio attribution.
- **Update state files** after completing work.
- **Create handover record** when finishing a session (see `docs/sessions.md`).

---

## Related Pilots

- **Pilot 008** — Cybersecurity Network Defense Portfolio (reference implementation)
- **Pilot 009** — Course Repository Template and Guidelines (canonical spec)
- **Pilot 010** — Intro to Cybersecurity CSC-7301 (companion course repo)
- **Pilots 400–410** — Cambrian College Postgraduate Cybersecurity Certificate course portfolios
