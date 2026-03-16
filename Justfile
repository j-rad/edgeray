# Justfile for EdgeRay Workspace
# Optimized for Arch Linux development with multi-platform targets

# Variables
ANDROID_TARGETS := "aarch64-linux-android armv7-linux-androideabi x86_64-linux-android"
CARGO_RELEASE_FLAGS := "--release"
RUSTRAY_PKG := "-p rustray"

# Default task
default: build-linux-gnu

# ============================================================================
# Linux Builds
# ============================================================================

# Build for Linux with GNU libc (standard development)
build-linux-gnu:
    @echo "🔨 Building rustray for Linux (GNU)..."
    cargo build {{CARGO_RELEASE_FLAGS}} {{RUSTRAY_PKG}}
    @echo "✅ Linux GNU build complete: target/release/rustray"

# Build for Linux with MUSL (static binary, portable)
build-linux-musl:
    @echo "🔨 Building rustray for Linux (MUSL static)..."
    @echo "Ensure target installed: rustup target add x86_64-unknown-linux-musl"
    cargo build {{CARGO_RELEASE_FLAGS}} {{RUSTRAY_PKG}} --target x86_64-unknown-linux-musl
    @echo "✅ Linux MUSL build complete: target/x86_64-unknown-linux-musl/release/rustray"

# Build with cross for MUSL (uses Docker, more reliable)
build-linux-musl-cross:
    @echo "🔨 Building with cross for Linux MUSL..."
    cross build {{CARGO_RELEASE_FLAGS}} {{RUSTRAY_PKG}} --target x86_64-unknown-linux-musl
    @echo "✅ Cross MUSL build complete"

# ============================================================================
# Android Builds
# ============================================================================

# Build Android libraries using cargo-ndk
build-android:
    @echo "🤖 Building rustray for Android..."
    @echo "Ensure cargo-ndk installed: cargo install cargo-ndk"
    @echo "Ensure NDK installed and ANDROID_NDK_HOME set"
    cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 \
        -o gen/android/app/src/main/jniLibs \
        build {{CARGO_RELEASE_FLAGS}} {{RUSTRAY_PKG}}
    @echo "✅ Android libraries built to gen/android/app/src/main/jniLibs/"

# Build Android APK using Gradle
build-android-apk: build-android
    @echo "📦 Building Android APK..."
    ./scripts/build_android.sh
    @echo "✅ APK built to dist/"

# ============================================================================
# Desktop App Builds (Tauri)
# ============================================================================

# Build desktop app with Tauri
build-desktop:
    @echo "🖥️ Building EdgeRay desktop app..."
    cd edgeray-app && npm run tauri build
    @echo "✅ Desktop build complete"

# Dev mode with hot reload
dev:
    @echo "🚀 Starting development server..."
    cd edgeray-app && npm run tauri dev

# ============================================================================
# Testing & Verification
# ============================================================================

# Run all tests
test:
    @echo "🧪 Running all tests..."
    cargo test --workspace

# Run rustray-specific tests
test-rustray:
    @echo "🧪 Running rustray tests..."
    cargo test {{RUSTRAY_PKG}} --all-features

# Run parser tests
test-parser:
    @echo "🧪 Running parser tests..."
    cargo test -p shared-types -- parser::

# gRPC Smoke Test - verify API is responsive
smoke-test:
    @echo "💨 Running smoke test..."
    @echo "Building release binary..."
    cargo build {{CARGO_RELEASE_FLAGS}} {{RUSTRAY_PKG}}
    @echo "Testing gRPC API responsiveness..."
    @echo "Note: Full smoke test requires running server"
    cargo test {{RUSTRAY_PKG}} -- --ignored smoke

# ============================================================================
# Benchmarks
# ============================================================================

# Run all benchmarks
bench:
    @echo "⚡ Running benchmarks..."
    cargo bench {{RUSTRAY_PKG}}

# Run specific benchmark
bench-geo:
    cargo bench {{RUSTRAY_PKG}} -- geo_mmap_vs_vec

bench-tls:
    cargo bench {{RUSTRAY_PKG}} -- utls_bench

bench-stealth:
    cargo bench {{RUSTRAY_PKG}} -- stealth_overhead

# ============================================================================
# Code Quality
# ============================================================================

# Format code
fmt:
    @echo "🎨 Formatting code..."
    cargo fmt --all

