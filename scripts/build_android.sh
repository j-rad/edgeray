#!/bin/bash
# Build signed Android APK with optimization
# Requires: Android SDK, NDK, and keystore configured

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/..\" && pwd)"
VERSION=$(grep -m1 'version' "${WORKSPACE_ROOT}/rustray/Cargo.toml" | cut -d'"' -f2)

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✓]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check prerequisites
if [ -z "${ANDROID_NDK_HOME:-}" ]; then
    log_error "ANDROID_NDK_HOME not set"
    exit 1
fi

if ! command -v cargo-ndk &> /dev/null; then
    log_error "cargo-ndk not found. Install with: cargo install cargo-ndk"
    exit 1
fi

# Step 1: Build native libraries
log_info "Building native libraries with cargo-ndk..."
cd "${WORKSPACE_ROOT}/rustray"

cargo ndk \
    -t arm64-v8a \
    -t armeabi-v7a \
    -t x86_64 \
    -o ../gen/android/app/src/main/jniLibs \
    build --release --features minimal-server

log_success "Native libraries built"

# Step 2: Generate UniFFI bindings
log_info "Generating UniFFI bindings..."
cargo run --bin uniffi-bindgen generate \
    src/ffi.udl \
    --language kotlin \
    --out-dir ../gen/android/app/src/main/java

log_success "UniFFI bindings generated"

# Step 3: Build APK with Gradle
log_info "Building APK with Gradle..."
cd "${WORKSPACE_ROOT}/gen/android"

if [ ! -f "gradlew" ]; then
    log_error "Gradle wrapper not found"
    exit 1
fi

./gradlew clean assembleRelease

log_success "APK built"

# Step 4: Sign APK (if keystore configured)
KEYSTORE_FILE="${WORKSPACE_ROOT}/gen/android/release.keystore"
KEYSTORE_PROPS="${WORKSPACE_ROOT}/gen/android/keystore.properties"

if [ -f "${KEYSTORE_FILE}" ] && [ -f "${KEYSTORE_PROPS}" ]; then
    log_info "Signing APK..."
    
    # Read keystore properties
    source <(grep = "${KEYSTORE_PROPS}" | sed 's/ *= */=/g')
    
    APK_PATH="app/build/outputs/apk/release/app-release-unsigned.apk"
    SIGNED_APK="app/build/outputs/apk/release/edgeray-${VERSION}-release.apk"
    
    jarsigner -verbose \
        -sigalg SHA256withRSA \
        -digestalg SHA-256 \
        -keystore "${KEYSTORE_FILE}" \
        -storepass "${storePassword}" \
        "${APK_PATH}" \
        "${keyAlias}"
    
    # Zipalign for optimization
    if command -v zipalign &> /dev/null; then
        log_info "Optimizing APK with zipalign..."
        zipalign -v 4 "${APK_PATH}" "${SIGNED_APK}"
    else
        mv "${APK_PATH}" "${SIGNED_APK}"
    fi
    
    log_success "APK signed: ${SIGNED_APK}"
else
    log_info "Keystore not configured, APK unsigned"
    SIGNED_APK="app/build/outputs/apk/release/app-release-unsigned.apk"
fi

# Step 5: Copy to dist and generate checksum
mkdir -p "${WORKSPACE_ROOT}/dist"
cp "${SIGNED_APK}" "${WORKSPACE_ROOT}/dist/"

cd "${WORKSPACE_ROOT}/dist"
sha256sum "$(basename ${SIGNED_APK})" > "$(basename ${SIGNED_APK}).sha256"

log_success "Android APK ready: ${WORKSPACE_ROOT}/dist/$(basename ${SIGNED_APK})"
log_info "Size: $(du -h $(basename ${SIGNED_APK}) | cut -f1)"

echo ""
echo "To install: adb install $(basename ${SIGNED_APK})"
