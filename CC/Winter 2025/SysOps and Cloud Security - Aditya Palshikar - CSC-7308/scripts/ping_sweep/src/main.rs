use pinger::{PingResult, ping};
use std::sync::mpsc;
use tokio::task;
use std::io::{self, Write};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    print!("Enter base IP (e.g., 192.168.1.0): ");
    io::stdout().flush().unwrap();
    let mut base_ip = String::new();
    io::stdin().read_line(&mut base_ip).unwrap();
    let base_ip = base_ip.trim();

    print!("Enter subnet mask (e.g., 24 for /24): ");
    io::stdout().flush().unwrap();
    let mut subnet_mask = String::new();
    io::stdin().read_line(&mut subnet_mask).unwrap();
    let subnet_mask: u8 = subnet_mask.trim().parse().unwrap();

    let ip_parts: Vec<u8> = base_ip
        .split('.')
        .map(|part| part.parse().unwrap())
        .collect();

    let num_hosts = 2u32.pow((32 - subnet_mask) as u32) - 1;

    for i in 1..=num_hosts {
        let mut ip_addr = ip_parts.clone();
        let host_part = i.to_be_bytes();

        let host_bits = 32 - subnet_mask;
        for j in 0..(host_bits as usize / 8) {
            ip_addr[4 - j - 1] = host_part[4 - j - 1];
        }

        let ip = Ipv4Addr::new(ip_addr[0], ip_addr[1], ip_addr[2], ip_addr[3]).to_string();
        let ping_result = ping_host(ip.clone()).await;

        if ping_result {
            println!("\nHost {} is up.", ip);
        } else {
            println!("Host {} is unreachable.", ip);
        }
    }
}

async fn ping_host(ip: String) -> bool {
    let result = task::spawn_blocking(move || {
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            match ping(ip.parse().unwrap(), None) {
                Ok(receiver) => {
                    for msg in receiver {
                        match msg {
                            PingResult::Pong(..) => {
                                tx.send(true).expect("Failed to send ping result");
                                return;
                            }
                            _ => continue,
                        }
                    }
                    tx.send(false).ok();
                }
                Err(_) => {
                    tx.send(false).ok();
                }
            }
        });

        rx.recv_timeout(std::time::Duration::from_secs(2)).unwrap_or(false)
    })
    .await
    .unwrap_or(false);

    result
}
