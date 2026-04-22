# Contributing to EdgeRay 🚀

First off, thank you for considering contributing to EdgeRay! It's people like you who make EdgeRay a better tool for everyone seeking privacy and freedom on the internet.

EdgeRay is a complex project spanning a high-performance Rust core, a desktop UI (Dioxus/Tauri), and a server management panel. We are currently in the **Stability & Hardening** phase, moving toward a production-ready v1.0 release.

---

## 🛠️ Getting Started

### Prerequisites

* **Rust**: Latests stable version (via `rustup`).
* **Node.js & pnpm**: Required for the `rr-ui` web frontend.
* **System Libraries**: (Linux specific)

  ```bash
  sudo apt-get update
  sudo apt-get install -y libgtk-3-dev libcairo2-dev libpango1.0-dev libatk1.0-dev \
      libgdk-pixbuf2.0-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev \
      libwebkit2gtk-4.1-dev
  ```

### Repository Structure

* **/rustray**: The core networking and transport daemon.
* **/edgeray-app**: The Tauri/Dioxus desktop application.
* **/rr-ui**: The server-side management panel.
* **/docs**: Detailed guides, manuals, and technical specifications.

---

## 🤝 How Can I Contribute?

### 1. Reporting Bugs 🐛

If you find a bug, please use the [Bug Report Template](https://github.com/faezbarghasa/edgeray-workspace/issues/new?template=bug_report.md). Be sure to include:

* Your OS and EdgeRay version.
* Sample `config.json` (redact your keys!).
* Steps to reproduce the issue.

### 2. Suggesting Enhancements ✨

Have an idea for a new transport or a UI improvement? Open a [Feature Request](https://github.com/faezbarghasa/edgeray-workspace/issues/new?template=feature_request.md).

### 3. Writing Code 💻

* **Fork the repo** and create your branch from `master`.
* Ensure your code passes all lint checks:

  ```bash
  cargo clippy --workspace -- -D warnings
  cargo fmt --all -- --check
  ```

* Add tests for any new functionality in the relevant crate's `tests/` directory.

---

## 🏗️ Development Lifecycle

### Running the Core (`rustray`)

```bash
cd rustray
cargo run -- -c config.json
```

### Running the Desktop UI (`edgeray-app`)

```bash
cd edgeray-app
sudo -E cargo tauri dev
```

### Running the Management Panel (`rr-ui`)

```bash
cd rr-ui
cargo run --features server -- run --port 54321
```

---

## 📋 Pull Request Process

1. Keep PRs focused. If you're fixing two unrelated things, submit two PRs.
2. Update the documentation if you're changing the user-facing interface or configuration.
3. Fill out the PR template completely.
4. Once submitted, a maintainer will review your code. Please be patient!

## 📜 Code of Conduct

By participating in this project, you agree to abide by the terms of our [Code of Conduct](./CODE_OF_CONDUCT.md).

## 🛡️ Security

If you discover a security vulnerability, please follow our [Security Policy](./SECURITY.md) instead of opening a public issue.

---

**EdgeRay Team**
*Building a freer internet, one packet at a time.*
