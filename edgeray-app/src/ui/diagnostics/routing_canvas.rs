//! Routing Logic Canvas — Touch-Responsive
//!
//! SVG-based node-link diagram showing traffic flow from
//! **App Source → Filter → Outbound**.
//!
//! Features:
//! - Node sizes scale with traffic volume
//! - Animated packet dots travel along links (speed ∝ volume)
//! - Tap a link to see `ConnectionStats` detail
//! - Flow-J links show Markov jitter visual noise on packet stream

use crate::components::ui::{GlassCard, Icon};
use dioxus::prelude::*;

// ─── Constants ─────────────────────────────────────────────────────────────────

/// Base node radius before traffic scaling.
const BASE_NODE_RADIUS: f32 = 18.0;
/// Minimum animated dot travel duration (seconds) for max-traffic links.
const MIN_DOT_DURATION_S: f32 = 0.5;
/// Maximum animated dot travel duration (seconds) for zero-traffic links.
const MAX_DOT_DURATION_S: f32 = 4.0;
/// Traffic volume considered "max" for scaling (10 MB/s).
const MAX_VOLUME_REFERENCE: u64 = 10_000_000;
/// Packet dot radius.
const DOT_RADIUS: f32 = 3.5;

// ─── Node Types ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    AppSource,
    Filter,
    Outbound,
}

impl NodeType {
    /// Column index (0 = left, 1 = center, 2 = right).
    pub fn column(&self) -> u8 {
        match self {
            NodeType::AppSource => 0,
            NodeType::Filter => 1,
            NodeType::Outbound => 2,
        }
    }

    /// Tailwind text color class.
    pub fn color_class(&self) -> &'static str {
        match self {
            NodeType::AppSource => "text-cyan-400",
            NodeType::Filter => "text-violet-400",
            NodeType::Outbound => "text-emerald-400",
        }
    }

    /// SVG fill color.
    pub fn fill_color(&self) -> &'static str {
        match self {
            NodeType::AppSource => "rgba(6,182,212,0.25)",
            NodeType::Filter => "rgba(139,92,246,0.25)",
            NodeType::Outbound => "rgba(16,185,129,0.25)",
        }
    }

    /// SVG stroke color.
    pub fn stroke_color(&self) -> &'static str {
        match self {
            NodeType::AppSource => "rgba(6,182,212,0.6)",
            NodeType::Filter => "rgba(139,92,246,0.6)",
            NodeType::Outbound => "rgba(16,185,129,0.6)",
        }
    }

    /// Material icon name.
    pub fn icon(&self) -> &'static str {
        match self {
            NodeType::AppSource => "apps",
            NodeType::Filter => "filter_alt",
            NodeType::Outbound => "cloud_upload",
        }
    }
}

// ─── Data Models ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct CanvasNode {
    pub id: String,
    pub label: String,
    pub node_type: NodeType,
    /// Traffic volume through this node (bytes/sec), used for scaling.
    pub traffic_volume: u64,
    /// Computed x position (populated by layout engine).
    pub x: f32,
    /// Computed y position.
    pub y: f32,
}

