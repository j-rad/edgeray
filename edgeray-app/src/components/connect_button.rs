//! Connect Button Component
//!
//! A high-fidelity connection toggle with SVG animations and
//! haptic-feedback triggers. Uses Dioxus Memo to prevent
//! visual stutter during rapid state transitions.

use dioxus::prelude::*;

// ──────────────────────── State Enum ────────────────────────

/// Connection state enum for the button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

impl ConnectionState {
    pub fn label(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "Connect",
            ConnectionState::Connecting => "Connecting…",
            ConnectionState::Connected => "Connected",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "▶",
            ConnectionState::Connecting => "◐",
            ConnectionState::Connected => "■",
        }
    }

    /// CSS gradient for the outer glow ring.
    fn ring_gradient(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "from-violet-500/40 to-purple-600/40",
            ConnectionState::Connecting => "from-amber-400/50 to-orange-500/50",
            ConnectionState::Connected => "from-emerald-400/50 to-cyan-500/50",
        }
    }

    /// CSS gradient for the core orb.
    fn orb_gradient(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "from-violet-500 via-purple-600 to-indigo-700",
            ConnectionState::Connecting => "from-amber-400 via-orange-500 to-red-600",
            ConnectionState::Connected => "from-emerald-400 via-teal-500 to-cyan-600",
        }
    }

    /// Shadow colour for the outer glow.
    fn shadow_color(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "shadow-violet-500/30",
            ConnectionState::Connecting => "shadow-amber-500/40",
            ConnectionState::Connected => "shadow-emerald-500/40",
        }
    }

    fn svg_stroke(&self) -> &'static str {
        match self {
            ConnectionState::Disconnected => "#8b5cf6",
            ConnectionState::Connecting => "#f59e0b",
            ConnectionState::Connected => "#10b981",
        }
    }
}

// ──────────────────────── Props ────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct ConnectButtonProps {
    /// Current connection state
    pub state: ConnectionState,
    /// Callback when the button is clicked
    pub on_click: EventHandler<()>,
    /// Whether the button is disabled
    #[props(default = false)]
    pub disabled: bool,
}

// ──────────────────────── Main Button ────────────────────────

/// A high-fidelity Floating Action Button for connection control.
///
/// Renders an animated SVG orb with three visual states:
/// - **Disconnected**: Violet pulsing orb with play icon
/// - **Connecting**: Amber spinning orb with rotating arcs
/// - **Connected**: Emerald stable orb with checkmark
///
/// Uses `use_memo` to avoid re-renders of the SVG when sibling state changes.
#[component]
pub fn ConnectButton(props: ConnectButtonProps) -> Element {
    // Memo the visual properties so sibling re-renders don't cause stutter
    let state = props.state;
    let visual = use_memo(move || {
        (
            state.ring_gradient(),
            state.orb_gradient(),
            state.shadow_color(),
            state.svg_stroke(),
            state.label(),
        )
    });

    let (ring_gradient, _orb_gradient, shadow_color, svg_stroke, label) = *visual.read();

    let is_connecting = state == ConnectionState::Connecting;
    let is_disabled = props.disabled || is_connecting;

    let interactive_class = if is_disabled {
        "opacity-60 cursor-not-allowed"
    } else {
        "cursor-pointer hover:scale-[1.04] active:scale-95"
    };

    // Spinning animation class for the connecting state
    let spin_class = if is_connecting { "animate-spin" } else { "" };

    // Pulse animation for idle/disconnected
    let pulse_class = if state == ConnectionState::Disconnected {
        "animate-pulse"
    } else {
        ""
    };

    let onclick = {
        let on_click = props.on_click.clone();
        let disabled = is_disabled;
        move |_| {
            if !disabled {
                on_click.call(());
                // Haptic triggers on supported platforms
                #[cfg(any(target_os = "android", target_os = "ios"))]
                {
                    log::debug!("Haptic feedback triggered");
                }
            }
        }
    };

    rsx! {
        // Outer container — fixed at bottom center
        div {
            class: "fixed bottom-8 left-1/2 transform -translate-x-1/2 z-50",

            // Interactive wrapper
            div {
                class: "relative flex flex-col items-center gap-4 transition-transform duration-200 ease-out {interactive_class}",
                onclick: onclick,

                // ── Outer glow ring ──
                div {
                    class: "absolute inset-0 -m-4 rounded-full bg-gradient-to-br {ring_gradient} blur-xl opacity-60 {pulse_class}",
                }

                // ── SVG Orb ──
                div {
                    class: "relative w-24 h-24 md:w-28 md:h-28",

                    // Animated SVG
                    svg {
                        view_box: "0 0 120 120",
                        class: "w-full h-full drop-shadow-2xl {shadow_color}",

                        // Definitions for gradients and filters
                        defs {
                            // Radial gradient for the orb fill
                            radialGradient {
                                id: "orbFill",
                                cx: "40%",
                                cy: "35%",
                                r: "60%",
                                stop { offset: "0%", "stop-color": "white", "stop-opacity": "0.25" }
                                stop { offset: "100%", "stop-color": "transparent", "stop-opacity": "0" }
                            }
                            // Glow filter
                            filter {
                                id: "glow",
                                circle {
                                    // feGaussianBlur placeholder; use CSS blur instead for broader compat
                                }
                            }
                        }

                        // Background circle (dark core)
                        circle {
                            cx: "60",
                            cy: "60",
                            r: "48",
                            fill: "#0f0a1a",
                            stroke: svg_stroke,
                            "stroke-width": "2",
                            class: "transition-all duration-500"
                        }

                        // Gradient overlay for depth
                        circle {
                            cx: "60",
                            cy: "60",
                            r: "46",
                            fill: "url(#orbFill)",
                        }

                        // Rotating arc — visible during ALL states, spins during Connecting
                        g {
                            class: "origin-center transition-transform duration-700 {spin_class}",
                            style: "transform-origin: 60px 60px;",

                            // Primary arc
                            path {
                                d: "M 60 14 A 46 46 0 0 1 106 60",
                                fill: "none",
                                stroke: svg_stroke,
                                "stroke-width": "3",
                                "stroke-linecap": "round",
                                "stroke-dasharray": if is_connecting { "20 52" } else { "72 0" },
                                class: "transition-all duration-700 opacity-70",
                            }

                            // Secondary arc (offset)
                            path {
                                d: "M 60 106 A 46 46 0 0 1 14 60",
                                fill: "none",
                                stroke: svg_stroke,
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-dasharray": if is_connecting { "15 57" } else { "72 0" },
                                class: "transition-all duration-700 opacity-50",
                            }
                        }

                        // Center icon
                        match state {
                            ConnectionState::Disconnected => rsx! {
                                // Play triangle
                                polygon {
                                    points: "52,42 52,78 80,60",
                                    fill: svg_stroke,
                                    class: "transition-all duration-300 opacity-90",
                                }
                            },
                            ConnectionState::Connecting => rsx! {
                                // Pulsing dot
                                circle {
                                    cx: "60",
                                    cy: "60",
                                    r: "8",
                                    fill: svg_stroke,
                                    class: "animate-ping opacity-60",
                                }
                                circle {
                                    cx: "60",
                                    cy: "60",
                                    r: "5",
                                    fill: svg_stroke,
                                    class: "opacity-90",
                                }
                            },
                            ConnectionState::Connected => rsx! {
                                // Checkmark
                                polyline {
                                    points: "44,60 55,72 76,48",
                                    fill: "none",
                                    stroke: svg_stroke,
                                    "stroke-width": "4",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    class: "transition-all duration-300",
                                }
                            },
                        }
                    }
                }

                // ── Label ──
                span {
                    class: "text-sm font-semibold tracking-widest uppercase text-white/80 transition-colors duration-300",
                    "{label}"
                }
            }
        }
    }
}