# Lint with clippy
lint:
    @echo "🔍 Running clippy..."
    cargo clippy --workspace --all-targets -- -D warnings

# Check for security vulnerabilities
audit:
    @echo "🔒 Running security audit..."
    cargo audit

# Run cargo deny checks
deny:
    cargo deny check

# Full CI check (format, lint, test)
ci: fmt lint test
    @echo "✅ CI checks passed"

# ============================================================================
# Clean & Maintenance
# ============================================================================

# Clean build artifacts
clean:
    @echo "🧹 Cleaning project..."
    cargo clean

# Clean and rebuild
rebuild: clean build-linux-gnu

# Update dependencies
update:
    cargo update

# Show binary sizes
sizes:
    @echo "📊 Binary sizes:"
    @ls -lh target/release/rustray 2>/dev/null || echo "No release binary"
    @ls -lh target/x86_64-unknown-linux-musl/release/rustray 2>/dev/null || echo "No MUSL binary"

# ============================================================================
# Documentation
# ============================================================================

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    cargo doc --no-deps --workspace --open

# ============================================================================
# Distribution Builds
# ============================================================================

# Build Wasm dashboard
build-wasm:
    @echo "📦 Building Wasm dashboard..."
    cd edgeray-app && dx build --release --platform web --package edgeray-app
    @echo "✅ Wasm dashboard built"

# Build Linux with embedded dashboard
dist-linux: build-wasm
    @echo "🐧 Building Linux distribution..."
    ./scripts/build_dist.sh linux

# Build Android APK
dist-android:
    @echo "🤖 Building Android distribution..."
    ./scripts/build_dist.sh android

# Build Windows installer
dist-windows:
    @echo "🪟 Building Windows distribution..."
    ./scripts/build_dist.sh windows

# Build OpenWrt package
dist-openwrt:
    @echo "📡 Building OpenWrt distribution..."
    ./scripts/build_dist.sh openwrt

# Build all distribution formats
dist-all: build-wasm
    @echo "🌍 Building all distributions..."
    ./scripts/build_dist.sh all

# Verify distribution artifacts
dist-verify:
    @echo "🔍 Verifying distribution artifacts..."
    @cd dist && sha256sum -c SHA256SUMS 2>/dev/null || echo "No checksums to verify"

# Clean distribution artifacts
dist-clean:
    @echo "🧹 Cleaning distribution artifacts..."
    rm -rf dist/*
    @echo "✅ Distribution artifacts cleaned"

# ============================================================================
# Headless Server (with embedded dashboard)
# ============================================================================

# Build headless server with minimal features
build-headless:
    @echo "🖥️ Building headless server with embedded dashboard..."
    cd rustray && cargo build --release --features minimal-server
    @echo "✅ Headless server: target/release/rustray"

# Run headless server
run-headless:
    cd rustray && cargo run --release --features minimal-server -- --config ../config.example.json

# ============================================================================
# Installation
# ============================================================================

# Install required tools for development
setup:
    @echo "🔧 Installing development tools..."
    rustup target add x86_64-unknown-linux-musl
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
    rustup target add aarch64-unknown-linux-musl
    cargo install cargo-ndk
    cargo install cargo-audit
    cargo install cargo-deny
    cargo install cargo-deb
    cargo install cross
    cargo install dioxus-cli
    cargo install tauri-cli
    @echo "✅ Development tools installed"

# Build Debian package
build-debian:
    @echo "📦 Building Debian package..."
    ./scripts/build_dist.sh linux
    @echo "✅ Debian package built"

# Build Windows MSI
build-windows:
    @echo "🪟 Building Windows MSI..."
    ./scripts/build_dist.sh windows
    @echo "✅ Windows MSI built"

# Build OpenWrt IPK
build-openwrt:
    @echo "📡 Building OpenWrt IPK..."
    ./scripts/build_dist.sh openwrt
    @echo "✅ OpenWrt IPK built"

# Test Debian package installation (requires sudo)
test-deb-install:
    @echo "🧪 Testing Debian package installation..."
    @if [ -f dist/*.deb ]; then \
        sudo dpkg -i dist/*.deb && \
        sudo systemctl status rustray && \
        sudo dpkg -r rustray-server; \
    else \
        echo "No .deb package found. Run 'just build-debian' first"; \
    fi

