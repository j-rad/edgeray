// edgeray-app/src/ui/diagnostics/rule_tracer.rs
//! Live Rule Tracer Component
//!
//! Shows real-time packet-to-rule matching with hierarchical flow visualization.
//! Displays the full path: Inbound → Rule Matching → Outbound

use crate::components::ui::Icon;
use dioxus::prelude::*;

/// Packet flow stage
#[derive(Clone, PartialEq, Debug)]
pub enum FlowStage {
    Inbound,
    RuleMatch,
    Outbound,
}

impl FlowStage {
    fn label(&self) -> &str {
        match self {
            FlowStage::Inbound => "Inbound",
            FlowStage::RuleMatch => "Rule Match",
            FlowStage::Outbound => "Outbound",
        }
    }

    fn icon(&self) -> &str {
        match self {
            FlowStage::Inbound => "input",
            FlowStage::RuleMatch => "rule",
            FlowStage::Outbound => "output",
        }
    }

    fn color(&self) -> &str {
        match self {
            FlowStage::Inbound => "text-cyan-400",
            FlowStage::RuleMatch => "text-violet-400",
            FlowStage::Outbound => "text-emerald-400",
        }
    }
}

/// A traced packet entry showing the full routing path
#[derive(Clone, PartialEq)]
pub struct TracedPacket {
    pub id: u32,
    pub timestamp: String,
    pub src_ip: String,
    pub dest_host: String,
    pub dest_port: u16,
    pub protocol: String,
    pub inbound_tag: String,
    pub matched_rule: String,
    pub rule_type: String,
    pub outbound_tag: String,
    pub action: TraceAction,
    pub latency_ms: Option<u32>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TraceAction {
    Proxy,
    Direct,
    Block,
}

impl TraceAction {
    fn color_class(&self) -> &str {
        match self {
            TraceAction::Proxy => "text-cyan-400 bg-cyan-500/10",
            TraceAction::Direct => "text-emerald-400 bg-emerald-500/10",
            TraceAction::Block => "text-red-400 bg-red-500/10",
        }
    }

    fn label(&self) -> &str {
        match self {
            TraceAction::Proxy => "PROXY",
            TraceAction::Direct => "DIRECT",
            TraceAction::Block => "BLOCK",
        }
    }

    fn icon(&self) -> &str {
        match self {
            TraceAction::Proxy => "vpn_lock",
            TraceAction::Direct => "public",
            TraceAction::Block => "block",
        }
    }
}

/// Protocol badge color helper - TCP orange, UDP purple per Phase 4 spec
fn protocol_badge_class(protocol: &str) -> &'static str {
    match protocol.to_uppercase().as_str() {
        "TCP" => "text-orange-400 bg-orange-500/10 border-orange-500/30",
        "UDP" => "text-violet-400 bg-violet-500/10 border-violet-500/30",
        "QUIC" => "text-cyan-400 bg-cyan-500/10 border-cyan-500/30",
        _ => "text-gray-400 bg-gray-500/10 border-gray-500/30",
    }
}

/// Rule Tracer Props
#[derive(Props, Clone, PartialEq)]
pub struct RuleTracerProps {
    /// Maximum number of entries to display
    #[props(default = 50)]
    pub max_entries: usize,
    /// Whether tracing is enabled
    #[props(default = true)]
    pub enabled: bool,
}

