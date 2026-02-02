[![Website](https://img.shields.io/badge/Website-zford.dev-000000?style=flat-square)](https://zford.dev)
[![Store](https://img.shields.io/badge/Store-staxdash.com-4CAF50?style=flat-square)](https://staxdash.com)
[![Ko‑Fi](https://img.shields.io/badge/Support-KoFi-FF5E5B?style=flat-square)](https://ko-fi.com/zforddev)

---

# **StaxPing**  
### **a product of StaxDash | life made simple**

StaxPing is a lightweight, cross‑platform network diagnostic tool designed to replace the messy, inconsistent mix of `ping`, `dig`, `traceroute`, and `curl` with one clean, predictable command.

Built in Rust for speed, safety, and portability — StaxPing provides a unified view of DNS resolution, ICMP latency, HTTP health, and optional hop‑by‑hop routing, all with clean, consistent output.

---

> This project has been flagged for a major upgrade — the potential here is huge, and it’s getting priority attention.
> — ZFordDev

---

## **Features**

- **DNS Resolution**  
  Fast, clean lookup with IPv4/IPv6 results and timing.

- **ICMP Ping**  
  Min/avg/max latency, packet loss, and jitter‑friendly timing.

- **HTTP Health Check**  
  Status code, response time, and final URL after redirects.

- **Optional Traceroute**  
  Hop‑by‑hop routing with aligned, readable output.

- **First‑Run Setup**  
  - EULA acceptance  
  - OS detection  
  - Capability checks (DNS, ICMP, HTTP, traceroute)  
  - Config stored in the user’s home directory  

After the first run, StaxPing works instantly with no prompts.

---

## **Usage**

Basic connectivity check:

```
staxping google.com
```

Include traceroute:

```
staxping google.com --trace
```

Advanced mode (reserved for future features):

```
staxping google.com --advanced
```

Help:

```
staxping --help
```

---

## **Installation**

### **Linux (Primary Target)**  
A `.deb` package is available for Debian/Ubuntu‑based systems.

Download the latest release:

```
wget https://github.com/ZFordDev/StaxPing/releases/download/V0.1.0/staxping_0.1.0_amd64.deb
sudo dpkg -i staxping_0.1.0_amd64.deb
```

APT repository support is planned for a future milestone.

### **Windows (Secondary Target)**  
A standalone `.exe` will be provided once Linux packaging is fully stabilized.

---

## **Config & First‑Run Behavior**

On first run, StaxPing will:

1. Display the EULA  
2. Detect your OS  
3. Check system capabilities  
4. Store a small config file:

Linux:
```
$HOME/.config/staxping/config.json
```

Windows:
```
%APPDATA%\StaxPing\config.json
```

After that, StaxPing runs without prompts.

---

## **Built With**

- **Rust** — safety, speed, portability  
- `trust-dns-resolver` — DNS resolution  
- `surge-ping` — ICMP ping  
- `reqwest` — HTTP checks  
- `tracert` — traceroute  
- `clap` — CLI argument parsing  
- `serde` — config handling  

---

## **Project Status**

StaxPing is currently in **active development**.  
The current milestone includes:

- First‑run logic  
- Config system  
- Capability detection  
- DNS, ICMP, and HTTP modules  
- Optional traceroute with clean fallback messaging  
- Polished, aligned CLI output  
- Linux `.deb` packaging for amd64  

Future milestones will introduce:

- Advanced flags  
- Extended diagnostics  
- APT repository support  
- Windows `.exe` distribution  

---

## **License**

StaxPing is **source‑available and noncommercial**.  
You may view, modify, and redistribute the source code for personal or internal use.

Commercial use of any kind requires explicit written permission from StaxDash.

See `LICENSE` and `EULA.txt` for full terms.

---

## **Part of the StaxDash Tool Suite**

---

## Explore More

[**zford.dev**](https://zford.dev) — the projects that shape the platform.  
Not everything makes the cut, but everything matters.

[**staxdash.com**](https://staxdash.com) — the storefront for all tools, big and small.  
Clean, minimal, purpose‑built utilities.

**Ko‑Fi** — support the work and help fuel the dream:  
https://ko-fi.com/zforddev

---

