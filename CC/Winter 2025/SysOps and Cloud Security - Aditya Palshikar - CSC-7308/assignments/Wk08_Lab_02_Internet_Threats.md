# Lab 02 — Preventing Internet Threats with File Blocking

> **Course:** CSC-7308 SysOps and Cloud Security — Winter 2025, Cambrian College
> **Lab Track:** Palo Alto Networks CSFv2 (Cloud Security Fundamentals v2)
> **Week:** 8

## Executive Summary

This lab demonstrates how to prevent internet-borne threats using **file blocking profiles** in Palo Alto Networks PAN-OS. File blocking is a critical layer in the defense-in-depth stack — complementing URL filtering, DNS security, and WildFire analysis. The exercise walks through creating a file blocking profile that blocks dangerous MIME types (e.g., `.exe`, `.scr`, `.hta`), binding the profile to a security policy rule, and validating enforcement by attempting to download blocked file types through the firewall.

---

## 1.2 Apply the File Blocking Profile to a Security Policy

**Objective:** Attach the newly created file blocking profile to an existing security policy rule so that traffic matching the rule is inspected for blocked file types.

- **Step 2** (Section 1.2, page 13 of the PDF) — The file blocking profile is selected and applied to the security policy rule in the PAN-OS web interface.

![PAN-OS security policy rule with file blocking profile applied](../screenshots/wk08_threat_prevention_1.png)

---

## 1.3 Test the File Blocking Profile

**Objective:** Verify that the firewall actively blocks downloads of prohibited file types by attempting to retrieve files that match the blocking rules.

- **Step 3** (Section 1.3, page 15 of the PDF) — A download attempt is made for a blocked file type; the firewall intercepts the transfer and presents a block page.

![Blocked file download attempt showing firewall interception](../screenshots/wk08_threat_prevention_2.png)

![Firewall block page displayed after attempting to download a prohibited file type](../screenshots/wk08_threat_prevention_3.png)

![PAN-OS threat log entry confirming the file blocking action](../screenshots/wk08_threat_prevention_4.png)

---

## Security Significance / Analysis

| Concept | Detail |
|---|---|
| **Defense-in-Depth Layer** | File blocking sits between DNS/URL filtering (pre-connection) and WildFire sandboxing (post-download), stopping known-dangerous file types before they ever reach the endpoint. |
| **Blocked MIME Types** | Executable formats such as `.exe`, `.scr`, and `.hta` are common delivery vehicles for malware droppers and fileless attack stagers. |
| **Policy Binding** | A profile is ineffective until attached to a security policy rule — this separation of definition and enforcement follows the principle of modular, auditable policy management. |
| **Verification** | Testing with an actual download attempt confirms that the data plane is enforcing the profile, not just the management plane configuration. |

File blocking profiles reduce the attack surface by preventing users from inadvertently downloading executable content. Combined with WildFire (which analyzes unknown files in a sandbox), this creates a layered inspection pipeline where known-bad files are blocked immediately and unknown files are held for analysis.

