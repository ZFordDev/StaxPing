# **StaxPing**  
### **a product of StaxDash | life made simple**

StaxPing is a lightweight, cross‑platform network diagnostic tool designed to replace the messy, inconsistent mix of `ping`, `dig`, `traceroute`, and `curl` with one clean, predictable command.

Built in Rust for speed, safety, and portability — StaxPing provides a unified view of DNS resolution, ICMP latency, HTTP health, and optional hop‑by‑hop routing, all with clean, consistent output.

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
StaxPing will be available as a `.deb` package and via APT:

```
sudo apt install staxping
```

(Coming soon — currently under active development.)

### **Windows (Secondary Target)**  
A standalone `.exe` will be provided once Linux packaging is complete.

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

Advanced flags and extended diagnostics will follow.

---

## **Part of the StaxDash Tool Suite**

StaxPing is one of the official StaxDash micro‑tools — a collection of small, fast, reliable utilities designed to make life simpler across Linux, Windows, and Docker environments.

Learn more at:  
**https://staxdash.com**

---