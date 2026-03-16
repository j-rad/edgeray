# Release Checklist v1.0.0

## Pre-Release Validation

- [ ] **Automated Tests**
  - [ ] Run `cargo test --all-features` (Unit Tests)
  - [ ] Run `cargo test --test signaling_stress` (Signaling Load)
  - [ ] Run `cargo test --test stability_e2e -- --ignored` (Root/NFTables check - Requires sudo)

- [ ] **Manual Verification**
  - [ ] **Handover Resilience**: Run `sudo scripts/stress_test.sh handover`. Verify session resumption logs.
  - [ ] **UI Diagnostics**:
    - Open Dashboard.
    - Click "Generate Report".
    - Verify `diagnostic_report.zip` downloads and contains `rustray.log`, `nftables.conf` (Linux).
  - [ ] **Atomic Kill-Switch**:
    - Simulate failure (e.g., kill `tun` interface or `set_core_healthy(false)` via dev tools).
    - Verify "Safety Badge" in UI turns Red/Offline immediately.

## Build & Distribution

- [ ] **Build Artifacts**
  - [ ] Run `scripts/build_dist.sh`.
  - [ ] Verify `target/release/rustray` exists and is executable.
  - [ ] Verify `edgeray-app` .deb and .apk bundles are generated (if environment supports).
  - [ ] Check UPX compression (run `file target/release/rustray`).

- [ ] **Cross-Platform Checks**
  - [ ] **OpenWrt**: Verify `.ipk` generation (Cross-compile manually if needed).
  - [ ] **Windows**: Verify `.msi` (Cross-compile required).
  - [ ] **Air-Gapped**: Ensure `edgeray-app/assets/vendor/` contains all fonts/icons (no remote calls).

## Release Tagging

- [ ] Update versions in `Cargo.toml` (rustray, edgeray-app).
- [ ] Update `package.json` in `edgeray-app`.
- [ ] Commit and Tag `v1.0.0`.
- [ ] Push to main.
