# EdgeRay Desktop Client (Tauri + Dioxus)

The `edgeray-app` crate is the consumer-facing Graphical User Interface (GUI) and desktop systems integration for the EdgeRay ecosystem. It unites the extreme speed of the `rustray` network daemon with a highly optimized, universally portable window manager.

## 🚀 Key Features

* **Phase 7 Integration Complete**: Full visual support for monitoring the Fallback Orchestrator, enabling HTTP/QUIC profiles, overriding the `smoltcp` stack MTU profiles, and controlling eBPF hooks via permission checks natively in the UI.
* **Dioxus-Powered**: Built entirely with Dioxus. React-like ergonomics with native desktop performance—no embedded Chrome browser required.
* **Tauri Runtime Integration**: Leverages Tauri for deep native OS integration (System Tray, Toast Notifications, Native OS Event Handling, IPC bridge).
* **Direct-to-Display Scanout**: Employs a kernel compositor bypass strictly for Linux X11/Wayland interfaces, pushing render latency of our high-speed network graphs to absolute zero.
* **The Obsidian Design System**: Beautiful aesthetics, fluid interactive arrays, translucent paneling, and real-time live routing visualizations built on TailwindCSS.
* **Privilege Orchestration**: Seamlessly handles administrative elevation routines across all operating systems to bind the local TUN interfaces for full global-app routing scenarios.

## 💻 Recommended IDE Setup

For modifying the front-end or Rust IPC bridges:
- [VS Code / RustRover](https://code.visualstudio.com/)
- `rust-analyzer`
- Tauri & Dioxus official VS Code plugins

## 🛠️ Building & Running

Due to the TUN virtual networking integration, the application must be granted permissions physically or run effectively to spin up `tun0`.

### Development Mode

Initialize the Dioxus hot-reloading server:
```bash
sudo -E cargo tauri dev
```

### Production Release

Compiles the final artifact with all symbols stripped for distribution:
```bash
cargo tauri build
```
