use pinger::{PingResult, ping};
use std::fmt;
use std::net::Ipv4Addr;
use std::sync::mpsc;
use tokio::task;

/// Errors that can occur during a ping sweep.
#[derive(Debug)]
enum SweepError {
    InvalidIp(String),
    InvalidMask(String),
    IoError(std::io::Error),
}

impl fmt::Display for SweepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SweepError::InvalidIp(s) => write!(f, "Invalid IP address: {s}"),
            SweepError::InvalidMask(s) => write!(f, "Invalid subnet mask: {s}"),
            SweepError::IoError(e) => write!(f, "I/O error: {e}"),
        }
    }
}

impl From<std::io::Error> for SweepError {
    fn from(e: std::io::Error) -> Self {
        SweepError::IoError(e)
    }
}

/// Parse a dotted-quad IP string into a 32-bit integer.
fn parse_ip(ip: &str) -> Result<u32, SweepError> {
    let addr: Ipv4Addr = ip
        .parse()
        .map_err(|_| SweepError::InvalidIp(ip.to_string()))?;
    Ok(u32::from(addr))
}

/// Parse and validate a CIDR prefix length (0–32).
fn parse_mask(mask: &str) -> Result<u8, SweepError> {
    let m: u8 = mask
        .parse()
        .map_err(|_| SweepError::InvalidMask(mask.to_string()))?;
    if m > 32 {
        return Err(SweepError::InvalidMask(format!("/{m} is out of range 0–32")));
    }
    Ok(m)
}

/// Compute the network base address and usable host count from a CIDR block.
/// Returns (network_address, num_usable_hosts).
fn subnet_range(ip: u32, prefix: u8) -> (u32, u32) {
    if prefix == 32 {
        return (ip, 1); // single-host address
    }
    let host_bits = 32 - prefix as u32;
    let mask = !((1u32 << host_bits) - 1); // e.g. /24 → 0xFFFFFF00
    let network = ip & mask;
    if prefix >= 31 {
        // /31 point-to-point: 2 usable, no broadcast convention
        let total = 1u32 << host_bits;
        return (network, total);
    }
    // Normal subnet: exclude network (.0) and broadcast (.255-equivalent)
    let total = (1u32 << host_bits) - 2;
    (network, total)
}

/// Convert a 32-bit address back to an Ipv4Addr.
fn ip_from_u32(ip: u32) -> Ipv4Addr {
    Ipv4Addr::from(ip)
}

fn read_line_trimmed(prompt: &str) -> Result<String, SweepError> {
    use std::io::Write;
    print!("{prompt}");
    std::io::stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), SweepError> {
    let base_ip_str = read_line_trimmed("Enter base IP (e.g., 192.168.1.0): ")?;
    let mask_str = read_line_trimmed("Enter subnet mask (e.g., 24 for /24): ")?;

    let base_ip = parse_ip(&base_ip_str)?;
    let prefix = parse_mask(&mask_str)?;
    let (network, num_hosts) = subnet_range(base_ip, prefix);

    println!(
        "Scanning {} ({num_hosts} host{})...\n",
        format!("{}/{prefix}", ip_from_u32(network)),
        if num_hosts == 1 { "" } else { "s" }
    );

    for i in 1..=num_hosts {
        let host_ip = ip_from_u32(network + i);
        let host_str = host_ip.to_string();
        let up = ping_host(host_str.clone()).await;

        if up {
            println!("Host {} is up.", host_str);
        } else {
            println!("Host {} is unreachable.", host_str);
        }
    }

    Ok(())
}

