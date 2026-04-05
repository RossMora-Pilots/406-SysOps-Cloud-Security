# Week 03 — Stopping Reconnaissance Attacks

## Session Info

| | |
|---|---|
| **Date** | 2025-01-21 |
| **Duration** | ~1.5–2 hours lecture + lab time |
| **Lab** | Palo Alto Networks SOFv2 Lab 05 — Stopping Reconnaissance Attacks |
| **Deliverable** | Individual lab report submission |

## Topics Covered

- Reconnaissance as the **first stage of the Cyber Kill Chain**
- Active vs. passive reconnaissance techniques
- Common active recon tools and behaviors: nmap, masscan, ping sweeps, port scans, banner grabbing
- Palo Alto Networks **Zone Protection Profiles**
- **DoS Protection Policies** and their role in absorbing recon traffic
- Threat log signatures for recon activity
- Tuning thresholds to avoid false positives while catching real scans

## Tools & Platforms

- **Palo Alto Networks NGFW** — Zone Protection, DoS Protection configuration
- **Threat logs** for recon signatures

## Key Concepts

### Why Reconnaissance Must Be Stopped Early

Reconnaissance is the **cheapest stage** for the attacker and the **most visible**. If an organization cannot detect port scans at its perimeter, it cannot detect the quieter follow-on stages (weaponization is invisible; exploitation is fast).

### Zone Protection Profile Components

| Component | Purpose |
|---|---|
| **Flood Protection** | Thresholds for SYN, UDP, ICMP, ICMPv6 flood attacks |
| **Reconnaissance Protection** | Detect/block TCP/UDP port scans and host sweeps |
| **Packet-Based Attack Protection** | Drop malformed packets, spoofed IPs, fragmented sessions |
| **Protocol Protection** | Enforce strict RFC adherence |

### Threshold Tuning

Two thresholds matter per scan type:

- **Alert threshold** — generate a threat log
- **Block IP threshold** — add the source to the blacklist for N seconds

Setting blocks too low = false positives (legitimate tools blocked). Setting too high = real attackers slip through. The tuning ritual: start permissive, observe baseline, tighten.

### Connection to Week 2's Ping Sweep

My own Rust ping-sweep tool is exactly the kind of traffic Zone Protection is tuned to detect. If I ran the tool inside a zone with protection enabled and sensible thresholds, I would:

1. See threat log entries for "ICMP host sweep"
2. Be added to the block-IP list after N hosts
3. Lose visibility into subsequent target responses

## Lab Deliverable

- Report submitted as DOCX — includes screenshots of Zone Protection profile configuration, DoS policy, and threat log entries showing detected reconnaissance activity.
- Sanitized PDF to be added to [`../assignments/`](../assignments/).

## Reflection

This week delivered the **attacker-defender** contrast the course is built on. Building a ping-sweep tool in Week 2 and immediately seeing it blocked by Week 3's Zone Protection Profile in lab context made the defensive value concrete.

A key lesson: defenders do not need to know the specific tool. Zone Protection does not care whether my scanner is written in Rust, Python, or bash — it pattern-matches on behavior (many destinations, low packet count per destination, consistent source).

## Connections

- **Week 2** — The Rust ping-sweep tool I built is the textbook attacker behavior this week's controls detect.
- **Week 4** — Wazuh rules detect the same behavior from host-side evidence (outbound ICMP floods logged at the host).
- **Week 5** — AutoFocus tracks subnets that are repeat sources of scanning activity.
- **CSC-7311 Ethical Hacking** — Offensive parallel: how attackers evade Zone Protection via distributed sources, slow scans, or decoys.

## References

- Palo Alto Networks "Zone Protection" administrator documentation (vendor site)
- MITRE ATT&CK: [Reconnaissance (TA0043)](https://attack.mitre.org/tactics/TA0043/)
- Course Lab PDF: `PAN_SOFv2_Lab_05.pdf` (vendor copyright — not redistributed)
- Course lecture transcript (local, Week 3)
