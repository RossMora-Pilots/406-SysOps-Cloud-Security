# Lab 03 — Container Networking and Security

> **Course:** CSC-7308 SysOps and Cloud Security — Winter 2025, Cambrian College
> **Lab Track:** Palo Alto Networks CSFv2 (Cloud Security Fundamentals v2)
> **Week:** 9

## Executive Summary

This lab explores **container networking and security** using Docker. The exercise covers the full lifecycle of pulling container images, running containers, inspecting their network configuration, testing inter-container connectivity, and exposing services via host-port mapping. By the end of the lab, an nginx web server runs inside a container and is reachable from the host network on port `8080` — illustrating how containerized services are published while remaining isolated at the network layer.

---

## Methodology

| Element | Detail |
|---|---|
| **Lab environment** | Palo Alto Networks CSFv2 with Docker runtime on lab VM |
| **Platform** | Docker Engine on Linux (lab virtual machine) |
| **Tools** | Docker CLI (`docker images`, `docker run`, `docker inspect`, `docker ps`), `ping`, web browser |
| **Approach** | Pull Ubuntu image → run container → inspect network config (bridge IP: `172.16.3.2`) → test inter-container ping → deploy nginx with `-p 8080:80` → verify web access from host |
| **Scope** | Container image management, network inspection, inter-container connectivity, host-port mapping, service verification |

---

## 1.2 Pull a Container Image and Run a Docker Container

**Objective:** Pull an Ubuntu image from Docker Hub, launch a container, inspect its network details, and verify inter-container connectivity.

- **Step 3** (Page 16) — Display the locally available Ubuntu container images using `docker images`.

![Terminal output of docker images command listing the pulled Ubuntu image](../screenshots/wk09_container_1.png)

- **Step 8** (Page 17) — View the details of the running container in JSON format using `docker inspect`.

![JSON output from docker inspect showing container network configuration](../screenshots/wk09_container_2.png)

- **Step 8** (Page 17) — Record the container's internal IP address: `172.16.3.2`.

![Terminal showing the container IP address 172.16.3.2 from docker inspect output](../screenshots/wk09_container_3.png)

- **Step 14** (Page 19) — Test container-to-container connectivity by running `ping` against the newly created container.

![Terminal output of ping command confirming connectivity to the container](../screenshots/wk09_container_4.png)

---

## 1.3 Map the Host Port to the Running Web Container and Access via Web Browser

**Objective:** Run an nginx container with host-port mapping (`-p 8080:80`) and verify the web server is accessible from the host at `http://192.168.50.10:8080`.

- **Step 4** (Page 22) — View the newly created nginx container and its port mapping using `docker ps`.

![Docker ps output showing the nginx container with port 8080:80 mapping](../screenshots/wk09_container_5.png)

- **Step 12** (Page 24) — Access the nginx container's default web page by browsing to `http://192.168.50.10:8080`.

![Web browser displaying the nginx default welcome page served from the container on port 8080](../screenshots/wk09_container_6.png)

---

## Key Commands Reference

| Command | Purpose |
|---|---|
| `docker images` | List locally available container images |
| `docker run` | Create and start a new container from an image |
| `docker inspect <container>` | Display detailed container metadata in JSON (including IP address) |
| `docker ps` | List running containers and their port mappings |
| `docker run -p 8080:80 nginx` | Run nginx and map host port `8080` to container port `80` |
| `ping 172.16.3.2` | Test network reachability to a container's internal IP |

---

## Security Significance / Analysis

| Concept | Detail |
|---|---|
| **Network Isolation** | Docker containers receive private IPs (e.g., `172.16.3.2`) on an internal bridge network, limiting their exposure to the host and external networks by default. |
| **Port Mapping as Attack Surface** | Mapping `-p 8080:80` explicitly publishes a service — each mapped port is an entry point that must be accounted for in firewall rules and vulnerability scans. |
| **Container Inspection** | `docker inspect` reveals runtime configuration (mounts, env vars, network settings) — an essential tool for incident response and security auditing of containerized workloads. |
| **Least Privilege Networking** | Containers should only expose the minimum ports required. Unnecessary port mappings widen the attack surface and violate the principle of least privilege. |
| **Image Supply Chain** | Pulling images from Docker Hub introduces supply-chain risk; production environments should use signed images from trusted registries with vulnerability scanning enabled. |

Understanding container networking is foundational to securing cloud-native workloads. The isolation provided by Docker's bridge network is a starting point, but production deployments require additional controls — network policies, service meshes, and runtime monitoring — to enforce segmentation and detect lateral movement between containers.

## Findings

| # | Task | Result | Significance |
|---|---|---|---|
| 1 | Image pull | ✅ Ubuntu image pulled from Docker Hub | Image available locally for container creation |
| 2 | Container network inspection | Bridge IP: **`172.16.3.2`** (private, isolated) | Container receives internal IP, not exposed externally |
| 3 | Inter-container connectivity | ✅ Ping successful between containers | Default bridge allows container-to-container communication |
| 4 | Host-port mapping | nginx on **`-p 8080:80`** | Service published to host network — explicit attack surface |
| 5 | Web access verification | ✅ nginx default page at `http://192.168.50.10:8080` | End-to-end proof of service exposure from container to host network |

## Conclusions

1. **Default isolation is only a starting point** — Docker's bridge network provides private IPs, but default settings allow unrestricted container-to-container communication within the same bridge.
2. **Port mapping creates explicit attack surface** — each `-p` flag publishes a service to the host network; every mapped port must be accounted for in firewall rules and vulnerability scans.
3. **Container inspection is essential for security auditing** — `docker inspect` reveals runtime configuration (mounts, environment variables, network settings) critical for incident response and compliance checks.

## Recommendations

1. **Implement network policies** — use Docker network segmentation or Kubernetes NetworkPolicies to restrict inter-container communication to only required paths.
2. **Scan container images** — integrate vulnerability scanning (e.g., Trivy, Snyk) into the image pull pipeline; only deploy images from trusted, signed registries.
3. **Minimize port mappings** — follow least-privilege networking: only expose ports that are explicitly required, and bind to specific interfaces rather than `0.0.0.0`.
4. **Add runtime monitoring** — deploy container runtime security tools (e.g., Falco, Sysdig) to detect anomalous behavior such as unexpected network connections or file modifications.