use crate::components::GlassCard;
use crate::constants::get_mesh_peers;
use crate::types::PeerStatus;
use dioxus::prelude::*;

#[component]
pub fn MeshMap() -> Element {
    let peers = get_mesh_peers();

    rsx! {
        div {
            class: "animate-fade-in pb-12",

            div {
                class: "flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6 sm:mb-8",
                div {
                    h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Network Topology" }
                    h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Mesh Map" }
                }
            }

            GlassCard {
                class: "relative h-[400px] sm:h-[500px] overflow-hidden".to_string(),

                // Background grid
                div { class: "absolute inset-0 bg-grid-pattern opacity-30" }

                // Center radar effect
                div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-48 h-48 rounded-full border border-primary/20" }
                div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-32 h-32 rounded-full border border-primary/10" }
                div { class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-16 h-16 rounded-full border border-primary/5" }

                // Peers
                for peer in &peers {
                    {
                        let status_class = match peer.status {
                            PeerStatus::Good => "border-emerald/50 shadow-[0_0_15px_rgba(0,255,163,0.3)]",
                            PeerStatus::Fair => "border-warning/50 shadow-[0_0_15px_rgba(212,255,0,0.3)]",
                            PeerStatus::Poor => "border-red-500/50 shadow-[0_0_15px_rgba(255,68,68,0.3)]",
                        };
                        let node_class = format!("w-10 h-10 sm:w-12 sm:h-12 rounded-full glass-panel flex items-center justify-center border transition-colors {}", status_class);
                        let pos_style = format!("left: {}%; top: {}%;", peer.x, peer.y);

                        rsx! {
                            div {
                                class: "absolute transform -translate-x-1/2 -translate-y-1/2",
                                style: "{pos_style}",

                                // Peer node
                                div {
                                    class: "relative group cursor-pointer",
                                    div {
                                        class: "{node_class}",
                                        span { class: "text-[10px] font-mono font-bold text-white", "{peer.id}" }
                                    }

                                    // Tooltip
                                    div {
                                        class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-2 bg-black/80 backdrop-blur-xl border border-white/10 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50",
                                        p { class: "text-xs font-bold text-white", "{peer.name}" }
                                        p { class: "text-[10px] text-gray-400 font-mono", "RTT: {peer.rtt}ms" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Legend
            div {
                class: "mt-4 flex items-center justify-center gap-6",
                div {
                    class: "flex items-center gap-2",
                    div { class: "w-3 h-3 rounded-full bg-emerald" }
                    span { class: "text-xs text-gray-400", "Good (<50ms)" }
                }
                div {
                    class: "flex items-center gap-2",
                    div { class: "w-3 h-3 rounded-full bg-warning" }
                    span { class: "text-xs text-gray-400", "Fair (<150ms)" }
                }
                div {
                    class: "flex items-center gap-2",
                    div { class: "w-3 h-3 rounded-full bg-red-500" }
                    span { class: "text-xs text-gray-400", "Poor (>150ms)" }
                }
            }
        }
    }
}
