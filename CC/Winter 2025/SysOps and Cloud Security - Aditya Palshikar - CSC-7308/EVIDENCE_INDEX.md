# Evidence & Screenshots Index

This guide groups evidence by week/topic and links each artifact with a short caption. Screenshots were extracted from lab submission DOCX files and follow the naming convention `wkNN_topic_index.png`.

**Total evidence artifacts: 44 screenshots + 4 diagrams/code files**

---

## Week 01 — Firewall Log Analysis (3 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk01_1](screenshots/wk01_firewall_logs_1.png) | Task Manager confirming named configuration loaded successfully — prerequisite for all subsequent lab tasks |
| 2 | ![wk01_2](screenshots/wk01_firewall_logs_2.png) | `timedatectl` output confirming NTP synchronization active — critical for cross-device log correlation |
| 3 | ![wk01_3](screenshots/wk01_firewall_logs_3.png) | Traffic log detail: Session ID **419**, NAT destination **91.189.91.48**, URL category **computer-and-internet-info** |

## Week 02 — Application Command Center (6 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk02_1](screenshots/wk02_acc_1.png) | Load Named Configuration dialog — `pan-sof-lab-02.xml` selected for ACC lab |
| 2 | ![wk02_2](screenshots/wk02_acc_2.png) | Simulated malware traffic generation script output — populating ACC with C2 patterns |
| 3 | ![wk02_3](screenshots/wk02_acc_3.png) | ACC threat activity widget — **Bredolab.Gen C2 Traffic** detected, ranked high severity |
| 4 | ![wk02_4](screenshots/wk02_acc_4.png) | ACC threat detail panel — metadata for Bredolab.Gen C2 detection |
| 5 | ![wk02_5](screenshots/wk02_acc_5.png) | ACC filtered view confirming **Bredolab.Gen Command and Control Traffic** identification |
| 6 | ![wk02_6](screenshots/wk02_acc_6.png) | Detailed threat log: source IP, destination IP, application, action taken, triggering signature |

**Original work (preserved):**

- [`scripts/ping_sweep/ping-sweep-diagram.svg`](scripts/ping_sweep/ping-sweep-diagram.svg) — Flow diagram of async ping-sweep tool
- [`scripts/ping_sweep/ping-sweep-flow.mermaid`](scripts/ping_sweep/ping-sweep-flow.mermaid) — Mermaid source for flow diagram
- [`scripts/ping_sweep/src/main.rs`](scripts/ping_sweep/src/main.rs) — Rust source code

## Week 03 — Reconnaissance Attack Prevention (7 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk03_1](screenshots/wk03_recon_1.png) | Zone Protection Profile creation — reconnaissance detection settings enabled |
| 2 | ![wk03_2](screenshots/wk03_recon_2.png) | Zone Protection Profile configuration finalized — thresholds defined |
| 3 | ![wk03_3](screenshots/wk03_recon_3.png) | DMZ zone selected for Zone Protection Profile assignment |
| 4 | ![wk03_4](screenshots/wk03_recon_4.png) | Profile applied to **3 zones** (trust, untrust, dmz) — full coverage confirmed |
| 5 | ![wk03_5](screenshots/wk03_recon_5.png) | Zenmap GUI ready to scan DMZ server at **192.168.50.10** |
| 6 | ![wk03_6](screenshots/wk03_recon_6.png) | Zenmap scan results after nmap reconnaissance against DMZ server |
| 7 | ![wk03_7](screenshots/wk03_recon_7.png) | Threat log: reconnaissance detected — attacker **192.168.1.20** → victim **192.168.50.10** |

## Week 04 — Build a SOC / Cyber Kill Chain (4 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk04_1](screenshots/wk04_cyber_kill_chain_1.png) | Cyber Kill Chain quiz — page 1: 7-stage framework knowledge assessment |
| 2 | ![wk04_2](screenshots/wk04_cyber_kill_chain_2.png) | Cyber Kill Chain quiz — page 2: defensive countermeasures per stage |
| 3 | ![wk04_3](screenshots/wk04_cyber_kill_chain_3.png) | Networking Concepts Part 2 — page 1: TCP/IP, subnetting, DNS |
| 4 | ![wk04_4](screenshots/wk04_cyber_kill_chain_4.png) | Networking Concepts Part 2 — page 2: routing and protocol analysis |

