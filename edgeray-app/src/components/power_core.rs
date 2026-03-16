// PowerCore Component — Obsidian "Heart" SVG Engine
//
// 3-Layer ring system with state-driven animations:
// - IDLE:       Breathing purple pulse (scale 0.95↔1.05 over 3s)
// - CONNECTING: High-speed cyan ring rotation (360° / 1.5s)
// - CONNECTED:  Cyan plasma hum + throughput-mapped spin speed
//
// Inner plasma uses an SVG mask with animated gradient sweep.

use dioxus::prelude::*;

/// Connection state controlling the PowerCore visual behavior.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerCoreState {
    Disconnected,
    Connecting,
    Secured,
}

#[derive(Props, Clone, PartialEq)]
pub struct PowerCoreProps {
    pub state: PowerCoreState,
    pub on_toggle: EventHandler<()>,
    /// Current throughput in bytes/sec — controls plasma spin speed when Secured.
    #[props(default = 0.0)]
    pub throughput_bps: f64,
}

/// Computes the outer ring rotation duration from throughput.
///
/// Maps `0 bps → 20s` (idle hum) down to `100 Mbps → 2s` (frantic spin).
/// Clamped to `[2, 20]` seconds.
fn ring_duration_from_throughput(bps: f64) -> f64 {
    let max_bps: f64 = 100_000_000.0; // 100 Mbps
    let ratio = (bps / max_bps).clamp(0.0, 1.0);
    // lerp from slow (20s) to fast (2s)
    20.0 - ratio * 18.0
}

/// Computes the plasma gradient sweep offset (0..360) from throughput.
///
/// Higher throughput = faster gradient sweep for the inner plasma.
fn plasma_sweep_duration(bps: f64) -> f64 {
    let max_bps: f64 = 100_000_000.0;
    let ratio = (bps / max_bps).clamp(0.0, 1.0);
    // lerp from slow (8s) to fast (1.5s)
    8.0 - ratio * 6.5
}

