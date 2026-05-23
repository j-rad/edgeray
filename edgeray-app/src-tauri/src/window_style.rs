//! Platform-Native Window Effects
//!
//! Applies system-level backdrop effects to achieve native visual depth:
//! - **Windows 11**: WinUI 3 "Mica" (falls back to Acrylic on Win10)
//! - **macOS**: Apple Vibrancy with `UnderWindowBackground` material
//! - **Linux/Other**: No-op (compositors handle this)

use tauri::{Runtime, WebviewWindow};

/// Apply platform-native window effects to the given webview window.
///
/// On Windows 11+, this uses Mica for the characteristic translucent material
/// that blends the desktop wallpaper into the window background. On older Windows,
/// it falls back to the Acrylic blur effect. On macOS, it applies the
/// `UnderWindowBackground` vibrancy material for a native frosted-glass look.
#[cfg(target_os = "windows")]
pub fn apply_window_effects<R: Runtime>(window: &WebviewWindow<R>) {
    use window_vibrancy::{apply_acrylic, apply_mica};

    // Attempt Mica first (Windows 11 build 22000+).
    // `apply_mica` returns Err on unsupported builds, so we chain to Acrylic.
    // We use dark mode preference for Mica if possible, but the crate handles it.
    if let Err(mica_err) = apply_mica(window, Some(true)) {
        log::debug!("Mica unavailable ({}), falling back to Acrylic", mica_err);
        // Acrylic with a dark-tinted RGBA overlay (18, 18, 18, 200)
        if let Err(acrylic_err) = apply_acrylic(window, Some((18, 18, 18, 200))) {
            log::warn!(
                "Failed to apply any backdrop effect: Mica={}, Acrylic={}",
                mica_err,
                acrylic_err
            );
        } else {
            log::info!("Applied Acrylic backdrop effect");
        }
    } else {
        log::info!("Applied Mica backdrop effect");
    }
}

/// Apply macOS vibrancy with the `UnderWindowBackground` material.
///
/// This gives the window a native frosted-glass appearance that adapts
/// to the user's light/dark mode and desktop wallpaper.
#[cfg(target_os = "macos")]
pub fn apply_window_effects<R: Runtime>(window: &WebviewWindow<R>) {
    use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

    // `UnderWindowBackground` provides the deepest desktop-blending material,
    // matching the look of native macOS system preferences panels.
    if let Err(e) = apply_vibrancy(
        window,
        NSVisualEffectMaterial::UnderWindowBackground,
        None,
        None,
    ) {
        // Fall back to HudWindow if the deeper material is unsupported
        log::debug!("UnderWindowBackground failed ({}), trying HudWindow", e);
        if let Err(fallback_err) =
            apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)
        {
            log::warn!("Failed to apply vibrancy: {}", fallback_err);
        } else {
            log::info!("Applied macOS HudWindow vibrancy");
        }
    } else {
        log::info!("Applied macOS UnderWindowBackground vibrancy");
    }
}

/// No-op on Linux and other platforms.
///
/// Window effects are handled by the compositor (KWin, Mutter, etc.)
/// or are not available. The CSS glassmorphism fallback applies automatically.
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn apply_window_effects<R: Runtime>(_window: &WebviewWindow<R>) {
    log::debug!("No native window effects available on this platform; CSS fallback active");
}