async fn ping_host(ip: String) -> bool {
    task::spawn_blocking(move || {
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let addr: Ipv4Addr = match ip.parse() {
                Ok(a) => a,
                Err(_) => {
                    tx.send(false).ok();
                    return;
                }
            };
            match ping(std::net::IpAddr::V4(addr), None) {
                Ok(receiver) => {
                    for msg in receiver {
                        if let PingResult::Pong(..) = msg {
                            tx.send(true).ok();
                            return;
                        }
                    }
                    tx.send(false).ok();
                }
                Err(_) => {
                    tx.send(false).ok();
                }
            }
        });

        rx.recv_timeout(std::time::Duration::from_secs(2))
            .unwrap_or(false)
    })
    .await
    .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- parse_ip ----------------------------------------------------------

    #[test]
    fn parse_ip_valid() {
        assert_eq!(parse_ip("192.168.1.0").unwrap(), 0xC0A80100);
    }

    #[test]
    fn parse_ip_loopback() {
        assert_eq!(parse_ip("127.0.0.1").unwrap(), 0x7F000001);
    }

    #[test]
    fn parse_ip_invalid_octet() {
        assert!(parse_ip("256.1.1.1").is_err());
    }

    #[test]
    fn parse_ip_garbage() {
        assert!(parse_ip("not-an-ip").is_err());
    }

    #[test]
    fn parse_ip_empty() {
        assert!(parse_ip("").is_err());
    }

    // -- parse_mask --------------------------------------------------------

    #[test]
    fn parse_mask_valid_24() {
        assert_eq!(parse_mask("24").unwrap(), 24);
    }

    #[test]
    fn parse_mask_valid_0() {
        assert_eq!(parse_mask("0").unwrap(), 0);
    }

    #[test]
    fn parse_mask_valid_32() {
        assert_eq!(parse_mask("32").unwrap(), 32);
    }

    #[test]
    fn parse_mask_too_large() {
        assert!(parse_mask("33").is_err());
    }

    #[test]
    fn parse_mask_negative() {
        assert!(parse_mask("-1").is_err());
    }

    #[test]
    fn parse_mask_non_numeric() {
        assert!(parse_mask("abc").is_err());
    }

    // -- subnet_range ------------------------------------------------------

    #[test]
    fn subnet_range_slash_24() {
        let ip = parse_ip("192.168.1.0").unwrap();
        let (net, hosts) = subnet_range(ip, 24);
        assert_eq!(net, 0xC0A80100);
        assert_eq!(hosts, 254); // .1 through .254
    }

    #[test]
    fn subnet_range_slash_24_non_zero_host() {
        // Even if the user enters .50, the network address is still .0
        let ip = parse_ip("192.168.1.50").unwrap();
        let (net, hosts) = subnet_range(ip, 24);
        assert_eq!(ip_from_u32(net), Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(hosts, 254);
    }

    #[test]
    fn subnet_range_slash_25() {
        let ip = parse_ip("10.0.0.0").unwrap();
        let (net, hosts) = subnet_range(ip, 25);
        assert_eq!(ip_from_u32(net), Ipv4Addr::new(10, 0, 0, 0));
        assert_eq!(hosts, 126); // .1 through .126
    }

    #[test]
    fn subnet_range_slash_25_upper_half() {
        let ip = parse_ip("10.0.0.200").unwrap();
        let (net, hosts) = subnet_range(ip, 25);
        assert_eq!(ip_from_u32(net), Ipv4Addr::new(10, 0, 0, 128));
        assert_eq!(hosts, 126);
    }

    #[test]
    fn subnet_range_slash_30() {
        let ip = parse_ip("172.16.0.4").unwrap();
        let (net, hosts) = subnet_range(ip, 30);
        assert_eq!(ip_from_u32(net), Ipv4Addr::new(172, 16, 0, 4));
        assert_eq!(hosts, 2); // .5 and .6
    }

    #[test]
    fn subnet_range_slash_31() {
        let ip = parse_ip("10.1.1.0").unwrap();
        let (net, hosts) = subnet_range(ip, 31);
        assert_eq!(hosts, 2); // point-to-point
    }

    #[test]
    fn subnet_range_slash_32() {
        let ip = parse_ip("10.1.1.5").unwrap();
        let (net, hosts) = subnet_range(ip, 32);
        assert_eq!(net, ip);
        assert_eq!(hosts, 1);
    }

    #[test]
    fn subnet_range_slash_16() {
        let ip = parse_ip("172.16.0.0").unwrap();
        let (net, hosts) = subnet_range(ip, 16);
        assert_eq!(ip_from_u32(net), Ipv4Addr::new(172, 16, 0, 0));
        assert_eq!(hosts, 65534);
    }

    #[test]
    fn subnet_range_slash_8() {
        let ip = parse_ip("10.0.0.0").unwrap();
        let (_net, hosts) = subnet_range(ip, 8);
        assert_eq!(hosts, 16_777_214);
    }

    // -- ip_from_u32 -------------------------------------------------------

    #[test]
    fn ip_roundtrip() {
        let original = "192.168.100.55";
        let num = parse_ip(original).unwrap();
        assert_eq!(ip_from_u32(num).to_string(), original);
    }

    // -- host iteration correctness ----------------------------------------

    #[test]
    fn first_and_last_host_slash_24() {
        let ip = parse_ip("192.168.1.0").unwrap();
        let (net, hosts) = subnet_range(ip, 24);
        let first = ip_from_u32(net + 1);
        let last = ip_from_u32(net + hosts);
        assert_eq!(first, Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(last, Ipv4Addr::new(192, 168, 1, 254));
    }

    #[test]
    fn first_and_last_host_slash_25() {
        let ip = parse_ip("10.0.0.128").unwrap();
        let (net, hosts) = subnet_range(ip, 25);
        let first = ip_from_u32(net + 1);
        let last = ip_from_u32(net + hosts);
        assert_eq!(first, Ipv4Addr::new(10, 0, 0, 129));
        assert_eq!(last, Ipv4Addr::new(10, 0, 0, 254));
    }

    #[test]
    fn first_and_last_host_slash_26() {
        let ip = parse_ip("10.0.0.64").unwrap();
        let (net, hosts) = subnet_range(ip, 26);
        let first = ip_from_u32(net + 1);
        let last = ip_from_u32(net + hosts);
        assert_eq!(first, Ipv4Addr::new(10, 0, 0, 65));
        assert_eq!(last, Ipv4Addr::new(10, 0, 0, 126));
    }
}