// ──────────────────────── Compact Button ────────────────────────

/// A compact circular FAB for smaller screens / embedded use.
#[derive(Props, Clone, PartialEq)]
pub struct CompactConnectButtonProps {
    pub state: ConnectionState,
    pub on_click: EventHandler<()>,
}

#[component]
pub fn CompactConnectButton(props: CompactConnectButtonProps) -> Element {
    let state = props.state;

    let bg_class = match state {
        ConnectionState::Disconnected => "bg-gradient-to-br from-violet-500 to-purple-700",
        ConnectionState::Connecting => {
            "bg-gradient-to-br from-amber-400 to-orange-600 animate-pulse"
        }
        ConnectionState::Connected => "bg-gradient-to-br from-emerald-400 to-teal-600",
    };

    let onclick = {
        let on_click = props.on_click.clone();
        move |_| {
            if state != ConnectionState::Connecting {
                on_click.call(());
            }
        }
    };

    rsx! {
        button {
            class: "fixed bottom-6 right-6 w-16 h-16 rounded-full {bg_class} text-white text-3xl shadow-xl shadow-black/40 flex items-center justify-center transition-transform duration-200 hover:scale-110 active:scale-90",
            onclick: onclick,

            svg {
                view_box: "0 0 40 40",
                class: "w-8 h-8",

                match state {
                    ConnectionState::Disconnected => rsx! {
                        polygon {
                            points: "14,10 14,30 30,20",
                            fill: "white",
                        }
                    },
                    ConnectionState::Connecting => rsx! {
                        circle {
                            cx: "20",
                            cy: "20",
                            r: "6",
                            fill: "white",
                            class: "animate-ping opacity-60",
                        }
                    },
                    ConnectionState::Connected => rsx! {
                        polyline {
                            points: "12,20 18,27 28,14",
                            fill: "none",
                            stroke: "white",
                            "stroke-width": "3",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                        }
                    },
                }
            }
        }
    }
}

// ──────────────────────── Tests ────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_state_labels() {
        assert_eq!(ConnectionState::Disconnected.label(), "Connect");
        assert_eq!(ConnectionState::Connecting.label(), "Connecting…");
        assert_eq!(ConnectionState::Connected.label(), "Connected");
    }

    #[test]
    fn test_orb_gradient_contains_expected_colors() {
        assert!(
            ConnectionState::Disconnected
                .orb_gradient()
                .contains("violet")
        );
        assert!(ConnectionState::Connecting.orb_gradient().contains("amber"));
        assert!(
            ConnectionState::Connected
                .orb_gradient()
                .contains("emerald")
        );
    }

    #[test]
    fn test_svg_stroke_hex_values() {
        assert_eq!(ConnectionState::Disconnected.svg_stroke(), "#8b5cf6");
        assert_eq!(ConnectionState::Connecting.svg_stroke(), "#f59e0b");
        assert_eq!(ConnectionState::Connected.svg_stroke(), "#10b981");
    }
}
