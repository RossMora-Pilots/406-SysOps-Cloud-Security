# Lab 06 — Securing Endpoints with Vulnerability Profiles

> **Course:** SysOps and Cloud Security (CSC-7308) — Winter 2025
> **Week:** 6
> **Platform:** Palo Alto Networks PAN-OS

## Executive Summary

This lab configured endpoint protection on a Palo Alto Networks firewall by installing antivirus dynamic updates, manually updating the Applications and Threats content database, creating a custom vulnerability signature, and testing the vulnerability protection profile. A notable finding was a version mismatch between the lab guide's expected content update file and the version available in the lab environment — demonstrating the importance of verifying update versions in production deployments.

---

## Methodology

| Element | Detail |
|---|---|
| **Lab environment** | Palo Alto Networks SOFv2 (Security Operations Fundamentals v2) |
| **Platform** | PAN-OS next-generation firewall — endpoint security features |
| **Tools** | PAN-OS dynamic updates panel, vulnerability signature editor, threat log viewer |
| **Approach** | Install AV dynamic updates → manually install Applications and Threats content update → create a custom vulnerability signature → commit configuration → test vulnerability protection profile via threat log verification |
| **Scope** | Full endpoint protection lifecycle: update management, custom signature creation, and detection validation |

---

## 1.1 — Download and Install Latest Antivirus Dynamic Updates (Step 2)

Navigating to the PAN-OS dynamic updates panel to download and install the latest antivirus signatures. Dynamic updates ensure the firewall's threat detection capabilities remain current against newly discovered malware:

![PAN-OS dynamic updates panel showing available antivirus update packages for download](../screenshots/wk06_endpoint_1.png)

![PAN-OS dynamic updates panel confirming successful antivirus update installation](../screenshots/wk06_endpoint_2.png)

## 1.2 — Install Manual Update of Applications and Threats (Step 12)

> **⚠️ Observation — Version Mismatch:**
> The lab instructions reference content update file `panupv2-all-contents-8786-8435`, but only `panupv2-all-contents-8624-7617` was available in the lab environment. This is likely due to the lab environment running an older content release than the guide was written for. I proceeded with the available version, as the lab objectives remain achievable regardless of the specific content version.

**Download:** Only the file **`panupv2-all-contents-8624-7617`** is available for download, though the instructions mention `panupv2-all-contents-8786-8435`. I proceed with the only file available.

![PAN-OS content update download panel showing the available Applications and Threats package](../screenshots/wk06_endpoint_3.png)

**Installation:** Only the file **`panupv2-all-contents-8624-7617`** was available for installation, though the instructions mention `panupv2-all-contents-8786-8435`. I proceed with the only file available.

![PAN-OS content update installation progress for the Applications and Threats package](../screenshots/wk06_endpoint_4.png)

**Verification:** The file **`panupv2-all-contents-8624-7617`** with a checkmark in the "Currently Installed" column. The instructions mention `panupv2-all-contents-8786-8435` but no such file was available for download and installation.

![PAN-OS dynamic updates panel showing the installed content version with a checkmark confirming active status](../screenshots/wk06_endpoint_5.png)

## 1.3 — Create a Custom Vulnerability Signature (Step 5)

Defining a custom vulnerability signature in PAN-OS to detect specific exploit patterns not yet covered by the built-in signature set. Custom signatures extend protection to organization-specific or emerging threats:

![PAN-OS custom vulnerability signature configuration — general settings and threat metadata](../screenshots/wk06_endpoint_6.png)

![PAN-OS custom vulnerability signature configuration — pattern matching rules and conditions](../screenshots/wk06_endpoint_7.png)

## 1.6 — Commit and Test Vulnerability Protection (Step 9)

Committing the configuration changes and testing the vulnerability protection profile to verify that the firewall correctly detects and blocks traffic matching both built-in and custom vulnerability signatures:

![PAN-OS commit dialog confirming successful configuration push to the firewall](../screenshots/wk06_endpoint_8.png)

![PAN-OS threat log showing detected vulnerability events confirming protection is active](../screenshots/wk06_endpoint_9.png)

---

## Security Significance & Analysis

- **Defense-in-depth at the network edge:** Vulnerability profiles on PAN-OS inspect traffic inline and block exploit attempts before they reach endpoints, complementing host-based protections like EDR and OS patching.
- **Content update lifecycle:** The version mismatch observed in Section 1.2 highlights a critical operational reality — content update versions in lab environments (and production) may not always align with documentation. Security teams must verify installed versions and understand that older content databases may lack signatures for recently disclosed CVEs.
- **Custom vulnerability signatures:** The ability to author custom signatures (Section 1.3) is essential for zero-day response and protecting against threats specific to an organization's technology stack before vendor signatures become available.
- **Testing and validation:** Committing changes and verifying detection through threat logs (Section 1.6) follows security best practice — every policy change should be validated to confirm it behaves as intended and does not introduce gaps or false positives.
- **Automated vs. manual updates:** Dynamic updates (Section 1.1) provide automated, scheduled protection, while manual updates (Section 1.2) give administrators control over specific content versions — balancing timeliness with stability in change-sensitive environments.

## Findings

| # | Task | Result | Significance |
|---|---|---|---|
| 1 | AV dynamic update | ✅ Latest antivirus signatures installed | Firewall threat detection current against known malware |
| 2 | Manual content update | ⚠️ Version mismatch — `8624-7617` available vs. `8786-8435` expected | Lab environment ran older content; proceeded with available version |
| 3 | Content verification | ✅ `panupv2-all-contents-8624-7617` confirmed as "Currently Installed" | Content database active and enforced |
| 4 | Custom vulnerability signature | ✅ Signature created with pattern matching rules | Extends protection to organization-specific threats |
| 5 | Protection validation | ✅ Threat log entries confirmed detection of vulnerability events | End-to-end proof that profiles are actively inspecting traffic |

## Conclusions

1. **Version mismatches are an operational reality** — the discrepancy between the lab guide's expected content version and the available version mirrors production scenarios where update timelines vary across environments.
2. **Custom signatures enable zero-day response** — the ability to author and deploy custom vulnerability signatures is essential for protecting against emerging threats before vendor signatures become available.
3. **Validation closes the loop** — testing the protection profile and verifying threat log entries confirms that configuration changes translate to actual traffic inspection, not just management-plane settings.

## Recommendations

1. **Automate update verification** — implement scheduled checks that compare installed content versions against the latest available, alerting on version lag.
2. **Document version requirements** — maintain a compatibility matrix of content update versions required for specific threat signatures in production environments.
3. **Maintain a custom signature library** — build and version-control a repository of custom vulnerability signatures for rapid deployment during zero-day response scenarios.

