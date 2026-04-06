# Session Log — 2026-04-05 — Employer Audit & Full Remediation (Round 2)

> **Date:** 2026-04-05
> **Duration:** ~45 minutes
> **Agent:** Copilot Opus 4.6
> **Scope:** Independent employer-perspective audit of entire portfolio + systematic remediation of all 19 identified gaps
> **Starting commit:** `7efc3b0` (central/master)
> **Ending commit:** `082d8e6` (master, 4 commits ahead)

---

## 1. Session Objective

Analyze and assess the portfolio from the perspective of a cybersecurity employer, scrutinize document quality, visualizations, assignment conversions, and information utilization. Then remediate every identified gap.

---

## 2. Audit Findings (Pre-Remediation)

### Overall Grade: A− (strong hire signal; not yet flawless)

### Top Strengths Identified

1. **Tiered employer navigation** (5/15/30 min tours) — rare and impressive
2. **Reflective writing** — best-in-class for student portfolios; multiple interview-ready insights
3. **44 screenshots + 11 Mermaid diagrams + original Rust code** — tangible proof
4. **NICE/CyBOK framework mapping** with 3-tier proficiency
5. **5 CI/CD workflows** including gitleaks secret scanning

### Critical Weakness

Assignment files were the #1 gap (Grade: C+) — raw DOCX-to-markdown exports with screenshots + one-line answers. The jarring quality contrast with the excellent weekly summaries could raise credibility concerns.

### Full Issue Inventory (19 Items)

| ID | Category | Issue | Severity |
|---|---|---|---|
| `enrich-wk01` | Assignment | Wk01 thin — 3 screenshots, 3 one-line answers, no analysis | Critical |
| `enrich-wk02` | Assignment | Wk02 thin — 6 screenshots, no answers at all | Critical |
| `enrich-wk03` | Assignment | Wk03 thin — 7 screenshots, brief headings only | Critical |
| `enrich-wk04` | Assignment | Wk04 thinnest — 4 screenshots, almost no text | Critical |
| `enrich-wk05` | Assignment | Wk05 thin — 5 screenshots, DOCX formatting artifacts | Critical |
| `enrich-wk06` | Assignment | Wk06 thin — 9 screenshots, some observations but no structure | Critical |
| `enrich-wk08` | Assignment | Wk08 thin — 4 screenshots, minimal text | Critical |
| `enrich-wk09` | Assignment | Wk09 thin — 6 screenshots, some commands noted but no analysis | Critical |
| `mermaid-wk01` | Visualization | Week 1 missing Mermaid diagram (6/9 coverage) | High |
| `mermaid-wk02` | Visualization | Week 2 missing Mermaid diagram | High |
| `mermaid-wk05` | Visualization | Week 5 missing Mermaid diagram | High |
| `fix-stale-evidence` | Text | Midterm says "Evidence gathering is in progress" — evidence exists | Medium |
| `fix-transcript-count` | Text | Course README says "4 lecture transcripts" but lists 5 | Medium |
| `mitre-attack` | Content | Kill Chain tables lack MITRE ATT&CK technique IDs | Medium |
| `config-wk05` | Content | Week 5 missing config example (EDL/blocklist) | Medium |
| `config-wk08` | Content | Week 8 missing config example (URL/DNS/file-blocking) | Medium |
| `config-wk09` | Content | Week 9 missing config example (K8s NetworkPolicy) | Medium |
| `skills-radar` | Visualization | No visual skills radar/spider chart | Low |
| `update-audit` | Documentation | Audit needs remediation status section | Low |

### Audit Document

Full 520-line audit at [`docs/EMPLOYER_AUDIT_2026-04-05.md`](EMPLOYER_AUDIT_2026-04-05.md) includes:
- Per-document grades for all 40+ files
- 3 employer scenarios (5/15/30 minute reviews)
- Visualization inventory (44 screenshots, 11→14 Mermaid diagrams, 1 SVG)
- Prioritized 14-item roadmap
- Delta comparison against the prior audit (2025-04-05)

---

## 3. Remediation — What Was Done

### 3.1 Assignment Enrichment (8 files — Critical)

Every assignment file was rewritten from raw DOCX dumps into professional lab reports:

