// StaxPing — Unified Network Diagnostics
// Copyright (c) 2026 StaxDash
//
// This source code is provided under the StaxPing Source‑Available License & EULA.
// You may view, modify, and redistribute this code for personal or internal use.
// Commercial use of any kind requires explicit written permission from StaxDash.
//
// Full license text available in LICENSE and EULA.md.

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub eula: bool,
    pub os: String,
    pub supports_icmp: bool,
    pub supports_trace: bool,
    pub supports_dns: bool,
    pub supports_http: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            eula: false,
            os: String::from("unknown"),
            supports_icmp: false,
            supports_trace: false,
            supports_dns: false,
            supports_http: false,
        }
    }
}

impl Config {
    /// Returns the full path to the config file based on OS.
    pub fn path() -> PathBuf {
        if cfg!(target_os = "linux") {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(format!("{}/.config/staxping/config.json", home))
        } else if cfg!(target_os = "windows") {
            let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
            PathBuf::from(format!("{}/StaxPing/config.json", appdata))
        } else if cfg!(target_os = "macos") {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            PathBuf::from(format!("{}/Library/Application Support/staxping/config.json", home))
        } else {
            PathBuf::from("config.json")
        }
    }

    /// Checks if the config file exists.
    pub fn exists() -> bool {
        Config::path().exists()
    }

    /// Loads the config file if it exists.
    pub fn load() -> Option<Self> {
        let path = Config::path();
        if !path.exists() {
            return None;
        }

        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    /// Saves the config to disk.
    pub fn save(&self) -> std::io::Result<()> {
        let path = Config::path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    /// Determines if first-run logic is needed.
    pub fn needs_first_run(&self) -> bool {
        !self.eula
            || self.os == "unknown"
            || !self.supports_icmp
            || !self.supports_dns
            || !self.supports_http
    }

    /// Creates a new config after first-run setup.
    pub fn new_after_first_run(
        os: &str,
        icmp: bool,
        trace: bool,
        dns: bool,
        http: bool,
    ) -> Self {
        Self {
            eula: true,
            os: os.to_string(),
            supports_icmp: icmp,
            supports_trace: trace,
            supports_dns: dns,
            supports_http: http,
        }
    }
}