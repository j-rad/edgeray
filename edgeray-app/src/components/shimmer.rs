//! Glass Shimmer — Skeleton Screen System
//!
//! Provides shimmer placeholder components for initial hydration.
//!
//! - `GlassShimmer`: Single shimmer block with configurable shape
//! - `GlassShimmerScreen`: Full dashboard skeleton that prevents layout shift

use dioxus::prelude::*;

/// Shape variant for shimmer placeholders.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ShimmerShape {
    /// Rounded rectangle (cards, panels)
    #[default]
    Rectangle,
    /// Circle (power core orb, avatars)
    Circle,
    /// Thin bar (text lines, labels)
    Bar,
}

impl ShimmerShape {
    /// Returns the Tailwind border-radius class for this shape.
    fn radius_class(self) -> &'static str {
        match self {
            ShimmerShape::Rectangle => "rounded-2xl",
            ShimmerShape::Circle => "rounded-full",
            ShimmerShape::Bar => "rounded-lg",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct GlassShimmerProps {
    /// Width/height sizing classes (e.g. "w-full h-32").
    #[props(default = "w-full h-32".to_string())]
    pub class: String,

    /// Shape of the shimmer placeholder.
    #[props(default)]
    pub shape: ShimmerShape,
}

/// Single shimmer placeholder block with sweeping gradient animation.
#[component]
pub fn GlassShimmer(props: GlassShimmerProps) -> Element {
    let radius = props.shape.radius_class();

    rsx! {
        div {
            class: "relative overflow-hidden {radius} {props.class} bg-white/5 border border-white/5 backdrop-blur-sm",

            // The shimmer sweep overlay
            div {
                class: "absolute inset-0 -translate-x-full animate-[shimmer_2s_infinite]",
                style: "background-image: linear-gradient(90deg, rgba(255, 255, 255, 0) 0%, rgba(255, 255, 255, 0.05) 20%, rgba(255, 255, 255, 0.1) 50%, rgba(255, 255, 255, 0.05) 80%, rgba(255, 255, 255, 0) 100%);",
            }
        }
    }
}

/// Full-screen skeleton layout matching the dashboard structure.
///
/// Renders shimmer placeholders in the exact positions where real UI
/// elements will appear, preventing layout shift during hydration:
/// - Large circle (PowerCore orb)
/// - 3 metric card rectangles (telemetry panel)
/// - Bottom navigation bar
#[component]
pub fn GlassShimmerScreen() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center gap-6 p-6 w-full max-w-4xl mx-auto animate-pulse",
            style: "padding-top: env(safe-area-inset-top); padding-bottom: calc(env(safe-area-inset-bottom) + 6rem);",

            // ── Status badge placeholder ──────────────────────────────
            GlassShimmer {
                class: "w-32 h-6".to_string(),
                shape: ShimmerShape::Bar,
            }

            // ── PowerCore orb placeholder ─────────────────────────────
            div {
                class: "flex justify-center my-8",
                GlassShimmer {
                    class: "w-48 h-48 md:w-64 md:h-64".to_string(),
                    shape: ShimmerShape::Circle,
                }
            }

            // ── Speed indicators placeholder ──────────────────────────
            div {
                class: "flex justify-center gap-8 w-full",
                GlassShimmer {
                    class: "w-20 h-4".to_string(),
                    shape: ShimmerShape::Bar,
                }
                GlassShimmer {
                    class: "w-20 h-4".to_string(),
                    shape: ShimmerShape::Bar,
                }
            }

            // ── Telemetry cards placeholder ───────────────────────────
            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-4 w-full",
                GlassShimmer { class: "w-full h-24".to_string() }
                GlassShimmer { class: "w-full h-24".to_string() }
                GlassShimmer { class: "w-full h-24".to_string() }
            }

            // ── Server list placeholder ───────────────────────────────
            div {
                class: "flex flex-col gap-3 w-full",
                GlassShimmer { class: "w-full h-16".to_string() }
                GlassShimmer { class: "w-full h-16".to_string() }
            }
        }

        // ── Bottom nav placeholder ────────────────────────────────────
        div {
            class: "fixed bottom-6 left-4 right-4 z-50 lg:hidden",
            style: "margin-bottom: env(safe-area-inset-bottom);",
            GlassShimmer { class: "w-full h-20 max-w-md mx-auto".to_string() }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shimmer_shape_radius_classes() {
        assert!(
            ShimmerShape::Rectangle
                .radius_class()
                .contains("rounded-2xl")
        );
        assert!(ShimmerShape::Circle.radius_class().contains("rounded-full"));
        assert!(ShimmerShape::Bar.radius_class().contains("rounded-lg"));
    }

    #[test]
    fn test_shimmer_shape_default_is_rectangle() {
        assert_eq!(ShimmerShape::default(), ShimmerShape::Rectangle);
    }

    #[test]
    fn test_shimmer_shape_equality() {
        assert_eq!(ShimmerShape::Circle, ShimmerShape::Circle);
        assert_ne!(ShimmerShape::Circle, ShimmerShape::Bar);
    }
}