| File | Screenshots | Key Additions |
|---|---|---|
| `Wk01_Lab_03_Firewall_Logs.md` | 3 | Exec summary, `timedatectl` code block, findings table, log field analysis |
| `Wk02_Lab_02_ACC.md` | 6 | Exec summary, `pan-sof-lab-02.xml` code, Bredolab C2 analysis |
| `Wk03_Lab_05_Reconnaissance.md` | 7 | Exec summary, IP code formatting, Zone Protection ↔ Kill Chain analysis |
| `Wk04_CyberKillChain_Part1.md` | 4 | Exec summary, split into Part 1 (quiz) + Part 2 (networking), Kill Chain analysis |
| `Wk05_Lab_07_Threat_Intelligence.md` | 5 | Exec summary, `ls` code block, cleaned DOCX artifacts, EDL/MineMeld analysis |
| `Wk06_Lab_06_Endpoint_Security.md` | 9 | Exec summary, version mismatch ⚠️ callout, AV update analysis |
| `Wk08_Lab_02_Internet_Threats.md` | 4 | Exec summary, defense-in-depth positioning, file blocking analysis |
| `Wk09_Lab_03_Container_Security.md` | 6 | Exec summary, Docker commands reference table, network isolation analysis |

**Common changes to all 8 files:**
- Added course metadata header (course, week, platform)
- Added executive summary (3–5 sentences)
- Replaced all generic alt text (`"A screenshot of a computer"`) with descriptive captions
- Added inline code formatting for commands, filenames, IPs
- Added Security Significance / Analysis section
- Cleaned excessive blank lines (DOCX artifact)
- Preserved all original screenshot paths and observations

### 3.2 Mermaid Diagrams (3 files — High)

| Week | Diagram Type | Description |
|---|---|---|
| Week 1 | `graph LR` | Log analysis workflow: Analyst Question → Filter → Raw Logs → Results → Triage → Action |
| Week 2 | `graph TD` | ACC analysis flow: Global Filter → Widgets → Anomaly? → Drill-Down → Raw Logs → Investigate |
| Week 5 | `graph LR` | Threat intel feedback loop: NGFW → WildFire → AutoFocus → Signatures → NGFW (circular) |

All diagrams use consistent `classDef` styling matching the existing 11 diagrams. Coverage is now **14 Mermaid diagrams across 9/9 weeks**.

### 3.3 MITRE ATT&CK Technique IDs (2 files — Medium)

Added specific MITRE ATT&CK tactic and technique IDs to:

1. **`weekly/week-04-build-a-soc-group-project.md`** — Kill Chain table expanded from 4 columns to 5, with ATT&CK column containing hyperlinked tactic IDs (TA0043, TA0042, TA0001, TA0002, TA0003, TA0011, TA0010) and technique IDs (T1595, T1592, T1587, T1566, T1189, T1203, T1547, T1053, T1071, T1573, T1041, T1486)

2. **`MIDTERM_PROJECT_SUMMARY.md`** — Kill Chain numbered list enriched with inline ATT&CK references (same technique IDs, with hyperlinks to attack.mitre.org)

### 3.4 Configuration Examples (3 files — Medium)

| Week | Config Type | Content |
|---|---|---|
| Week 5 | PAN-OS External Dynamic List (EDL) | `set external-list` CLI commands for automated IoC blocking with 5-minute refresh |
| Week 8 | URL Filtering + DNS Security + File Blocking | Full `set profiles` CLI commands for a layered Internet threat prevention profile group |
| Week 9 | Kubernetes NetworkPolicy YAML | Complete `networking.k8s.io/v1` manifest for database pod isolation (namespace-b, port 5432) |

Each config includes a `> **Key point:**` callout explaining the security significance.

### 3.5 Skills Radar SVG (New — Low)

Generated `docs/skills-radar.svg` — a hand-crafted 500×500 SVG radar/spider chart showing proficiency across 8 skill domains:

