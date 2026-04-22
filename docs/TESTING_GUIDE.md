# Testing EdgeRay 🧪

EdgeRay is built for high-reliability environments where network visibility is a security threat. This document outlines our testing strategies and how we aim for production-level stability.

---

## 🏗️ Technical Testing Strategy

We maintain a layered testing approach to ensure 99.9% reliability for the `rustray` engine and its attached UI modules.

### 1. Unit Testing `cargo test`

Individual protocol handlers (VLESS/Flow-J), encryption logic, and the `smoltcp` stack have a 95% unit test coverage target.

```bash
# Run unit tests across the workspace
cargo test --workspace
```

### 2. Integration Testing `tests/`

Our integration tests simulate a local client and server bridge over a virtual TUN interface to ensure the entire packet lifecycle is functional.

```bash
# Run stability and end-to-end integration tests
cd rustray
cargo test --test stability_e2e
```

### 3. Fuzz Testing `rustray/fuzz`

We use `cargo-fuzz` (LibFuzzer) to stress-test our protocol parsers and header decoders against malformed data.

* **Target**: Ensure the engine never panics when receiving hostile or random DPI probes.

```bash
# (Requires cargo-fuzz)
cd rustray/fuzz
cargo fuzz run flow_j_header_fuzzer
```

### 4. Performance Benchmarking `benches/`

We monitor the microsecond latency and zero-copy efficiency of the core using `criterion`.

```bash
# Run all performance benchmarks
cargo bench
```

---

## 🛰️ Simulated Network Environments

To simulate real-world DPI (Deep Packet Inspection), we use gated Linux network namespaces (`ip netns`) to artificially throttle and drop packets.

* **Lossy Network**: 20% random packet drop to test **Elastic FEC**.
* **Throttled Network**: 1Mbps cap to test **Brutal-QUIC**'s congestion controller.
* **Blocked Network**: 100% block on IP/Port to test the **Fallback Orchestrator**.

---

## 💄 UI Testing (`edgeray-app`)

Tests for the Dioxus/Tauri desktop application ensure that permissions are handled correctly and the telemetry graphs are rendering without latency.

* **Mock Core**: The UI can be launched with a mock gRPC server to test state transitions without requiring the real `rustray` daemon.
* **Playwright / WebDriver**: (In-Progress) Automated end-to-end UI testing for the Tauri window across different resolutions.

---

## 📈 Quality Assurance Matrix

| Module | Test Type | Coverage Goal | Status |
|--------|-----------|---------------|--------|
| `rustray-proto` | Fuzzing | 100% | ✅ High |
| `rustray-fec` | Unit Tests | 95% | ✅ Ready |
| `rustray-api` | Integration | 80% | ⚠️ In-Progress |
| `edgeray-app` | Manual QA | 100% | ✅ Verified |
| `rr-ui` | API Tests | 75% | ⚠️ In-Progress |

---

**EdgeRay Team**
*Building for zero failures.*
