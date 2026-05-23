# EdgeRay Workspace

**EdgeRay** is a next-generation, high-performance, cross-platform VPN client and universal proxy core built completely in Rust. Utilizing the Tauri framework and Dioxus for its graphical interface, and an advanced custom network engine based on `smoltcp`, EdgeRay provides unparalleled censorship circumvention and network reliability.

## 🚀 Project Status
**Phase 7 / 7 Finalized**: The architecture is 100% complete and compile-safe. We are now entering the **Stability & Hardening** phase.

## 🤝 Help Wanted: Path to Production
We are currently seeking contributors to help EdgeRay reach its **v1.0 Production Ready** milestone. Key areas for contribution include:
*   **Security Audits**: Protocol verification for `Flow-J`.
*   **Stress Testing**: DPI-resistant performance benchmarks.
*   **Mobile Development**: Finalizing Android/iOS native integrations.
*   **Documentation**: Improving technical guides and translations.

See our [Roadmap to v1.0](./docs/ROADMAP_TO_V1.md) for more details.

## 🌟 Comprehensive Features

### 1. Advanced Evasion & Multi-Transport Architecture
*   **eBPF Handshake Mutilator:** Kernel-level TLS ClientHello packet slicing using `aya` to bypass DPI algorithms.
*   **Flow-J Protocol:** A proprietary, dynamically shifting polyglot protocol with modes: Direct Stealth (REALITY), CDN Relay (xhttp), and IoT Camouflage (MQTT).
*   **Relay Domain Fronting:** Encapsulates TCP streams into HTTP requests targeting fronted relays (e.g., Google Apps Script) to bypass IP-level blocks.
*   **Statistical Traffic Shaping:** Markov-chain based `BehaviorSynthesizer` that mimics specific protocol patterns (Modbus/TCP, MQTT, HTTPS) to defeat statistical DPI.
*   **Elastic FEC (Forward Error Correction):** Reed-Solomon based parity packet injection to heal aggressively throttled UDP connections on the fly.
*   **P2P Decentralized Relays:** NAT traversal and decentralized routing via BLAKE3 authentication for instances where direct outbound IPs are blackholed.
*   **Decoy Mimicry & REALITY:** Advanced camouflage imitating standard TLS 1.3 browser handshakes, falling back to legitimate server content on unauthenticated probes.
*   **MITM Interception Core:** Integrated Man-in-the-Middle engine for transparent TLS termination and on-the-fly certificate generation for deep traffic analysis.
*   **SIP003 Interoperability:** Native plugin support to transparently proxy UDP over legacy TCP infrastructure.

### 2. High-Performance Networking Core
*   **Brutal-QUIC Congestion Control:** Custom congestion controller designed to maximize throughput over lossy and suppressed connections.
*   **Userspace TCP/IP Stack:** Built on `smoltcp` combined with the `tun-rs` cross-platform TUN interface, allowing true VPN encapsulation without OS-level kernel hooks, complete with legacy industrial device TCP mimicry.
*   **Zero-Copy Execution:** Utilizing `bytes`, kernel-level `splice()`, and lock-free concurrency to forward packets with microsecond latency.

### 3. Cross-Platform UI & Telemetry
*   **Obsidian Design System:** Gorgeous dark-mode, glassmorphic UI built in Dioxus and TailwindCSS.
*   **Direct-to-Display Scanout:** Custom compositor bypass implementation to achieve zero-latency UI rendering on supported Linux managers.
*   **Live Telemetry Dashboard:** Real-time routing visualization, connection health graphs, and network metrics driven by a self-healing gRPC UDS manager.
*   **Mobile-First Bindings:** Full UniFFI code generation for seamless native integration into Android (Kotlin) and iOS (Swift).

---

## 📂 Project Structure

- **[rustray](./rustray/README.md)**: The headless engine (Rust). Handles the core transport logic, evasions, routing, and the gRPC control plane.
- **[edgeray-app](./edgeray-app/README.md)**: The Desktop client UI built with Tauri and Dioxus.
- **[rr-ui](./rr-ui/README.md)**: Web-based panel and data models for managing EdgeRay and Xray-compatible cores on remote servers.
- **[rustray-lite](./rustray-lite/README.md)**: A lightweight version of the `rustray` core, designed for embedded systems with limited resources.
- **shared-types**: Shared protocol and configuration types limit overhead.

---

## 🛠️ Building & Prerequisites

### Desktop (Linux)

You need the development libraries for Tauri and Dioxus:

```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libcairo2-dev libpango1.0-dev libatk1.0-dev \
    libgdk-pixbuf2.0-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev \
    libwebkit2gtk-4.1-dev
```

**Note:** The application must be run with `sudo` (or as Administrator) to spin up the TUN interface.

```bash
sudo -E cargo tauri dev
```

### Android (Mobile Bindings)

Building mobile bindings requires the `uniffi` tool.

```bash
cargo install cargo-ndk
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -t x86_64-linux-android -o ../gen/android/jniLibs build --release
```

Generate Kotlin FFI bindings using UniFFI:
```bash
cargo run -p edgeray-core --bin uniffi-bindgen generate --library target/debug/libedgeray_core.so --language kotlin --out-dir gen/android
```

---

## 🤝 Contributing & Community

We welcome contributions from everyone! Whether you're a developer, a security researcher, or a technical writer, your help is invaluable.

* **[Contributing Guide](./CONTRIBUTING.md)**: How to get started and set up your dev environment.
* **[Testing Guide](./docs/TESTING_GUIDE.md)**: Our strategy for reaching 99.9% reliability.
* **[Security Policy](./SECURITY.md)**: How to responsibly disclose vulnerabilities.
* **[Code of Conduct](./CODE_OF_CONDUCT.md)**: Our commitment to a healthy community.

---

## 📜 License & Acknowledgments

EdgeRay is licensed under the **MIT License**. See the [LICENSE](./LICENSE) file for details.

**Author**: [j-rad](https://github.com/faezbarghasa/edgeray-workspace)
**EdgeRay Team** 2024-2026.
