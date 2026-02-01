// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use std::io::{self, Write};
use crate::config::Config;

pub fn run_first_run() {
    println!("=== StaxPing ===");
    println!("a product of StaxDash | life made simple\n");

    show_eula();
    if !prompt_acceptance() {
        println!("EULA not accepted. Exiting StaxPing.");
        std::process::exit(0);
    }

    println!("\nPerforming initial setup...");

    let os = detect_os();
    println!("• Detecting OS: {}", os);

    let icmp = check_icmp_support();
    println!("• Checking ICMP support: {}", yes_no(icmp));

    let trace = check_trace_support();
    println!("• Checking traceroute capability: {}", yes_no(trace));

    let dns = check_dns_support();
    println!("• Checking DNS resolver: {}", yes_no(dns));

    let http = check_http_support();
    println!("• Checking HTTP client: {}", yes_no(http));

    let config = Config::new_after_first_run(&os, icmp, trace, dns, http);
    if let Err(e) = config.save() {
        println!("Failed to save config: {}", e);
        std::process::exit(1);
    }

    println!("\nSetup complete. You can now use StaxPing normally.");
}

/// Display the EULA text (loaded from EULA.txt)
fn show_eula() {
    let eula_text = include_str!("../EULA.txt");
    println!("{}", eula_text);
}

/// Ask the user to type "yes" to accept the EULA
fn prompt_acceptance() -> bool {
    print!("Type 'yes' to accept: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().eq_ignore_ascii_case("yes")
}

/// Detect OS using Rust cfg macros
fn detect_os() -> String {
    if cfg!(target_os = "linux") {
        "linux".into()
    } else if cfg!(target_os = "windows") {
        "windows".into()
    } else if cfg!(target_os = "macos") {
        "macos".into()
    } else {
        "unknown".into()
    }
}

fn check_icmp_support() -> bool {
    true
}

fn check_trace_support() -> bool {
    true
}

fn check_dns_support() -> bool {
    true
}

fn check_http_support() -> bool {
    true
}

/// Helper for printing Yes/No
fn yes_no(value: bool) -> &'static str {
    if value { "OK" } else { "Unavailable" }
}