# Week 01 — Firewall Log Analysis

## Session Info

| | |
|---|---|
| **Date** | 2025-01-07 |
| **Duration** | ~1.5 hours lecture + lab time |
| **Lab** | Palo Alto Networks SOFv2 Lab 03 — Analyzing Firewall Logs |
| **Deliverable** | Individual lab report submission |

## Topics Covered

- Course orientation, objectives, grading, and the Palo Alto Networks Cybersecurity Academy platform
- Palo Alto Next-Generation Firewall (NGFW) architecture at a high level
- Log types in PAN-OS: traffic, threat, URL filtering, WildFire, data filtering, system, configuration
- Monitor tab navigation: filters, custom columns, time-range queries
- Interpreting log fields: source/destination zones, application, action, bytes, severity

## Tools & Platforms

- **Palo Alto Networks NGFW** (cloud lab instance)
- **PAN-OS Monitor tab** for log analysis
- **Log filtering DSL** (e.g., `( action eq block ) and ( severity eq high )`)

## Key Concepts

### Log Types vs. Log Purposes

Each PAN-OS log type answers a different question:

| Log Type | Question It Answers |
|---|---|
| Traffic | "Who talked to whom, and was it allowed?" |
| Threat | "What malicious activity was detected?" |
| URL Filtering | "Which sites did users visit?" |
| WildFire | "Was this file analyzed and verdicted?" |
| Data Filtering | "Did sensitive data leave the network?" |
| System | "What did the firewall itself do?" |
| Configuration | "Who changed what on this firewall?" |

### Log Filtering as Investigation

Effective log analysis is **question-driven, not log-driven**. A good analyst asks "did any user try to reach a known-bad domain in the last 24 hours?" and expresses it as a filter — they do not scroll through raw logs.

### Zone-to-Zone Traffic Context

Every traffic log entry has a source zone and destination zone. Understanding your **zone architecture** (trust, untrust, dmz, vpn) is prerequisite to interpreting logs. An `action=allow` log from `untrust → trust` is far more interesting than the same log from `trust → untrust`.

## Lab Deliverable

- Report submitted as DOCX — includes screenshots of filtered log queries, observations about detected traffic patterns, and analysis of identified threats.
- Sanitized PDF version to be added to [`../assignments/`](../assignments/).

## Reflection

The first week set the rhythm: **every** subsequent discussion would trace back to what the firewall saw and said. If you cannot read a firewall log, you cannot operate any of the later tools in this course — SIEM, threat intelligence, endpoint protection all depend on telemetry that begins here.

The biggest surprise: how much noise a production firewall generates. Scrolling raw logs is useless; disciplined filter-writing is the entire job.

## Connections

- **Week 2** — The ACC visualizes aggregates of these same logs.
- **Week 3** — Recognizing recon patterns depends on reading traffic + threat logs carefully.
- **Week 4** — Wazuh ingests similar log streams from hosts; the analytical mindset transfers.

## References

- Palo Alto Networks "Logs" documentation — [PAN-OS Admin Guide](https://docs.paloaltonetworks.com/pan-os) (vendor site)
- Course Lab PDF: `PAN_SOFv2_Lab_03.pdf` (vendor copyright — not redistributed)
