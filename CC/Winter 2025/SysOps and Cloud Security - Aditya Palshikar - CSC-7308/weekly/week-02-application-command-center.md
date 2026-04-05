# Week 02 — Application Command Center & Network Tools

## Session Info

| | |
|---|---|
| **Date** | 2025-01-14 |
| **Duration** | ~2+ hours lecture + lab time |
| **Lab** | Palo Alto Networks SOFv2 Lab 02 — Using the Application Command Center |
| **Deliverable** | Individual lab report + independent Rust ping-sweep tool |

## Topics Covered

- New approach to security: why perimeter-only defenses are insufficient against evolving threats
- Palo Alto Networks platform overview: **Strata** (NGFW), **Prisma** (Cloud), **Cortex** (Future/AI)
- Application Command Center (ACC): traffic analytics dashboard built on firewall logs
- Widgets: application usage, top sources/destinations, threat activity, URL filtering, HIP matches
- Time-range queries, global filters, drill-downs from widget to log view
- Supporting material: network scanning with custom tooling (original Rust implementation)

## Tools & Platforms

- **Palo Alto Networks ACC** — consolidated analytics view
- **Rust / Tokio** — independent async ping-sweep tool ([`../scripts/ping_sweep/`](../scripts/ping_sweep/))

## Key Concepts

### ACC as an Analyst's Starting Point

Raw logs tell you what happened; **ACC tells you the shape of what's happening**. A good SOC workflow starts at ACC with an open-ended scan ("is anything unusual?"), drills into suspicious widgets, and only then pivots to raw logs with a precise filter.

### The Strata / Prisma / Cortex Decomposition

Palo Alto's platform cleanly maps to enterprise surfaces:

- **Strata** — on-prem and perimeter (NGFW, the original product)
- **Prisma** — cloud workloads, SaaS (CASB), cloud-native apps
- **Cortex** — analytics, XDR, AI-driven detection across all telemetry

This decomposition is useful even when evaluating competing vendors: any modern security platform needs stories for **perimeter**, **cloud**, and **analytics**.

### Global Filters and Context

ACC widgets respond to global filters. Setting `zone=untrust` once reshapes every widget to answer "from the untrusted side, what is happening?" This is more productive than re-filtering each log type manually.

## Lab Deliverable

Two deliverables this week:

1. **ACC Lab Report** — standard lab submission with screenshots of widget configurations, observations about top applications, and analysis of traffic patterns.

2. **Independent Rust Ping-Sweep CLI** — ([`../scripts/ping_sweep/`](../scripts/ping_sweep/))
   - Async network-discovery tool written in Rust
   - Tokio runtime for concurrent ICMP operations
   - MPSC channels for thread-safe results
   - Subnet arithmetic from CIDR notation
   - User-interactive CLI

This second deliverable was self-initiated — a way to deepen understanding of the traffic that ACC would observe. By building a scanner, I saw exactly what traffic the ACC was classifying.

### Methodology
1. Accessed the Application Command Center (ACC) and configured global time-range filters
2. Analyzed top-application, top-source, and top-destination widgets to identify traffic patterns
3. Applied global zone filter (untrust) to reshape all widgets simultaneously
4. Drilled down from widget aggregates to raw log entries for specific application sessions
5. *Independent work:* Built the Rust async ping-sweep tool and correlated its ICMP traffic with ACC observations

## Reflection

> **💡 Key Takeaway:** The best way to understand a defender's view is to generate the attack traffic yourself — building a scanner reveals exactly what the firewall sees.

This week bridged **observing** (ACC) and **generating** (the ping sweep). Writing the scanner forced me to think about what the firewall sees: every IP I pinged would surface as an ICMP session in the traffic log and likely aggregate in an ACC "network activity" widget.

The Rust choice was deliberate — I wanted a concurrent, memory-safe implementation that could scan quickly without burning memory. The `spawn_blocking` pattern for bridging async Tokio to the blocking `pinger` crate was a real lesson about runtime boundaries.

Key takeaway: **the best way to understand a defender's view is to generate the traffic yourself**.

## Evidence

- **Lab Submission:** [Lab 02 — Using the Application Command Center (ACC)](../assignments/Wk02_Lab_02_ACC.md)
- **Screenshots:** [6 images](../screenshots/) — `wk02_acc_1.png` through `wk02_acc_6.png`

## Connections

- **Week 3** — The ping-sweep tool generates exactly the kind of traffic that Week 3's reconnaissance-prevention controls are designed to detect and block.
- **Week 4** — Wazuh would ingest host-side evidence of the same scan (outbound ICMP floods from a source host).
- **Week 5** — AutoFocus would classify subnets that produce high-volume recon traffic.

## Original Work Artifacts

- [`../scripts/ping_sweep/README.md`](../scripts/ping_sweep/README.md) — Project overview
- [`../scripts/ping_sweep/code-explanation.md`](../scripts/ping_sweep/code-explanation.md) — Line-by-line walkthrough
- [`../scripts/ping_sweep/ping-sweep-diagram.svg`](../scripts/ping_sweep/ping-sweep-diagram.svg) — Flow diagram

## References

- Palo Alto Networks ACC documentation (vendor site)
- Tokio [Async Runtime documentation](https://tokio.rs/)
- Course Lab PDF: `PAN_SOFv2_Lab_02.pdf` (vendor copyright — not redistributed)
- Course lecture transcript (local, Week 2)
