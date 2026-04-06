# Detailed Code Interpretation — Async Ping Sweep (v0.2)

## 1. Dependencies and Imports
```rust
use pinger::{PingResult, ping};
use std::fmt;
use std::net::Ipv4Addr;
use std::sync::mpsc;
use tokio::task;
```
- `pinger`: External crate for ICMP ping operations
- `fmt`: Display trait implementation for custom error type
- `mpsc`: Multi-producer, single-consumer channel for thread communication
- `tokio`: Asynchronous runtime for concurrent task execution
- `Ipv4Addr`: Standard library type for IPv4 address handling and conversion

## 2. Custom Error Type
```rust
#[derive(Debug)]
enum SweepError {
    InvalidIp(String),
    InvalidMask(String),
    IoError(std::io::Error),
}
```
- Replaces raw `.unwrap()` calls with typed error variants
- `InvalidIp` — captures the offending input string for diagnostic messages
- `InvalidMask` — handles out-of-range (>32) and non-numeric CIDR values
- `IoError` — wraps standard I/O failures (stdin/stdout)
- Implements `Display` for user-friendly error messages and `From<std::io::Error>` for `?` operator support

## 3. IP and Mask Parsing
```rust
fn parse_ip(ip: &str) -> Result<u32, SweepError> {
    let addr: Ipv4Addr = ip.parse()
        .map_err(|_| SweepError::InvalidIp(ip.to_string()))?;
    Ok(u32::from(addr))
}

fn parse_mask(mask: &str) -> Result<u8, SweepError> {
    let m: u8 = mask.parse()
        .map_err(|_| SweepError::InvalidMask(mask.to_string()))?;
    if m > 32 {
        return Err(SweepError::InvalidMask(format!("/{m} is out of range 0–32")));
    }
    Ok(m)
}
```
- **`parse_ip`** converts a dotted-quad string to a `u32` via the standard `Ipv4Addr` type — no manual octet splitting needed
- **`parse_mask`** validates the CIDR prefix is in range \[0, 32\] — rejects "33", "-1", "abc" with clear messages
- Both return `Result` types — errors propagate via `?` instead of panicking

## 4. Subnet Range Calculation (Bitwise)
```rust
fn subnet_range(ip: u32, prefix: u8) -> (u32, u32) {
    if prefix == 32 { return (ip, 1); }
    let host_bits = 32 - prefix as u32;
    let mask = !((1u32 << host_bits) - 1);
    let network = ip & mask;
    if prefix >= 31 {
        return (network, 1u32 << host_bits);
    }
    let total = (1u32 << host_bits) - 2;
    (network, total)
}
```
This is the **critical fix** from v0.1. The original code used integer division (`host_bits / 8`) to decide how many octets to overwrite — this failed for any prefix not on an 8-bit boundary (e.g., /25, /26, /30).

**How the bitwise approach works:**
1. Compute the network mask by inverting the lower `host_bits` bits: `/25` → `0xFFFFFF80`
2. AND with the input IP to get the network address: `10.0.0.200 & 0xFFFFFF80` = `10.0.0.128`
3. Subtract 2 from total addresses (network + broadcast) to get usable hosts: `/25` → 128 − 2 = **126**
4. Special cases: `/32` (single host), `/31` (point-to-point — RFC 3021, no broadcast)

**Verification for tricky subnets:**
| Prefix | Host bits | Total addresses | Usable hosts |
|--------|-----------|-----------------|--------------|
| /24    | 8         | 256             | 254          |
| /25    | 7         | 128             | 126          |
| /26    | 6         | 64              | 62           |
| /30    | 2         | 4               | 2            |
| /31    | 1         | 2               | 2 (RFC 3021) |
| /32    | 0         | 1               | 1            |

## 5. Main Application Structure
```rust
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
```
- `#[tokio::main]` sets up the async runtime
- `main()` delegates to `run()` which returns `Result<(), SweepError>`
- Errors are printed to stderr with a non-zero exit code — no panics reach the user

## 6. Scanning Loop
```rust
for i in 1..=num_hosts {
    let host_ip = ip_from_u32(network + i);
    ...
}
```
- `network + 1` is the first usable host; `network + num_hosts` is the last
- Avoids network address (`.0` equivalent) and broadcast address (`.255` equivalent)
- `ip_from_u32` converts back to `Ipv4Addr` for display and ping

## 7. Ping Function
```rust
async fn ping_host(ip: String) -> bool {
    task::spawn_blocking(move || {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let addr: Ipv4Addr = match ip.parse() { ... };
            match ping(std::net::IpAddr::V4(addr), None) {
                Ok(receiver) => { ... }
                Err(_) => { tx.send(false).ok(); }
            }
        });
        rx.recv_timeout(Duration::from_secs(2)).unwrap_or(false)
    }).await.unwrap_or(false)
}
```
- `spawn_blocking` moves the synchronous ping into Tokio's blocking thread pool
- Inner `std::thread::spawn` + channel pattern isolates the pinger library's blocking I/O
- 2-second timeout prevents hanging on unreachable hosts
- All error paths return `false` (unreachable) — no panics

## 8. Test Suite
The `#[cfg(test)]` module contains **22 unit tests** covering:
- **`parse_ip`** — valid IPs, loopback, invalid octets, garbage input, empty string
- **`parse_mask`** — valid boundaries (0, 24, 32), out-of-range (33), non-numeric
- **`subnet_range`** — correctness for /8, /16, /24, /25, /26, /30, /31, /32; verifies non-zero host input gets masked to correct network
- **`ip_from_u32`** — roundtrip conversion
- **Host iteration** — verifies first and last host addresses for /24, /25, /26

Run with `cargo test --verbose`.

## Key Technical Concepts

1. **Error Handling** — Custom `SweepError` enum with `Display` and `From` implementations; `?` operator propagation; no `.unwrap()` on user input
2. **Bitwise Subnet Arithmetic** — Proper network mask computation, host range calculation, and special-case handling for /31 and /32
3. **Asynchronous Programming** — Tokio runtime, `spawn_blocking` for CPU/IO-bound work, channel-based result collection
4. **Thread Safety** — `move` closures transfer ownership across thread boundaries; MPSC channels for safe cross-thread communication
5. **Network Programming** — `u32` ↔ `Ipv4Addr` conversion, CIDR prefix handling, ICMP ping via external crate
6. **Testing** — 22 unit tests, boundary conditions, error cases, roundtrip verification
