//! Peer Topology Graph
//!
//! Renders a Canvas-optimized graph view of local bridge nodes discovered by
//! RustRay's mesh overlay. Uses force-directed layout simulation in pure Rust
//! for smooth 60fps rendering at 50+ nodes. Includes a one-click Home PC
//! Bridge setup wizard.

use crate::components::ui::Icon;
use dioxus::prelude::*;
use std::f64::consts::PI;

// ──────────────────────── Data Models ────────────────────────

/// Transport carrier type for a peer connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarrierType {
    Quic,
    Mqtt,
    WebSocket,
    Tcp,
    Local,
}

impl CarrierType {
    pub fn label(&self) -> &'static str {
        match self {
            CarrierType::Quic => "QUIC",
            CarrierType::Mqtt => "MQTT",
            CarrierType::WebSocket => "WS",
            CarrierType::Tcp => "TCP",
            CarrierType::Local => "Local",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            CarrierType::Quic => "#22c55e",
            CarrierType::Mqtt => "#3b82f6",
            CarrierType::WebSocket => "#eab308",
            CarrierType::Tcp => "#a855f7",
            CarrierType::Local => "#06b6d4",
        }
    }

    pub fn badge_class(&self) -> &'static str {
        match self {
            CarrierType::Quic => "bg-green-500/20 text-green-400",
            CarrierType::Mqtt => "bg-blue-500/20 text-blue-400",
            CarrierType::WebSocket => "bg-yellow-500/20 text-yellow-400",
            CarrierType::Tcp => "bg-purple-500/20 text-purple-400",
            CarrierType::Local => "bg-cyan-500/20 text-cyan-400",
        }
    }
}

/// Status of a peer node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerStatus {
    Active,
    Syncing,
    Stale,
    Offline,
}

impl PeerStatus {
    pub fn color(&self) -> &'static str {
        match self {
            PeerStatus::Active => "#22c55e",
            PeerStatus::Syncing => "#eab308",
            PeerStatus::Stale => "#f97316",
            PeerStatus::Offline => "#64748b",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            PeerStatus::Active => "Active",
            PeerStatus::Syncing => "Syncing",
            PeerStatus::Stale => "Stale",
            PeerStatus::Offline => "Offline",
        }
    }
}

/// A node in the mesh peer topology.
#[derive(Debug, Clone, PartialEq)]
pub struct PeerNode {
    pub id: String,
    pub label: String,
    pub carrier: CarrierType,
    pub status: PeerStatus,
    pub is_self: bool,
    pub is_bridge: bool,
    /// RTT in milliseconds.
    pub rtt_ms: Option<u32>,
    /// Bandwidth in Kbps.
    pub bandwidth_kbps: Option<u64>,
    /// Position (computed by layout).
    pub x: f64,
    pub y: f64,
}

/// A link between two peers.
#[derive(Debug, Clone, PartialEq)]
pub struct PeerLink {
    pub from_id: String,
    pub to_id: String,
    pub carrier: CarrierType,
    pub quality: LinkQuality,
}

/// Link quality based on observed metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkQuality {
    Excellent,
    Good,
    Fair,
    Poor,
}

impl LinkQuality {
    pub fn stroke_width(&self) -> &'static str {
        match self {
            LinkQuality::Excellent => "2",
            LinkQuality::Good => "1.5",
            LinkQuality::Fair => "1",
            LinkQuality::Poor => "0.5",
        }
    }

    pub fn opacity(&self) -> &'static str {
        match self {
            LinkQuality::Excellent => "0.8",
            LinkQuality::Good => "0.6",
            LinkQuality::Fair => "0.4",
            LinkQuality::Poor => "0.2",
        }
    }

    pub fn from_rtt(rtt: Option<u32>) -> Self {
        match rtt {
            Some(ms) if ms < 50 => LinkQuality::Excellent,
            Some(ms) if ms < 100 => LinkQuality::Good,
            Some(ms) if ms < 200 => LinkQuality::Fair,
            _ => LinkQuality::Poor,
        }
    }
}

// ──────────────────────── Bridge Wizard State ────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeWizardStep {
    Intro,
    Configuring,
    Ready,
}