/// Live Rule Tracer Component with Hierarchical Flow View
#[component]
pub fn RuleTracer(props: RuleTracerProps) -> Element {
    // Mock data for demonstration - would come from real packet tracing
    let packets = use_signal(|| {
        vec![
            TracedPacket {
                id: 1,
                timestamp: "21:10:15.234".to_string(),
                src_ip: "192.168.1.100".to_string(),
                dest_host: "google.com".to_string(),
                dest_port: 443,
                protocol: "TCP".to_string(),
                inbound_tag: "socks-in".to_string(),
                matched_rule: "geosite:google".to_string(),
                rule_type: "domain".to_string(),
                outbound_tag: "proxy-us".to_string(),
                action: TraceAction::Proxy,
                latency_ms: Some(42),
            },
            TracedPacket {
                id: 2,
                timestamp: "21:10:15.156".to_string(),
                src_ip: "192.168.1.100".to_string(),
                dest_host: "api.local".to_string(),
                dest_port: 8080,
                protocol: "TCP".to_string(),
                inbound_tag: "http-in".to_string(),
                matched_rule: "domain:local".to_string(),
                rule_type: "suffix".to_string(),
                outbound_tag: "direct".to_string(),
                action: TraceAction::Direct,
                latency_ms: Some(2),
            },
            TracedPacket {
                id: 3,
                timestamp: "21:10:14.998".to_string(),
                src_ip: "192.168.1.100".to_string(),
                dest_host: "tracker.ads.com".to_string(),
                dest_port: 443,
                protocol: "TCP".to_string(),
                inbound_tag: "tun-in".to_string(),
                matched_rule: "geosite:category-ads".to_string(),
                rule_type: "geosite".to_string(),
                outbound_tag: "block".to_string(),
                action: TraceAction::Block,
                latency_ms: None,
            },
            TracedPacket {
                id: 4,
                timestamp: "21:10:14.887".to_string(),
                src_ip: "192.168.1.100".to_string(),
                dest_host: "github.com".to_string(),
                dest_port: 22,
                protocol: "TCP".to_string(),
                inbound_tag: "socks-in".to_string(),
                matched_rule: "geosite:github".to_string(),
                rule_type: "domain".to_string(),
                outbound_tag: "proxy-sg".to_string(),
                action: TraceAction::Proxy,
                latency_ms: Some(68),
            },
        ]
    });

    let selected_packet = use_signal::<Option<u32>>(|| None);

    rsx! {
        div { class: "{crate::components::ui::glass::PANEL} rounded-2xl p-6 h-full flex flex-col",
            // Header with controls
            div { class: "flex items-center justify-between mb-4 pb-4 border-b border-white/10",
                div { class: "flex items-center gap-3",
                    Icon { name: "troubleshoot", class: "text-violet-400 text-xl" }
                    h3 { class: "text-lg font-semibold text-white", "Rule Tracer" }
                }
                div { class: "flex items-center gap-4",
                    // Recording indicator
                    div { class: "flex items-center gap-2",
                        div {
                            class: format!(
                                "w-2.5 h-2.5 rounded-full {}",
                                if props.enabled { "bg-emerald-500 animate-pulse shadow-[0_0_8px_rgba(16,185,129,0.6)]" } else { "bg-gray-500" }
                            )
                        }
                        span { class: "text-xs font-mono text-gray-400",
                            if props.enabled { "Recording" } else { "Paused" }
                        }
                    }
                    // Packet count
                    span { class: "text-xs font-mono text-gray-500",
                        "{packets.read().len()} packets"
                    }
                }
            }

            // Hierarchical Flow Legend
            div { class: "flex items-center gap-6 mb-4 px-2",
                for stage in [FlowStage::Inbound, FlowStage::RuleMatch, FlowStage::Outbound] {
                    div { class: "flex items-center gap-2",
                        Icon { name: stage.icon().to_string(), class: format!("text-sm {}", stage.color()) }
                        span { class: format!("text-xs font-medium {}", stage.color()), "{stage.label()}" }
                    }
                    if !matches!(stage, FlowStage::Outbound) {
                        Icon { name: "arrow_forward", class: "text-gray-600 text-xs" }
                    }
                }
            }

            // Packet List with Flow Visualization
            div { class: "flex-1 overflow-auto space-y-2",
                for packet in packets.read().iter().take(props.max_entries) {
                    {packet_flow_card(packet.clone(), selected_packet)}
                }
            }
        }
    }
}