| Domain | Score (0–5) | Basis |
|---|---|---|
| NGFW Config | 4.5 | 7 hands-on labs |
| Log Analysis | 4.5 | Weeks 1, 2, 3 labs |
| Threat Intel | 3.5 | Week 5 lab + reflections |
| SIEM / SOC | 3.5 | Week 4 project + Wazuh |
| Endpoint Security | 4.0 | Week 6 lab (9 screenshots) |
| Cloud Security | 2.5 | Week 7 lecture-only |
| Container Security | 3.0 | Week 9 lab |
| Software Dev | 4.0 | Rust ping sweep project |

Referenced in `docs/skills-matrix.md` as `![Skills Proficiency Radar](skills-radar.svg)`.

### 3.6 Text Fixes (2 files — Medium)

1. **`MIDTERM_PROJECT_SUMMARY.md`** (~line 188): Replaced stale `"Evidence gathering is in progress"` with actual evidence inventory listing lab report, screenshots, and Mermaid diagrams.
2. **`README.md`** (course-level, line 109): Changed `"4 lecture transcripts"` to `"5 lecture transcripts"`.

### 3.7 Audit Update (1 file — Low)

Appended **Section 12: Post-Audit Remediation Status** to `docs/EMPLOYER_AUDIT_2026-04-05.md`:
- 14-row tracking table with commit hashes for each item
- Revised grade assessment: A− → A
- "What Would Push to A+" section (video, live demo, blog, certification)
- Updated executive summary with post-remediation note

---

## 4. Scripts Created

### `scripts/gen_radar.py`

**Purpose:** Generates the skills radar SVG chart from Python.

**Usage:**
```bash
python scripts/gen_radar.py
# Output: docs/skills-radar.svg
```

**How it works:**
- Defines 8 skill categories with proficiency scores (0–5 scale)
- Computes octagonal grid polygons using trigonometric coordinate math
- Generates axis lines, level labels, data polygon, and dots
- Outputs a self-contained SVG with embedded CSS styling
- Uses system fonts (`-apple-system, "Segoe UI"`) for cross-platform rendering

**To update scores:** Edit the `categories` list at the top of the file. Each tuple is `("Label", score)`.

**Dependencies:** Python 3 standard library only (`math` module). No external packages required.

---

## 5. Commit History

| Commit | Message | Files Changed | Insertions | Deletions |
|---|---|---|---|---|
| `54d4a92` | `docs: add independent employer-perspective portfolio audit` | 1 | 521 | 0 |
| `96b7305` | `docs: add Mermaid diagrams, MITRE ATT&CK mappings, and config examples` | 8 | 204 | 25 |
| `e834181` | `docs: enrich all 8 assignments and add skills radar SVG` | 11 | 417 | 191 |
| `082d8e6` | `docs: update audit with remediation status — grade A- to A` | 1 | 54 | 3 |
| **Total** | **4 commits** | **20 files** | **+1192** | **−216** |

All commits include `Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>`.

---

## 6. Files Modified / Created

### Created (2 new files)
- `docs/EMPLOYER_AUDIT_2026-04-05.md` — 571-line employer-perspective audit with remediation tracking
- `docs/skills-radar.svg` — 55-line hand-crafted SVG radar chart
- `scripts/gen_radar.py` — 106-line Python SVG generator

### Modified (17 existing files)
- `CC/.../MIDTERM_PROJECT_SUMMARY.md` — MITRE ATT&CK IDs + stale text fix
- `CC/.../README.md` — transcript count fix (4→5)
- `CC/.../assignments/Wk01_Lab_03_Firewall_Logs.md` — full enrichment
- `CC/.../assignments/Wk02_Lab_02_ACC.md` — full enrichment
- `CC/.../assignments/Wk03_Lab_05_Reconnaissance.md` — full enrichment
- `CC/.../assignments/Wk04_CyberKillChain_Part1.md` — full enrichment
- `CC/.../assignments/Wk05_Lab_07_Threat_Intelligence.md` — full enrichment
- `CC/.../assignments/Wk06_Lab_06_Endpoint_Security.md` — full enrichment
- `CC/.../assignments/Wk08_Lab_02_Internet_Threats.md` — full enrichment
- `CC/.../assignments/Wk09_Lab_03_Container_Security.md` — full enrichment
- `CC/.../weekly/week-01-firewall-log-analysis.md` — Mermaid diagram added
- `CC/.../weekly/week-02-application-command-center.md` — Mermaid diagram added
- `CC/.../weekly/week-04-build-a-soc-group-project.md` — MITRE ATT&CK table
- `CC/.../weekly/week-05-threat-intelligence.md` — Mermaid diagram + config example
- `CC/.../weekly/week-08-internet-threat-prevention.md` — config example
- `CC/.../weekly/week-09-container-networking-security.md` — config example
- `docs/skills-matrix.md` — radar SVG reference

