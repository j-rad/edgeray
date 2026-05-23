//!
//! Visualizes the mesh network graph using Dioxus with high-performance CSS transforms.
//! Displays nodes, links, and real-time RTT metrics fetched via the BackendDriver.
//! Allows selecting an "Exit Node" for routing with one tap.

#[allow(unused_imports)]
use crate::components::ui::Icon;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::f64::consts::PI;

/// Mesh node representation for the graph
#[derive(Clone, PartialEq, Debug)]
pub struct MeshNode {
    pub id: String,
    pub label: String,
    pub address: String,
    pub x: f64,
    pub y: f64,
    pub is_exit: bool,
    pub is_connected: bool,
    pub rtt: Option<u32>,
    pub nat_type: String,
    pub carrier: Option<String>,
    pub safety_score: u8, // 0-100
}

/// Link between nodes
#[derive(Clone, PartialEq, Debug)]
pub struct MeshLink {
    pub source_id: String,
    pub target_id: String,
    pub rtt: Option<u32>,
    pub is_active: bool,
    pub quality: LinkQuality,
}

/// Link quality indicator
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LinkQuality {
    Excellent, // <50ms
    Good,      // 50-100ms
    Fair,      // 100-200ms
    Poor,      // >200ms
    Unknown,
}

impl LinkQuality {
    pub fn from_rtt(rtt: Option<u32>) -> Self {
        match rtt {
            Some(r) if r < 50 => LinkQuality::Excellent,
            Some(r) if r < 100 => LinkQuality::Good,
            Some(r) if r < 200 => LinkQuality::Fair,
            Some(_) => LinkQuality::Poor,
            None => LinkQuality::Unknown,
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            LinkQuality::Excellent => "#22c55e", // green-500
            LinkQuality::Good => "#84cc16",      // lime-500
            LinkQuality::Fair => "#eab308",      // yellow-500
            LinkQuality::Poor => "#ef4444",      // red-500
            LinkQuality::Unknown => "#6b7280",   // gray-500
        }
    }
}

/// Aggregate mesh safety status for kill-switch
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MeshSafetyStatus {
    /// All paths secure, full protection
    Secure,
    /// Some nodes degraded but protected
    Degraded,
    /// Critical - some traffic may be exposed
    Warning,
    /// Kill switch should activate
    Critical,
}

impl MeshSafetyStatus {
    pub fn from_nodes(nodes: &[MeshNode]) -> Self {
        if nodes.is_empty() {
            return MeshSafetyStatus::Critical;
        }

        let connected = nodes.iter().filter(|n| n.is_connected).count();
        let avg_safety: f64 = nodes
            .iter()
            .filter(|n| n.is_connected)
            .map(|n| n.safety_score as f64)
            .sum::<f64>()
            / connected.max(1) as f64;

        if connected == 0 {
            MeshSafetyStatus::Critical
        } else if avg_safety >= 80.0 {
            MeshSafetyStatus::Secure
        } else if avg_safety >= 60.0 {
            MeshSafetyStatus::Degraded
        } else if avg_safety >= 40.0 {
            MeshSafetyStatus::Warning
        } else {
            MeshSafetyStatus::Critical
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            MeshSafetyStatus::Secure => "#22c55e",
            MeshSafetyStatus::Degraded => "#eab308",
            MeshSafetyStatus::Warning => "#f97316",
            MeshSafetyStatus::Critical => "#ef4444",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            MeshSafetyStatus::Secure => "Secure",
            MeshSafetyStatus::Degraded => "Degraded",
            MeshSafetyStatus::Warning => "Warning",
            MeshSafetyStatus::Critical => "Critical",
        }
    }
}

/// Calculate circular layout positions for nodes
fn calculate_circular_layout(
    node_count: usize,
    center_x: f64,
    center_y: f64,
    radius: f64,
) -> Vec<(f64, f64)> {
    (0..node_count)
        .map(|i| {
            let angle = (i as f64 / node_count as f64) * 2.0 * PI - PI / 2.0;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            (x, y)
        })
        .collect()
}

