# Building EdgeRay

Complete guide to building EdgeRay for all supported platforms.

## Prerequisites

### All Platforms

- **Rust**: 1.75 or later

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Dioxus CLI**: For building the web dashboard

  ```bash
  cargo install dioxus-cli
  ```

### Linux (Debian/Ubuntu)

```bash
# Build essentials
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# For .deb packaging
cargo install cargo-deb

# For cross-compilation
cargo install cross
```

### Windows

- **Visual Studio 2022** with "Desktop development with C++" workload
- **WiX Toolset 3.11+** for MSI creation
  - Download from: <https://wixtoolset.org/>
- **Tauri CLI**:

  ```powershell
  cargo install tauri-cli
  ```

### Android

- **Android NDK r25+**
  - Download from: <https://developer.android.com/ndk/downloads>
  - Set `ANDROID_NDK_HOME` environment variable

- **cargo-ndk**:

  ```bash
  cargo install cargo-ndk
  ```

- **Java JDK 17+**

  ```bash
  sudo apt install openjdk-17-jdk  # Linux
  # Or download from: https://adoptium.net/
  ```

### OpenWrt

- **cross** for cross-compilation:

  ```bash
  cargo install cross
  ```

- **Docker** (required by cross):

  ```bash
  # Linux
  sudo apt install docker.io
  sudo usermod -aG docker $USER
  # Log out and back in for group changes to take effect
  ```

---

## Quick Start

### Build for Current Platform

```bash
# Using Justfile (recommended)
just build-debian

# Or using the build script directly
./scripts/build_dist.sh linux
```

### Build All Platforms

```bash
just dist-all
```

This will create distributable packages in the `dist/` directory.

---

## Platform-Specific Builds

### Linux (.deb)

```bash
# Build Debian package
just build-debian

# Output: dist/rustray-server_0.1.0_amd64.deb
```

**Installation**:

```bash
sudo dpkg -i dist/rustray-server_*.deb
sudo systemctl start rustray
sudo systemctl status rustray
```

**Configuration**: `/etc/rustray/config.json`  
**Logs**: `sudo journalctl -u rustray -f`

### Windows (.msi)

```bash
# Build Windows MSI
just build-windows

# Output: dist/edgeray-app_0.1.0_x64_en-US.msi
```

**Note**: Cross-compilation from Linux requires additional setup. Building on Windows is recommended.

**Installation**: Double-click the MSI file or use:

```powershell
msiexec /i edgeray-app_0.1.0_x64_en-US.msi
```

### Android (.apk)

```bash
# Ensure ANDROID_NDK_HOME is set
export ANDROID_NDK_HOME=/path/to/ndk

# Build APK
just build-android-apk

# Output: dist/edgeray-0.1.0-release.apk
```

**Installation**:

```bash
adb install dist/edgeray-*.apk
```

**Signing** (for production):

1. Create a keystore:

   ```bash
   keytool -genkey -v -keystore release.keystore -alias edgeray \
     -keyalg RSA -keysize 2048 -validity 10000
   ```

2. Create `gen/android/keystore.properties`:

   ```properties
   storeFile=release.keystore
   storePassword=YOUR_PASSWORD
   keyAlias=edgeray
   keyPassword=YOUR_PASSWORD
   ```

3. Build will automatically sign the APK

### OpenWrt (.ipk)

```bash
# Build for OpenWrt routers
just build-openwrt

# Output: dist/rustray_0.1.0-1_*.ipk
```

**Installation** on router:

```bash
opkg update
opkg install rustray_*.ipk
/etc/init.d/rustray enable
/etc/init.d/rustray start
```

**Configuration**: `/etc/config/rustray` (UCI format)

---

## Advanced Build Options

### Custom Optimization

Edit `rustray/Cargo.toml` profile settings:

```toml
[profile.release]
strip = true
lto = "fat"
codegen-units = 1
panic = "abort"
opt-level = 3  # or "z" for size
```

### Feature Flags

```bash
# Minimal server (no gRPC, smaller binary)
cargo build --release --features minimal-server

# Full server (all features)
cargo build --release --features full-server

# With QUIC support
cargo build --release --features full-server,quic
```

### Cross-Compilation

```bash
# For ARM64 Linux
cross build --release --target aarch64-unknown-linux-gnu

# For MIPS (OpenWrt)
cross build --release --target mipsel-unknown-linux-musl
```

---

## Verification

### Check Binary Size

```bash
ls -lh target/release/rustray
# Should be < 15MB for full-server, < 8MB for minimal-server
```

### Verify Checksums

```bash
just dist-verify
# Or manually:
cd dist && sha256sum -c SHA256SUMS
```

### Test Installation

```bash
# Debian
just test-deb-install

# Manual test
sudo dpkg -i dist/*.deb
sudo systemctl status rustray
```

---

## Troubleshooting

### "cargo-deb not found"

```bash
cargo install cargo-deb
```

### "ANDROID_NDK_HOME not set"

```bash
export ANDROID_NDK_HOME=/path/to/android-ndk-r25c
# Add to ~/.bashrc or ~/.zshrc for persistence
```

### "cross: command not found"

```bash
cargo install cross
```

### Docker permission denied

```bash
sudo usermod -aG docker $USER
# Log out and back in
```

### Windows: "WiX not found"

Download and install WiX Toolset from <https://wixtoolset.org/>

### Build fails with "out of memory"

```bash
# Reduce parallel jobs
cargo build --release -j 2
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-deb dioxus-cli
      - run: just build-debian
      - uses: actions/upload-artifact@v3
        with:
          name: debian-package
          path: dist/*.deb
```

---

## Performance Benchmarks

Run benchmarks to validate build performance:

```bash
cargo bench --bench dist_performance
```

Expected results:

- **Throughput**: > 1 Gbps
- **Connections**: 1000+ concurrent
- **Memory**: < 100MB resident

---

## Next Steps

- See [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) for release process
- See [ADVANCED_BUILD.md](ADVANCED_BUILD.md) for advanced topics
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines
