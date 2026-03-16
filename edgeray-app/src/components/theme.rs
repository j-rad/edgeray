//! Obsidian Design System — Theme Constants
//!
//! Centralized palette, spacing, and reusable Tailwind class bundles
//! for the EdgeRay "Obsidian" visual identity.
//!
//! - **Base**: #020203 (near-black)
//! - **Accents**: Electric Cyan (#00F2FF), Cyber Purple (#BC13FE)
//! - **Surfaces**: Glassmorphism with backdrop-blur-xl

/// Deep obsidian black — the app background.
pub const OBSIDIAN_BG: &str = "#020203";
/// Electric cyan accent.
pub const ELECTRIC_CYAN: &str = "#00F2FF";
/// Cyber purple accent.
pub const CYBER_PURPLE: &str = "#BC13FE";
/// Emerald success accent.
pub const EMERALD_SUCCESS: &str = "#10B981";
/// Amber warning accent.
pub const AMBER_WARNING: &str = "#F59E0B";
/// Red failure accent.
pub const RED_FAILURE: &str = "#EF4444";

/// Electric cyan as RGB tuple for box-shadow usage.
pub const CYAN_RGB: &str = "0,242,255";
/// Cyber purple as RGB tuple.
pub const PURPLE_RGB: &str = "188,19,254";

// ─── Tailwind Class Bundles ────────────────────────────────────────────────────

/// Glassmorphism surface classes (card backgrounds).
pub mod glass {
    /// Primary glass surface — frosted panel.
    pub const SURFACE: &str = "bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl";
    /// Glass surface with shadow.
    pub const SURFACE_SHADOW: &str =
        "bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl shadow-xl";
    /// Compact glass surface for inline elements.
    pub const SURFACE_COMPACT: &str =
        "bg-white/5 backdrop-blur-lg border border-white/8 rounded-xl";
    /// Glass overlay (for modals, drawers).
    pub const OVERLAY: &str =
        "bg-black/60 backdrop-blur-2xl border border-white/5 rounded-2xl shadow-2xl";
    /// Sidebar / panel full-height surface.
    pub const PANEL: &str = "bg-white/5 backdrop-blur-xl border-r border-white/10 h-full";
}

/// Typography classes.
pub mod text {
    /// Page title.
    pub const TITLE: &str = "text-lg md:text-xl font-bold text-white";
    /// Section header.
    pub const SECTION: &str = "text-sm font-semibold text-white";
    /// Body text.
    pub const BODY: &str = "text-sm text-gray-300";
    /// Muted secondary text.
    pub const MUTED: &str = "text-xs text-gray-500";
    /// Monospace data text.
    pub const MONO: &str = "text-xs font-mono text-gray-300";
    /// Cyan accent text.
    pub const ACCENT_CYAN: &str = "text-cyan-400";
    /// Purple accent text.
    pub const ACCENT_PURPLE: &str = "text-purple-400";
}

/// Spacing constants for responsive padding.
pub mod spacing {
    /// Card padding (mobile / desktop).
    pub const CARD_MOBILE: &str = "p-4";
    pub const CARD_DESKTOP: &str = "p-6 md:p-8";
    /// Section gap.
    pub const SECTION_GAP: &str = "space-y-4 md:space-y-6";
    /// Safe area padding (top/bottom for mobile notches).
    pub const SAFE_TOP: &str = "pt-safe";
    pub const SAFE_BOTTOM: &str = "pb-safe";
}

/// Animation classes.
pub mod anim {
    /// Breathing pulse (idle states).
    pub const BREATHE: &str = "animate-pulse";
    /// Spin (connecting states).
    pub const SPIN_SLOW: &str = "animate-spin-slow";
    /// Glow pulse (active states).
    pub const GLOW_PULSE: &str = "animate-pulse-glow";
    /// GPU acceleration hint.
    pub const GPU_ACCEL: &str = "will-change-transform";
}

// ─── Runtime Helpers ───────────────────────────────────────────────────────────

