# SysOps and Cloud Security — Aditya Palshikar — CSC-7308

![Portfolio CI](https://github.com/<owner>/<repo>/actions/workflows/portfolio-ci.yml/badge.svg)

> **Start Here** — course overview, learning outcomes, weekly navigation, and evidence.

## Course Identification

| Field | Value |
|---|---|
| **Course Code** | CSC-7308 |
| **Course Title** | SysOps and Cloud Security |
| **Term** | Winter 2025 (202501) |
| **Section / CRN** | 002 / 11819 |
| **Instructor** | Aditya Palshikar |
| **Program** | Postgraduate Cybersecurity Certificate |
| **Institution** | Cambrian College, Sudbury, Ontario |
| **Delivery Mode** | Synchronous lectures + asynchronous hands-on labs |
| **Credit Hours** | 3 |
| **Delivered Weeks** | 9 of 12 (at portfolio snapshot) |

## Quick Links

- **Weekly Summaries** → [`weekly/`](weekly/) — Individual week write-ups (Week 1 – Week 9)
- **Build a SOC (Group Project)** → [`MIDTERM_PROJECT_SUMMARY.md`](MIDTERM_PROJECT_SUMMARY.md)
- **Final Vulnerability Assessment** → [`FINAL_EXAM_VULNERABILITY_ASSESSMENT.md`](FINAL_EXAM_VULNERABILITY_ASSESSMENT.md)
- **Course Reflection** → [`COURSE_REFLECTION.md`](COURSE_REFLECTION.md)
- **Scripts & Code** → [`SCRIPTS_README.md`](SCRIPTS_README.md)
- **Evidence Index** → [`EVIDENCE_INDEX.md`](EVIDENCE_INDEX.md)
- **Assignments (sanitized)** → [`assignments/`](assignments/)
- **Screenshots** → [`screenshots/`](screenshots/)
- **Scripts (student-authored)** → [`scripts/`](scripts/)
- **Scripts (reference)** → [`scripts-extra/`](scripts-extra/)

## Learning Outcomes

By the end of this course, a student should be able to:

1. **Analyze firewall logs** from next-generation firewalls (NGFWs) to identify threats, policy violations, and traffic anomalies.
2. **Configure and operate** the Palo Alto Networks Application Command Center (ACC) for traffic analytics, application visibility, and threat correlation.
3. **Detect and prevent reconnaissance** attacks including port scans, network discovery, and application enumeration.
4. **Design and deploy** an open-source Security Operations Center (SOC) using Wazuh SIEM, integrating log sources, alert correlation, and incident response.
5. **Apply the Cyber Kill Chain** framework to map attacker tactics to defender responses.
6. **Leverage threat intelligence** platforms (Palo Alto AutoFocus) for proactive, prevention-based security.
7. **Harden endpoints** using vulnerability profiles, antivirus signatures, and policy-based controls.
8. **Understand cloud security** models (IaaS, PaaS, SaaS) and the shared responsibility model across AWS, Azure, and GCP.
9. **Secure containers** (Docker, Kubernetes) with Microsoft Defender and policy-based networking.
10. **Protect against internet threats** using URL filtering, DNS security, and WildFire malware analysis.

## Platforms & Technologies Covered

### Palo Alto Networks (Primary Platform)

- **Strata** — Enterprise NGFW (Next-Generation Firewall) suite
- **Prisma Cloud** — Cloud-native workload and configuration security
- **Cortex** — XDR (Extended Detection and Response) / future-focused AI security
- **AutoFocus** — Cloud-based threat intelligence
- **WildFire** — Malware analysis and zero-day prevention
- **Lab Frameworks** — SOFv2 (Security Operating Fundamentals v2) and CSFv2 (Cloud Security Fundamentals v2)

### Open-Source SIEM

- **Wazuh** — Log aggregation, HIDS, FIM, threat detection, active response (Week 4 capstone)

### Cloud Platforms (conceptual)

- **AWS**, **Microsoft Azure**, **Google Cloud Platform (GCP)**, **Apple Cloud**
- **Microsoft Defender for Cloud** / for Containers
- **Microsoft Entra ID** (formerly Azure AD)

### Container Technologies

- **Docker** — Containerization fundamentals
- **Kubernetes** — Orchestration, networking, security policies

### Programming

- **Rust** — Async networking (Tokio runtime) for an independent ping-sweep extension to Week 2

## Weekly Topic Map

| Week | Date | Topic | Lab |
|---:|---|---|---|
| [1](weekly/week-01-firewall-log-analysis.md) | 2025-01-07 | Firewall Log Analysis | SOFv2 Lab 03 |
| [2](weekly/week-02-application-command-center.md) | 2025-01-14 | Application Command Center + Independent Rust Scanner | SOFv2 Lab 02 |
| [3](weekly/week-03-reconnaissance-prevention.md) | 2025-01-21 | Stopping Reconnaissance Attacks | SOFv2 Lab 05 |
| [4](weekly/week-04-build-a-soc-group-project.md) | 2025-01-28 | **Build a SOC — Group Capstone** | Wazuh deployment |
| [5](weekly/week-05-threat-intelligence.md) | 2025-02-04 | Threat Intelligence with AutoFocus | SOFv2 Lab 07 |
| [6](weekly/week-06-endpoint-security.md) | 2025-02-11 | Endpoint Security & Vulnerability Profiles | SOFv2 Lab 06 |
| [7](weekly/week-07-cloud-computing-containers.md) | 2025-02-18 | Cloud Computing & Container Security | (lecture-only) |
| [8](weekly/week-08-internet-threat-prevention.md) | 2025-03-03 | Internet Threat Prevention | CSFv2 Lab 02 |
| [9](weekly/week-09-container-networking-security.md) | 2025-03-10 | Container Networking & Security | CSFv2 Lab 03 |

## Assessment Structure

| Component | Weight | Status |
|---|---|---|
| Weekly Labs (8 individual) | Graded | Delivered |
| Group Project: Build a SOC | Capstone | Delivered (Part 1) |
| Final Exam / Project | TBD | Weeks 10–12 |

## Course Artifacts

- **9 video lectures** (~15 hours of instruction; excluded from repo, stored locally)
- **4 lecture transcripts** (Weeks 2, 3, 5, 6, 7 — referenced in weekly summaries)
- **8 hands-on labs** — SOFv2 (6 labs) + CSFv2 (2 labs)
- **1 group project** (Build a SOC with Wazuh)
- **Original work** — Rust async ping-sweep CLI tool ([`scripts/ping_sweep/`](scripts/ping_sweep/))

## Naming Conventions (for contributors)

- **Screenshots:** `wkNN_topic_index.png` (e.g., `wk01_firewall_logs_1.png`) or `ScreenshotN_ShortDesc.png`
- **Weekly summaries:** `weekly/week-NN-<short-topic>.md` (lowercase, hyphens, no spaces)
- **Scripts (student):** `scripts/` with a README per project
- **Scripts (external):** `scripts-extra/` with provenance notes
- **Assignments:** `assignments/AssignmentNN_<short-title>.pdf` (sanitized PDFs only)

## Privacy & Attribution

- **Retained:** Instructor name (Aditya Palshikar) and portfolio owner name (Ross Moravec) as public attribution.
- **Removed:** Other students' names, student IDs, group-project team rosters.
- **Referenced only:** Third-party lab manuals (Palo Alto SOFv2/CSFv2), Wazuh docs, vendor platforms. Not redistributed.
- **Excluded:** Large video files (~1.5 GB of MP4s).

## Related Courses in this Program

| Pilot | Course | Term |
|---|---|---|
| 400 | Fundamentals of IT | Fall 2024 |
| 401 | CSEC Infrastructure | Fall 2024 |
| 402 | Business Continuity | Fall 2024 |
| 403 | Policies & Compliance | Fall 2024 |
| 404 | Communications for Cybersecurity | Fall 2024 |
| 405 | Mobile & Wireless Security | Winter 2025 |
| **406** | **SysOps and Cloud Security** | **Winter 2025** ← this repo |
| 407 | CSEC Tool Development | Winter 2025 |
| 408 | IT Security Forensics | Winter 2025 |
| 409 | Ethical Hacking | Winter 2025 |
| 410 | Malware Development & Analysis | Winter 2025 |
| 411 | Cybersecurity Capstone | Winter 2025 |

---

_Last updated: 2026-04-04. Portfolio snapshot reflects Weeks 1–9 delivered content._
