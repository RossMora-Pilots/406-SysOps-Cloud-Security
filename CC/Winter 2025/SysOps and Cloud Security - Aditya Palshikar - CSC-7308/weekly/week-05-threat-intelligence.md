# Week 05 — Threat Intelligence (AutoFocus)

## Session Info

| | |
|---|---|
| **Date** | 2025-02-04 |
| **Duration** | ~1.5 hours lecture + lab time |
| **Lab** | Palo Alto Networks SOFv2 Lab 07 — Threat Intelligence |
| **Deliverable** | Individual lab report submission |

## Topics Covered

- Threat intelligence definition and value in modern security
- Proactive vs. reactive security postures
- **Palo Alto Networks AutoFocus** — cloud-based threat intelligence platform
- AutoFocus **tags** and **tag groups** for threat family classification
- IoC (Indicator of Compromise) correlation at scale
- Integrating threat intelligence with NGFW policy and SIEM correlation
- **WildFire** cloud-based malware analysis and verdict delivery

## Tools & Platforms

- **Palo Alto Networks AutoFocus** — threat intelligence dashboard
- **NGFW threat prevention** profile integration
- **WildFire** — sandboxed malware analysis

## Key Concepts

### What Makes Threat Intelligence "Actionable"

Raw IoCs (IPs, hashes, domains) are worthless in isolation. They become **actionable** when:

1. **Contextualized** — tied to a specific threat actor, campaign, or malware family
2. **Timely** — delivered fast enough to matter (hours, not weeks)
3. **Targeted** — scoped to industries, geographies, or technology stacks the defender actually has
4. **Integrated** — automatically feeds controls (NGFW blocklists, SIEM rules) without human copy-paste

AutoFocus delivers (1)–(4) natively by tying raw indicators to Palo Alto's global telemetry.

### Proactive Prevention vs. Reactive Detection

Traditional security is **reactive**: wait for an alert, then respond. Threat-intel-driven security is **proactive**: if AutoFocus sees a domain weaponized elsewhere, your NGFW blocks it *before* your users ever try to reach it.

### AutoFocus Tag Ecosystem

- **Malware family tags** — `Mirai`, `Emotet`, `Lockbit`, etc.
- **Actor tags** — specific threat groups
- **Campaign tags** — tied to observed operations
- **Industry/Geography tags** — targeting context

### WildFire Integration

WildFire receives unknown-verdict samples from Palo Alto firewalls globally, analyzes them in sandboxes, and publishes verdicts back. A sample that was benign an hour ago can become known-malicious now — your firewall sees the new verdict and blocks future instances.

## Lab Deliverable

- Report submitted as DOCX — includes screenshots of AutoFocus tag views, IoC detail pages, and how indicators were pulled into firewall policy or external blocklists.
- Sanitized PDF to be added to [`../assignments/`](../assignments/).

### Methodology
1. Accessed the Palo Alto AutoFocus threat intelligence portal
2. Explored tag views for known malware families, identifying IoC types (hashes, domains, IPs)
3. Correlated AutoFocus indicators with NGFW threat prevention logs
4. Examined WildFire verdict delivery pipeline — sample submission, analysis, signature publication
5. Documented how threat intelligence feeds integrate into NGFW blocklists for automated prevention

## Reflection

> **💡 Key Takeaway:** Threat intelligence is a feedback loop, not a data feed — every firewall's observations strengthen the collective defense of all participants.

Threat intelligence reframes security from "my network" to "the network of networks." If an attacker burned an IoC against another organization's firewall yesterday, my firewall should benefit from that knowledge today.

The most important conceptual shift: **intelligence is a loop, not a feed**. AutoFocus is not just pushing data to the firewall; the firewall's observations feed WildFire which updates AutoFocus which updates every customer. Participation strengthens the whole.

A reservation: threat intel creates **signature fatigue** risk. Blocklists grow unbounded; old indicators stale; false positives bleed credibility. Tuning cadence matters.

## Evidence

- **Lab Submission:** [Lab 07 — Threat Intelligence](../assignments/Wk05_Lab_07_Threat_Intelligence.md)
- **Screenshots:** [5 images](../screenshots/) — `wk05_threat_intel_1.png` through `wk05_threat_intel_5.png`

## Connections

- **Week 4** — Threat intel feeds SIEM correlation rules in Wazuh (or any SIEM).
- **Week 6** — Vulnerability profiles use similar signature streams.
- **Week 8** — Internet threat prevention depends on URL / DNS intel exactly like this.
- **CSC-7312 Malware Analysis** — This is the consumer of WildFire sandbox analysis.

## References

- Palo Alto Networks AutoFocus [product page](https://www.paloaltonetworks.com/cortex/autofocus) (vendor)
- Palo Alto Networks WildFire [documentation](https://docs.paloaltonetworks.com/wildfire) (vendor)
- Course Lab PDF: `PAN_SOFv2_Lab_07.pdf` (vendor copyright — not redistributed)
- Course lecture transcript (local, Week 5)
