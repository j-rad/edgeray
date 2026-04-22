# EdgeRay Official User Manual 📖

Welcome to the definitive guide for **EdgeRay**! Whether you are a system administrator deploying secure server nodes or an end-user seeking unrestricted internet access against total network blockages, this manual will guide you through setting up, configuring, and utilizing EdgeRay's powerful, multi-transport censorship circumvention engine.

## Table of Contents

1. [Understanding EdgeRay](#1-understanding-edgeray)
2. [Installation & Setup](#2-installation--setup)
3. [The EdgeRay Desktop Client](#3-the-edgeray-desktop-client)
4. [Deploying a Node (`rr-ui` Server Mode)](#4-deploying-a-node-rr-ui-server-mode)
5. [Advanced Evasion: Flow-J & eBPF](#5-advanced-evasion-flow-j--ebpf)
6. [Resilient Transports: QUIC & P2P](#6-resilient-transports-quic--p2p)
7. [Configuring the RustRay Core Manually](#7-configuring-the-rustray-core-manually)
8. [Troubleshooting & Logs](#8-troubleshooting--logs)

---

## 1. Understanding EdgeRay

**EdgeRay** is an ecosystem built natively in memory-safe Rust, consisting of three primary modules:

*   **`rustray` (The Core Engine):** The invisible, high-performance network engine. It handles all encryption, evasion protocols (Flow-J, Brutal-QUIC, SIP003, P2P), routing decisions, and the self-healing Fallback Orchestrator. Extensively bypasses Deep Packet Inspection (DPI).
*   **EdgeRay Desktop (`edgeray-app`):** The stunning graphical interface you install on your computer (Windows, macOS, Linux). Powered by Dioxus and Tauri, offering zero-latency Direct-to-Display metric rendering.
*   **`rr-ui` (The Server Panel):** A web-based Actix+Svelte dashboard installed on your remote server (e.g., VPS) to manage user accounts, inbound routing rules, and auto-deploy legacy/modern proxy instances.

---

## 2. Installation & Setup

### For End-Users (Desktop Client)

The easiest way to use EdgeRay is through the official desktop application:

1.  Navigate to the **Releases** page of the EdgeRay repository.
2.  Download the package for your operating system (`.exe`, `.dmg`, `.deb`, or `.AppImage`).
3.  **Important for Linux Users:** Because EdgeRay sets up a virtual `TUN` adapter for full-device VPN support via a custom `smoltcp` stack, the app must be run with administrative (`sudo`) privileges.
4.  Launch the app and paste your `edgeray://` connection link (provided by your server administrator).

### For Node Operators (Server Panel)

If you are setting up your own secure proxy server:

1.  SSH into your remote Ubuntu/Debian server.
2.  Bring up the EdgeRay `rr-ui` pre-compiled server daemon:
    ```bash
    ./rr-ui run --port 54321
    ```
3.  Navigate to `http://<your-server-ip>:54321` in your browser. Follow the setup wizard to connect the local `rustray` backend. The UI provides a one-click deployment for new protocols.

---

## 3. The EdgeRay Desktop Client

EdgeRay's **Obsidian Design System** brings a dark-mode, glassmorphic UI that offers both extreme simplicity and deep technical observability.

*   **The Power Button:** The large center ring toggle connects or disconnects the secure tunnel. 
*   **Telemetry Dashboard:** Clicking the "Monitor" icon opens the real-time panel. Powered by gRPC and a self-healing UNIX socket manager, you get live, millisecond-accurate latency, throughput graphs, and P2P path resolution data.
*   **Global Kill Switch:** An atomic flag in the core guarantees zero data leakage if the engine drops.
*   **Routing Mode:**
    *   *Global:* Routes 100% of your computer's internet traffic through `tun0`.
    *   *Smart (Geo/Rule):* Automatically limits the proxy tunnel to restricted sites while using your direct internet for domestic traffic to maximize throughput.

---

## 4. Deploying a Node (`rr-ui` Server Mode)

In the `rr-ui` panel, navigate to "Inbounds" to spin up listening protocols on your server. 

### Protocol Selection Guide

*   **Flow-J:** Choose this if you face **extreme censorship** (e.g., blanket UDP blocking or TLS fingerprinting). Flow-J can steganographically hide your traffic in HTTP/HTTPS, or as fake MQTT IoT sensor data.
*   **Hysteria 2 / QUIC:** Select this for raw speed. Enhanced with the **Brutal-QUIC** fixed-rate congestion controller, it rams packets through lossy networks without waiting for traditional TCP window scaling. 
*   **P2P Relays:** Create a peer ring using BLAKE3 authentication. If the main server IP is blocked in a specific country, a client automatically connects through unblocked peers.
*   **SIP003 Plugin Mode:** Need to connect to older infrastructure? Spin up a Shadowsocks inbound that supports transparent UDP-over-TCP via legacy SIP003 wrappers.

---

## 5. Advanced Evasion: Flow-J & eBPF

EdgeRay goes far beyond standard encryption.

### The eBPF Handshake Mutilator
EdgeRay utilizes Linux kernel-level hooks via `aya` to physically slice TLS ClientHello handshakes apart across multiple packets. Before traffic leaves your computer, the eBPF module mutilates predictable packet sizes, completely shattering DPI systems trying to recognize the signature of VPN usage.

### Flow-J Modes
*   **Mode A (REALITY):** Makes your traffic look precisely like an authenticated session to a highly trusted corporate website (e.g., Apple or Microsoft). Unauthorized DPI probes are harmlessly forwarded to the real corporate server.
*   **Mode B (CDN Relay):** Utilizes `xhttp` semantics to hide behind Cloudflare/AWS Edge caches.
*   **Mode C (IoT Steganography):** Wraps traffic payloads into fake JSON telemetry meant for Smart Homes/Industrial control panels, operating under the radar of enterprise IP whitelists.

### App-Layer Desync
Instead of letting normal TCP streams run predictably, EdgeRay dynamically injects fragmented window frames to desynchronize stateful firewall trackers analyzing the stream.

---

## 6. Resilient Transports: QUIC & P2P

When networks don't just block, but *throttle* your traffic, EdgeRay utilizes next-generation resilience mechanisms.

*   **Elastic FEC (Forward Error Correction):** Using Reed-Solomon parity math, EdgeRay embeds "repair packets" alongside your real data. If an ISP drops 50% of your packets randomly, the engine uses the repair data to rebuild the missing packets on the fly—meaning ZERO speed loss or round-trip retries required.
*   **The Fallback Orchestrator:** EdgeRay continuously health-checks your current routing node in the background. If the government blocks your current transport, the Orchestrator instantly hot-swaps your connection to a backup protocol or a **P2P Relay** pathway before your video stream even stutters. 

---

## 7. Configuring the RustRay Core Manually

For advanced users and CI pipelines, the headless `rustray` engine can run via a `config.json` that fully supports both legacy layouts and the new orchestration syntax.

Example `config.json` featuring the Orchestrator and P2P relay:
```json
{
  "log": { "loglevel": "info" },
  "inbounds": [{
    "port": 1080,
    "listen": "127.0.0.1",
    "protocol": "socks",
    "settings": { "auth": "noauth", "udp": true }
  }],
  "outbounds": [{
    "protocol": "flow-j",
    "tag": "primary-reality",
    "settings": {
      "mode": "auto",
      "uuid": "...",
      "fec": { "enabled": true, "data_shards": 10, "parity_shards": 3 },
      "reality": {
        "dest": "www.microsoft.com:443",
        "server_names": ["www.microsoft.com"]
      }
    }
  }],
  "orchestrator": {
    "enabled": true,
    "probe_interval_ms": 2500,
    "fallback_chain": ["primary-reality", "backup-p2p"]
  }
}
```
Run the headless core in a terminal:
```bash
./rustray -c config.json
```

---

## 8. Troubleshooting & Logs

**Q: I hit connect, but no pages will load, yet the core says 'connected'.**
*   *Solution:* Verify system time! REALITY and QUIC strictly require your system clock to be accurate to within 60 seconds of real-world time to defeat replay attacks. Update your NTP syncer.

**Q: The app crashes on Linux when clicking Connect.**
*   *Solution:* Do you have the TUN drivers installed on your distribution? Ensure `sudo` was used to allow EdgeRay to allocate the `tun0` virtual loop interfaces.

**Q: The P2P relay path is showing extremely high latency.**
*   *Solution:* P2P NAT traversal bounces between residential peers. Ensure your primary outbounds are properly prioritized in the Fallback Orchestrator, and that the P2P swarm you are connected to isn't on the opposite side of the globe.

**Q: How do I export diagnostic logs for bug reports?**
*   *Solution:* In the Desktop App, navigate to **Settings > Diagnostics > Export Triage Archive**. This produces a sanitized `.zip` file of telemetry and routing metrics you can upload to our GitHub Issues tracker without compromising your private keys.
