// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use reqwest::Client;
use std::time::Instant;

pub struct HttpResult {
    pub status: u16,
    pub time_ms: u128,
    pub final_url: String,
}

pub async fn check_http(target: &str) -> Result<HttpResult, String> {
    let url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("https://{}", target)
    };

    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .map_err(|e| format!("HTTP client init failed: {}", e))?;

    let start = Instant::now();

    let resp = client
        .head(&url)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let elapsed = start.elapsed().as_millis();
    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();

    Ok(HttpResult {
        status,
        time_ms: elapsed,
        final_url,
    })
}