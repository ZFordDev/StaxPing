# StaxPing Classic (Archived)

StaxPing Classic is the original experimental version of **StaxPing**, built in Rust as a learning project focused on system‑level networking, diagnostics, and CLI design.

This repository is now **archived**.  
It remains available only for historical reference and for users interested in the early Rust experiments that led to later tooling.

---

## Project Background

StaxPing Classic was created as a personal exploration into:

- Rust networking primitives  
- ICMP, DNS, HTTP, and routing inspection  
- Cross‑platform CLI ergonomics  
- Capability checks and structured output formatting  

It served as a practical way to learn Rust, system calls, and low‑level diagnostics — but it was never intended to compete with established tools or provide meaningful advantages over existing alternatives.

---

## Why It’s Archived

This project is being retired because:

- It does not offer benefits over standard tools (`ping`, `dig`, `curl`, etc.)  
- The “all‑in‑one” approach proved too broad for long‑term maintainability  
- The ecosystem has since moved toward **smaller, focused utilities**  
- A new architecture is being developed under the **StaxDash** organization  

StaxPing Classic is frozen as a snapshot of early experimentation.

---

## What Remains Available

Although archived, the following still work:

- Historical releases  
- Linux `.deb` builds  
- Windows `.exe` builds  
- The original Rust source code  
- The classic configuration and onboarding flow  

These are preserved for anyone studying the original implementation or Rust CLI design patterns.

---

## Successor Projects

Development has moved to the **StaxDash** organization, where tools are being rebuilt with a cleaner, modular architecture:

- **Staxup2**
- **Staxinit**
- **Crafty-Snap** 

You can follow active development here:

👉 https://github.com/StaxDash

---

## Installation (Legacy)

### Linux (.deb)
```bash
wget https://github.com/ZFordDev/StaxPing/releases/download/V0.1.0/staxping_0.1.0_amd64.deb
sudo dpkg -i staxping_0.1.0_amd64.deb
```

### Windows (.exe)
Download from the Releases page.
PATH handling is automatic on first run.

## Historical Notes
StaxPing Classic included:
- DNS, ICMP, HTTP, and traceroute modules
- A simple onboarding EULA
- Capability checks
- A single‑binary distribution model
These features remain in the archived codebase for reference only.

## Author
Created by ZFordDev as a Rust learning project.
Thanks to everyone who tested early builds and provided feedback during its experimental phase.

## If You Found It Useful
Even though the project is archived:
- ⭐ Star the repo if it helped you learn Rust or networking
- ☕ Support the developer: https://ko-fi.com/zforddev
StaxPing Classic may be retired, but the lessons learned from it continue in the next‑generation StaxDash ecosystem.