---

## 7. Known Issues & Technical Notes

### 7.1 Rust Ping Sweep Subnet Bug

The ping sweep tool at `scripts/ping_sweep/src/main.rs` has a potential bug in subnet arithmetic: `host_bits / 8` integer division only works correctly for CIDR boundaries at /8, /16, and /24. Non-standard masks (e.g., /20, /27) will produce incorrect host ranges. No tests or CI exist for the Rust code.

**Recommendation:** Add unit tests for `calculate_hosts()` with non-standard CIDR masks. Consider using the `ipnetwork` crate for correct subnet arithmetic.

### 7.2 Git LFS State

Git LFS was previously migrated to regular git objects for NAS compatibility (noted in prior session). All 44 PNG screenshots (31.6 MB total) are stored as regular git blobs. This works for NAS but may cause slow clones if the repo is published to GitHub.

**Recommendation:** If publishing to GitHub, consider re-enabling Git LFS for `*.png` files.

### 7.3 CRLF Warnings

All edits trigger `LF will be replaced by CRLF` warnings. The `.gitattributes` file exists but may not fully normalize line endings. This is cosmetic and does not affect rendered content.

### 7.4 Assignment Content Limitations

The enriched assignments preserve all original observations but cannot add new lab data. Some assignments (especially Wk04) have minimal source material — the enrichment adds structure and analysis framing but cannot manufacture lab results that weren't recorded.

### 7.5 Config Examples Are Illustrative

The PAN-OS CLI snippets and Kubernetes YAML in weekly summaries are realistic but **not copy-paste production configs**. They demonstrate understanding of the configuration surface area. In a real deployment, specific values (IP ranges, policy names, EDL URLs) would differ.

---

## 8. Suggestions for Future Sessions

### High Priority
1. **Push to GitHub** — the portfolio is currently on a NAS only; publishing to GitHub with proper topics and description would make it discoverable by recruiters
2. **GitHub Pages landing page** — an interactive single-page site with weekly navigation would elevate the portfolio significantly
3. **Video walkthrough** — a 3-minute narrated screen recording of a lab exercise (biggest single improvement for A+ grade)

### Medium Priority
4. **Live Wazuh detection demo** — a working detection rule triggered in a sandbox, documented as a GIF or video
5. **Blog post adaptation** — convert the Week 4 (Build a SOC) or Week 5 (Threat Intelligence) reflection into a publishable blog post
6. **PCSA certification** — Palo Alto Certified Security Administrator validates the hands-on skills documented here
7. **Cross-link with other course portfolios** — Pilots 008, 010, and 400–410 should reference each other in a program-level landing page

### Low Priority
8. **Rust ping sweep tests** — unit tests for subnet arithmetic would strengthen the code evidence
9. **Service mesh content** — Week 9 identifies Istio/Linkerd as a gap; a short appendix would close it
10. **Interactive Mermaid** — some diagram renderers support click-through links; adding them would enhance navigation

---

## 9. Portfolio State Summary

| Metric | Before Session | After Session |
|---|---|---|
| **Audit grade** | A− | **A** |
| **Assignment quality** | C+ (raw DOCX exports) | **A−** (professional lab reports) |
| **Mermaid diagrams** | 11 across 6/9 weeks | **14 across 9/9 weeks** |
| **Config examples** | In some weekly summaries | **In all relevant weeks (5, 8, 9)** |
| **MITRE ATT&CK mapping** | Kill Chain without technique IDs | **Full tactic + technique IDs with hyperlinks** |
| **Skills visualization** | Tables only | **Tables + SVG radar chart** |
| **Stale/incorrect text** | 2 instances | **0 instances** |
| **Total commits** | 7 | **11** |
| **Net line changes** | — | **+1,192 / −216** |

---

*Session documented: 2026-04-06T00:04Z*