/// Renders a single packet's flow through the routing stack
fn packet_flow_card(packet: TracedPacket, mut selected: Signal<Option<u32>>) -> Element {
    let is_expanded = selected.read().map_or(false, |id| id == packet.id);
    let action_class = packet.action.color_class();

    rsx! {
        div {
            class: format!(
                "{} rounded-xl transition-all duration-200 cursor-pointer {}",
                crate::components::ui::glass::CARD,
                if is_expanded { "ring-1 ring-violet-500/50 bg-white/5" } else { "hover:bg-white/10" }
            ),
            onclick: move |_| {
                let current = *selected.read();
                if current == Some(packet.id) {
                    selected.set(None);
                } else {
                    selected.set(Some(packet.id));
                }
            },

            div { class: "p-4 flex gap-4",
                // Left Column: Timestamp & Protocol
                div { class: "flex flex-col items-center gap-2 min-w-[60px]",
                    span { class: "text-[10px] font-mono text-gray-500", "{packet.timestamp}" }
                    span {
                        class: format!("px-1.5 py-0.5 rounded text-[9px] font-bold font-mono border shrink-0 {}", protocol_badge_class(&packet.protocol)),
                        "{packet.protocol}"
                    }
                }

                // Middle Column: Vertical Timeline
                div { class: "flex flex-col items-center",
                    // Top Dot
                    div { class: "w-2 h-2 rounded-full bg-cyan-500 shadow-[0_0_8px_rgba(6,182,212,0.5)]" }
                    // Dashed Line
                     div { class: "w-0.5 flex-1 border-l border-dashed border-gray-600 my-1 min-h-[40px]" }
                    // Bottom Dot
                    div { class: format!("w-2 h-2 rounded-full {}", if matches!(packet.action, TraceAction::Block) { "bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.5)]" } else { "bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]" }) }
                }

                // Right Column: Flow Details
                div { class: "flex-1 flex flex-col gap-3 py-1",
                    // Inbound Node
                    div { class: "flex items-center gap-2",
                         Icon { name: "input", class: "text-cyan-400 text-sm" }
                         div { class: "flex flex-col",
                            span { class: "text-xs font-bold text-gray-300", "Inbound" }
                            span { class: "text-[10px] font-mono text-cyan-400", "{packet.inbound_tag}" }
                         }
                    }

                    // Rule Match Node (Middle)
                    div { class: "flex items-center gap-2 pl-2",
                         Icon { name: "rule", class: "text-violet-400 text-xs" }
                         span { class: "text-xs font-mono text-violet-400 truncate", "{packet.matched_rule}" }
                    }

                    // Outbound Node
                    div { class: "flex items-center justify-between",
                        div { class: "flex items-center gap-2",
                             Icon { name: "output", class: "text-emerald-400 text-sm" }
                             div { class: "flex flex-col",
                                span { class: "text-xs font-bold text-gray-300", "Outbound" }
                                span { class: "text-[10px] font-mono text-emerald-400", "{packet.outbound_tag}" }
                             }
                        }
                        // Action Badge
                         span {
                            class: format!("px-2 py-0.5 rounded text-[10px] font-bold self-end {}", action_class),
                            "{packet.action.label()}"
                        }
                    }
                }

                // Expand Icon
                 Icon {
                    name: if is_expanded { "expand_less" } else { "expand_more" },
                    class: "text-gray-500 text-sm self-start mt-1"
                }
            }

            // Expanded details
            if is_expanded {
                div { class: "px-4 pb-4 pt-0 ml-[88px] border-l border-white/5 pl-4",
                    div { class: "grid grid-cols-2 gap-4 pt-2",
                        // Source
                        DetailItem { label: "Source IP", value: packet.src_ip.clone() }
                        // Destination
                        DetailItem { label: "Destination", value: format!("{}:{}", packet.dest_host, packet.dest_port) }
                        // Latency
                        DetailItem { label: "Latency", value: packet.latency_ms.map(|l| format!("{}ms", l)).unwrap_or("-".to_string()) }
                         // Rule type
                        DetailItem { label: "Rule Type", value: packet.rule_type.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn DetailItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-0.5",
            span { class: "text-[10px] font-bold text-gray-500 uppercase tracking-wider", "{label}" }
            span { class: "text-xs font-mono text-gray-300", "{value}" }
        }
    }
}