## Week 05 — Threat Intelligence / MineMeld (5 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk05_1](screenshots/wk05_threat_intel_1.png) | Docker volumes created for MineMeld persistent storage — deployment verification |
| 2 | ![wk05_2](screenshots/wk05_threat_intel_2.png) | `docker-compose.yml` in vi — MineMeld service definition, ports, and volume mounts |
| 3 | ![wk05_3](screenshots/wk05_threat_intel_3.png) | PAN-OS EDL config — MineMeld high-confidence indicator list integrated into security policy |
| 4 | ![wk05_4](screenshots/wk05_threat_intel_4.png) | PAN-OS EDL config — MineMeld bad IP list added as second threat feed source |
| 5 | ![wk05_5](screenshots/wk05_threat_intel_5.png) | EDL panel — block list indicators populated and available for policy enforcement |

## Week 06 — Endpoint Security & Vulnerability Profiles (9 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk06_1](screenshots/wk06_endpoint_1.png) | Dynamic updates panel — AV update packages available for download |
| 2 | ![wk06_2](screenshots/wk06_endpoint_2.png) | AV update installation confirmed — signatures current |
| 3 | ![wk06_3](screenshots/wk06_endpoint_3.png) | Content update: `panupv2-all-contents-8624-7617` available (version mismatch with lab guide) |
| 4 | ![wk06_4](screenshots/wk06_endpoint_4.png) | Manual Applications and Threats content update installation in progress |
| 5 | ![wk06_5](screenshots/wk06_endpoint_5.png) | Content version `8624-7617` confirmed as "Currently Installed" with checkmark |
| 6 | ![wk06_6](screenshots/wk06_endpoint_6.png) | Custom vulnerability signature — general settings and threat metadata defined |
| 7 | ![wk06_7](screenshots/wk06_endpoint_7.png) | Custom vulnerability signature — pattern matching rules and conditions |
| 8 | ![wk06_8](screenshots/wk06_endpoint_8.png) | Commit dialog — configuration successfully pushed to firewall |
| 9 | ![wk06_9](screenshots/wk06_endpoint_9.png) | Threat log — vulnerability events detected, confirming protection profile is active |

## Week 07 — Cloud Computing & Containers

_No lab screenshots — this was a lecture-only week. Concepts are documented in the [weekly summary](weekly/week-07-cloud-computing-containers.md) and reinforced with hands-on labs in Weeks 8–9._

## Week 08 — Internet Threat Prevention (4 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk08_1](screenshots/wk08_threat_prevention_1.png) | File blocking profile attached to security policy rule — enforcement active |
| 2 | ![wk08_2](screenshots/wk08_threat_prevention_2.png) | Blocked file download — firewall intercepted `.exe`/`.scr`/`.hta` transfer |
| 3 | ![wk08_3](screenshots/wk08_threat_prevention_3.png) | Firewall block page displayed to user after prohibited file download attempt |
| 4 | ![wk08_4](screenshots/wk08_threat_prevention_4.png) | Threat log entry confirming file blocking action with file type and policy details |

## Week 09 — Container Networking & Security (6 screenshots)

| # | Screenshot | Description |
|---|---|---|
| 1 | ![wk09_1](screenshots/wk09_container_1.png) | `docker images` — Ubuntu container image pulled from Docker Hub |
| 2 | ![wk09_2](screenshots/wk09_container_2.png) | `docker inspect` JSON — container network configuration and metadata |
| 3 | ![wk09_3](screenshots/wk09_container_3.png) | Container internal IP **172.16.3.2** extracted from inspect output |
| 4 | ![wk09_4](screenshots/wk09_container_4.png) | Inter-container `ping` — connectivity confirmed between containers on bridge network |
| 5 | ![wk09_5](screenshots/wk09_container_5.png) | `docker ps` — nginx container with port mapping **8080:80** visible |
| 6 | ![wk09_6](screenshots/wk09_container_6.png) | nginx default welcome page at **http://192.168.50.10:8080** — service accessible from host |

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
