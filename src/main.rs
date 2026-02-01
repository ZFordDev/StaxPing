// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

mod config;
mod first_run;
mod dns;
mod ping;
mod http;
mod trace;

use clap::Parser;
use config::Config;

/// Simple aligned key/value printer
fn kv(label: &str, value: impl std::fmt::Display) {
    println!("  {:<12} {}", label, value);
}

/// CLI argument structure
#[derive(Parser, Debug)]
#[command(
    name = "StaxPing",
    version = "0.1.0",
    about = "A clean, unified network diagnostic tool by StaxDash.",
    long_about = "StaxPing performs DNS lookup, ICMP ping, HTTP checks, and optional traceroute.\n\
It provides a clean, unified interface for quick network diagnostics.",
    override_usage = "staxping [TARGET] [OPTIONS]",
    help_template = "
{name} v{version}
{about}

USAGE:
  {usage}

ARGS:
{positionals}

OPTIONS:
{options}
"
)]
struct Cli {
    /// The target domain or IP to test
    target: Option<String>,

    /// Enable traceroute
    #[arg(long)]
    trace: bool,

    /// Advanced mode (future)
    #[arg(short = 'A', long)]
    advanced: bool,
}

#[tokio::main]
async fn main() {
    // Load config
    let config = Config::load();

    match config {
        Some(cfg) => {
            if cfg.needs_first_run() {
                first_run::run_first_run();
                return;
            }
        }
        None => {
            first_run::run_first_run();
            return;
        }
    }

    // Parse CLI arguments
    let cli = Cli::parse();

    // If no target provided -> show friendly hint
    if cli.target.is_none() {
        println!("StaxPing needs a target to run diagnostics.\n");
        println!("Try:");
        println!("  staxping example.com");
        println!("  staxping example.com --trace");
        println!("  staxping --help");
        return;
    }

    // Extract target
    let target = cli.target.unwrap();

    // Top-level banner
    println!("========================================");
    println!("  StaxPing v0.1.0 — Network Diagnostics");
    println!("  Target: {}", target);
    println!("========================================\n");

    // DNS Section
    println!("=== DNS ===============================");

    let dns_result = match dns::resolve_domain(&target).await {
        Ok(result) => {
            if !result.ipv4.is_empty() {
                kv("IPv4:", format!("{:?}", result.ipv4));
            }
            if !result.ipv6.is_empty() {
                kv("IPv6:", format!("{:?}", result.ipv6));
            }
            kv("Lookup:", format!("{} ms", result.lookup_ms));
            result
        }
        Err(e) => {
            kv("DNS error:", e);
            return;
        }
    };

    // Use the first IPv4 address for ping
    let ping_ip = if !dns_result.ipv4.is_empty() {
        dns_result.ipv4[0].clone()
    } else if !dns_result.ipv6.is_empty() {
        dns_result.ipv6[0].clone()
    } else {
        println!("No valid IPs found for ping.");
        return;
    };

    // Ping Section
    println!("\n=== Ping ==============================");

    match ping::run_ping(&ping_ip).await {
        Ok(result) => {
            kv("Sent:", result.sent);
            kv("Received:", result.received);
            kv("Loss:", format!("{:.1}%", result.loss));
            kv("Min:", format!("{:.2} ms", result.min_ms));
            kv("Avg:", format!("{:.2} ms", result.avg_ms));
            kv("Max:", format!("{:.2} ms", result.max_ms));
        }
        Err(e) => {
            kv("Ping error:", e);
        }
    }

    // HTTP Section
    println!("\n=== HTTP ==============================");

    match http::check_http(&target).await {
        Ok(result) => {
            kv("Status:", result.status);
            kv("Time:", format!("{} ms", result.time_ms));
            kv("Final URL:", result.final_url);
        }
        Err(e) => {
            kv("HTTP error:", e);
        }
    }

    // Traceroute Section
    if cli.trace {
        println!("\n=== Traceroute ========================");

        // Use the first IPv4 for traceroute
        let trace_ip = if !dns_result.ipv4.is_empty() {
            dns_result.ipv4[0].clone()
        } else if !dns_result.ipv6.is_empty() {
            dns_result.ipv6[0].clone()
        } else {
            println!("No valid IPs found for traceroute.");
            return;
        };

        match trace::run_trace(&trace_ip).await {
            Ok(result) => {
                for hop in result.hops {
                    let times: Vec<String> = hop.times_ms.iter()
                        .map(|t| format!("{:.2} ms", t))
                        .collect();

                    println!(
                        "  {:>2}  {:<15}  {}",
                        hop.hop,
                        hop.ip,
                        times.join("  ")
                    );
                }
            }
            Err(e) => {
                kv("Trace error:", e);
            }
        }
    }

    if cli.advanced {
        println!("\n(advanced mode enabled)");
    }
}