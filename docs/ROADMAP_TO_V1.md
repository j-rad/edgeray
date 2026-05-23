# Roadmap to EdgeRay v1.0 (Production Ready) 🚀

This document outlines the high-priority goals and features that must be achieved before EdgeRay is considered a **stable, production-ready** product.

---

## 🏗️ Phase 1: Core Stability & Hardening (In-Progress) 🛡️

The goal is to move beyond the experimental phase and ensure the core daemon is rock-solid under various network constraints.

* **Audit**: Engage with external security researchers for a peer-reviewed security audit of the `Flow-J` protocol.
* **Memory leak protection**: Use `Valgrind` or `Miri` to ensure zero allocations leaks in `rustray` during high-throughput sessions.
* **Packet-Parsing Safety**: Implement more robust fuzz testing on VLESS, VMess, and Shadowsocks parsing logic.
* **Multi-Platform Stability**: Stabilize the `tun-rs` driver on Windows (Wintun) and macOS (utun) through extensive QA testing.

---

## 🏎️ Phase 2: Performance Optimization & Testing 🧪

Before Production, the application must reach a high performance benchmark.

* **IO_uring Migration**: On modern Linux, migrate `rustray` to use `tokio-uring` to reduce context-switching cost.
* **Automated Benchmarking**: Implement a CI pipeline that automatically benchmarks each commit and rejects performance regressions.
* **Simulated DPI Stress Testing**: Maintain a test suite of simulated DPI environments (GFW/MCI mimicry) and ensure 99% reliability across all circumvention methods.
* **Elastic FEC Auto-Tuning**: Develop an algorithm that detects localized packet loss and automatically adjusts the Forward Error Correction parity shards.

---

## 🎨 Phase 3: Desktop & User Experience Polish 💄

A Production tool needs a friction-less experience for the non-technical user.

* **Stable Mobile Build**: Reach 100% feature parity on Android (Kotlin) and iOS (Swift) using the unified `rustray` core.
* **Native Tray Integration**: Deepen System Tray functionality for both Windows/macOS to allow node switching without opening the main UI.
* **Diagnostics Export**: Expand the Triage Archive to include sanitized system-level network logs for faster debugging.
* **One-Click Deployment**: Simplify the `rr-ui` server-side deployment with an automated installer script.

---

## ⛓️ Phase 4: Decentralization & Mesh Scaling 🌐

Finalizing the resiliency features of the platform.

* **P2P Peer Discovery**: Shift the current P2P relay system from a static neighbor list to a dynamic DHT-based peer discovery.
* **Decentralized Signaling**: Research and implement post-quantum secure decentralized signaling for node handshakes.
* **Incentivized Relays**: A protocol for rewarding relay operators within the EdgeRay network.

---

**EdgeRay v1.0 Target Date: Late 2026**
*Contribute now and help us reach the goal!*
