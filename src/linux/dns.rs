// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use trust_dns_resolver::{
    TokioAsyncResolver,
    name_server::{GenericConnector, TokioRuntimeProvider},
};
use std::time::Instant;

pub struct DnsResult {
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
    pub lookup_ms: u128,
}

pub async fn resolve_domain(domain: &str) -> Result<DnsResult, String> {
    // Build the required connector
    let connector = GenericConnector::new(TokioRuntimeProvider::default());

    // Correct constructor for trust-dns-resolver
    let resolver = TokioAsyncResolver::from_system_conf(connector)
        .map_err(|e| format!("Resolver init failed: {}", e))?;

    let start = Instant::now();

    // A records
    let ipv4_lookup = resolver.ipv4_lookup(domain).await;
    let ipv4: Vec<String> = match ipv4_lookup {
        Ok(lookup) => lookup.iter().map(|ip| ip.to_string()).collect(),
        Err(_) => vec![],
    };

    // AAAA records
    let ipv6_lookup = resolver.ipv6_lookup(domain).await;
    let ipv6: Vec<String> = match ipv6_lookup {
        Ok(lookup) => lookup.iter().map(|ip| ip.to_string()).collect(),
        Err(_) => vec![],
    };

    let elapsed = start.elapsed().as_millis();

    Ok(DnsResult {
        ipv4,
        ipv6,
        lookup_ms: elapsed,
    })
}