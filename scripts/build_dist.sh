#!/bin/bash
set -e

# EdgeRay Production Distribution Build Script
# Targets: Windows (.msi), Android (.apk), Debian (.deb), OpenWrt (.ipk)
# Features: LTO optimization, UPX compression, code signing

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   EdgeRay v1.0.0 Production Build Pipeline                ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Configuration
VERSION="1.0.0"
BUILD_DIR="$(pwd)/dist"
ARTIFACTS_DIR="$BUILD_DIR/artifacts"
RUSTRAY_BIN="target/release/rustray"

# Create build directories
mkdir -p "$ARTIFACTS_DIR"/{windows,android,debian,openwrt}

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# ============================================================================
# Step 1: Offline Asset Audit
# ============================================================================
log_info "[1/6] Auditing Offline Assets..."

if grep -r "http://" edgeray-app/src 2>/dev/null | grep -v "localhost" | grep -v "127.0.0.1" | grep -v "dns-query"; then
    log_warn "HTTP links found in source code"
fi

if grep -r "jsdelivr\|unpkg" edgeray-app/src 2>/dev/null; then
    log_error "CDN links found (jsdelivr/unpkg). Build Aborted."
    exit 1
fi

log_success "Asset audit passed"

# ============================================================================
# Step 2: Build Headless Core (rustray) with LTO
# ============================================================================
log_info "[2/6] Building Headless Core (rustray) with LTO..."

# Enable LTO in Cargo.toml temporarily
export RUSTFLAGS="-C lto=fat -C embed-bitcode=yes -C codegen-units=1 -C opt-level=3"

cargo build --release --bin rustray --features full-server

if [ ! -f "$RUSTRAY_BIN" ]; then
    log_error "Rustray binary not found at $RUSTRAY_BIN"
    exit 1
fi

log_success "Rustray binary built successfully"

# UPX Compression
if command -v upx &> /dev/null; then
    log_info "Compressing rustray binary with UPX (Best LZMA)..."
    cp "$RUSTRAY_BIN" "$RUSTRAY_BIN.uncompressed"
    upx --best --lzma "$RUSTRAY_BIN" || log_warn "UPX compression failed, continuing with uncompressed binary"
    log_success "Binary compressed"
else
    log_warn "UPX not found. Skipping compression."
fi

# Copy to artifacts
cp "$RUSTRAY_BIN" "$ARTIFACTS_DIR/rustray-linux-x86_64"
log_success "Rustray binary copied to artifacts"

# ============================================================================
# Step 3: Build Desktop App (Linux .deb)
# ============================================================================
log_info "[3/6] Building Desktop App (Linux .deb)..."

cd edgeray-app

# Install UI dependencies
if [ -f "package.json" ]; then
    npm install --silent 2>/dev/null || log_warn "npm install failed"
fi

# Build Tauri bundle (creates .deb on Linux)
cargo tauri build --bundles deb