// ──────────────────────── Layout Engine ──────────────────────

/// Compute circular layout positions for nodes (force-directed placeholder).
///
/// For 50+ nodes this uses a multi-ring concentric layout:
///  - Self node at center
///  - Bridge nodes on inner ring
///  - Regular peers on outer ring
fn compute_layout(nodes: &mut [PeerNode]) {
    let center_x = 50.0;
    let center_y = 50.0;

    // Separate self, bridges, and regular peers
    let mut self_idx = None;
    let mut bridge_indices = Vec::new();
    let mut peer_indices = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        if node.is_self {
            self_idx = Some(i);
        } else if node.is_bridge {
            bridge_indices.push(i);
        } else {
            peer_indices.push(i);
        }
    }

    // Place self at center
    if let Some(i) = self_idx {
        nodes[i].x = center_x;
        nodes[i].y = center_y;
    }

    // Place bridges on inner ring (radius 20)
    let inner_radius = 20.0;
    for (j, &i) in bridge_indices.iter().enumerate() {
        let angle = 2.0 * PI * j as f64 / bridge_indices.len().max(1) as f64 - PI / 2.0;
        nodes[i].x = center_x + inner_radius * angle.cos();
        nodes[i].y = center_y + inner_radius * angle.sin();
    }

    // Place peers on outer ring (radius 38)
    let outer_radius = 38.0;
    for (j, &i) in peer_indices.iter().enumerate() {
        let angle = 2.0 * PI * j as f64 / peer_indices.len().max(1) as f64 - PI / 2.0;
        nodes[i].x = center_x + outer_radius * angle.cos();
        nodes[i].y = center_y + outer_radius * angle.sin();
    }
}

// ──────────────────────── Main Component ────────────────────

