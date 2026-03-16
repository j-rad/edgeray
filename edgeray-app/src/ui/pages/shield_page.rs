//! Shield Status / Censorship Diagnostics Page
//!
//! Provides a visual "Censorship Map" showing where blocks are occurring
//! (Local, DNS, Server) and integrates the FEC gauge for active error correction.

use crate::components::ui::Icon;
use crate::ui::components::fec_gauge::FecGauge;
use dioxus::prelude::*;

// ──────────────────────── Block Origin ────────────────────────

/// Where a censorship block is detected in the connection pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockOrigin {
    /// Client-side / local firewall or DPI
    Local,
    /// DNS poisoning or hijacking
    Dns,
    /// Server-side block or IP blacklist
    Server,
    /// No block detected
    Clear,
}

impl BlockOrigin {
    pub fn label(&self) -> &'static str {
        match self {
            BlockOrigin::Local => "Local DPI",
            BlockOrigin::Dns => "DNS Poisoning",
            BlockOrigin::Server => "Server Block",
            BlockOrigin::Clear => "Clear",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            BlockOrigin::Local => "text-red-400",
            BlockOrigin::Dns => "text-amber-400",
            BlockOrigin::Server => "text-orange-400",
            BlockOrigin::Clear => "text-emerald-400",
        }
    }

    pub fn bg_class(&self) -> &'static str {
        match self {
            BlockOrigin::Local => "bg-red-500/10 border-red-500/20",
            BlockOrigin::Dns => "bg-amber-500/10 border-amber-500/20",
            BlockOrigin::Server => "bg-orange-500/10 border-orange-500/20",
            BlockOrigin::Clear => "bg-emerald-500/10 border-emerald-500/20",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            BlockOrigin::Local => "shield",
            BlockOrigin::Dns => "dns",
            BlockOrigin::Server => "cloud_off",
            BlockOrigin::Clear => "check_circle",
        }
    }

    pub fn svg_color(&self) -> &'static str {
        match self {
            BlockOrigin::Local => "#f87171",
            BlockOrigin::Dns => "#fbbf24",
            BlockOrigin::Server => "#fb923c",
            BlockOrigin::Clear => "#34d399",
        }
    }
}

// ──────────────────────── Pipeline Node ──────────────────────

/// A node in the censorship detection pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineNode {
    pub label: String,
    pub origin: BlockOrigin,
    pub detail: String,
    pub latency_ms: Option<u32>,
}

// ──────────────────────── Shield Page ────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct ShieldPageProps {
    #[props(default)]
    pub on_back: Option<EventHandler<()>>,
}

