# Evidence & Screenshots Index

This guide groups evidence by week/topic and links each artifact with a short caption. Screenshots were extracted from lab submission DOCX files and follow the naming convention `wkNN_topic_index.png`.

**Total evidence artifacts: 44 screenshots + 4 diagrams/code files**

---

## Week 01 — Firewall Log Analysis (3 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk01_1](screenshots/wk01_firewall_logs_1.png) | Palo Alto Monitor tab — filtered traffic log view |
| 2 | ![wk01_2](screenshots/wk01_firewall_logs_2.png) | Threat logs showing blocked/flagged sessions |
| 3 | ![wk01_3](screenshots/wk01_firewall_logs_3.png) | Custom log filter applied to identify traffic patterns |

## Week 02 — Application Command Center (6 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk02_1](screenshots/wk02_acc_1.png) | ACC dashboard — application usage overview |
| 2 | ![wk02_2](screenshots/wk02_acc_2.png) | Top applications widget with traffic breakdown |
| 3 | ![wk02_3](screenshots/wk02_acc_3.png) | Top sources/destinations analytics |
| 4 | ![wk02_4](screenshots/wk02_acc_4.png) | ACC global filter configuration |
| 5 | ![wk02_5](screenshots/wk02_acc_5.png) | Widget drill-down to raw log entries |
| 6 | ![wk02_6](screenshots/wk02_acc_6.png) | ACC threat activity correlation view |

**Original work (preserved):**

- [`scripts/ping_sweep/ping-sweep-diagram.svg`](scripts/ping_sweep/ping-sweep-diagram.svg) — Flow diagram of async ping-sweep tool
- [`scripts/ping_sweep/ping-sweep-flow.mermaid`](scripts/ping_sweep/ping-sweep-flow.mermaid) — Mermaid source for flow diagram
- [`scripts/ping_sweep/src/main.rs`](scripts/ping_sweep/src/main.rs) — Rust source code

## Week 03 — Reconnaissance Attack Prevention (7 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk03_1](screenshots/wk03_recon_1.png) | Zone Protection Profile — configuration overview |
| 2 | ![wk03_2](screenshots/wk03_recon_2.png) | Flood protection threshold settings |
| 3 | ![wk03_3](screenshots/wk03_recon_3.png) | Reconnaissance protection — scan detection settings |
| 4 | ![wk03_4](screenshots/wk03_recon_4.png) | DoS Protection Policy configuration |
| 5 | ![wk03_5](screenshots/wk03_recon_5.png) | Threat log — port scan detection entries |
| 6 | ![wk03_6](screenshots/wk03_recon_6.png) | Block-IP action triggered by recon traffic |
| 7 | ![wk03_7](screenshots/wk03_recon_7.png) | Zone protection profile attached to security zone |

## Week 04 — Build a SOC / Cyber Kill Chain (4 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk04_1](screenshots/wk04_cyber_kill_chain_1.png) | Cyber Kill Chain — stage mapping analysis |
| 2 | ![wk04_2](screenshots/wk04_cyber_kill_chain_2.png) | Kill Chain stages with attacker/defender actions |
| 3 | ![wk04_3](screenshots/wk04_cyber_kill_chain_3.png) | Wazuh detection rule mapping to kill chain |
| 4 | ![wk04_4](screenshots/wk04_cyber_kill_chain_4.png) | Defender response framework documentation |

## Week 05 — Threat Intelligence / AutoFocus (5 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk05_1](screenshots/wk05_threat_intel_1.png) | AutoFocus portal — threat intelligence dashboard |
| 2 | ![wk05_2](screenshots/wk05_threat_intel_2.png) | Malware family tag view with IoC details |
| 3 | ![wk05_3](screenshots/wk05_threat_intel_3.png) | IoC correlation across Palo Alto cloud telemetry |
| 4 | ![wk05_4](screenshots/wk05_threat_intel_4.png) | WildFire verdict delivery pipeline |
| 5 | ![wk05_5](screenshots/wk05_threat_intel_5.png) | Threat intelligence integration with NGFW policy |

## Week 06 — Endpoint Security & Vulnerability Profiles (9 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk06_1](screenshots/wk06_endpoint_1.png) | Vulnerability Protection profile — severity actions |
| 2 | ![wk06_2](screenshots/wk06_endpoint_2.png) | Antivirus profile configuration |
| 3 | ![wk06_3](screenshots/wk06_endpoint_3.png) | Anti-Spyware profile — C2 callback detection |
| 4 | ![wk06_4](screenshots/wk06_endpoint_4.png) | WildFire Analysis profile settings |
| 5 | ![wk06_5](screenshots/wk06_endpoint_5.png) | Security Profile Group assembly |
| 6 | ![wk06_6](screenshots/wk06_endpoint_6.png) | Profile group attached to security policy rule |
| 7 | ![wk06_7](screenshots/wk06_endpoint_7.png) | Threat database — signature update status |
| 8 | ![wk06_8](screenshots/wk06_endpoint_8.png) | Security policy with combined protection profiles |
| 9 | ![wk06_9](screenshots/wk06_endpoint_9.png) | Committed configuration — profile enforcement active |

## Week 07 — Cloud Computing & Containers

_No lab screenshots — this was a lecture-only week. Concepts are documented in the [weekly summary](weekly/week-07-cloud-computing-containers.md) and reinforced with hands-on labs in Weeks 8–9._

## Week 08 — Internet Threat Prevention (4 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk08_1](screenshots/wk08_threat_prevention_1.png) | URL Filtering profile — category-based controls |
| 2 | ![wk08_2](screenshots/wk08_threat_prevention_2.png) | DNS Security configuration settings |
| 3 | ![wk08_3](screenshots/wk08_threat_prevention_3.png) | File Blocking profile — MIME type rules |
| 4 | ![wk08_4](screenshots/wk08_threat_prevention_4.png) | Integrated threat prevention policy in CSFv2 context |

## Week 09 — Container Networking & Security (6 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk09_1](screenshots/wk09_container_1.png) | Container networking — Docker bridge configuration |
| 2 | ![wk09_2](screenshots/wk09_container_2.png) | Kubernetes cluster — pod and service overview |
| 3 | ![wk09_3](screenshots/wk09_container_3.png) | NetworkPolicy resource — east-west segmentation |
| 4 | ![wk09_4](screenshots/wk09_container_4.png) | Pod Security Standards enforcement |
| 5 | ![wk09_5](screenshots/wk09_container_5.png) | Container image vulnerability scanning |
| 6 | ![wk09_6](screenshots/wk09_container_6.png) | Container workload protection overview |

---

## Diagrams & Code Artifacts (Original Work)

| File | Description |
|---|---|
| [`scripts/ping_sweep/ping-sweep-diagram.svg`](scripts/ping_sweep/ping-sweep-diagram.svg) | Flow diagram showing async ping-sweep state machine |
| [`scripts/ping_sweep/ping-sweep-flow.mermaid`](scripts/ping_sweep/ping-sweep-flow.mermaid) | Mermaid source for flow diagram |
| [`scripts/ping_sweep/src/main.rs`](scripts/ping_sweep/src/main.rs) | Rust async ping-sweep source code |
| [`scripts/ping_sweep/code-explanation.md`](scripts/ping_sweep/code-explanation.md) | Detailed line-by-line walkthrough of Rust implementation |

---

*Last updated: 2026-04-05. 44 screenshots extracted from lab submission documents.*
