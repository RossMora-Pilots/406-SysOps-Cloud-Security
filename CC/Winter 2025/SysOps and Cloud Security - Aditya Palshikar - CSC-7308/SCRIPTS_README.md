# Scripts — Usage and Notes

This document summarizes the purpose and basic usage of each script in this course repository.

## Student-Created Scripts (`scripts/`)

### `ping_sweep/` — Async Rust Ping-Sweep CLI

Independent original work extending the **Week 2** lab on network tools. An asynchronous ICMP ping-sweep utility written in Rust using the Tokio runtime.

**Features**
- Concurrent ping operations across a subnet via Tokio tasks
- Thread-safe result communication via MPSC channels
- Subnet arithmetic from CIDR notation
- User-driven CLI prompts

**Inputs**
- Base IP (e.g., `192.168.1.0`)
- Subnet mask in CIDR (e.g., `24` for `/24`)

**Output**
- Per-host reachability status printed to stdout

**Files**
| File | Purpose |
|---|---|
| [`ping_sweep/code-explanation.md`](scripts/ping_sweep/code-explanation.md) | Detailed line-by-line walkthrough of the Rust implementation |
| [`ping_sweep/ping-sweep-flow.mermaid`](scripts/ping_sweep/ping-sweep-flow.mermaid) | Mermaid source for control-flow diagram |
| [`ping_sweep/ping-sweep-diagram.svg`](scripts/ping_sweep/ping-sweep-diagram.svg) | Rendered SVG flow diagram |
| [`ping_sweep/README.md`](scripts/ping_sweep/README.md) | Project-level README |

**Dependencies** (Cargo.toml sketch)
```toml
[dependencies]
pinger = "1.0"
tokio = { version = "1", features = ["full"] }
```

**Note:** The Rust source file ([`ping_sweep/src/main.rs`](scripts/ping_sweep/src/main.rs)) and [`Cargo.toml`](scripts/ping_sweep/Cargo.toml) are included in the repository. See `code-explanation.md` for a detailed line-by-line walkthrough.

## Provided / External Scripts (`scripts-extra/`)

_No external scripts currently archived. Reference scripts used in labs (Palo Alto NGFW commands, Wazuh active-response helpers) remain with their respective platforms and are not redistributed here._

## Safety Notice

Review scripts before running; test in lab VMs or isolated subnets. Some scripts invoke networking primitives that should not be run against unauthorized hosts. Use with appropriate privileges and explicit authorization.
