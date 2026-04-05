# Portfolio Audit — Employer-Perspective Assessment

> **Audit date:** 2026-04-05
> **Perspective:** Cybersecurity hiring manager at a mid-to-large security company
> **Scope:** Every file in the 406-SysOps-Cloud-Security repository
> **Methodology:** Full read of all documents, evaluation of formatting, visualizations, completeness, professional presentation, and information utilization

---

## Overall Verdict

**Grade: B− / Promising Foundation, Not Yet Employer-Ready**

The portfolio demonstrates strong **structural thinking**, **consistent documentation habits**, and **genuine technical reflection**. The writing quality is well above typical student portfolios. However, multiple critical gaps — most notably the total absence of visual evidence, assignment artifacts, and source code — mean a hiring manager clicking through this repository today would find a **skeleton with excellent bones but no flesh**.

---

## 1. Strengths (What a Hiring Manager Would Notice Positively)

### 1.1 Employer-Conscious Navigation (A)

The root README provides a **tiered tour** (5-minute, 15-minute, 30-minute) — a rare and impressive touch. Most student portfolios assume the reader has unlimited time. This signals professional communication awareness.

### 1.2 Consistent Weekly Structure (A−)

All 9 weekly summaries follow a uniform template (Session Info → Topics → Tools → Key Concepts → Lab Deliverable → Reflection → Connections → References). This consistency makes the portfolio scannable and demonstrates process discipline.

### 1.3 Reflective Depth (A)

The reflections are not generic. Examples of standout insight:

- Week 1: *"Scrolling raw logs is useless; disciplined filter-writing is the entire job."*
- Week 3: *"Defenders do not need to know the specific tool. Zone Protection does not care whether my scanner is written in Rust, Python, or bash — it pattern-matches on behavior."*
- Week 4: *"Not 'what tools do we have?' but 'which stages can we see?'"*
- Week 7: *"Every cloud security failure I have read about post-hoc traces back to a misunderstanding of where the provider's responsibility ends and the customer's begins."*
- Week 9: *"The host perimeter model assumes machines are atomic units; containers break that assumption."*

These demonstrate **analytical thinking**, not just rote lab completion. A technical interviewer would value this.

### 1.4 Cross-Week Connections (A−)

Every weekly summary includes a "Connections" section linking forward and backward to other weeks and adjacent courses. This shows systems-level thinking — understanding how topics compose into a security program rather than treating each lab as isolated.

### 1.5 Skills Framework Mapping (B+)

The `docs/skills-matrix.md` maps skills to both **NICE (NIST SP 800-181)** and **CyBOK** frameworks. This is industry-standard practice and demonstrates awareness of how employers categorize competencies.

### 1.6 Independent Code Work (B+)

The Rust async ping-sweep project shows initiative beyond the curriculum. The `code-explanation.md` is a thorough line-by-line walkthrough. The connection back to course themes (Week 2 ACC → Week 3 recon prevention → Week 5 threat intel) is well-articulated.

### 1.7 Privacy & Sanitization Discipline (A)

The privacy policy (`docs/privacy-and-sanitization.md`) is comprehensive: PII handling, copyright respect, secret scanning enforcement, incident response procedure. This signals awareness of data governance — a real-world operational concern.

### 1.8 CI/CD Pipeline (B+)

Five GitHub Actions workflows (CI baseline, portfolio link checking, markdownlint, gitleaks, PM evidence) show professional development practices. Secret scanning on every push is particularly relevant for a cybersecurity portfolio.

### 1.9 Annotated Bibliography (B+)

