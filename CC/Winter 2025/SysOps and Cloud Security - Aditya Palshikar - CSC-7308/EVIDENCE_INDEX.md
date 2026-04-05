# Evidence & Screenshots Index

This guide groups evidence by week/topic and links each artifact with a short caption. Follow the naming convention `wkNN_topic_index.png` for weekly lab screenshots or `ScreenshotN_ShortDesc.png` for general evidence.

## General Evidence

_No general screenshots yet. Add to [`screenshots/`](screenshots/) as evidence is captured._

## Week 01 — Firewall Log Analysis
_Pending — add `wk01_firewall_logs_1.png` through `wk01_firewall_logs_N.png`._

Expected evidence:
- Palo Alto Monitor tab with filtered traffic logs
- Threat logs showing blocked sessions
- Custom log filter applied to identify specific attack patterns

## Week 02 — Application Command Center
_Pending — add `wk02_acc_1.png` through `wk02_acc_N.png`._

Expected evidence:
- ACC dashboard showing application usage breakdown
- Top applications, users, and destinations over a time window
- Custom widget configuration

**Original work (preserved):**
- [`scripts/ping_sweep/ping-sweep-diagram.svg`](scripts/ping_sweep/ping-sweep-diagram.svg) — Flow diagram of async ping-sweep tool
- [`scripts/ping_sweep/ping-sweep-flow.mermaid`](scripts/ping_sweep/ping-sweep-flow.mermaid) — Mermaid source for flow diagram

## Week 03 — Reconnaissance Attack Prevention
_Pending — add `wk03_recon_1.png` through `wk03_recon_N.png`._

Expected evidence:
- Zone protection profile configuration
- DoS protection policy
- Port scan detection in threat logs

## Week 04 — Build a SOC (Group Project)
_Pending — add `wk04_wazuh_1.png` through `wk04_wazuh_N.png`._

Expected evidence:
- Wazuh Dashboard overview (agents, alerts, rules)
- Cyber Kill Chain mapping table
- Rule triggered example (SSH brute-force detection)
- FIM alert on modified system file
- Active-response firewall-drop demonstration

## Week 05 — Threat Intelligence (AutoFocus)
_Pending — add `wk05_autofocus_1.png` through `wk05_autofocus_N.png`._

Expected evidence:
- AutoFocus tag view for a threat family
- IoC correlation across the Palo Alto cloud
- Export of AutoFocus indicators into NGFW block list

## Week 06 — Endpoint Security & Vulnerability Profiles
_Pending — add `wk06_endpoint_1.png` through `wk06_endpoint_N.png`._

Expected evidence:
- Vulnerability Protection profile configuration
- Antivirus profile attached to security policy
- WildFire analysis profile
- Security policy with combined protection profiles

## Week 07 — Cloud Computing & Containers
_Pending — add `wk07_cloud_1.png` through `wk07_cloud_N.png`._

Expected evidence:
- Microsoft Defender for Cloud posture score
- Azure container registry scan findings
- Cloud deployment-model comparison (IaaS/PaaS/SaaS)

## Week 08 — Internet Threat Prevention
_Pending — add `wk08_threat_prevention_1.png` through `wk08_threat_prevention_N.png`._

Expected evidence:
- URL filtering category block list
- DNS security action table
- File blocking profile results

## Week 09 — Container Networking & Security
_Pending — add `wk09_container_1.png` through `wk09_container_N.png`._

Expected evidence:
- Container namespace / network policy configuration
- Kubernetes pod security admission enforcement
- Container workload protection (Prisma Cloud / Defender)

## Diagrams Preserved (Original Work)

| File | Description |
|---|---|
| [`scripts/ping_sweep/ping-sweep-diagram.svg`](scripts/ping_sweep/ping-sweep-diagram.svg) | Flow diagram showing async ping-sweep state machine |
| [`scripts/ping_sweep/ping-sweep-flow.mermaid`](scripts/ping_sweep/ping-sweep-flow.mermaid) | Mermaid source for flow diagram |
| [`scripts/ping_sweep/code-explanation.md`](scripts/ping_sweep/code-explanation.md) | Detailed walkthrough of Rust implementation |

---

*Last updated: 2026-04-04. Screenshots pending collection.*
