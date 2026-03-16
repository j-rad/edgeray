//! Responsive Telemetry Panel
//!
//! Displays 3 metric cards (Upload, Download, Latency) with real-time sparklines.
//! - Mobile: `flex-col` stack
//! - Desktop: `grid-cols-3` horizontal grid
//!
//! Each card uses `InteractiveSparkline` and `GlassCard` with `NeonBorder::Ignite`.

use dioxus::prelude::*;

use crate::components::glass_card::{CardPadding, GlassCard, NeonBorder};
use crate::components::sparkline::{InteractiveSparkline, SparklineBuffer};
use crate::components::theme;

/// Props for the telemetry panel.
#[derive(Props, Clone, PartialEq)]
pub struct TelemetryPanelProps {
    /// Upload throughput in bytes/sec.
    pub bps_out: f64,
    /// Download throughput in bytes/sec.
    pub bps_in: f64,
    /// Latency in milliseconds.
    pub ping_ms: f64,
    /// Upload sparkline history.
    pub upload_history: SparklineBuffer,
    /// Download sparkline history.
    pub download_history: SparklineBuffer,
    /// Latency sparkline history.
    pub ping_history: SparklineBuffer,
}

/// Format bytes/sec into a human-readable throughput string.
fn format_throughput(bps: f64) -> String {
    if bps >= 1_000_000_000.0 {
        format!("{:.1} Gbps", bps / 1_000_000_000.0)
    } else if bps >= 1_000_000.0 {
        format!("{:.1} Mbps", bps / 1_000_000.0)
    } else if bps >= 1_000.0 {
        format!("{:.1} Kbps", bps / 1_000.0)
    } else {
        format!("{:.0} bps", bps)
    }
}

/// Format latency in ms.
fn format_latency(ms: f64) -> String {
    if ms < 1.0 {
        format!("{:.1} ms", ms)
    } else {
        format!("{:.0} ms", ms)
    }
}

/// Responsive telemetry grid with 3 metric cards.
#[component]
pub fn ResponsiveTelemetryPanel(props: TelemetryPanelProps) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-3 gap-4",

            // ── Upload Card ─────────────────────────────────────────
            MetricCard {
                icon: "↑",
                label: "Upload",
                value: format_throughput(props.bps_out),
                color: theme::CYBER_PURPLE.to_string(),
                buffer: props.upload_history.clone(),
                sparkline_id: "upload",
            }

            // ── Download Card ───────────────────────────────────────
            MetricCard {
                icon: "↓",
                label: "Download",
                value: format_throughput(props.bps_in),
                color: theme::ELECTRIC_CYAN.to_string(),
                buffer: props.download_history.clone(),
                sparkline_id: "download",
            }

            // ── Latency Card ────────────────────────────────────────
            MetricCard {
                icon: "⏱",
                label: "Latency",
                value: format_latency(props.ping_ms),
                color: theme::EMERALD_SUCCESS.to_string(),
                buffer: props.ping_history.clone(),
                sparkline_id: "latency",
            }
        }
    }
}

/// Individual metric card with sparkline.
#[derive(Props, Clone, PartialEq)]
struct MetricCardProps {
    icon: String,
    label: &'static str,
    value: String,
    color: String,
    buffer: SparklineBuffer,
    sparkline_id: &'static str,
}

#[component]
fn MetricCard(props: MetricCardProps) -> Element {
    rsx! {
        GlassCard {
            border: NeonBorder::Ignite,
            padding: CardPadding::Compact,

            div {
                class: "flex flex-col gap-2",

                // Header row: icon + label + value
                div {
                    class: "flex items-center justify-between",

                    div {
                        class: "flex items-center gap-2",
                        span {
                            class: "text-lg",
                            style: "color: {props.color};",
                            "{props.icon}"
                        }
                        span {
                            class: "{theme::text::MUTED} uppercase tracking-wider",
                            "{props.label}"
                        }
                    }

                    span {
                        class: "text-base font-mono font-semibold text-white",
                        "{props.value}"
                    }
                }

                // Sparkline chart
                div {
                    class: "h-12",
                    InteractiveSparkline {
                        buffer: props.buffer.clone(),
                        width: 200.0,
                        height: 48.0,
                        color: props.color.clone(),
                        id: props.sparkline_id.to_string(),
                        class: "w-full h-full",
                    }
                }
            }
        }
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_throughput_bps() {
        assert_eq!(format_throughput(500.0), "500 bps");
    }

    #[test]
    fn test_format_throughput_kbps() {
        assert_eq!(format_throughput(1_500.0), "1.5 Kbps");
    }

    #[test]
    fn test_format_throughput_mbps() {
        assert_eq!(format_throughput(50_000_000.0), "50.0 Mbps");
    }

    #[test]
    fn test_format_throughput_gbps() {
        assert_eq!(format_throughput(2_500_000_000.0), "2.5 Gbps");
    }

    #[test]
    fn test_format_latency_sub_ms() {
        assert_eq!(format_latency(0.5), "0.5 ms");
    }

    #[test]
    fn test_format_latency_normal() {
        assert_eq!(format_latency(42.0), "42 ms");
    }

    #[test]
    fn test_format_throughput_zero() {
        assert_eq!(format_throughput(0.0), "0 bps");
    }
}
