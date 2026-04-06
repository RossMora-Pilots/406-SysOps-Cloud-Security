# ROADMAP — SysOps and Cloud Security (CSC-7308) — Winter 2025 (Pilot 406)

This pilot creates a public, employer-facing course portfolio repository for **SysOps and Cloud Security (CSC-7308)**, Winter 2025, taught by Instructor Aditya Palshikar. It reuses the Pilot 009 template structure and conventions and is modeled on Pilots 008 and 010.

## Now

- [x] Establish course repo structure (README, docs, assignments, artifacts, scripts)
- [x] Align conventions with Pilot 008/009/010 (naming, labels, evidence)
- [x] Author root README with course overview, skills, and navigation
- [x] Create course-level README with weekly topic map
- [x] Author per-week summaries (Weeks 1–9) with topics, tools, and reflections
- [x] Document Build a SOC (group project) in MIDTERM_PROJECT_SUMMARY.md
- [x] Preserve original Rust ping sweep code and documentation
- [x] Create EVIDENCE_INDEX.md and SCRIPTS_README.md
- [x] Configure `portfolio/config.json` with course metadata
- [x] Install CI workflows (portfolio-ci, pm-evidence, markdownlint, gitleaks)
- [x] Add COURSE_REFLECTION.md with learning outcomes

## Next

- [x] Capture and add week-by-week screenshots (wkNN_topic_N.png) — 44 screenshots
- [x] Sanitize and add PDF versions of student submissions to `assignments/`
- [x] Add skills matrix (`docs/skills-matrix.md`) with NICE/CyBOK alignment
- [x] Populate `docs/references.md` with external reading
- [x] Enrich all 8 assignment files with executive summaries, analysis sections, and descriptive alt text
- [x] Add Mermaid diagrams to all 9 weekly summaries (14 diagrams total)
- [x] Add MITRE ATT&CK technique IDs to Kill Chain mappings
- [x] Add config examples (EDL, URL/DNS/file-blocking, K8s NetworkPolicy)
- [x] Generate skills radar SVG chart (`docs/skills-radar.svg`)
- [ ] Run PM plumbing to generate `artifacts/roadmap.json`
- [ ] Push to GitHub; set repo topics and description

## Later

- [ ] Optional GitHub Pages landing page with interactive weekly navigation
- [ ] Architecture diagrams for the Build a SOC project (Wazuh deployment)
- [ ] Learning reflection video or screencast (3-minute narrated lab walkthrough)
- [ ] Cross-link with Pilots 008 and 010 in a program-level landing page
- [ ] Live Wazuh detection demo (working rule triggered in sandbox)
- [ ] Blog post adaptation from a weekly reflection
- [ ] Rust ping sweep unit tests for subnet arithmetic
- [ ] Service mesh (Istio/Linkerd) appendix for Week 9

## Milestones (Definition of Done)

- [x] Course repo structure finalized and documented
- [x] Root README is employer-facing with Quick Start, Skills, Weekly Topic Map
- [x] Course README has comprehensive navigation and context
- [x] All 9 delivered weeks have individual summary files
- [x] Build a SOC group project has a full writeup
- [x] Original code work (Rust ping sweep) preserved with documentation
- [x] No PII of other students or secrets in repository
- [x] Evidence artifacts present (roadmap.json, state.json, sessions index)
- [x] Screenshots captured and indexed (44 PNGs, 31.6 MB)
- [x] All 8 assignments enriched to professional lab report quality
- [x] Mermaid diagrams in all 9 weekly summaries (14 total)
- [x] Skills radar SVG generated and linked
- [x] Independent employer audit completed, all gaps remediated (grade: A)
- [ ] Repo published to GitHub with topics and description

## Runbook

- **PM loop:** `scripts/pm.sh run` (auto-commit artifacts)
- **Publish:** `PM_PUSH=1 scripts/pm.sh all`
- **Issues (optional):** `scripts/pm.sh sync` (requires `gh` + `GH_TOKEN`)
- **Regenerate README sections:** `PYTHONPATH=. scripts/portfolio/run.sh all`
- **Distribute AI skills:** `scripts/distribute_skills.sh`
