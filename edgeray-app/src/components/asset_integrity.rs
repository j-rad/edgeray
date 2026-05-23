//! Asset Integrity Management
//!
//! Ensures all critical UI assets are embedded in the binary and verified at startup.
//! This prevents "Asset Dehydration" in restricted network environments where CDNs
//! are throttled or blocked. Every visual resource must be self-contained.

use log::{error, info};

/// Critical CSS assets embedded directly in the binary via `include_str!`.
/// These are baked in at compile time — zero network requests needed at runtime.
pub const STYLES_CSS: &str = include_str!("../../assets/styles.css");
pub const INTER_CSS: &str = include_str!("../../assets/vendor/inter.css");
pub const JETBRAINS_MONO_CSS: &str = include_str!("../../assets/vendor/jetbrains-mono.css");
pub const CUSTOM_THEME_CSS: &str = include_str!("../../assets/custom_theme.css");

/// Material Symbols CSS for icon rendering (local bundle).
pub const MATERIAL_SYMBOLS_CSS: &str = include_str!("../../assets/vendor/material-symbols.css");

/// Verifies asset integrity at startup and panics on critical failure.
/// In a zero-network environment, these MUST be present for the UI to render correctly.
/// A missing asset means the binary is corrupt or the build pipeline is broken.
pub fn verify_assets() {
    info!("╔══════════════════════════════════════════════╗");
    info!("║   EdgeRay Asset Integrity Check              ║");
    info!("╚══════════════════════════════════════════════╝");

    let assets: &[(&str, usize, bool)] = &[
        ("Tailwind/Styles CSS", STYLES_CSS.len(), true),
        ("Inter Font CSS", INTER_CSS.len(), true),
        ("JetBrains Mono CSS", JETBRAINS_MONO_CSS.len(), true),
        ("Custom Theme CSS", CUSTOM_THEME_CSS.len(), true),
        ("Material Symbols CSS", MATERIAL_SYMBOLS_CSS.len(), false),
    ];

    let mut critical_missing = 0u32;
    let mut total_bytes = 0usize;

    for (name, size, is_critical) in assets.iter() {
        if *size == 0 {
            if *is_critical {
                error!("CRITICAL ASSET MISSING: {} — UI will be dehydrated", name);
                critical_missing += 1;
            } else {
                info!("Optional asset empty: {} (icons may fall back)", name);
            }
        } else {
            info!("  ✓ {} ({} bytes)", name, size);
            total_bytes += size;
        }
    }

    info!("Total embedded CSS payload: {} bytes", total_bytes);

    if critical_missing > 0 {
        panic!(
            "ASSET_DEHYDRATION_ERROR: {} critical asset(s) missing. \
             The binary is corrupt or the build pipeline excluded required assets. \
             Cannot operate in zero-network mode.",
            critical_missing
        );
    }

    info!("Asset Integrity: 100% OK. Zero-network operation ready.");
}