# Copy artifacts
if [ -d "src-tauri/target/release/bundle/deb" ]; then
    cp src-tauri/target/release/bundle/deb/*.deb "$ARTIFACTS_DIR/debian/" 2>/dev/null || log_warn "No .deb files found"
    log_success "Debian package created"
else
    log_warn "Debian bundle directory not found"
fi

cd ..

# ============================================================================
# Step 4: Build Android App (Signed .apk)
# ============================================================================
log_info "[4/6] Building Android App (Signed .apk)..."

if [ -n "$ANDROID_HOME" ]; then
    cd edgeray-app
    
    # Check for signing keystore
    if [ -f "$ANDROID_KEYSTORE_PATH" ] && [ -n "$ANDROID_KEYSTORE_PASSWORD" ]; then
        log_info "Building signed APK..."
        
        # Configure signing in gradle.properties
        cat > gen/android/gradle.properties << EOF
RELEASE_STORE_FILE=$ANDROID_KEYSTORE_PATH
RELEASE_STORE_PASSWORD=$ANDROID_KEYSTORE_PASSWORD
RELEASE_KEY_ALIAS=$ANDROID_KEY_ALIAS
RELEASE_KEY_PASSWORD=$ANDROID_KEY_PASSWORD
EOF
        
        cargo tauri android build --apk --release
        
        # Copy signed APK
        if [ -d "gen/android/app/build/outputs/apk/release" ]; then
            cp gen/android/app/build/outputs/apk/release/*.apk "$ARTIFACTS_DIR/android/" 2>/dev/null
            log_success "Signed Android APK created"
        fi
    else
        log_warn "Keystore not configured. Building unsigned APK..."
        cargo tauri android build --apk
        
        if [ -d "gen/android/app/build/outputs/apk/debug" ]; then
            cp gen/android/app/build/outputs/apk/debug/*.apk "$ARTIFACTS_DIR/android/" 2>/dev/null
            log_warn "Created unsigned APK (debug)"
        fi
    fi
    
    cd ..
else
    log_warn "ANDROID_HOME not set. Skipping Android build."
fi

# ============================================================================
# Step 5: Build Windows MSI (Cross-compilation)
# ============================================================================
log_info "[5/6] Building Windows MSI..."

if command -v cargo-wix &> /dev/null; then
    cd edgeray-app
    
    # Install Windows target if not present
    rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
    
    # Build Windows binary
    cargo build --release --target x86_64-pc-windows-gnu
    
    # Create MSI with WiX
    if [ -f "wix/main.wxs" ]; then
        cargo wix --target x86_64-pc-windows-gnu --output "$ARTIFACTS_DIR/windows/edgeray-${VERSION}-x64.msi"
        log_success "Windows MSI created"
        
        # Sign MSI if certificate available
        if [ -f "$WINDOWS_CERT_PATH" ] && command -v osslsigncode &> /dev/null; then
            log_info "Signing Windows MSI..."
            osslsigncode sign -certs "$WINDOWS_CERT_PATH" -key "$WINDOWS_KEY_PATH" \
                -n "EdgeRay" -i "https://edgeray.io" -t http://timestamp.digicert.com \
                -in "$ARTIFACTS_DIR/windows/edgeray-${VERSION}-x64.msi" \
                -out "$ARTIFACTS_DIR/windows/edgeray-${VERSION}-x64-signed.msi"
            mv "$ARTIFACTS_DIR/windows/edgeray-${VERSION}-x64-signed.msi" "$ARTIFACTS_DIR/windows/edgeray-${VERSION}-x64.msi"
            log_success "MSI signed"
        fi
    else
        log_warn "WiX configuration not found. Skipping MSI creation."
    fi
    
    cd ..
else
    log_warn "cargo-wix not installed. Skipping Windows MSI build."
    log_info "Install with: cargo install cargo-wix"
fi

# ============================================================================
# Step 6: Build OpenWrt Package (.ipk)
# ============================================================================
log_info "[6/6] Building OpenWrt Package (.ipk)..."

if [ -d "openwrt" ]; then
    # Build for OpenWrt target
    if command -v opkg &> /dev/null || [ -n "$OPENWRT_SDK" ]; then
        log_info "Building OpenWrt package..."
        
        # Create package structure
        OPENWRT_PKG="$BUILD_DIR/openwrt-pkg"
        mkdir -p "$OPENWRT_PKG"/{CONTROL,usr/bin,etc/init.d}
        
        # Copy binary
        cp "$RUSTRAY_BIN" "$OPENWRT_PKG/usr/bin/rustray"
        chmod +x "$OPENWRT_PKG/usr/bin/rustray"
        
        # Create control file
        cat > "$OPENWRT_PKG/CONTROL/control" << EOF
Package: edgeray
Version: $VERSION
Architecture: x86_64
Maintainer: EdgeRay Team <team@edgeray.io>
Section: net
Priority: optional
Description: EdgeRay VPN Core
 Advanced VPN core with Reality, uTLS, and mesh networking
EOF
        
        # Create init script
        cat > "$OPENWRT_PKG/etc/init.d/edgeray" << 'EOF'
#!/bin/sh /etc/rc.common
START=99
STOP=10

start() {
    /usr/bin/rustray --config /etc/edgeray/config.json &
}

stop() {
    killall rustray
}
EOF
        chmod +x "$OPENWRT_PKG/etc/init.d/edgeray"
        
        # Build IPK
        cd "$BUILD_DIR"
        tar czf openwrt-pkg/data.tar.gz -C openwrt-pkg usr etc
        tar czf openwrt-pkg/control.tar.gz -C openwrt-pkg/CONTROL .
        echo "2.0" > openwrt-pkg/debian-binary
        ar r "$ARTIFACTS_DIR/openwrt/edgeray_${VERSION}_x86_64.ipk" \
            openwrt-pkg/debian-binary \
            openwrt-pkg/control.tar.gz \
            openwrt-pkg/data.tar.gz
        cd ..
        
        log_success "OpenWrt IPK created"
    else
        log_warn "OpenWrt SDK not found. Skipping IPK build."
    fi
else
    log_warn "OpenWrt configuration not found. Skipping IPK build."
fi

# ============================================================================
# Generate Checksums and Signatures
# ============================================================================
log_info "Generating checksums..."

cd "$ARTIFACTS_DIR"
find . -type f \( -name "*.apk" -o -name "*.deb" -o -name "*.msi" -o -name "*.ipk" -o -name "rustray-*" \) -exec sha256sum {} \; > SHA256SUMS
log_success "SHA256 checksums generated"

# GPG signing if available
if command -v gpg &> /dev/null && [ -n "$GPG_KEY_ID" ]; then
    gpg --detach-sign --armor -u "$GPG_KEY_ID" SHA256SUMS
    log_success "GPG signature created"
fi

cd - > /dev/null

# ============================================================================
# Build Summary
# ============================================================================
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║   Build Complete!                                          ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
log_info "Artifacts located in: $ARTIFACTS_DIR"
echo ""
log_success "Linux Core:    $ARTIFACTS_DIR/rustray-linux-x86_64"
[ -f "$ARTIFACTS_DIR/debian/"*.deb ] && log_success "Debian Package: $(ls $ARTIFACTS_DIR/debian/*.deb 2>/dev/null | head -1)"
[ -f "$ARTIFACTS_DIR/android/"*.apk ] && log_success "Android APK:    $(ls $ARTIFACTS_DIR/android/*.apk 2>/dev/null | head -1)"
[ -f "$ARTIFACTS_DIR/windows/"*.msi ] && log_success "Windows MSI:    $(ls $ARTIFACTS_DIR/windows/*.msi 2>/dev/null | head -1)"
[ -f "$ARTIFACTS_DIR/openwrt/"*.ipk ] && log_success "OpenWrt IPK:    $(ls $ARTIFACTS_DIR/openwrt/*.ipk 2>/dev/null | head -1)"
echo ""
log_info "Checksums: $ARTIFACTS_DIR/SHA256SUMS"
[ -f "$ARTIFACTS_DIR/SHA256SUMS.asc" ] && log_info "Signature: $ARTIFACTS_DIR/SHA256SUMS.asc"
echo ""
