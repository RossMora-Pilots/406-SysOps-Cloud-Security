# Lab 05 — Stopping Reconnaissance Attacks

**Course:** SysOps and Cloud Security (CSC-7308) — Week 3
**Student:** Ross Moravec
**Platform:** Palo Alto Networks PAN-OS (Firewall Lab Environment)

---

## Executive Summary

This lab demonstrates how Palo Alto Networks Zone Protection Profiles defend network zones against reconnaissance attacks. A Zone Protection Profile was created and applied to the trust, untrust, and dmz zones. An `nmap`/Zenmap scan was then launched from the client machine (`192.168.1.20`) against the DMZ server (`192.168.50.10`) to simulate attacker reconnaissance. The firewall's threat logs confirmed that the reconnaissance activity was detected and logged, validating the protection profile's effectiveness.

---

## 1.1 Create a Zone Protection Profile

**Step 2** — Configure the Zone Protection Profile with reconnaissance protection settings in PAN-OS:

![PAN-OS Zone Protection Profile creation dialog with reconnaissance protection settings](../screenshots/wk03_recon_1.png)

**Step 3** — Finalize the Zone Protection Profile configuration:

![PAN-OS Zone Protection Profile configuration completion](../screenshots/wk03_recon_2.png)

## 1.2 Apply the Zone Protection Profile to Zones and Commit

**Step 5** — Click on the **dmz** zone to assign the Zone Protection Profile:

![PAN-OS zone configuration showing the dmz zone selected for Zone Protection Profile assignment](../screenshots/wk03_recon_3.png)

**Step 5** — Zone Protection Profile applied to all three zones (trust, untrust, dmz):

![PAN-OS zone list showing Zone Protection Profile applied to trust, untrust, and dmz zones](../screenshots/wk03_recon_4.png)

## 1.3 Perform a Reconnaissance Attack on the DMZ Server

**Step 3** — Zenmap interface before starting the scan against `192.168.50.10`:

![Zenmap GUI ready to launch a scan against the DMZ server at 192.168.50.10](../screenshots/wk03_recon_5.png)

**Step 3** — Zenmap scan results after the reconnaissance scan completes:

![Zenmap GUI showing scan results after running nmap against the DMZ server](../screenshots/wk03_recon_6.png)

## 1.4 Monitor and Analyze the Threat Logs

**Step 2** — Threat log details showing the firewall detected the reconnaissance attack. The attacker is `192.168.1.20` (client machine) and the victim is `192.168.50.10` (DMZ server):

![PAN-OS threat log detail showing detected reconnaissance from 192.168.1.20 to 192.168.50.10](../screenshots/wk03_recon_7.png)

---

## Security Significance / Analysis

- **Zone Protection Profiles** are a critical first line of defense in Palo Alto Networks firewalls. They operate at the zone ingress level, detecting and mitigating flood attacks, reconnaissance scans, and packet-based attacks before traffic reaches security policy evaluation.
- **Reconnaissance detection** is mapped to the first stage of the Lockheed Martin Cyber Kill Chain. By detecting port scans and network sweeps early, defenders gain visibility into attacker activity and can respond before exploitation occurs.
- **Key takeaway:** Applying Zone Protection Profiles to all zones (trust, untrust, dmz) ensures that reconnaissance from any direction — including lateral movement from compromised internal hosts — is logged and can trigger alerts.