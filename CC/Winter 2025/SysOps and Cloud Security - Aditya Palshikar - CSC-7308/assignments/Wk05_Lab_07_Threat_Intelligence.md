# Lab 07 — Threat Intelligence

> **Course:** SysOps and Cloud Security (CSC-7308) — Winter 2025
> **Week:** 5
> **Platform:** Palo Alto Networks PAN-OS with MineMeld

## Executive Summary

This lab deployed MineMeld, Palo Alto Networks' open-source threat intelligence processing framework, using Docker containers. MineMeld was configured to ingest external threat intelligence feeds, process indicators of compromise (IoCs), and output high-confidence threat lists. These lists were then integrated into PAN-OS security policy as External Dynamic Lists (EDLs), enabling automated blocking of known malicious IP addresses without manual rule updates.

---

## Methodology

| Element | Detail |
|---|---|
| **Lab environment** | Palo Alto Networks SOFv2 with Docker host |
| **Platform** | PAN-OS NGFW + MineMeld (Docker containers) |
| **Tools** | Docker, docker-compose, MineMeld web interface, PAN-OS EDL configuration |
| **Approach** | Deploy MineMeld via Docker → configure threat intelligence feeds → create high-confidence and bad-IP output nodes → integrate as EDLs in PAN-OS security policy → verify indicator population |
| **Scope** | End-to-end threat intelligence pipeline: feed ingestion → indicator processing → firewall policy enforcement |

---

## 1.1 — Deploy MineMeld via Docker (Step 4)

Listing created Docker volumes to verify MineMeld's persistent storage was provisioned correctly:

```bash
ls /var/lib/docker/volumes
```

![Terminal output showing Docker volumes created for the MineMeld deployment](../screenshots/wk05_threat_intel_1.png)

## 1.2 — Configure MineMeld Container (Step 3)

Viewing the `docker-compose.yml` file in the `vi` editor to inspect the MineMeld service definition, port mappings, and volume mounts:

![Vi editor displaying the docker-compose.yml configuration for MineMeld services](../screenshots/wk05_threat_intel_2.png)

## 1.4 — Integrate High-Confidence Threat List (Step 8)

Adding the MineMeld high-confidence indicator list as an External Dynamic List (EDL) source in the PAN-OS security policy. This enables the firewall to automatically block traffic matching high-confidence threat indicators:

![PAN-OS security policy configuration adding the MineMeld high-confidence EDL](../screenshots/wk05_threat_intel_3.png)

## 1.5 — Integrate Bad IP List (Step 17)

Adding the MineMeld bad IP list as a second EDL source in the security policy. This list contains IP addresses identified as malicious across multiple threat intelligence feeds:

![PAN-OS security policy configuration adding the MineMeld bad IP EDL](../screenshots/wk05_threat_intel_4.png)

## 1.5 — Verify EDL Block List Indicators (Step 23)

Viewing the block list indicators populated in the External Dynamic List (EDL) to confirm that threat intelligence feeds are being ingested and indicators are available for policy enforcement:

![PAN-OS External Dynamic List panel displaying populated block list indicators from MineMeld](../screenshots/wk05_threat_intel_5.png)

---

## Security Significance & Analysis

- **Automated threat response:** Integrating MineMeld with PAN-OS EDLs creates an automated pipeline from threat intelligence feeds to firewall enforcement, reducing the time between threat discovery and protection from hours to minutes.
- **Defense in depth:** Using multiple feed types (high-confidence indicators and bad IP lists) provides layered coverage — high-confidence feeds minimize false positives while broader bad-IP feeds cast a wider net.
- **Docker-based deployment:** Containerizing MineMeld isolates the threat intelligence processing engine, simplifies updates, and ensures reproducible deployments via `docker-compose.yml`.
- **External Dynamic Lists (EDLs):** EDLs are a key PAN-OS feature that allows security policy to reference dynamically updated indicator lists without requiring manual commits for each new threat — essential for keeping pace with evolving threat landscapes.
- **Operational consideration:** Feed quality and update frequency directly impact the effectiveness of automated blocking. Stale or low-quality feeds can introduce false positives that disrupt legitimate traffic.

## Findings

| # | Task | Result | Significance |
|---|---|---|---|
| 1 | MineMeld Docker deployment | ✅ Volumes provisioned, containers running | Isolated TI processing engine with persistent storage |
| 2 | docker-compose configuration | ✅ Service definition, ports, volumes verified | Reproducible deployment via infrastructure-as-code |
| 3 | High-confidence EDL integration | ✅ Indicators populated in PAN-OS EDL | Automated blocking of high-confidence malicious IPs |
| 4 | Bad IP list EDL integration | ✅ Second EDL source added and populated | Broader coverage from multiple threat feeds |
| 5 | Block list verification | ✅ Indicators visible in EDL panel | Confirmed end-to-end pipeline from feed to enforcement |

## Conclusions

1. **Automated threat intelligence reduces response time** — the MineMeld → EDL pipeline eliminates manual indicator management, cutting the time from threat discovery to firewall enforcement from hours to minutes.
2. **Layered feed strategy improves coverage** — using both high-confidence (low false-positive) and broader bad-IP feeds balances precision with coverage.
3. **Docker simplifies TI infrastructure** — containerized deployment isolates the threat intelligence engine, simplifies updates, and ensures reproducibility.

## Recommendations

1. **Monitor feed freshness** — implement automated checks to alert when feed update timestamps exceed expected intervals (stale feeds create blind spots).
2. **Add STIX/TAXII feeds** — expand beyond the default MineMeld sources to include industry-specific threat intelligence sharing communities.
3. **Establish feed quality metrics** — track false-positive rates per feed source and adjust confidence thresholds to minimize impact on legitimate traffic.