#[component]
pub fn PowerCore(props: PowerCoreProps) -> Element {
    let (outer_color, inner_glow_style, status_text) = match props.state {
        PowerCoreState::Disconnected => (
            "#64748b", // slate
            "box-shadow: 0 0 20px rgba(100,116,139,0.3);",
            "Disconnected",
        ),
        PowerCoreState::Connecting => (
            "#22d3ee", // cyan
            "box-shadow: 0 0 40px rgba(34,211,238,0.6);",
            "Connecting",
        ),
        PowerCoreState::Secured => (
            "#00F2FF", // Electric Cyan
            "box-shadow: 0 0 60px rgba(0,242,255,0.8);",
            "Secured",
        ),
    };

    let ring_dur = match props.state {
        PowerCoreState::Disconnected => 0.0, // no rotation
        PowerCoreState::Connecting => 1.5,   // fast fixed spin
        PowerCoreState::Secured => ring_duration_from_throughput(props.throughput_bps),
    };

    let plasma_dur = plasma_sweep_duration(props.throughput_bps);

    // Idle breathing scale anim (Disconnected only)
    let breathe_class = if props.state == PowerCoreState::Disconnected {
        "animate-pulse"
    } else {
        ""
    };

    rsx! {
        div {
            class: "flex flex-col items-center gap-4",

            // 3-Layer SVG Ring System
            button {
                class: "relative size-48 md:size-56 lg:size-64 cursor-pointer group transition-transform hover:scale-105 active:scale-95 {breathe_class}",
                onclick: move |_| props.on_toggle.call(()),

                svg {
                    class: "absolute inset-0 w-full h-full will-change-transform",
                    view_box: "0 0 200 200",
                    xmlns: "http://www.w3.org/2000/svg",

                    // ── Gradient Defs ────────────────────────────────────
                    defs {
                        // Outer ring gradient
                        linearGradient {
                            id: "pc-outer-grad",
                            x1: "0%", y1: "0%", x2: "100%", y2: "100%",
                            stop { offset: "0%", stop_color: "{outer_color}" }
                            stop { offset: "100%", stop_color: "#2563eb" }
                        }
                        // Plasma sweep gradient (rotating)
                        linearGradient {
                            id: "pc-plasma-grad",
                            x1: "0%", y1: "0%", x2: "100%", y2: "0%",
                            stop { offset: "0%", stop_color: "rgba(0,242,255,0.0)" }
                            stop { offset: "50%", stop_color: "rgba(0,242,255,0.4)" }
                            stop { offset: "100%", stop_color: "rgba(0,242,255,0.0)" }

                            // Plasma sweep animation (Secured only)
                            if props.state == PowerCoreState::Secured {
                                animateTransform {
                                    attribute_name: "gradientTransform",
                                    r#type: "rotate",
                                    values: "0 100 100;360 100 100",
                                    dur: "{plasma_dur}s",
                                    repeat_count: "indefinite",
                                }
                            }
                        }
                        // Circular mask for plasma
                        mask {
                            id: "pc-plasma-mask",
                            circle {
                                cx: "100", cy: "100", r: "55",
                                fill: "white",
                            }
                        }
                    }

                    // ── Layer 1: Outer Ring ──────────────────────────────
                    g {
                        // Ring rotation animation
                        if ring_dur > 0.0 {
                            animateTransform {
                                attribute_name: "transform",
                                r#type: "rotate",
                                from: "0 100 100",
                                to: "360 100 100",
                                dur: "{ring_dur}s",
                                repeat_count: "indefinite",
                            }
                        }

                        circle {
                            cx: "100", cy: "100", r: "90",
                            fill: "none",
                            stroke: "url(#pc-outer-grad)",
                            stroke_width: "4",
                            stroke_linecap: "round",
                            stroke_dasharray: "565",
                            stroke_dashoffset: if props.state == PowerCoreState::Disconnected { "565" } else { "140" },
                            class: "transition-all duration-1000",
                        }
                    }

                    // ── Layer 2: Middle Ring (subtle glow) ──────────────
                    circle {
                        cx: "100", cy: "100", r: "75",
                        fill: "none",
                        stroke: "rgba(34,211,238,0.2)",
                        stroke_width: "2",
                    }

                    // ── Layer 3: Inner Core + Plasma ────────────────────
                    circle {
                        cx: "100", cy: "100", r: "60",
                        fill: "#0f172a",
                    }

                    // Plasma flow (Secured only) — fills inner core with cyan gradient sweep
                    if props.state == PowerCoreState::Secured {
                        circle {
                            cx: "100", cy: "100", r: "200",
                            fill: "url(#pc-plasma-grad)",
                            mask: "url(#pc-plasma-mask)",
                            opacity: "0.7",
                        }
                    }
                }

                // Status Pulse — Radial Glow overlay
                if props.state == PowerCoreState::Secured {
                    div {
                        class: "absolute inset-0 rounded-full pointer-events-none animate-pulse-glow",
                        style: "{inner_glow_style}",
                    }
                }

                // Center Icon
                div {
                    class: "absolute inset-0 flex flex-col items-center justify-center text-white",
                    div {
                        class: "text-4xl font-bold font-mono",
                        if props.state == PowerCoreState::Disconnected {
                            "▶"
                        } else if props.state == PowerCoreState::Connecting {
                            "◐"
                        } else {
                            "■"
                        }
                    }
                }
            }

            // Status Text
            div {
                class: "text-sm font-mono text-slate-400",
                "{status_text}"
            }
        }
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_duration_idle() {
        let dur = ring_duration_from_throughput(0.0);
        assert!((dur - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_ring_duration_max_throughput() {
        let dur = ring_duration_from_throughput(100_000_000.0);
        assert!((dur - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_ring_duration_mid() {
        let dur = ring_duration_from_throughput(50_000_000.0);
        assert!(dur > 2.0 && dur < 20.0);
    }

    #[test]
    fn test_ring_duration_clamped_above_max() {
        let dur = ring_duration_from_throughput(500_000_000.0);
        assert!((dur - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_plasma_sweep_idle() {
        let dur = plasma_sweep_duration(0.0);
        assert!((dur - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_plasma_sweep_max() {
        let dur = plasma_sweep_duration(100_000_000.0);
        assert!((dur - 1.5).abs() < 0.01);
    }
}