impl CanvasNode {
    /// Radius scaled by traffic volume.
    pub fn scaled_radius(&self) -> f32 {
        if self.traffic_volume == 0 {
            return BASE_NODE_RADIUS;
        }
        let log_scale = (self.traffic_volume as f64).log2() as f32 / 4.0;
        BASE_NODE_RADIUS + log_scale.min(12.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CanvasLink {
    pub source_id: String,
    pub target_id: String,
    pub traffic_volume: u64,
    /// Whether this link uses Flow-J (triggers jitter visualization).
    pub is_flowj: bool,
}

/// Connection statistics revealed on tap.
#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionStats {
    pub link_source: String,
    pub link_target: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub latency_ms: f32,
    pub buffer_usage_pct: f32,
    pub protocol: String,
}

// ─── Layout Engine ─────────────────────────────────────────────────────────────

/// Computes x/y positions for all nodes in a 3-column left-to-right layout.
///
/// Pure function — no side-effects, fully testable.
pub fn compute_layout(nodes: &mut [CanvasNode], canvas_width: f32, canvas_height: f32) {
    if nodes.is_empty() || canvas_width <= 0.0 || canvas_height <= 0.0 {
        return;
    }

    let col_x = [
        canvas_width * 0.15,
        canvas_width * 0.50,
        canvas_width * 0.85,
    ];

    let mut col_counts = [0u32; 3];
    for node in nodes.iter() {
        let c = node.node_type.column() as usize;
        if c < 3 {
            col_counts[c] += 1;
        }
    }

    let mut col_indices = [0u32; 3];
    for node in nodes.iter_mut() {
        let c = node.node_type.column() as usize;
        if c >= 3 {
            continue;
        }
        node.x = col_x[c];
        let count = col_counts[c];
        let index = col_indices[c];
        if count == 1 {
            node.y = canvas_height / 2.0;
        } else {
            let spacing = canvas_height / (count + 1) as f32;
            node.y = spacing * (index + 1) as f32;
        }
        col_indices[c] += 1;
    }
}

/// Finds the (x, y) position of an outbound node by its id.
///
/// Returns `None` if the node is not found or is not an Outbound type.
pub fn outbound_position(outbound_id: &str, nodes: &[CanvasNode]) -> Option<(f32, f32)> {
    nodes
        .iter()
        .find(|n| n.id == outbound_id && n.node_type == NodeType::Outbound)
        .map(|n| (n.x, n.y))
}

/// Finds the (x, y) position of any node by its id.
pub fn node_position(node_id: &str, nodes: &[CanvasNode]) -> Option<(f32, f32)> {
    nodes.iter().find(|n| n.id == node_id).map(|n| (n.x, n.y))
}

/// Computes the packet dot animation duration from traffic volume.
///
/// Higher volume → faster dots.
pub fn dot_duration_s(traffic_volume: u64) -> f32 {
    if traffic_volume >= MAX_VOLUME_REFERENCE {
        return MIN_DOT_DURATION_S;
    }
    if traffic_volume == 0 {
        return MAX_DOT_DURATION_S;
    }
    let ratio = traffic_volume as f32 / MAX_VOLUME_REFERENCE as f32;
    MAX_DOT_DURATION_S - (ratio * (MAX_DOT_DURATION_S - MIN_DOT_DURATION_S))
}

/// Generates a pseudo-random jitter offset for Flow-J visualization.
///
/// Uses a simple XOR-shift to create Markov-like visual noise per frame.
pub fn flowj_jitter_offset(seed: u32, frame: u32) -> (f32, f32) {
    let mut state = seed.wrapping_add(frame.wrapping_mul(2654435761));
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    let dx = ((state % 100) as f32 / 100.0 - 0.5) * 6.0;
    state = state.wrapping_mul(1103515245).wrapping_add(12345);
    let dy = ((state % 100) as f32 / 100.0 - 0.5) * 6.0;
    (dx, dy)
}

// ─── Component Props ───────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct RoutingCanvasProps {
    pub nodes: Vec<CanvasNode>,
    pub links: Vec<CanvasLink>,
    /// Pre-computed stats for each link (keyed by source→target).
    #[props(default = Vec::new())]
    pub stats: Vec<ConnectionStats>,
    #[props(default = 360)]
    pub width: u32,
    #[props(default = 280)]
    pub height: u32,
}

// ─── Component ─────────────────────────────────────────────────────────────────

/// Touch-responsive routing topology canvas.
#[component]
pub fn RoutingCanvas(props: RoutingCanvasProps) -> Element {
    let w = props.width as f32;
    let h = props.height as f32;

    // Compute node positions
    let mut nodes = props.nodes.clone();
    compute_layout(&mut nodes, w, h);

    let selected_link = use_signal::<Option<(String, String)>>(|| None);

    rsx! {
        GlassCard {
            class: "p-4 md:p-6",
            children: rsx! {
                div { class: "flex items-center gap-3 mb-3",
                    Icon { name: "hub".to_string(), class: "text-violet-400 text-lg".to_string() }
                    h3 { class: "text-sm font-semibold text-white", "Routing Topology" }
                }

                // SVG canvas
                div { class: "overflow-x-auto -mx-2 px-2",
                    svg {
                        width: "{props.width}",
                        height: "{props.height}",
                        view_box: "0 0 {w} {h}",
                        class: "block mx-auto",

                        // CSS animations for dots
                        defs {
                            // Inline <style> for the animated dots
                        }

                        // ── Render links ──
                        for link in props.links.iter() {
                            {render_link(link, &nodes, w, selected_link)}
                        }

                        // ── Render nodes ──
                        for node in nodes.iter() {
                            {render_node(node)}
                        }
                    }
                }

                // ── Tapped link detail ──
                {render_selected_stats(&selected_link, &props.stats)}

                // ── Legend ──
                div { class: "mt-3 flex items-center justify-center gap-4",
                    for (label , color) in [("App", "bg-cyan-400"), ("Filter", "bg-violet-400"), ("Outbound", "bg-emerald-400")] {
                        div { class: "flex items-center gap-1.5",
                            div { class: format!("w-2 h-2 rounded-full {}", color) }
                            span { class: "text-[10px] text-gray-500 uppercase", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

// ─── Link Rendering ────────────────────────────────────────────────────────────

fn render_link(
    link: &CanvasLink,
    nodes: &[CanvasNode],
    _canvas_width: f32,
    mut selected: Signal<Option<(String, String)>>,
) -> Element {
    let src_pos = match node_position(&link.source_id, nodes) {
        Some(p) => p,
        None => return rsx! {},
    };
    let tgt_pos = match node_position(&link.target_id, nodes) {
        Some(p) => p,
        None => return rsx! {},
    };

    let is_selected = selected
        .read()
        .as_ref()
        .map_or(false, |(s, t)| s == &link.source_id && t == &link.target_id);

    let line_color = if is_selected {
        "rgba(0,242,255,0.6)"
    } else {
        "rgba(255,255,255,0.12)"
    };

    let duration = dot_duration_s(link.traffic_volume);
    let is_flowj = link.is_flowj;
    let source_id = link.source_id.clone();
    let target_id = link.target_id.clone();

    // Jitter offsets for Flow-J links
    let (jx, jy) = if is_flowj {
        flowj_jitter_offset(
            link.source_id.len() as u32 ^ link.target_id.len() as u32,
            42,
        )
    } else {
        (0.0, 0.0)
    };

    let mid_x = (src_pos.0 + tgt_pos.0) / 2.0 + jx;
    let mid_y = (src_pos.1 + tgt_pos.1) / 2.0 + jy;

    // Bezier: slight curve via midpoint offset for Flow-J, straight for normal
    let path_d = if is_flowj {
        format!(
            "M{},{} Q{},{} {},{}",
            src_pos.0, src_pos.1, mid_x, mid_y, tgt_pos.0, tgt_pos.1
        )
    } else {
        format!("M{},{} L{},{}", src_pos.0, src_pos.1, tgt_pos.0, tgt_pos.1)
    };

    // Unique animation name per link
    let anim_id = format!(
        "dot_{}_{}",
        source_id.replace('-', ""),
        target_id.replace('-', "")
    );

    rsx! {
        // Visible line
        path {
            d: "{path_d}",
            stroke: "{line_color}",
            stroke_width: if is_selected { "2.5" } else { "1.5" },
            fill: "none",
            stroke_dasharray: if is_flowj { "4 3" } else { "" },
        }

        // Transparent hit area for tap
        path {
            d: "{path_d}",
            stroke: "transparent",
            stroke_width: "24",
            fill: "none",
            cursor: "pointer",
            onclick: {
                let src = source_id.clone();
                let tgt = target_id.clone();
                move |_| {
                    let current = selected.read().clone();
                    if current.as_ref().map_or(false, |(s, t)| s == &src && t == &tgt) {
                        selected.set(None);
                    } else {
                        selected.set(Some((src.clone(), tgt.clone())));
                    }
                }
            },
        }

        // Animated packet dot
        circle {
            r: "{DOT_RADIUS}",
            fill: if is_flowj { "rgba(168,85,247,0.8)" } else { "rgba(6,182,212,0.7)" },
            filter: if is_flowj { "drop-shadow(0 0 4px rgba(168,85,247,0.5))" } else { "drop-shadow(0 0 3px rgba(6,182,212,0.4))" },
            animateMotion {
                dur: "{duration}s",
                repeat_count: "indefinite",
                path: "{path_d}",
                id: "{anim_id}",
            }
        }

        // Second dot for high-traffic links (volume > 2MB/s)
        if link.traffic_volume > 2_000_000 {
            circle {
                r: "{DOT_RADIUS}",
                fill: if is_flowj { "rgba(168,85,247,0.5)" } else { "rgba(6,182,212,0.4)" },
                animateMotion {
                    dur: "{duration}s",
                    begin: "{duration / 2.0}s",
                    repeat_count: "indefinite",
                    path: "{path_d}",
                }
            }
        }
    }
}

// ─── Node Rendering ────────────────────────────────────────────────────────────

fn render_node(node: &CanvasNode) -> Element {
    let r = node.scaled_radius();
    let text_y = node.y + r + 14.0;
    let label = node.label.clone();

    rsx! {
        // Outer glow ring
        circle {
            cx: "{node.x}",
            cy: "{node.y}",
            r: "{r + 3.0}",
            fill: "none",
            stroke: "{node.node_type.stroke_color()}",
            stroke_width: "1",
            opacity: "0.4",
        }
        // Main circle
        circle {
            cx: "{node.x}",
            cy: "{node.y}",
            r: "{r}",
            fill: "{node.node_type.fill_color()}",
            stroke: "{node.node_type.stroke_color()}",
            stroke_width: "1.5",
        }
        // Icon text (Material Symbol codepoint — rendered as SVG text)
        text {
            x: "{node.x}",
            y: "{node.y}",
            text_anchor: "middle",
            dominant_baseline: "central",
            font_size: "14",
            fill: "rgba(255,255,255,0.8)",
            font_family: "'Material Symbols Outlined'",
            "{node.node_type.icon()}"
        }
        // Label below
        text {
            x: "{node.x}",
            y: "{text_y}",
            text_anchor: "middle",
            font_size: "9",
            fill: "rgba(255,255,255,0.5)",
            font_family: "'Inter', sans-serif",
            "{label}"
        }
    }
}

// ─── Selected Link Stats ───────────────────────────────────────────────────────

fn render_selected_stats(
    selected: &Signal<Option<(String, String)>>,
    stats: &[ConnectionStats],
) -> Element {
    let sel = selected.read();
    let Some((ref src, ref tgt)) = *sel else {
        return rsx! {};
    };

    let stat = stats
        .iter()
        .find(|s| s.link_source == *src && s.link_target == *tgt);

    rsx! {
        div { class: "mt-3 p-3 rounded-lg bg-white/5 border border-white/5 animate-in fade-in",
            if let Some(s) = stat {
                div { class: "grid grid-cols-2 gap-3",
                    div {
                        span { class: "text-[10px] font-bold text-gray-500 uppercase block", "Protocol" }
                        span { class: "text-xs font-mono text-cyan-400", "{s.protocol}" }
                    }
                    div {
                        span { class: "text-[10px] font-bold text-gray-500 uppercase block", "Latency" }
                        span { class: "text-xs font-mono text-gray-300", "{s.latency_ms:.1}ms" }
                    }
                    div {
                        span { class: "text-[10px] font-bold text-gray-500 uppercase block", "Sent" }
                        span { class: "text-xs font-mono text-gray-300", "{format_bytes(s.bytes_sent)}" }
                    }
                    div {
                        span { class: "text-[10px] font-bold text-gray-500 uppercase block", "Received" }
                        span { class: "text-xs font-mono text-gray-300", "{format_bytes(s.bytes_received)}" }
                    }
                    div {
                        span { class: "text-[10px] font-bold text-gray-500 uppercase block", "Buffer" }
                        span {
                            class: format!("text-xs font-mono {}",
                                if s.buffer_usage_pct > 80.0 { "text-amber-400" }
                                else if s.buffer_usage_pct > 95.0 { "text-red-400" }
                                else { "text-gray-300" }
                            ),
                            "{s.buffer_usage_pct:.0}%"
                        }
                    }
                }
            } else {
                div { class: "text-center py-2",
                    span { class: "text-xs text-gray-500", "No stats available for {src} → {tgt}" }
                }
            }
        }
    }
}

/// Format bytes into a human-readable string.
fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_nodes() -> Vec<CanvasNode> {
        vec![
            CanvasNode {
                id: "app-0".into(),
                label: "EdgeRay".into(),
                node_type: NodeType::AppSource,
                traffic_volume: 5_000_000,
                x: 0.0,
                y: 0.0,
            },
            CanvasNode {
                id: "filter-geo".into(),
                label: "GeoIP".into(),
                node_type: NodeType::Filter,
                traffic_volume: 3_000_000,
                x: 0.0,
                y: 0.0,
            },
            CanvasNode {
                id: "out-reality".into(),
                label: "REALITY FRA".into(),
                node_type: NodeType::Outbound,
                traffic_volume: 2_800_000,
                x: 0.0,
                y: 0.0,
            },
            CanvasNode {
                id: "out-direct".into(),
                label: "DIRECT".into(),
                node_type: NodeType::Outbound,
                traffic_volume: 500_000,
                x: 0.0,
                y: 0.0,
            },
        ]
    }

    #[test]
    fn test_compute_layout_positions() {
        let mut nodes = make_test_nodes();
        compute_layout(&mut nodes, 360.0, 280.0);

        // AppSource in column 0 (x ≈ 54)
        assert!((nodes[0].x - 54.0).abs() < 1.0);
        // Filter in column 1 (x ≈ 180)
        assert!((nodes[1].x - 180.0).abs() < 1.0);
        // Outbounds in column 2 (x ≈ 306)
        assert!((nodes[2].x - 306.0).abs() < 1.0);
        assert!((nodes[3].x - 306.0).abs() < 1.0);

        // Single node in column 0 → centered at height/2
        assert!((nodes[0].y - 140.0).abs() < 1.0);
        // Two outbound nodes spread evenly
        assert!(nodes[2].y < nodes[3].y);
    }

    #[test]
    fn test_compute_layout_empty() {
        let mut nodes: Vec<CanvasNode> = Vec::new();
        compute_layout(&mut nodes, 360.0, 280.0);
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_compute_layout_zero_dimensions() {
        let mut nodes = make_test_nodes();
        let original = nodes.clone();
        compute_layout(&mut nodes, 0.0, 280.0);
        // Positions unchanged
        for (n, o) in nodes.iter().zip(original.iter()) {
            assert_eq!(n.x, o.x);
            assert_eq!(n.y, o.y);
        }
    }

    #[test]
    fn test_outbound_position_found() {
        let mut nodes = make_test_nodes();
        compute_layout(&mut nodes, 360.0, 280.0);

        let pos = outbound_position("out-reality", &nodes);
        assert!(pos.is_some());
        let (x, y) = pos.unwrap();
        assert!((x - 306.0).abs() < 1.0);
        assert!(y > 0.0);
    }

    #[test]
    fn test_outbound_position_not_found() {
        let nodes = make_test_nodes();
        assert!(outbound_position("nonexistent", &nodes).is_none());
    }

    #[test]
    fn test_outbound_position_wrong_type() {
        let mut nodes = make_test_nodes();
        compute_layout(&mut nodes, 360.0, 280.0);
        // app-0 is AppSource, not Outbound
        assert!(outbound_position("app-0", &nodes).is_none());
    }

    #[test]
    fn test_dot_duration_bounds() {
        assert!((dot_duration_s(0) - MAX_DOT_DURATION_S).abs() < 0.01);
        assert!((dot_duration_s(MAX_VOLUME_REFERENCE) - MIN_DOT_DURATION_S).abs() < 0.01);
        assert!((dot_duration_s(MAX_VOLUME_REFERENCE * 2) - MIN_DOT_DURATION_S).abs() < 0.01);

        // Mid-range
        let mid = dot_duration_s(MAX_VOLUME_REFERENCE / 2);
        assert!(mid > MIN_DOT_DURATION_S);
        assert!(mid < MAX_DOT_DURATION_S);
    }

    #[test]
    fn test_dot_duration_monotonic() {
        let d1 = dot_duration_s(100_000);
        let d2 = dot_duration_s(1_000_000);
        let d3 = dot_duration_s(5_000_000);
        assert!(d1 > d2);
        assert!(d2 > d3);
    }

    #[test]
    fn test_flowj_jitter_deterministic() {
        let (dx1, dy1) = flowj_jitter_offset(42, 10);
        let (dx2, dy2) = flowj_jitter_offset(42, 10);
        assert_eq!(dx1, dx2);
        assert_eq!(dy1, dy2);
    }

    #[test]
    fn test_flowj_jitter_bounded() {
        for seed in 0..100 {
            for frame in 0..50 {
                let (dx, dy) = flowj_jitter_offset(seed, frame);
                assert!(dx.abs() <= 3.0, "dx={} out of range", dx);
                assert!(dy.abs() <= 3.0, "dy={} out of range", dy);
            }
        }
    }

    #[test]
    fn test_flowj_jitter_varies_with_frame() {
        let (dx1, dy1) = flowj_jitter_offset(42, 0);
        let (dx2, dy2) = flowj_jitter_offset(42, 1);
        // Should not be identical for different frames
        assert!(dx1 != dx2 || dy1 != dy2);
    }

    #[test]
    fn test_scaled_radius() {
        let zero = CanvasNode {
            id: "a".into(),
            label: "A".into(),
            node_type: NodeType::AppSource,
            traffic_volume: 0,
            x: 0.0,
            y: 0.0,
        };
        assert_eq!(zero.scaled_radius(), BASE_NODE_RADIUS);

        let big = CanvasNode {
            id: "b".into(),
            label: "B".into(),
            node_type: NodeType::AppSource,
            traffic_volume: 10_000_000,
            x: 0.0,
            y: 0.0,
        };
        assert!(big.scaled_radius() > BASE_NODE_RADIUS);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1_500), "1.5 KB");
        assert_eq!(format_bytes(2_500_000), "2.5 MB");
        assert_eq!(format_bytes(3_500_000_000), "3.5 GB");
    }

    #[test]
    fn test_node_type_columns() {
        assert_eq!(NodeType::AppSource.column(), 0);
        assert_eq!(NodeType::Filter.column(), 1);
        assert_eq!(NodeType::Outbound.column(), 2);
    }

    #[test]
    fn test_layout_100_plus_nodes_no_panic() {
        let mut nodes: Vec<CanvasNode> = Vec::with_capacity(120);
        for i in 0..10 {
            nodes.push(CanvasNode {
                id: format!("app-{i}"),
                label: format!("App {i}"),
                node_type: NodeType::AppSource,
                traffic_volume: (i as u64 + 1) * 100_000,
                x: 0.0,
                y: 0.0,
            });
        }
        for i in 0..30 {
            nodes.push(CanvasNode {
                id: format!("filter-{i}"),
                label: format!("Rule {i}"),
                node_type: NodeType::Filter,
                traffic_volume: (i as u64 + 1) * 50_000,
                x: 0.0,
                y: 0.0,
            });
        }
        for i in 0..80 {
            nodes.push(CanvasNode {
                id: format!("out-{i}"),
                label: format!("Dest {i}"),
                node_type: NodeType::Outbound,
                traffic_volume: (i as u64 + 1) * 30_000,
                x: 0.0,
                y: 0.0,
            });
        }
        assert_eq!(nodes.len(), 120);
        compute_layout(&mut nodes, 800.0, 600.0);

        // Verify all positions were set
        for node in &nodes {
            assert!(node.x > 0.0, "Node {} has x=0", node.id);
            assert!(node.y > 0.0, "Node {} has y=0", node.id);
        }
    }
}