/// Shield Status page – visual censorship forensics dashboard.
///
/// Shows a pipeline diagram of the connection path with indicators
/// at each stage (Local → DNS → Transport → Server) plus an
/// integrated FEC gauge for error correction monitoring.
#[component]
pub fn ShieldPage(props: ShieldPageProps) -> Element {
    // Simulated pipeline state — in production, populated by the diagnostics engine
    let pipeline = use_signal(|| {
        vec![
            PipelineNode {
                label: "Client Egress".to_string(),
                origin: BlockOrigin::Clear,
                detail: "No local DPI detected".to_string(),
                latency_ms: Some(2),
            },
            PipelineNode {
                label: "DNS Resolution".to_string(),
                origin: BlockOrigin::Dns,
                detail: "Poisoned response detected for target domain — switched to DoH"
                    .to_string(),
                latency_ms: Some(85),
            },
            PipelineNode {
                label: "TLS Handshake".to_string(),
                origin: BlockOrigin::Clear,
                detail: "REALITY fingerprint accepted".to_string(),
                latency_ms: Some(120),
            },
            PipelineNode {
                label: "Server Relay".to_string(),
                origin: BlockOrigin::Clear,
                detail: "Upstream reachable".to_string(),
                latency_ms: Some(45),
            },
        ]
    });

    // FEC state
    let fec_overhead = use_signal(|| 12.5f32);
    let fec_recovery = use_signal(|| 98.2f32);
    let fec_active = use_signal(|| true);

    // Flow-J & Multiport Metrics
    let flow_j_active = use_signal(|| true);
    let multiport_sockets = use_signal(|| 16);
    let protection_level = use_signal(|| 85); // 0-100

    // Overall shield status
    let shield_status = use_memo(move || {
        let nodes = pipeline.read();
        let blocked_count = nodes
            .iter()
            .filter(|n| n.origin != BlockOrigin::Clear)
            .count();
        if blocked_count == 0 {
            ("Secure", "text-emerald-400", "shield", "bg-emerald-500/10")
        } else if blocked_count == 1 {
            (
                "Mitigated",
                "text-amber-400",
                "gpp_maybe",
                "bg-amber-500/10",
            )
        } else {
            ("Under Attack", "text-red-400", "gpp_bad", "bg-red-500/10")
        }
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-5xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // ── Header ──
            header {
                class: "flex items-center justify-between mb-8",

                div {
                    class: "flex items-center gap-4",
                    if let Some(on_back) = &props.on_back {
                        button {
                            class: "p-2 rounded-xl bg-white/5 hover:bg-white/10 text-slate-400 transition-all",
                            onclick: {
                                let handler = on_back.clone();
                                move |_| handler.call(())
                            },
                            Icon { name: "arrow_back", class: "text-xl" }
                        }
                    }
                    div {
                        class: format!(
                            "p-3 rounded-2xl {}",
                            shield_status.read().3
                        ),
                        Icon { name: shield_status.read().2, class: format!("text-[28px] {}", shield_status.read().1) }
                    }
                    div {
                        h2 { class: "text-2xl font-bold text-white tracking-tight", "Shield Status" }
                        p {
                            class: format!("text-sm font-semibold mt-1 {}", shield_status.read().1),
                            "{shield_status.read().0}"
                        }
                    }
                }

                // Status badge
                div {
                    class: format!(
                        "px-4 py-2 rounded-full text-xs font-semibold border {}",
                        if shield_status.read().0 == "Secure" {
                            "bg-emerald-500/10 text-emerald-400 border-emerald-500/20"
                        } else if shield_status.read().0 == "Mitigated" {
                            "bg-amber-500/10 text-amber-400 border-amber-500/20"
                        } else {
                            "bg-red-500/10 text-red-400 border-red-500/20"
                        }
                    ),
                    "{shield_status.read().0}"
                }
            }

            // ── Protection Level Gauge ──
            section {
                class: "mb-10 grid grid-cols-1 md:grid-cols-3 gap-6",

                // Gauge Card
                div {
                    class: "col-span-1 p-6 rounded-3xl bg-slate-900/30 border border-white/5 flex flex-col items-center justify-center relative overflow-hidden",
                    div { class: "absolute inset-0 bg-gradient-to-br from-primary/5 to-transparent pointer-events-none" }

                    div { class: "relative w-40 h-40 flex items-center justify-center",
                         svg {
                            view_box: "0 0 100 100",
                            class: "w-full h-full transform -rotate-90",
                            circle {
                                cx: "50", cy: "50", r: "45",
                                fill: "none", stroke: "#1e293b", "stroke-width": "8"
                            }
                            circle {
                                cx: "50", cy: "50", r: "45",
                                fill: "none", stroke: "#10b981", "stroke-width": "8",
                                "stroke-dasharray": "283",
                                "stroke-dashoffset": "{283.0 - (283.0 * (*protection_level.read() as f32 / 100.0))}",
                                "stroke-linecap": "round",
                                class: "transition-all duration-1000 ease-out"
                            }
                        }
                        div { class: "absolute flex flex-col items-center",
                            span { class: "text-3xl font-bold text-white", "{protection_level.read()}%" }
                            span { class: "text-[10px] text-slate-400 uppercase tracking-wider", "Protection" }
                        }
                    }
                }

                // Metrics Cards
                div { class: "col-span-2 grid grid-cols-2 gap-4",
                    div { class: "p-5 rounded-2xl bg-white/[0.03] border border-white/5 flex flex-col justify-between",
                        div { class: "flex justify-between items-start",
                            Icon { name: "water", class: "text-primary text-2xl" }
                            span { class: if *flow_j_active.read() { "w-2 h-2 rounded-full bg-emerald-500" } else { "w-2 h-2 rounded-full bg-slate-600" } }
                        }
                        div {
                            div { class: "text-2xl font-bold text-white mt-2", if *flow_j_active.read() { "Active" } else { "Inactive" } }
                            div { class: "text-xs text-slate-500 mt-1", "Flow-J Protocol" }
                        }
                    }

                    div { class: "p-5 rounded-2xl bg-white/[0.03] border border-white/5 flex flex-col justify-between",
                        div { class: "flex justify-between items-start",
                            Icon { name: "hub", class: "text-cyan-400 text-2xl" }
                            span { class: "text-xs font-mono text-cyan-400/70", "Dynamic" }
                        }
                        div {
                            div { class: "text-2xl font-bold text-white mt-2", "{multiport_sockets.read()}" }
                            div { class: "text-xs text-slate-500 mt-1", "Multiport Sockets" }
                        }
                    }
                }
            }

            // ── Censorship Map (Pipeline Diagram) ──
            section {
                class: "mb-10",

                h3 {
                    class: "text-sm font-semibold text-slate-400 uppercase tracking-wider mb-4",
                    "Forensics Timeline"
                }

                // Pipeline Visualization
                div {
                    class: "relative",

                    // SVG connection lines
                    svg {
                        view_box: "0 0 800 80",
                        class: "w-full h-12 mb-2",

                        for i in 0..pipeline.read().len().saturating_sub(1) {
                            {
                                let nodes = pipeline.read();
                                let x1 = (i as f32 / (nodes.len() - 1) as f32 * 700.0 + 50.0) as i32;
                                let x2 = ((i + 1) as f32 / (nodes.len() - 1) as f32 * 700.0 + 50.0) as i32;
                                let color = nodes[i + 1].origin.svg_color();

                                rsx! {
                                    line {
                                        x1: "{x1}",
                                        y1: "40",
                                        x2: "{x2}",
                                        y2: "40",
                                        stroke: color,
                                        "stroke-width": "3",
                                        "stroke-linecap": "round",
                                        "stroke-dasharray": if nodes[i + 1].origin != BlockOrigin::Clear { "8 4" } else { "none" },
                                        class: if nodes[i + 1].origin != BlockOrigin::Clear { "animate-pulse" } else { "" },
                                    }
                                }
                            }
                        }

                        // Node dots
                        for (i, node) in pipeline.read().iter().enumerate() {
                            {
                                let x = (i as f32 / (pipeline.read().len() - 1).max(1) as f32 * 700.0 + 50.0) as i32;
                                let color = node.origin.svg_color();
                                let is_blocked = node.origin != BlockOrigin::Clear;

                                rsx! {
                                    // Pulse ring for blocked nodes
                                    if is_blocked {
                                        circle {
                                            cx: "{x}",
                                            cy: "40",
                                            r: "18",
                                            fill: "none",
                                            stroke: color,
                                            "stroke-width": "1",
                                            class: "animate-ping opacity-30",
                                        }
                                    }

                                    // Main dot
                                    circle {
                                        cx: "{x}",
                                        cy: "40",
                                        r: "10",
                                        fill: "#0f172a",
                                        stroke: color,
                                        "stroke-width": "3",
                                    }

                                    // Inner indicator
                                    circle {
                                        cx: "{x}",
                                        cy: "40",
                                        r: "4",
                                        fill: color,
                                    }
                                }
                            }
                        }
                    }

                    // Node cards
                    div {
                        class: "grid grid-cols-2 md:grid-cols-4 gap-3 mt-4",

                        for node in pipeline.read().iter() {
                            div {
                                class: format!(
                                    "flex flex-col gap-2 p-4 rounded-2xl border transition-all {} hover:scale-[1.02]",
                                    node.origin.bg_class()
                                ),

                                // Top row
                                div {
                                    class: "flex items-center gap-2",
                                    Icon { name: node.origin.icon_name(), class: format!("text-lg {}", node.origin.color_class()) }
                                    span { class: "text-xs font-semibold text-white", "{node.label}" }
                                }

                                // Status
                                span {
                                    class: format!("text-xs font-medium {}", node.origin.color_class()),
                                    "{node.origin.label()}"
                                }

                                // Detail
                                p { class: "text-[11px] text-slate-400 leading-relaxed", "{node.detail}" }

                                // Latency
                                if let Some(ms) = node.latency_ms {
                                    div {
                                        class: "flex items-center gap-1 mt-1",
                                        Icon { name: "timer", class: "text-[12px] text-slate-500" }
                                        span { class: "text-[10px] text-slate-500 font-mono", "{ms}ms" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── FEC Gauge Section ──
            section {
                class: "mb-8",

                h3 {
                    class: "text-sm font-semibold text-slate-400 uppercase tracking-wider mb-4",
                    "Forward Error Correction"
                }

                div {
                    class: "flex flex-col md:flex-row items-center gap-8 p-6 rounded-3xl bg-slate-900/30 border border-white/5",

                    // FEC Gauge
                    div {
                        class: "flex-shrink-0",
                        FecGauge {
                            overhead_percent: *fec_overhead.read(),
                            recovery_rate: *fec_recovery.read(),
                            active: *fec_active.read(),
                        }
                    }

                    // FEC Details
                    div {
                        class: "flex-1 grid grid-cols-2 gap-4 text-sm",

                        div {
                            class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                            div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "Mode" }
                            div { class: "text-white font-medium", "Reed-Solomon (8,4)" }
                        }
                        div {
                            class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                            div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "Packets Recovered" }
                            div { class: "text-emerald-400 font-mono", "1,847" }
                        }
                        div {
                            class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                            div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "Bandwidth Overhead" }
                            div { class: "text-amber-400 font-mono", "{fec_overhead.read():.1}%" }
                        }
                        div {
                            class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                            div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "Irrecoverable" }
                            div { class: "text-slate-300 font-mono", "12" }
                        }
                    }
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
    fn test_block_origin_labels() {
        assert_eq!(BlockOrigin::Local.label(), "Local DPI");
        assert_eq!(BlockOrigin::Dns.label(), "DNS Poisoning");
        assert_eq!(BlockOrigin::Server.label(), "Server Block");
        assert_eq!(BlockOrigin::Clear.label(), "Clear");
    }

    #[test]
    fn test_block_origin_colors_not_empty() {
        for origin in &[
            BlockOrigin::Local,
            BlockOrigin::Dns,
            BlockOrigin::Server,
            BlockOrigin::Clear,
        ] {
            assert!(!origin.color_class().is_empty());
            assert!(!origin.bg_class().is_empty());
            assert!(!origin.svg_color().is_empty());
        }
    }

    #[test]
    fn test_pipeline_node_construction() {
        let node = PipelineNode {
            label: "Test".to_string(),
            origin: BlockOrigin::Dns,
            detail: "Poisoned".to_string(),
            latency_ms: Some(100),
        };
        assert_eq!(node.origin, BlockOrigin::Dns);
        assert_eq!(node.latency_ms, Some(100));
    }
}
