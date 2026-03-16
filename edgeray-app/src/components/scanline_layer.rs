//! Scanline Layer — Technical Grid Overlay
//!
//! A pointer-events-none overlay that renders:
//! 1. A 2px technical grid pattern (like drafting paper)
//! 2. CRT-style horizontal scanlines
//!
//! Combined opacity: 2.5% — subtle enough to be felt, not seen.

use dioxus::prelude::*;

/// Grid cell size in pixels.
const GRID_SIZE: u32 = 2;
/// Grid line opacity (0–1).
const GRID_OPACITY: f32 = 0.025;
/// Scanline spacing in pixels.
const SCANLINE_SPACING: u32 = 4;
/// Scanline opacity.
const SCANLINE_OPACITY: f32 = 0.015;

#[derive(Props, Clone, PartialEq)]
pub struct ScanlineLayerProps {
    /// Extra CSS class to apply to the overlay container.
    #[props(default = String::new())]
    pub class: String,
}

/// Full-viewport overlay rendering a subtle technical grid and CRT scanlines.
///
/// This component must be placed early in the DOM tree (before content)
/// and uses `pointer-events: none` so it never intercepts interaction.
#[component]
pub fn ScanlineLayer(props: ScanlineLayerProps) -> Element {
    let grid_svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{g}" height="{g}"><rect width="{g}" height="{g}" fill="none"/><path d="M {g} 0 L 0 0 0 {g}" fill="none" stroke="rgba(255,255,255,{go})" stroke-width="0.5"/></svg>"##,
        g = GRID_SIZE,
        go = GRID_OPACITY,
    );
    let grid_data_uri = format!(
        "url(\"data:image/svg+xml,{}\")",
        urlencoding_minimal(&grid_svg)
    );

    let scanline_svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="{s}"><rect width="1" height="1" fill="rgba(255,255,255,{so})"/></svg>"##,
        s = SCANLINE_SPACING,
        so = SCANLINE_OPACITY,
    );
    let scanline_data_uri = format!(
        "url(\"data:image/svg+xml,{}\")",
        urlencoding_minimal(&scanline_svg)
    );

    let combined_bg = format!(
        "background-image: {}, {}; background-repeat: repeat;",
        grid_data_uri, scanline_data_uri
    );

    rsx! {
        div {
            class: format!("fixed inset-0 z-[1] pointer-events-none {}", props.class),
            style: "{combined_bg}",
        }
    }
}

/// Minimal URL encoding for inline SVG data URIs.
///
/// Only escapes characters that break data: URIs — avoids pulling in a full
/// percent-encoding crate.
fn urlencoding_minimal(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 32);
    for c in input.chars() {
        match c {
            '#' => out.push_str("%23"),
            '<' => out.push_str("%3C"),
            '>' => out.push_str("%3E"),
            '"' => out.push_str("%22"),
            '\n' => out.push_str("%0A"),
            '\r' => {}
            _ => out.push(c),
        }
    }
    out
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlencoding_minimal_escapes_special_chars() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg">#test</svg>"#;
        let encoded = urlencoding_minimal(input);
        assert!(!encoded.contains('<'));
        assert!(!encoded.contains('>'));
        assert!(!encoded.contains('"'));
        assert!(!encoded.contains('#'));
        assert!(encoded.contains("%3C"));
        assert!(encoded.contains("%3E"));
        assert!(encoded.contains("%22"));
        assert!(encoded.contains("%23"));
    }

    #[test]
    fn test_urlencoding_preserves_normal_chars() {
        let input = "hello world 123";
        assert_eq!(urlencoding_minimal(input), "hello world 123");
    }

    #[test]
    fn test_grid_constants() {
        assert_eq!(GRID_SIZE, 2);
        assert!(GRID_OPACITY > 0.0 && GRID_OPACITY < 0.1);
    }

    #[test]
    fn test_scanline_constants() {
        assert_eq!(SCANLINE_SPACING, 4);
        assert!(SCANLINE_OPACITY > 0.0 && SCANLINE_OPACITY < 0.1);
    }
}
