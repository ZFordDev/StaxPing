// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct TraceHop {
    pub hop: u32,
    pub host: String,
    pub ip: String,
    pub times_ms: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct TraceResult {
    pub hops: Vec<TraceHop>,
}

pub async fn run_trace(target: &str) -> Result<TraceResult, String> {
    // Try system traceroute first
    let traceroute_path = if std::path::Path::new("/usr/bin/traceroute").exists() {
        "/usr/bin/traceroute"
    } else if std::path::Path::new("/bin/traceroute").exists() {
        "/bin/traceroute"
    } else {
        // Fallback to pure Rust traceroute
        return run_trace_fallback(target).await;
    };

    let output = Command::new(traceroute_path)
        .arg("-n")
        .arg("-w")
        .arg("2")
        .arg(target)
        .output()
        .await
        .map_err(|e| format!("Failed to run traceroute: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    parse_traceroute(&stdout)
}

fn parse_traceroute(output: &str) -> Result<TraceResult, String> {
    let mut hops = Vec::new();

    for line in output.lines() {
        // Skip header line
        if line.starts_with("traceroute") {
            continue;
        }

        // Example hop line:
        // 1  192.168.1.1  1.234 ms  1.567 ms  1.890 ms
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        let hop_num = parts[0].parse::<u32>().unwrap_or(0);
        let ip = parts[1].to_string();

        let mut times = Vec::new();
        for part in parts.iter().skip(2) {
            if part.ends_with("ms") {
                let clean = part.replace("ms", "");
                if let Ok(val) = clean.parse::<f64>() {
                    times.push(val);
                }
            }
        }

        hops.push(TraceHop {
            hop: hop_num,
            host: ip.clone(),
            ip,
            times_ms: times,
        });
    }

    Ok(TraceResult { hops })
}

use socket2::{Socket, Domain, Type, Protocol};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;
use tokio::time::{timeout, Duration};

async fn run_trace_fallback(target: &str) -> Result<TraceResult, String> {
    let dest_ip: IpAddr = target
        .parse()
        .map_err(|_| format!("Invalid IP address for traceroute: {}", target))?;

    let max_hops = 30;
    let timeout_ms = 2000;

    let mut hops = Vec::new();

    for ttl in 1..=max_hops {
        let hop = trace_single_hop(dest_ip, ttl, timeout_ms).await?;

        // detects blocked ICMP responses
        if ttl == 1 && hop.ip == "*" {
            return Err("Traceroute requires elevated privileges in this environment".into());
        }

        hops.push(hop.clone());

        if hop.ip == dest_ip.to_string() {
            break;
        }
    }

    Ok(TraceResult { hops })
}

async fn trace_single_hop(dest: IpAddr, ttl: u32, timeout_ms: u64) -> Result<TraceHop, String> {
    let udp_socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
        .map_err(|e| format!("Socket error: {}", e))?;

    udp_socket.set_ttl(ttl)
        .map_err(|e| format!("Failed to set TTL: {}", e))?;

    let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
    udp_socket.bind(&local_addr.into())
        .map_err(|e| format!("Bind error: {}", e))?;

    let dest_addr = SocketAddr::new(dest, 33434 + ttl as u16);

    let start = Instant::now();
    udp_socket.send_to(&[0u8], &dest_addr.into())
        .map_err(|e| format!("Send error: {}", e))?;

    // Allocate MaybeUninit buffer on heap
    let mut buf = Box::new([std::mem::MaybeUninit::<u8>::uninit(); 512]);

    let socket_clone = udp_socket.try_clone()
        .map_err(|e| format!("Socket clone error: {}", e))?;

    let recv_future = tokio::task::spawn_blocking(move || {
        socket_clone.recv_from(&mut *buf)
    });

    let result = timeout(Duration::from_millis(timeout_ms), recv_future).await;

    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    let (ip, times_ms) = match result {
        Ok(Ok(Ok((_size, addr)))) => {
            let ip = addr.as_socket().unwrap().ip().to_string();
            (ip, vec![elapsed])
        }
        _ => ("*".into(), vec![elapsed]),
    };

    Ok(TraceHop {
        hop: ttl,
        host: ip.clone(),
        ip,
        times_ms,
    })
}