/// Determines if the given viewport width is "mobile" (< 1024px).
///
/// Pure function — no side effects, fully testable.
pub fn is_mobile(viewport_width: u32) -> bool {
    viewport_width < 1024
}

/// Determines the breakpoint tier for the given width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Breakpoint {
    /// < 640px — small phone
    Sm,
    /// 640..1024px — large phone / tablet
    Md,
    /// >= 1024px — desktop
    Lg,
}

impl Breakpoint {
    pub fn from_width(width: u32) -> Self {
        if width < 640 {
            Breakpoint::Sm
        } else if width < 1024 {
            Breakpoint::Md
        } else {
            Breakpoint::Lg
        }
    }

    pub fn is_mobile(self) -> bool {
        matches!(self, Breakpoint::Sm | Breakpoint::Md)
    }

    pub fn is_desktop(self) -> bool {
        matches!(self, Breakpoint::Lg)
    }
}

/// Returns the inline CSS variable string for safe-area-inset env values.
///
/// Inject this as `style` on the root container to ensure notch-safe padding.
pub fn safe_area_style() -> &'static str {
    "padding-top: env(safe-area-inset-top); padding-bottom: env(safe-area-inset-bottom); padding-left: env(safe-area-inset-left); padding-right: env(safe-area-inset-right);"
}

/// Returns Obsidian root background CSS.
pub fn obsidian_bg_style() -> String {
    format!("background-color: {}; min-height: 100vh;", OBSIDIAN_BG)
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mobile_breakpoint() {
        assert!(is_mobile(320));
        assert!(is_mobile(768));
        assert!(is_mobile(1023));
        assert!(!is_mobile(1024));
        assert!(!is_mobile(1920));
    }

    #[test]
    fn test_breakpoint_from_width() {
        assert_eq!(Breakpoint::from_width(320), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_width(639), Breakpoint::Sm);
        assert_eq!(Breakpoint::from_width(640), Breakpoint::Md);
        assert_eq!(Breakpoint::from_width(1023), Breakpoint::Md);
        assert_eq!(Breakpoint::from_width(1024), Breakpoint::Lg);
        assert_eq!(Breakpoint::from_width(2560), Breakpoint::Lg);
    }

    #[test]
    fn test_breakpoint_is_mobile() {
        assert!(Breakpoint::Sm.is_mobile());
        assert!(Breakpoint::Md.is_mobile());
        assert!(!Breakpoint::Lg.is_mobile());
    }

    #[test]
    fn test_breakpoint_is_desktop() {
        assert!(!Breakpoint::Sm.is_desktop());
        assert!(!Breakpoint::Md.is_desktop());
        assert!(Breakpoint::Lg.is_desktop());
    }

    #[test]
    fn test_glass_surface_classes_contain_backdrop_blur() {
        assert!(glass::SURFACE.contains("backdrop-blur-xl"));
        assert!(glass::SURFACE.contains("bg-white/5"));
        assert!(glass::SURFACE.contains("border-white/10"));
    }

    #[test]
    fn test_safe_area_style_contains_env() {
        let style = safe_area_style();
        assert!(style.contains("env(safe-area-inset-top)"));
        assert!(style.contains("env(safe-area-inset-bottom)"));
        assert!(style.contains("env(safe-area-inset-left)"));
        assert!(style.contains("env(safe-area-inset-right)"));
    }

    #[test]
    fn test_obsidian_bg_style() {
        let bg = obsidian_bg_style();
        assert!(bg.contains(OBSIDIAN_BG));
        assert!(bg.contains("min-height: 100vh"));
    }

    #[test]
    fn test_palette_hex_format() {
        assert!(OBSIDIAN_BG.starts_with('#'));
        assert!(ELECTRIC_CYAN.starts_with('#'));
        assert!(CYBER_PURPLE.starts_with('#'));
        assert_eq!(OBSIDIAN_BG.len(), 7);
        assert_eq!(ELECTRIC_CYAN.len(), 7);
        assert_eq!(CYBER_PURPLE.len(), 7);
    }
}
