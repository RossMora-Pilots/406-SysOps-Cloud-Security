# Lab 03 — Analyzing Firewall Logs

> **Course:** SysOps and Cloud Security (CSC-7308) — Winter 2025, Cambrian College
> **Week:** 1
> **Lab Environment:** Palo Alto Networks SOFv2

## Executive Summary

This lab introduced PAN-OS firewall log analysis on a Palo Alto Networks next-generation firewall. The exercise covered three core operational tasks: loading a named device configuration, verifying system time synchronization on a Linux-based management host, and inspecting individual traffic log entries to extract session metadata — including session ID, destination NAT IP, and URL filtering category. These skills form the foundation for day-to-day SOC log triage on Palo Alto platforms.

---

## 1.0 — Load Named Configuration

**Objective:** In the Task Manager > All Tasks window, verify that the configuration load has successfully completed.

![Task Manager showing the completed Load configuration task with status and start time](../screenshots/wk01_firewall_logs_1.png)

The Task Manager confirms the named configuration loaded without errors. Verifying task completion before proceeding ensures the firewall is operating with the expected policy set.

---

## 1.1 — Verify System Time

**Objective:** Log in as `root` and run the `timedatectl` command to confirm the system clock configuration.

```bash
timedatectl
```

![Terminal output of the timedatectl command showing system clock, time zone, and NTP synchronization status](../screenshots/wk01_firewall_logs_2.png)

Accurate system time is critical for log correlation across devices. If the firewall's clock drifts from other infrastructure components, timestamps in SIEM aggregations become unreliable, complicating incident timelines.

---

## 1.2 — Examine Traffic Log Entry Details

**Objective:** Select a traffic log entry and identify the Session ID, NAT IP of the destination, and URL category.

![PAN-OS traffic log detail view showing session metadata, NAT translation, and URL category fields](../screenshots/wk01_firewall_logs_3.png)

| Field | Value |
|---|---|
| **Session ID** | `419` |
| **NAT IP (Destination)** | `91.189.91.48` |
| **URL Category** | `computer-and-internet-info` |

- **Session ID `419`** — uniquely identifies this traffic flow in the firewall's session table, enabling analysts to correlate related log entries across the traffic, threat, and URL filtering logs.
- **NAT IP `91.189.91.48`** — the post-NAT destination address, representing the public IP the internal host ultimately reached.
- **Category `computer-and-internet-info`** — the PAN-DB URL filtering classification for the destination, used by security policies to permit, block, or alert on web traffic by category.

---

## Security Significance / Analysis

Firewall log analysis is one of the most frequent tasks performed by SOC analysts. This lab demonstrated three essential competencies:

1. **Configuration management** — Loading and verifying named configurations ensures the firewall enforces the intended security policy before any traffic analysis begins.
2. **Time synchronization** — NTP-aligned clocks are a prerequisite for accurate log correlation across firewalls, endpoints, and SIEMs. A time skew of even a few seconds can derail forensic timelines.
3. **Log field interpretation** — Extracting session IDs, NAT translations, and URL categories from traffic logs enables analysts to reconstruct user activity, identify policy violations, and feed indicators into threat-hunting workflows.

Together, these skills establish the operational baseline required for more advanced PAN-OS monitoring tasks covered in subsequent labs.