/// Interactive Mesh Graph Component
#[component]
pub fn MeshTopology(
    nodes: Signal<Vec<MeshNode>>,
    links: Signal<Vec<MeshLink>>,
    active_exit_node_id: Signal<Option<String>>,
    on_select_exit_node: EventHandler<String>,
    on_node_details: EventHandler<String>,
) -> Element {
    // Calculate safety status from all nodes
    let safety_status = use_memo(move || MeshSafetyStatus::from_nodes(&nodes.read()));

    // Graph dimensions
    let graph_width = 400.0_f64;
    let graph_height = 400.0_f64;
    let center_x = graph_width / 2.0;
    let center_y = graph_height / 2.0;
    let radius = 140.0_f64;

    // Compute positions when nodes change
    let positioned_nodes = use_memo(move || {
        let nodes_read = nodes.read();
        let positions = calculate_circular_layout(nodes_read.len(), center_x, center_y, radius);

        nodes_read
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let (x, y) = positions.get(i).copied().unwrap_or((center_x, center_y));
                MeshNode {
                    x,
                    y,
                    ..node.clone()
                }
            })
            .collect::<Vec<_>>()
    });

    // Build position lookup for link rendering
    let node_positions = use_memo(move || {
        positioned_nodes
            .read()
            .iter()
            .map(|n| (n.id.clone(), (n.x, n.y)))
            .collect::<HashMap<_, _>>()
    });

    rsx! {
        div {
            class: "mesh-topology-container flex flex-col gap-4",

            // Safety Status Bar
            div {
                class: "mesh-safety-bar flex items-center justify-between px-4 py-3 rounded-xl",
                style: "background: linear-gradient(135deg, {safety_status.read().color()}22, {safety_status.read().color()}11);",

                div {
                    class: "flex items-center gap-3",
                    div {
                        class: "w-3 h-3 rounded-full animate-pulse",
                        style: "background-color: {safety_status.read().color()}; box-shadow: 0 0 12px {safety_status.read().color()};",
                    }
                    span {
                        class: "font-semibold",
                        style: "color: {safety_status.read().color()};",
                        "Mesh Status: {safety_status.read().label()}"
                    }
                }

                div {
                    class: "text-sm text-gray-500",
                    "{positioned_nodes.read().iter().filter(|n| n.is_connected).count()} / {positioned_nodes.read().len()} nodes connected"
                }
            }

            // Graph Container
            div {
                class: "mesh-graph relative w-full bg-gradient-to-br from-gray-900 via-gray-850 to-gray-900 rounded-2xl overflow-hidden shadow-2xl border border-gray-800/50",
                style: "height: 400px; min-height: 400px;",

                // Background Grid Pattern
                svg {
                    class: "absolute inset-0 w-full h-full opacity-10",
                    view_box: "0 0 400 400",
                    defs {
                        pattern {
                            id: "mesh-grid",
                            width: "20",
                            height: "20",
                            pattern_units: "userSpaceOnUse",
                            path {
                                d: "M 20 0 L 0 0 0 20",
                                fill: "none",
                                stroke: "#374151",
                                stroke_width: "0.5",
                            }
                        }
                    }
                    rect {
                        width: "100%",
                        height: "100%",
                        fill: "url(#mesh-grid)",
                    }
                }

                // Links Layer (SVG)
                svg {
                    class: "absolute inset-0 w-full h-full pointer-events-none",
                    view_box: "0 0 400 400",

                    // Render links with glow effect
                    for link in links.read().iter() {
                        {
                            let positions = node_positions.read();
                            let source_pos = positions.get(&link.source_id).copied().unwrap_or((200.0, 200.0));
                            let target_pos = positions.get(&link.target_id).copied().unwrap_or((200.0, 200.0));
                            let color = link.quality.color();
                            let opacity = if link.is_active { "0.8" } else { "0.3" };
                            let stroke_width = if link.is_active { "2" } else { "1" };

                            rsx! {
                                g {
                                    // Glow effect
                                    if link.is_active {
                                        line {
                                            x1: "{source_pos.0}",
                                            y1: "{source_pos.1}",
                                            x2: "{target_pos.0}",
                                            y2: "{target_pos.1}",
                                            stroke: "{color}",
                                            stroke_width: "6",
                                            opacity: "0.2",
                                            stroke_linecap: "round",
                                        }
                                    }
                                    // Main line
                                    line {
                                        x1: "{source_pos.0}",
                                        y1: "{source_pos.1}",
                                        x2: "{target_pos.0}",
                                        y2: "{target_pos.1}",
                                        stroke: "{color}",
                                        stroke_width: "{stroke_width}",
                                        opacity: "{opacity}",
                                        stroke_linecap: "round",
                                        stroke_dasharray: if link.is_active { "" } else { "4,4" },
                                    }
                                    // RTT label
                                    if let Some(rtt) = link.rtt {
                                        text {
                                            x: "{(source_pos.0 + target_pos.0) / 2.0}",
                                            y: "{(source_pos.1 + target_pos.1) / 2.0 - 5.0}",
                                            fill: "{color}",
                                            font_size: "9",
                                            text_anchor: "middle",
                                            "{rtt}ms"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Nodes Layer (HTML for better interactivity)
                for node in positioned_nodes.read().iter() {
                    {
                        let node_id = node.id.clone();
                        let is_exit = active_exit_node_id.read().as_ref() == Some(&node.id);
                        let node_for_select = node_id.clone();
                        let node_for_details = node_id.clone();

                        let node_class = if is_exit {
                            "bg-gradient-to-br from-green-500 to-emerald-600 border-green-400 text-white scale-110"
                        } else if node.is_connected {
                            "bg-gradient-to-br from-blue-600 to-indigo-700 border-blue-500/50 text-blue-100 hover:scale-105 hover:border-blue-400"
                        } else {
                            "bg-gray-700 border-gray-600 text-gray-400"
                        };

                        let status_dot_class = if node.is_connected { "bg-green-500" } else { "bg-gray-500" };
                        let label_class = if is_exit { "text-green-400" } else if node.is_connected { "text-gray-300" } else { "text-gray-500" };

                        let rtt_class = match LinkQuality::from_rtt(node.rtt) {
                            LinkQuality::Excellent => "bg-green-900/80 text-green-300 border-green-700",
                            LinkQuality::Good => "bg-lime-900/80 text-lime-300 border-lime-700",
                            LinkQuality::Fair => "bg-yellow-900/80 text-yellow-300 border-yellow-700",
                            _ => "bg-red-900/80 text-red-300 border-red-700",
                        };

                        rsx! {
                            div {
                                key: "{node_id}",
                                class: "absolute transition-all duration-300 ease-out cursor-pointer",
                                style: "left: {node.x}px; top: {node.y}px; transform: translate(-50%, -50%);",

                                // Node Circle
                                div {
                                    class: "relative group",
                                    onclick: move |_| on_select_exit_node.call(node_for_select.clone()),

                                    // Outer ring for exit node
                                    if is_exit {
                                        div {
                                            class: "absolute inset-[-4px] rounded-full animate-spin-slow",
                                            style: "background: conic-gradient(from 0deg, #22c55e, #3b82f6, #22c55e); animation: spin 3s linear infinite;",
                                        }
                                    }

                                    // Main node circle
                                    div {
                                        class: "relative w-12 h-12 rounded-full flex items-center justify-center border-2 shadow-lg transition-all duration-200 {node_class}",

                                        // Node icon/initial
                                        span {
                                            class: "font-bold text-sm select-none",
                                            "{node.label.chars().next().unwrap_or('?')}"
                                        }

                                        // Connection status dot
                                        div {
                                            class: "absolute -bottom-1 -right-1 w-3 h-3 rounded-full border-2 border-gray-900 {status_dot_class}",
                                        }
                                    }

                                    // Shield Icon for Secure Nodes
                                    if node.safety_score >= 80 {
                                        div {
                                            class: "absolute -top-2 -left-2 w-5 h-5 bg-gray-900 rounded-full flex items-center justify-center shadow-lg border border-emerald-500/50 z-20",
                                            title: "Vault Secured",
                                            Icon { name: "verified_user", class: "text-emerald-400 text-xs" }
                                        }
                                    }

                                    // Hover tooltip
                                    div {
                                        class: "absolute left-1/2 bottom-full mb-2 transform -translate-x-1/2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-50",

                                        div {
                                            class: "bg-gray-800/95 backdrop-blur-sm rounded-lg px-3 py-2 text-xs shadow-xl border border-gray-700/50 whitespace-nowrap",

                                            div { class: "font-semibold text-white mb-1", "{node.label}" }
                                            div { class: "text-gray-400", "{node.address}" }
                                            if let Some(rtt) = node.rtt {
                                                div {
                                                    class: "text-green-400 mt-1",
                                                    "RTT: {rtt}ms"
                                                }
                                            }
                                            if let Some(carrier) = &node.carrier {
                                                div { class: "text-blue-400", "Via: {carrier}" }
                                            }
                                        }
                                    }
                                }

                                // Node label
                                div {
                                    class: "absolute top-full left-1/2 transform -translate-x-1/2 mt-2 text-xs font-medium whitespace-nowrap {label_class}",
                                    "{node.label}"
                                }

                                // RTT Badge
                                if let Some(rtt) = node.rtt {
                                    div {
                                        class: "absolute -top-2 -right-3 px-1.5 py-0.5 text-[10px] font-medium rounded-full border {rtt_class}",
                                        "{rtt}ms"
                                    }
                                }

                                // Details button on hover
                                button {
                                    class: "absolute -bottom-2 left-1/2 transform -translate-x-1/2 opacity-0 group-hover:opacity-100 transition-opacity bg-gray-800 hover:bg-gray-700 text-gray-300 text-[10px] px-2 py-0.5 rounded-full border border-gray-700",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        on_node_details.call(node_for_details.clone());
                                    },
                                    "Details"
                                }
                            }
                        }
                    }
                }

                // Center Hub (Local Device)
                div {
                    class: "absolute left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2 z-20",

                    // Pulsing ring
                    div {
                        class: "absolute inset-[-8px] rounded-full bg-blue-500/20 animate-ping",
                    }

                    // Main hub
                    div {
                        class: "relative w-16 h-16 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-full flex items-center justify-center border-4 border-blue-400/50 shadow-2xl",
                        style: "box-shadow: 0 0 30px rgba(59, 130, 246, 0.5);",

                        span { class: "text-white font-bold text-sm", "YOU" }
                    }
                }

                // Legend
                div {
                    class: "absolute bottom-3 left-3 flex items-center gap-4 text-[10px] text-gray-500",

                    div {
                        class: "flex items-center gap-1",
                        div { class: "w-2 h-2 rounded-full bg-green-500" }
                        span { "Exit" }
                    }
                    div {
                        class: "flex items-center gap-1",
                        div { class: "w-2 h-2 rounded-full bg-blue-500" }
                        span { "Connected" }
                    }
                    div {
                        class: "flex items-center gap-1",
                        div { class: "w-2 h-2 rounded-full bg-gray-500" }
                        span { "Offline" }
                    }
                }
            }

            // Quick Actions
            div {
                class: "flex gap-2",

                button {
                    class: "flex-1 py-3 px-4 bg-gradient-to-r from-green-600 to-emerald-600 text-white font-medium rounded-xl shadow-lg hover:shadow-green-500/20 hover:scale-[1.02] transition-all flex items-center justify-center gap-2",
                    disabled: active_exit_node_id.read().is_none(),
                    "🚀 Route via Exit"
                }

                button {
                    class: "py-3 px-4 bg-gray-800 text-gray-300 font-medium rounded-xl border border-gray-700 hover:bg-gray-750 hover:border-gray-600 transition-all",
                    "⟳ Refresh"
                }
            }
        }
    }
}

/// Minimal node card for list view alternative
#[component]
pub fn MeshNodeCard(
    node: MeshNode,
    is_exit: bool,
    on_select: EventHandler<String>,
    on_details: EventHandler<String>,
) -> Element {
    let quality = LinkQuality::from_rtt(node.rtt);
    let status_class = if node.is_connected {
        "bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]"
    } else {
        "bg-gray-500"
    };

    rsx! {
        div {
            class: "mesh-node-card relative bg-white/5 backdrop-blur-sm rounded-xl border border-white/10 p-4 hover:border-blue-500/50 transition-all cursor-pointer",
            onclick: move |_| on_select.call(node.id.clone()),

            // Exit indicator
            if is_exit {
                div {
                    class: "absolute -top-1 -right-1 w-4 h-4 bg-green-500 rounded-full border-2 border-gray-900 flex items-center justify-center",
                    span { class: "text-[8px] text-white", "✓" }
                }
            }

            // Shield indicator for list view
            if node.safety_score >= 80 {
                 div {
                    class: "absolute -top-1 -left-1 w-5 h-5 bg-gray-900 rounded-full border border-emerald-500/50 flex items-center justify-center shadow-sm z-10",
                    Icon { name: "verified_user", class: "text-emerald-400 text-[10px]" }
                }
            }

            div {
                class: "flex items-center justify-between",

                div {
                    class: "flex items-center gap-3",

                    // Status indicator
                    div {
                        class: "w-3 h-3 rounded-full {status_class}",
                    }

                    div {
                        div {
                            class: "font-semibold text-gray-100",
                            "{node.label}"
                        }
                        div {
                            class: "text-xs text-gray-500",
                            "{node.address}"
                        }
                    }
                }

                // RTT / Quality
                if let Some(rtt) = node.rtt {
                    div {
                        class: "text-right",
                        div {
                            class: "font-medium",
                            style: "color: {quality.color()};",
                            "{rtt}ms"
                        }
                        if let Some(carrier) = &node.carrier {
                            div {
                                class: "text-xs text-gray-500",
                                "{carrier}"
                            }
                        }
                    }
                }
            }
        }
    }
}
