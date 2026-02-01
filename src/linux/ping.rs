// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::net::IpAddr;
use std::time::{Duration, Instant};
use rand::Rng;
use tokio::process::Command;

pub struct PingResult {
    pub sent: u32,
    pub received: u32,
    pub loss: f32,
    pub min_ms: f64,
    pub avg_ms: f64,
    pub max_ms: f64,
}

pub async fn run_ping(ip: &str) -> Result<PingResult, String> {
    // Try surge-ping first
    match run_raw_ping(ip).await {
        Ok(result) => return Ok(result),
        Err(e) => {
            if e.contains("Operation not permitted") {
                // Fall back to /bin/ping
                return run_fallback_ping(ip).await;
            } else {
                return Err(e);
            }
        }
    }
}

async fn run_raw_ping(ip: &str) -> Result<PingResult, String> {
    let ip: IpAddr = ip.parse().map_err(|_| format!("Invalid IP address: {}", ip))?;

    let client = Client::new(&Config::default())
        .map_err(|e| format!("Ping client init failed: {}", e))?;

    let identifier = PingIdentifier(rand::thread_rng().gen());
    let mut sequence = PingSequence(0);

    let mut pinger = client.pinger(ip, identifier).await;

    let mut times = Vec::new();
    let mut received = 0u32;
    let sent = 4u32;

    for _ in 0..sent {
        let start = Instant::now();

        let payload = b"staxping";
        let result = pinger.ping(sequence, payload).await;
        sequence.0 += 1;

        match result {
            Ok((_packet, _addr)) => {
                received += 1;
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                times.push(elapsed);
            }
            Err(_) => {}
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    Ok(calculate_stats(sent, received, times))
}

async fn run_fallback_ping(ip: &str) -> Result<PingResult, String> {
    let output = Command::new("/bin/ping")
        .arg("-c")
        .arg("4")
        .arg(ip)
        .output()
        .await
        .map_err(|e| format!("Failed to run /bin/ping: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    parse_ping_output(&stdout)
}

fn parse_ping_output(output: &str) -> Result<PingResult, String> {
    let mut sent = 4;
    let mut received = 0;
    let mut min = 0.0;
    let mut avg = 0.0;
    let mut max = 0.0;

    for line in output.lines() {
        if line.contains("packets transmitted") {
            // Example: "4 packets transmitted, 4 received, 0% packet loss"
            let parts: Vec<&str> = line.split(',').collect();
            sent = parts[0].trim().split(' ').next().unwrap().parse().unwrap_or(4);
            received = parts[1].trim().split(' ').next().unwrap().parse().unwrap_or(0);
        }

        if line.contains("min/avg/max") {
            // Example: "rtt min/avg/max/mdev = 12.345/14.567/16.789/0.123 ms"
            let stats = line.split('=').nth(1).unwrap().trim();
            let values: Vec<&str> = stats.split('/').collect();
            min = values[0].parse().unwrap_or(0.0);
            avg = values[1].parse().unwrap_or(0.0);
            max = values[2].parse().unwrap_or(0.0);
        }
    }

    let loss = ((sent - received) as f32 / sent as f32) * 100.0;

    Ok(PingResult {
        sent,
        received,
        loss,
        min_ms: min,
        avg_ms: avg,
        max_ms: max,
    })
}

fn calculate_stats(sent: u32, received: u32, times: Vec<f64>) -> PingResult {
    let loss = ((sent - received) as f32 / sent as f32) * 100.0;

    let (min_ms, avg_ms, max_ms) = if !times.is_empty() {
        let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg = times.iter().sum::<f64>() / times.len() as f64;
        (min, avg, max)
    } else {
        (0.0, 0.0, 0.0)
    };

    PingResult {
        sent,
        received,
        loss,
        min_ms,
        avg_ms,
        max_ms,
    }
}