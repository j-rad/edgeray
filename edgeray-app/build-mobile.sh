#!/bin/bash
set -e
echo "Running build-mobile.sh from $(pwd)"

# Navigate to app root if we are in src-tauri
if [ -f "tauri.conf.json" ]; then
    echo "Found tauri.conf.json, moving up to app root"
    cd ..
fi

echo "Work dir is now: $(pwd)"

# Run dx build
echo "Starting Dioxus build..."
dx build --release --platform web --package edgeray-app

# Define target paths
# Assuming edgeray-app is a direct child of workspace
WORKSPACE_DIR="$(pwd)/.."
# Verify matches strictly
TARGET_DIR="${WORKSPACE_DIR}/target/dx/edgeray-app/release/web/public"

echo "Looking for artifacts in: $TARGET_DIR"

if [ -d "$TARGET_DIR" ]; then
    echo "Artifacts directory found."
    mkdir -p dist
    # Use cp -r with . to avoid glob issues
    cp -rv "$TARGET_DIR"/. dist/
    echo "Assets copied to dist/"
else
    echo "Error: Artifacts directory not found at $TARGET_DIR"
    echo "Listing ../target/dx for debugging:"
    ls -R "${WORKSPACE_DIR}/target/dx" || echo "Cannot list target dir"
    exit 1
fi
