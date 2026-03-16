# EdgeRay

EdgeRay is a cross-platform client built with Rust, Tauri, Dioxus, and a custom network engine based on smoltcp.

## Project Structure

- **edgeray-core**: The headless engine (Rust). Handles TUN device, TCP/IP stack (smoltcp), and proxying.
- **edgeray-app**: The UI client (Tauri + Dioxus).
- **shared-types**: Shared data structures between core and app.
- **gen**: Generated native code for Android/iOS.

## Prerequisites

### Desktop (Linux)

You need to install development libraries for Tauri and Dioxus:

```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libcairo2-dev libpango1.0-dev libatk1.0-dev \
    libgdk-pixbuf2.0-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev \
    libwebkit2gtk-4.1-dev
```

**Note:** On desktop, the application must be run with `sudo` (or as Administrator) to create the TUN interface.

```bash
sudo -E cargo tauri dev
```

### Android

```bash
# Install cargo-ndk
cargo install cargo-ndk

# Build for all targets and output to jniLibs (Update path to match your Android project)
cargo ndk -t aarch64-linux-android -t armv7-linux-androideabi -t x86_64-linux-android -o ../edgeray-app/gen/android/app/src/main/jniLibs build --release
```

### iOS

```bash
# Install cargo-lipo
cargo install cargo-lipo

# Build universal static library (output: target/universal/release/libedgeray_core.a)
cargo lipo --release
```

## Building

### Core

```bash
cd edgeray-core
cargo build
```

### Desktop App

```bash
cd edgeray-app
cargo tauri dev
```

### Mobile Bindings

The native bindings (Kotlin/Swift) are generated using `uniffi-bindgen`.

```bash
# Generate Kotlin bindings
cargo run -p edgeray-core --bin uniffi-bindgen generate --library target/debug/libedgeray_core.so --language kotlin --out-dir gen/android

# Generate Swift bindings
cargo run -p edgeray-core --bin uniffi-bindgen generate --library target/debug/libedgeray_core.so --language swift --out-dir gen/apple
```
