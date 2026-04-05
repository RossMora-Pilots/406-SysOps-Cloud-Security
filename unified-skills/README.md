# Unified Skills

Reusable AI agent skills shared across Claude, Codex, and Gemini. Each skill lives in its own subdirectory with a `SKILL.md` describing its purpose, triggers, and usage.

## Distribution

Run [`../scripts/distribute_skills.sh`](../scripts/distribute_skills.sh) to create symlinks from `.claude/skills/`, `.codex/skills/`, and `.gemini/skills/` to this directory.

## Current Skills

| Skill | Purpose |
|---|---|
| [`pm-tools/`](pm-tools/) | PM workflow orchestration (roadmap parsing, session indexing) |

## Adding a New Skill

Create a subdirectory with a `SKILL.md` file following this structure:

```markdown
---
name: skill-name
description: one-sentence trigger (used by agents to decide when to load this)
---

# Skill: Name

## When to Use
[Conditions that should load this skill]

## Usage
[How to apply]

## Examples
[Concrete usage examples]
```

See the Pilot 009 template and Pilot 008 for established skill patterns.