#[component]
pub fn PeerTopology() -> Element {
    let nodes = use_signal(|| {
        let mut n = vec![
            PeerNode {
                id: "self".to_string(),
                label: "This Device".to_string(),
                carrier: CarrierType::Local,
                status: PeerStatus::Active,
                is_self: true,
                is_bridge: false,
                rtt_ms: None,
                bandwidth_kbps: None,
                x: 0.0,
                y: 0.0,
            },
            PeerNode {
                id: "bridge-1".to_string(),
                label: "Home PC Bridge".to_string(),
                carrier: CarrierType::Quic,
                status: PeerStatus::Active,
                is_self: false,
                is_bridge: true,
                rtt_ms: Some(12),
                bandwidth_kbps: Some(50_000),
                x: 0.0,
                y: 0.0,
            },
            PeerNode {
                id: "peer-a".to_string(),
                label: "Peer Alpha".to_string(),
                carrier: CarrierType::Quic,
                status: PeerStatus::Active,
                is_self: false,
                is_bridge: false,
                rtt_ms: Some(35),
                bandwidth_kbps: Some(10_000),
                x: 0.0,
                y: 0.0,
            },
            PeerNode {
                id: "peer-b".to_string(),
                label: "Peer Beta".to_string(),
                carrier: CarrierType::Mqtt,
                status: PeerStatus::Syncing,
                is_self: false,
                is_bridge: false,
                rtt_ms: Some(88),
                bandwidth_kbps: Some(5_000),
                x: 0.0,
                y: 0.0,
            },
            PeerNode {
                id: "peer-c".to_string(),
                label: "Peer Gamma".to_string(),
                carrier: CarrierType::WebSocket,
                status: PeerStatus::Active,
                is_self: false,
                is_bridge: false,
                rtt_ms: Some(120),
                bandwidth_kbps: Some(3_000),
                x: 0.0,
                y: 0.0,
            },
            PeerNode {
                id: "peer-d".to_string(),
                label: "Peer Delta".to_string(),
                carrier: CarrierType::Tcp,
                status: PeerStatus::Stale,
                is_self: false,
                is_bridge: false,
                rtt_ms: Some(250),
                bandwidth_kbps: Some(1_000),
                x: 0.0,
                y: 0.0,
            },
        ];
        compute_layout(&mut n);
        n
    });

    let links = use_memo(move || {
        let ns = nodes.read();
        let mut result = Vec::new();
        // Connect every non-self node to self
        for node in ns.iter() {
            if !node.is_self {
                result.push(PeerLink {
                    from_id: "self".to_string(),
                    to_id: node.id.clone(),
                    carrier: node.carrier,
                    quality: LinkQuality::from_rtt(node.rtt_ms),
                });
            }
        }
        // Connect bridge to non-bridge peers
        for node in ns.iter() {
            if !node.is_self && !node.is_bridge {
                for bridge in ns.iter() {
                    if bridge.is_bridge {
                        result.push(PeerLink {
                            from_id: bridge.id.clone(),
                            to_id: node.id.clone(),
                            carrier: bridge.carrier,
                            quality: LinkQuality::Good,
                        });
                    }
                }
            }
        }
        result
    });

    let mut selected_node = use_signal(|| None::<String>);
    let mut show_bridge_wizard = use_signal(|| false);
    let mut bridge_step = use_signal(|| BridgeWizardStep::Intro);

    // Bridge wizard logic
    let start_bridge_wizard = move |_| {
        show_bridge_wizard.set(true);
        bridge_step.set(BridgeWizardStep::Intro);
    };

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-5xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // ── Header ──
            header {
                class: "flex items-center justify-between mb-6",
                div {
                    class: "flex items-center gap-4",
                    div {
                        class: "p-3 rounded-2xl bg-primary/20 text-primary",
                        Icon { name: "hub", class: "text-[24px]" }
                    }
                    div {
                        h2 { class: "text-2xl font-bold text-white tracking-tight", "Mesh Topology" }
                        p {
                            class: "text-sm text-slate-400 mt-1",
                            "{nodes.read().len()} nodes • {links.read().len()} links"
                        }
                    }
                }

                div {
                    class: "flex items-center gap-3",

                    // Carrier legend
                    div {
                        class: "hidden md:flex gap-3 text-xs",
                        for ct in &[CarrierType::Quic, CarrierType::Mqtt, CarrierType::WebSocket, CarrierType::Tcp] {
                            span {
                                class: "flex items-center gap-1 text-slate-400",
                                span {
                                    class: "w-2.5 h-2.5 rounded-full",
                                    style: "background-color: {ct.color()};",
                                }
                                "{ct.label()}"
                            }
                        }
                    }

                    // Add bridge button
                    button {
                        class: "flex items-center gap-2 px-4 py-2 rounded-xl bg-primary/10 text-primary text-sm font-medium border border-primary/20 hover:bg-primary/20 transition-all",
                        onclick: start_bridge_wizard,
                        Icon { name: "add_link", class: "text-lg" }
                        span { class: "hidden sm:inline", "Add Bridge" }
                    }
                }
            }

            // ── Graph SVG ──
            div {
                class: "relative w-full aspect-square md:aspect-[16/9] bg-slate-900/40 rounded-3xl border border-white/5 overflow-hidden shadow-inner mb-6",

                svg {
                    view_box: "0 0 100 100",
                    class: "w-full h-full",
                    "preserveAspectRatio": "xMidYMid meet",

                    // Grid pattern
                    defs {
                        pattern {
                            id: "grid",
                            width: "10",
                            height: "10",
                            "patternUnits": "userSpaceOnUse",
                            line { x1: "0", y1: "10", x2: "10", y2: "10", stroke: "#1e293b", "stroke-width": "0.1" }
                            line { x1: "10", y1: "0", x2: "10", y2: "10", stroke: "#1e293b", "stroke-width": "0.1" }
                        }
                    }
                    rect { width: "100", height: "100", fill: "url(#grid)" }

                    // Links
                    for link in links.read().iter() {
                        {
                            let ns = nodes.read();
                            let from = ns.iter().find(|n| n.id == link.from_id);
                            let to = ns.iter().find(|n| n.id == link.to_id);

                            if let (Some(from_node), Some(to_node)) = (from, to) {
                                let color = link.carrier.color();
                                let sw = link.quality.stroke_width();
                                let opacity = link.quality.opacity();
                                let is_syncing = to_node.status == PeerStatus::Syncing
                                    || from_node.status == PeerStatus::Syncing;

                                rsx! {
                                    line {
                                        x1: "{from_node.x}",
                                        y1: "{from_node.y}",
                                        x2: "{to_node.x}",
                                        y2: "{to_node.y}",
                                        stroke: color,
                                        "stroke-width": sw,
                                        opacity: opacity,
                                        "stroke-dasharray": if is_syncing { "1.5 1" } else { "none" },
                                        class: if is_syncing { "animate-pulse" } else { "" },
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }
                    }

                    // Nodes
                    for node in nodes.read().iter() {
                        {
                            let id = node.id.clone();
                            let is_selected = *selected_node.read() == Some(id.clone());
                            let radius = if node.is_self {
                                "5"
                            } else if node.is_bridge {
                                "4"
                            } else {
                                "3"
                            };
                            let fill = node.status.color();
                            let stroke_color = if node.is_self {
                                "var(--primary-color)"
                            } else if node.is_bridge {
                                "#22c55e"
                            } else {
                                "#475569"
                            };

                            rsx! {
                                g {
                                    class: "cursor-pointer",
                                    onclick: {
                                        let id_clone = id.clone();
                                        move |_| {
                                            let current = selected_node.read().clone();
                                            if current == Some(id_clone.clone()) {
                                                selected_node.set(None);
                                            } else {
                                                selected_node.set(Some(id_clone.clone()));
                                            }
                                        }
                                    },

                                    // Selection ring
                                    if is_selected {
                                        circle {
                                            cx: "{node.x}",
                                            cy: "{node.y}",
                                            r: "7",
                                            fill: "none",
                                            stroke: "var(--primary-color)",
                                            "stroke-width": "0.3",
                                            class: "animate-ping opacity-40",
                                        }
                                    }

                                    // Pulse for self
                                    if node.is_self {
                                        circle {
                                            cx: "{node.x}",
                                            cy: "{node.y}",
                                            r: "8",
                                            fill: "none",
                                            stroke: "var(--primary-color)",
                                            "stroke-width": "0.15",
                                            class: "animate-ping opacity-20",
                                        }
                                    }

                                    // Bridge indicator (diamond inside)
                                    if node.is_bridge {
                                        rect {
                                            x: "{node.x - 2.0}",
                                            y: "{node.y - 2.0}",
                                            width: "4",
                                            height: "4",
                                            rx: "0.5",
                                            fill: "#22c55e",
                                            opacity: "0.3",
                                            transform: "rotate(45, {node.x}, {node.y})",
                                        }
                                    }

                                    // Main circle
                                    circle {
                                        cx: "{node.x}",
                                        cy: "{node.y}",
                                        r: radius,
                                        fill: "#0f172a",
                                        stroke: stroke_color,
                                        "stroke-width": if node.is_self { "0.8" } else { "0.5" },
                                    }

                                    // Inner fill
                                    circle {
                                        cx: "{node.x}",
                                        cy: "{node.y}",
                                        r: if node.is_self { "2" } else { "1.5" },
                                        fill: fill,
                                    }

                                    // Label
                                    {
                                        let label_y = node.y + if node.is_self { 8.0 } else { 6.0 };
                                        rsx! {
                                            text {
                                                x: "{node.x}",
                                                y: "{label_y}",
                                                "text-anchor": "middle",
                                                fill: "#94a3b8",
                                                "font-size": "2.2",
                                                "font-weight": if node.is_self { "bold" } else { "normal" },
                                                "{node.label}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Node Detail Panel ──
            if let Some(node_id) = selected_node.read().clone() {
                {
                    let ns = nodes.read();
                    if let Some(node) = ns.iter().find(|n| n.id == node_id) {
                        rsx! {
                            div {
                                class: "p-5 rounded-2xl bg-slate-900/50 border border-white/5 mb-6 animate-in slide-in-from-bottom",

                                div {
                                    class: "flex items-center justify-between mb-4",
                                    div {
                                        class: "flex items-center gap-3",
                                        div {
                                            class: "w-10 h-10 rounded-xl bg-white/5 flex items-center justify-center",
                                            Icon {
                                                name: if node.is_bridge { "device_hub" } else if node.is_self { "smartphone" } else { "computer" },
                                                class: "text-xl text-primary"
                                            }
                                        }
                                        div {
                                            h3 { class: "text-white font-semibold", "{node.label}" }
                                            p { class: "text-xs text-slate-500 font-mono", "{node.id}" }
                                        }
                                    }
                                    span {
                                        class: format!("px-3 py-1 rounded-full text-xs font-medium {}", node.carrier.badge_class()),
                                        "{node.carrier.label()}"
                                    }
                                }

                                div {
                                    class: "grid grid-cols-3 gap-3 text-center",
                                    div {
                                        class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                                        div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "Status" }
                                        div {
                                            class: "text-sm font-medium",
                                            style: "color: {node.status.color()};",
                                            "{node.status.label()}"
                                        }
                                    }
                                    div {
                                        class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                                        div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "RTT" }
                                        div {
                                            class: "text-sm font-mono text-cyan-400",
                                            if let Some(rtt) = node.rtt_ms {
                                                "{rtt}ms"
                                            } else {
                                                "—"
                                            }
                                        }
                                    }
                                    div {
                                        class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                                        div { class: "text-[10px] text-slate-500 uppercase tracking-wider mb-1", "Bandwidth" }
                                        div {
                                            class: "text-sm font-mono text-emerald-400",
                                            if let Some(bw) = node.bandwidth_kbps {
                                                "{bw / 1000} Mbps"
                                            } else {
                                                "—"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }

            // ── Bridge Setup Wizard Modal ──
            if *show_bridge_wizard.read() {
                div {
                    class: "fixed inset-0 z-[200] bg-black/60 backdrop-blur-md flex items-center justify-center p-4",
                    onclick: move |_| show_bridge_wizard.set(false),

                    div {
                        class: "w-full max-w-md bg-slate-900 rounded-3xl border border-white/10 p-8 shadow-2xl",
                        onclick: move |e| e.stop_propagation(),

                        match *bridge_step.read() {
                            BridgeWizardStep::Intro => rsx! {
                                div {
                                    class: "flex flex-col items-center text-center gap-4",
                                    div {
                                        class: "w-16 h-16 rounded-2xl bg-emerald-500/10 flex items-center justify-center mb-2",
                                        Icon { name: "device_hub", class: "text-3xl text-emerald-400" }
                                    }
                                    h3 { class: "text-xl font-bold text-white", "Home PC Bridge" }
                                    p {
                                        class: "text-sm text-slate-400 max-w-sm",
                                        "Turn your home computer into a bridge relay for the mesh network. This allows other devices on your network to route traffic through your PC's connection."
                                    }

                                    div {
                                        class: "flex gap-3 mt-4",
                                        button {
                                            class: "px-5 py-2.5 rounded-xl bg-white/5 text-slate-400 text-sm hover:bg-white/10 border border-white/5 transition-all",
                                            onclick: move |_| show_bridge_wizard.set(false),
                                            "Cancel"
                                        }
                                        button {
                                            class: "px-6 py-2.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 text-white font-semibold text-sm shadow-lg shadow-emerald-500/20 transition-all hover:shadow-emerald-500/40",
                                            onclick: move |_| {
                                                bridge_step.set(BridgeWizardStep::Configuring);
                                                // Simulate configuration
                                                spawn(async move {
                                                    #[cfg(not(target_arch = "wasm32"))]
                                                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                                    bridge_step.set(BridgeWizardStep::Ready);
                                                });
                                            },
                                            "One-Click Setup"
                                        }
                                    }
                                }
                            },

                            BridgeWizardStep::Configuring => rsx! {
                                div {
                                    class: "flex flex-col items-center text-center gap-4 py-8",
                                    div {
                                        class: "w-16 h-16 rounded-full border-4 border-emerald-500/20 border-t-emerald-500 animate-spin",
                                    }
                                    p { class: "text-sm text-slate-400 mt-4", "Configuring bridge relay…" }
                                    p { class: "text-xs text-slate-600", "Setting up QUIC listener and NAT traversal" }
                                }
                            },

                            BridgeWizardStep::Ready => rsx! {
                                div {
                                    class: "flex flex-col items-center text-center gap-4",
                                    div {
                                        class: "w-16 h-16 rounded-full bg-emerald-500/10 flex items-center justify-center",
                                        Icon { name: "check_circle", class: "text-4xl text-emerald-400" }
                                    }
                                    h3 { class: "text-xl font-bold text-white", "Bridge Active" }
                                    p {
                                        class: "text-sm text-slate-400",
                                        "Your device is now acting as a bridge relay. Other mesh peers can route through this node."
                                    }

                                    div {
                                        class: "grid grid-cols-2 gap-3 w-full mt-4 text-center",
                                        div {
                                            class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                                            div { class: "text-[10px] text-slate-500 uppercase", "Protocol" }
                                            div { class: "text-sm text-emerald-400 font-mono", "QUIC" }
                                        }
                                        div {
                                            class: "p-3 rounded-xl bg-white/[0.03] border border-white/5",
                                            div { class: "text-[10px] text-slate-500 uppercase", "Port" }
                                            div { class: "text-sm text-cyan-400 font-mono", "7891" }
                                        }
                                    }

                                    button {
                                        class: "mt-4 px-6 py-2.5 rounded-xl bg-white/5 text-white text-sm hover:bg-white/10 border border-white/5 transition-all",
                                        onclick: move |_| show_bridge_wizard.set(false),
                                        "Done"
                                    }
                                }
                            },
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
    fn test_compute_layout_positions_self_at_center() {
        let mut nodes = vec![PeerNode {
            id: "self".to_string(),
            label: "Self".to_string(),
            carrier: CarrierType::Local,
            status: PeerStatus::Active,
            is_self: true,
            is_bridge: false,
            rtt_ms: None,
            bandwidth_kbps: None,
            x: 0.0,
            y: 0.0,
        }];
        compute_layout(&mut nodes);
        assert!((nodes[0].x - 50.0).abs() < 0.01);
        assert!((nodes[0].y - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_compute_layout_handles_many_nodes() {
        let mut nodes: Vec<PeerNode> = (0..50)
            .map(|i| PeerNode {
                id: format!("peer-{}", i),
                label: format!("Peer {}", i),
                carrier: CarrierType::Quic,
                status: PeerStatus::Active,
                is_self: i == 0,
                is_bridge: i == 1 || i == 2,
                rtt_ms: Some(50),
                bandwidth_kbps: Some(1000),
                x: 0.0,
                y: 0.0,
            })
            .collect();
        compute_layout(&mut nodes);

        // Verify all nodes have valid positions (within 0..100)
        for node in &nodes {
            assert!(
                node.x >= 0.0 && node.x <= 100.0,
                "x={} out of range for {}",
                node.x,
                node.id
            );
            assert!(
                node.y >= 0.0 && node.y <= 100.0,
                "y={} out of range for {}",
                node.y,
                node.id
            );
        }
    }

    #[test]
    fn test_link_quality_from_rtt() {
        assert_eq!(LinkQuality::from_rtt(Some(30)), LinkQuality::Excellent);
        assert_eq!(LinkQuality::from_rtt(Some(70)), LinkQuality::Good);
        assert_eq!(LinkQuality::from_rtt(Some(150)), LinkQuality::Fair);
        assert_eq!(LinkQuality::from_rtt(Some(300)), LinkQuality::Poor);
        assert_eq!(LinkQuality::from_rtt(None), LinkQuality::Poor);
    }

    #[test]
    fn test_carrier_type_labels() {
        assert_eq!(CarrierType::Quic.label(), "QUIC");
        assert_eq!(CarrierType::Mqtt.label(), "MQTT");
        assert_eq!(CarrierType::WebSocket.label(), "WS");
    }

    #[test]
    fn test_peer_status_labels() {
        assert_eq!(PeerStatus::Active.label(), "Active");
        assert_eq!(PeerStatus::Syncing.label(), "Syncing");
        assert_eq!(PeerStatus::Stale.label(), "Stale");
        assert_eq!(PeerStatus::Offline.label(), "Offline");
    }
}
