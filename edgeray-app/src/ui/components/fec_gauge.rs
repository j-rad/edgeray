//! FEC Gauge Component
//!
//! Visualizes Forward Error Correction metrics:
//! - Overhead percentage (bandwidth cost)
//! - Recovery rate (packet loss mitigated)
//! - Active state

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FecGaugeProps {
    pub overhead_percent: f32,
    pub recovery_rate: f32,
    pub active: bool,
}

#[component]
pub fn FecGauge(props: FecGaugeProps) -> Element {
    // Map 0-50% overhead to -90 to +90 degrees for needle
    let rotation = (props.overhead_percent / 50.0 * 180.0 - 90.0).clamp(-90.0, 90.0);

    let color_class = if props.active {
        "text-emerald-400"
    } else {
        "text-slate-500"
    };
    let stroke_class = if props.active {
        "stroke-emerald-500"
    } else {
        "stroke-slate-700"
    };

    // Arc length for SVG dasharray (approximate for semi-circle)
    // r=40, circumference = 2*pi*40 = 251. Semi-circle = 126.
    let max_dash = 126.0;
    let dash_offset = max_dash - (max_dash * (props.overhead_percent / 50.0).clamp(0.0, 1.0));

    rsx! {
        div {
            class: "flex flex-col items-center justify-center",

            // Gauge
            div {
                class: "relative w-40 h-24 overflow-hidden flex justify-center",

                // SVG Arc
                svg {
                    view_box: "0 0 100 55",
                    class: "w-full h-full",

                    // Background track
                    path {
                        d: "M 10 50 A 40 40 0 0 1 90 50",
                        fill: "none",
                        stroke: "#1e293b", // slate-800
                        "stroke-width": "8",
                        "stroke-linecap": "round",
                    }

                    // Active value arc
                    path {
                        d: "M 10 50 A 40 40 0 0 1 90 50",
                        fill: "none",
                        class: "{stroke_class} transition-all duration-1000 ease-out",
                        "stroke-width": "8",
                        "stroke-linecap": "round",
                        "stroke-dasharray": "{max_dash}",
                        "stroke-dashoffset": "{dash_offset}",
                    }
                }

                // Needle
                div {
                    class: "absolute bottom-0 left-1/2 w-1 h-[80%] bg-white origin-bottom transition-all duration-500 ease-out -ml-0.5 rounded-full shadow-lg z-10",
                    style: "transform: rotate({rotation}deg); transform-origin: bottom center; bottom: 4px;"
                }

                // Pivot
                div {
                    class: "absolute bottom-0 left-1/2 w-4 h-4 bg-slate-900 rounded-full border-2 border-slate-600 -ml-2 translate-y-2 z-20",
                }
            }

            // Value
            div {
                class: "text-center -mt-2",
                div {
                    class: "text-2xl font-bold {color_class}",
                    "{props.overhead_percent:.1}%"
                }
                div {
                    class: "text-[10px] text-slate-500 uppercase tracking-wider font-semibold",
                    "Overhead"
                }
            }
        }
    }
}