`docs/references.md` includes vendor docs, NIST standards, CIS benchmarks, academic papers (the original Cyber Kill Chain paper, Anderson's *Security Engineering*), and programming references. This is stronger than most student portfolios.

### 1.10 Course Contextualization (A−)

`docs/course-context.md` places CSC-7308 within the full two-term program using an ASCII dependency diagram. The "Why This Course Matters" section is genuinely compelling: *"Most cybersecurity breaches are not due to exotic zero-days. They are due to misconfigured firewall rules, poor log hygiene, missing endpoint coverage..."*

---

## 2. Critical Weaknesses (Deal-Breakers for Employers)

### 2.1 ZERO Visual Evidence (F)

**This is the single most damaging gap.**

The `screenshots/` directory contains only a README placeholder. The `EVIDENCE_INDEX.md` lists expected screenshots for all 9 weeks — every single one is marked "_Pending_". There are no:

- Firewall dashboard screenshots
- ACC widget captures
- Wazuh dashboard views
- Zone Protection configurations
- Threat log entries
- URL filtering results
- Container security configurations

**Employer impact:** A hiring manager reviewing this portfolio would see extensive *claims* of hands-on work but *zero visual proof*. In cybersecurity, where demonstrating tool proficiency is critical, this is a credibility gap. The portfolio reads as "I did these things, trust me" rather than "here is evidence I did these things."

**Remediation:** Extract screenshots from DOCX lab submissions. Even 2–3 screenshots per week (18–27 total) would transform the portfolio from text-only to evidence-backed.

### 2.2 ZERO Assignment PDFs (F)

The `assignments/` directory contains only a README explaining the naming convention and stating PDFs are "pending conversion." Eight lab reports and one Cyber Kill Chain submission were completed — none are present.

**Employer impact:** Lab submissions are the primary artifact of hands-on work. Without them, the weekly summaries are unsupported claims.

**Remediation:** Convert DOCX submissions to sanitized PDFs. Redact PII and copyrighted imagery. Even partial sanitization (e.g., text-only extracts) would be better than nothing.

### 2.3 No Actual Source Code (D)

The Rust ping-sweep project — the portfolio's only original code artifact — does **not contain the source file** (`main.rs`). The `SCRIPTS_README.md` acknowledges this: *"The original single-file implementation can be reconstructed from the explanation."*

**Employer impact:** A developer portfolio without runnable code is like a chef's portfolio without recipes. The `code-explanation.md` is excellent documentation, but hiring managers and technical reviewers expect to see actual code they can read, clone, and potentially run.

**Remediation:** Add `main.rs` (or `src/main.rs`) and `Cargo.toml` to the `ping_sweep/` directory. The code already exists in the explanation — it just needs to be assembled into files.

### 2.4 Broken CI Badges (D)

Lines 3–6 of the root README contain placeholder badges:

```markdown
![Portfolio CI](https://github.com/<owner>/<repo>/actions/workflows/portfolio-ci.yml/badge.svg)
```

The `<owner>/<repo>` placeholders have never been replaced with actual GitHub coordinates.

**Employer impact:** Broken badges on the landing page are the first thing a reviewer sees. They signal an unfinished or unmaintained project.

**Remediation:** Replace `<owner>/<repo>` with the actual GitHub owner and repository name, or remove badges until the repo is published.

---

## 3. Visualization Assessment

### 3.1 Current Visualization Inventory

| Visualization | Location | Quality |
|---|---|---|
| Ping-sweep flow SVG | `scripts/ping_sweep/ping-sweep-diagram.svg` | Basic — flat colors, no gradients, simple rectangles |
| Ping-sweep Mermaid source | `scripts/ping_sweep/ping-sweep-flow.mermaid` | Functional but not rendered in any MD file |
| Wazuh architecture (ASCII art) | Week 4 summary | Text-only; not a proper diagram |
| Program dependency tree (ASCII) | `docs/course-context.md` | Text-only; not rendered as graphic |
| Course arc progression (text) | `COURSE_REFLECTION.md` | Text block; not a timeline |

**Total proper visual artifacts: 1 (the SVG)**. For a 9-week, 20+ file portfolio, this is severely under-visualized.

### 3.2 Visualizations That Should Be Added

| # | Visualization | Target Location | Priority | Format |
|---|---|---|---|---|
| 1 | **Wazuh SOC Architecture Diagram** | `MIDTERM_PROJECT_SUMMARY.md` | HIGH | Mermaid or SVG |
| 2 | **Cyber Kill Chain Diagram** (7-stage visual with defender mapping) | `MIDTERM_PROJECT_SUMMARY.md` + Week 4 | HIGH | Mermaid or SVG |
| 3 | **Shared Responsibility Model Diagram** (IaaS/PaaS/SaaS layers) | Week 7 summary | HIGH | Mermaid or SVG |
| 4 | **Course Progression Timeline** (Weeks 1–9, topic arc) | Root README or `COURSE_REFLECTION.md` | MEDIUM | Mermaid Gantt or timeline |
| 5 | **Defense-in-Depth Layer Diagram** (perimeter → SIEM → endpoint → cloud → container) | `COURSE_REFLECTION.md` or Root README | MEDIUM | Mermaid or SVG |
| 6 | **Skills Radar/Spider Chart** (domain proficiency self-assessment) | Root README or `docs/skills-matrix.md` | MEDIUM | SVG |
| 7 | **NGFW Security Policy Profile Stack** (rule → profile group → individual profiles) | Week 6 summary | MEDIUM | Mermaid |
| 8 | **URL/DNS/File Blocking Defense Layers** | Week 8 summary | LOW | Mermaid |
| 9 | **Container vs. VM Isolation Model** | Week 9 summary | LOW | Mermaid |
| 10 | **Kubernetes Network Policy Diagram** (east-west vs north-south) | Week 9 summary | LOW | Mermaid |
| 11 | **Program Curriculum Map** (replace ASCII art in course-context.md) | `docs/course-context.md` | LOW | Mermaid |
| 12 | **Ping-sweep SVG upgrade** (improve visual quality, add async layer annotations) | `scripts/ping_sweep/` | LOW | SVG redesign |

### 3.3 Visualization Quality Assessment

The existing SVG (`ping-sweep-diagram.svg`) is functional but **below professional portfolio grade**:

- Flat, basic color palette (Material Design defaults but no sophistication)
- No annotations explaining the async/blocking boundary
- No legend or labeling for the color coding
- Simple rectangular boxes without any iconography
- The diamond decision node is acceptable but sparse

**Recommendation:** All new diagrams should use a consistent visual language — either Mermaid (for maintainability and GitHub rendering) or a unified SVG template (for visual polish). Mermaid is strongly preferred because GitHub renders it natively in markdown.

---

## 4. Document-by-Document Professional Formatting Assessment

### 4.1 Root README.md — Grade: B+

**Strengths:** Tiered tour, skills section, weekly topic map table, repo structure tree, privacy section.
**Weaknesses:** Broken badges (line 3–6). The "At a Glance" table is slightly redundant with the course-level README. No hero image or visual banner. The skills list is text-only — a visual skills chart would be more impactful.

### 4.2 Course README.md — Grade: B+

**Strengths:** Comprehensive course identification table, learning outcomes, platform inventory, weekly topic map with clickable links, assessment structure, related courses table.
**Weaknesses:** Broken badge (line 3). "Assessment Structure" table shows "TBD" for final exam weight — either update or remove. The "Course Artifacts" section lists video lectures as excluded without explaining why (a brief "(too large for Git; stored locally)" would help).

### 4.3 Weekly Summaries (×9) — Grade: B+

**Strengths:** Consistent structure, strong reflections, cross-connections, reference citations.
**Weaknesses:**
- No embedded screenshots or evidence links (all pending).
- Key concept tables are text-only — some would benefit from diagrams (e.g., Zone Protection components in Week 3, Kill Chain in Week 4, SRM in Week 7, Pod Security Standards in Week 9).
- Some weeks lack a "Key Takeaway" bold callout that could serve as a pull quote for quick scanning.
- Week 7 mentions "light attendance (5 of 22 students)" — while honest, this detail has no employer value and could be removed.

### 4.4 MIDTERM_PROJECT_SUMMARY.md — Grade: B

**Strengths:** Clear project structure, deliverable table, skills demonstrated section, thoughtful reflection, good "key takeaways" list.
**Weaknesses:**
- No Wazuh architecture diagram (described in text, not shown).
- No Cyber Kill Chain visual.
- Evidence section says "in progress" — still empty.
- The "Status" section with 🟡 yellow circles is honest but shows incompleteness prominently.
- No sample Wazuh detection rules (even pseudocode/sanitized examples would add credibility).
- No link to Part 1 submission (pending in assignments/).

### 4.5 COURSE_REFLECTION.md — Grade: A−

**Strengths:** Executive summary, course arc narrative, "Three Mental Models That Mattered Most" (excellent framing), before/after skills table, "What I Would Do Differently" (shows maturity).
**Weaknesses:**
- No visualizations (course arc is text block, could be a timeline).
- "Honest Limitations of This Snapshot" could be shortened — listing 4 limitations reads as 4 excuses to an employer.
- The before/after table could use proficiency ratings (1–5) instead of prose for scanability.

### 4.6 EVIDENCE_INDEX.md — Grade: C

**Strengths:** Well-structured per-week organization, expected evidence lists, preserved diagram references.
**Weaknesses:** The entire document is a promise. Every week section says "_Pending_". The "Diagrams Preserved" section (2 files) is the only actual content. As-is, this document highlights the evidence gap rather than filling it.

### 4.7 SCRIPTS_README.md — Grade: B−

**Strengths:** Clear feature list, dependency documentation, file table.
**Weaknesses:** Acknowledges missing source code. The "safety notice" is good but brief. The "Provided / External Scripts" section is empty — the entire `scripts-extra/` is empty.

### 4.8 FINAL_EXAM_VULNERABILITY_ASSESSMENT.md — Grade: D

This is effectively a placeholder. It says "Pending — Weeks 10–12 had not been delivered." While honest, an employer sees a file titled "Final Exam" that contains no content. Either populate it when ready or do not create it until there is content.

### 4.9 docs/skills-matrix.md — Grade: B+

**Strengths:** NICE and CyBOK mapping, technology skills inventory with checkboxes, legend.
**Weaknesses:** No proficiency levels (just checked/unchecked). No visual representation. All checkboxes are checked — no items show "introduced conceptually" which would add honesty and nuance.

### 4.10 docs/references.md — Grade: A−

**Strengths:** Annotated, categorized, includes academic papers alongside vendor docs.
**Weaknesses:** Minor — some links may go stale. Could benefit from date-of-access annotations for academic rigor.

### 4.11 docs/privacy-and-sanitization.md — Grade: A

Near-flawless. Comprehensive, actionable, includes incident response procedure. No changes needed.

### 4.12 docs/course-context.md — Grade: A−

**Strengths:** Full curriculum map, ASCII dependency diagram, "Why This Course Matters" is compelling.
**Weaknesses:** ASCII diagram should be replaced with Mermaid for GitHub rendering. Instructor names for other courses may need consent consideration.

### 4.13 portfolio/config.json — Grade: B+

Well-structured metadata. Metrics are accurate. Evidence paths reference files that exist. References are valid URLs.

### 4.14 ping_sweep/ — Grade: B

**Strengths:** Thorough README, detailed code-explanation.md, SVG and Mermaid diagram, ethics/safety notice, course relevance section.
**Weaknesses:** No actual source code (main.rs, Cargo.toml). SVG is basic quality. Mermaid file is not embedded/rendered in any markdown document.

---

## 5. Information Utilization Assessment

### 5.1 Source Materials Available vs. Used

| Source Material | Available | Used in Portfolio | Utilization |
|---|---|---|---|
| 9 video lectures (~15 hrs) | ✅ Local | ❌ No key-frame captures, no timestamp references | **0%** |
| 4+ lecture transcripts | ✅ Local (Wk 2,3,5,6,7) | 🟡 Referenced in weekly summaries as "local" | **10%** — mentioned but not distilled |
| 8 lab DOCX submissions | ✅ Local | ❌ Not converted to PDF, screenshots not extracted | **0%** |
| 1 Cyber Kill Chain submission (Wk 4) | ✅ Local | ❌ Not sanitized/converted | **0%** |
| Wazuh Project.pdf (3.4 MB) | ✅ Local | 🟡 Referenced, not distilled | **5%** |
| Wazuh Lab Docs (6.3 MB) | ✅ Local | 🟡 Referenced, not distilled | **5%** |
| Lab PDFs (SOFv2 ×6, CSFv2 ×2) | ✅ Referenced | ❌ Cannot redistribute; but methodology notes could be extracted | **0%** |
| Rust source code (main.rs) | ✅ Local | 🟡 Explained but not committed | **70%** (explanation thorough, file absent) |
| ping-sweep SVG + Mermaid | ✅ Committed | ✅ Present and linked | **90%** |

**Overall information utilization: ~20%**

The portfolio has access to rich source materials — lecture recordings, transcripts, lab submissions with embedded screenshots, reference documentation — and has used almost none of them as actual evidence artifacts. The writing *describes* what was learned, but the *proof* remains locked in local DOCX files.

### 5.2 Specific Missed Opportunities

1. **Lab screenshots** — Every DOCX submission contains screenshots of firewall configurations, ACC dashboards, threat logs, etc. Extracting and captioning even 2–3 per week would add ~20 evidence images.

2. **Lecture transcript quotes** — Weeks 2, 3, 5, 6, 7 have transcripts. Pulling 1–2 key instructor quotes per week (with attribution) would add pedagogical depth.

3. **Command/configuration examples** — Labs involved PAN-OS CLI commands and configuration steps. Sanitized examples (e.g., a zone protection profile XML snippet, a Wazuh rule XML block) would demonstrate hands-on fluency.

4. **Wazuh rule examples** — The Kill Chain mapping describes detection rules conceptually. Showing even 2–3 actual Wazuh XML rules (from the open-source ruleset, not copyrighted material) would ground the discussion.

5. **Lab methodology summaries** — While lab PDFs cannot be redistributed, a 3–5 sentence "what I did in the lab" methodology summary per week would add substance.

---

## 6. Comparative Benchmark

### How does this portfolio compare to what a cybersecurity employer expects?

| Criterion | Industry Expectation | This Portfolio | Gap |
|---|---|---|---|
| **Demonstrates tool proficiency** | Screenshots, config snippets, CLI output | Text descriptions only | LARGE |
| **Shows original work** | Runnable code, project repos | Explanation without source file | MEDIUM |
| **Professional formatting** | Clean markdown, diagrams, consistent style | Good markdown, minimal diagrams | MEDIUM |
| **Evidence of hands-on labs** | Lab reports, screenshots, results | Described but not shown | LARGE |
| **Industry framework awareness** | NICE, MITRE ATT&CK, Kill Chain, CIS | NICE + CyBOK + Kill Chain + CIS ✅ | NONE |
| **Reflective practice** | Demonstrates learning, not just doing | Strong ✅ | NONE |
| **CI/CD & DevOps practices** | Version control, automation, linting | 5 workflows, conventional commits ✅ | NONE (minor: broken badges) |
| **Privacy & ethics awareness** | Responsible data handling | Comprehensive policy ✅ | NONE |
| **Visual communication** | Architecture diagrams, flow charts | 1 basic SVG + ASCII art | LARGE |

---

## 7. Prioritized Remediation Roadmap

### Tier 1 — Critical (Do These First)

| # | Action | Impact | Effort |
|---|---|---|---|
| 1 | **Extract and add screenshots from DOCX submissions** (2–3 per week, 18–27 images) | Transforms portfolio from text-only to evidence-backed | Medium (1–2 sessions) |
| 2 | **Convert and add lab PDFs** to `assignments/` (sanitized) | Provides actual deliverable artifacts | Medium (1 session) |
| 3 | **Add `main.rs` and `Cargo.toml`** to `scripts/ping_sweep/` | Makes the only code project actually contain code | Low (30 min) |
| 4 | **Fix CI badge URLs** (replace `<owner>/<repo>` with actual values) | Fixes the first thing any reviewer sees | Low (5 min) |

### Tier 2 — High Value (Major Quality Improvement)

| # | Action | Impact | Effort |
|---|---|---|---|
| 5 | **Add Mermaid diagrams** for Wazuh architecture, Cyber Kill Chain, Shared Responsibility Model, and Defense-in-Depth | Adds 4+ professional visualizations | Medium (1 session) |
| 6 | **Add a course progression timeline** (Mermaid Gantt or custom) to root README or reflection | Visual narrative of the learning arc | Low (1 hour) |
| 7 | **Add sanitized Wazuh rule examples** (from the public open-source ruleset) to Week 4 / midterm summary | Grounds the Kill Chain discussion with concrete detection logic | Low (1 hour) |
| 8 | **Add PAN-OS configuration snippets** (zone protection profile, vulnerability profile, URL filter) as sanitized examples in relevant weekly summaries | Demonstrates config-level proficiency | Medium (1 session) |
| 9 | **Upgrade the ping-sweep SVG** with async-layer annotations, legend, and professional styling | Raises the only existing diagram to portfolio grade | Low–Medium |

### Tier 3 — Polish (Nice-to-Have)

| # | Action | Impact | Effort |
|---|---|---|---|
| 10 | **Add a skills radar/spider chart** (SVG) to root README | Visual skill summary for quick scanning | Low |
| 11 | **Replace ASCII diagrams** in `docs/course-context.md` and Week 4 with Mermaid | Consistent visual language | Low |
| 12 | **Add proficiency levels** to skills-matrix.md (not just checkboxes) | Adds nuance and honesty | Low |
| 13 | **Remove or defer** `FINAL_EXAM_VULNERABILITY_ASSESSMENT.md` until content exists | Removes a visible placeholder | Low (5 min) |
| 14 | **Remove** the "light attendance (5 of 22)" detail from Week 7 | No employer value; mildly negative | Low (2 min) |
| 15 | **Add a brief professional bio** or link to LinkedIn/resume in root README | Connects portfolio to the person | Low |
| 16 | **Enable GitHub Pages** with a landing page | Makes the portfolio web-accessible with navigation | Medium |
| 17 | **Add "Key Takeaway" bold callouts** to each weekly summary for quick scanning | Improves scanability | Low |

---

## 8. Summary Scorecard

| Category | Grade | Weight | Notes |
|---|---|---|---|
| **Structure & Navigation** | A | 15% | Tiered tours, consistent templates, clear hierarchy |
| **Writing Quality** | A− | 15% | Strong reflections, analytical depth, professional tone |
| **Visual Evidence** | F | 20% | Zero screenshots, zero lab output, zero evidence images |
| **Visualizations & Diagrams** | D+ | 15% | 1 basic SVG, rest is ASCII art or absent |
| **Code & Artifacts** | C− | 10% | No source code file, no assignment PDFs, empty directories |
| **Framework & Standards Awareness** | A | 10% | NICE, CyBOK, Kill Chain, MITRE, CIS, NIST all referenced |
| **Professional Polish** | B− | 10% | Broken badges, placeholder files, inconsistent completion |
| **Information Utilization** | D+ | 5% | ~20% of available source material actually used |
| **Weighted Overall** | **B−** | 100% | **Strong skeleton, critical evidence gaps** |

---

## 9. One-Paragraph Employer Verdict

*"This candidate clearly understands SysOps and cloud security concepts at a strong academic level. The portfolio structure is unusually thoughtful for a student — the tiered navigation, privacy policy, and framework mappings show professional awareness. However, the portfolio currently contains zero visual evidence of hands-on lab work, no assignment submissions, and the only code project lacks its source file. The writing describes extensive tool experience (Palo Alto NGFW, Wazuh SIEM, container security) but provides no proof. If the screenshots, PDFs, source code, and a handful of architecture diagrams were added, this would be a compelling portfolio. As-is, it's a well-organized promise."*

---

*Audit completed: 2026-04-05. Conducted by reviewing every file in the repository.*
