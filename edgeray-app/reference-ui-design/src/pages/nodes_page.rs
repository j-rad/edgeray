use crate::components::GlassCard;
use crate::constants::get_nodes;
use dioxus::prelude::*;

#[component]
pub fn NodesPage() -> Element {
    let nodes = get_nodes();

    rsx! {
        div {
            class: "animate-fade-in pb-12",

            // Header Row
            div {
                class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6 sm:mb-8",
                div {
                    h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Global Network" }
                    h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Active Nodes" }
                }
                div {
                    class: "flex items-center gap-2 sm:gap-3",
                    button {
                        class: "flex items-center gap-2 px-3 sm:px-4 py-2 sm:py-2.5 rounded-xl bg-primary/10 border border-primary/30 text-primary text-xs font-bold uppercase tracking-wider hover:bg-primary/20 transition-colors",
                        svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                            path { d: "M5 12.55a11 11 0 0 1 14.08 0M8.53 16.11a6 6 0 0 1 6.95 0M12 20h.01" }
                        }
                        "Add Sub"
                    }
                    button {
                        class: "flex items-center gap-2 px-3 sm:px-4 py-2 sm:py-2.5 rounded-xl bg-white/5 border border-white/10 text-gray-300 text-xs font-bold uppercase tracking-wider hover:bg-white/10 transition-colors",
                        svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                            line { x1: "12", y1: "5", x2: "12", y2: "19" }
                            line { x1: "5", y1: "12", x2: "19", y2: "12" }
                        }
                        "Add Node"
                    }
                    button {
                        class: "w-10 h-10 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center text-gray-400 hover:text-white hover:bg-white/10 transition-colors",
                        svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                            polygon { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }
                        }
                    }
                }
            }

            // Nodes Grid
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4",
                for node in nodes.iter() {
                    {
                        let status_color = match node.status.as_str() {
                            "Online" => "bg-emerald text-emerald",
                            "Maintenance" => "bg-warning text-warning",
                            _ => "bg-gray-500 text-gray-500"
                        };
                        let status_badge_bg = match node.status.as_str() {
                            "Online" => "bg-emerald/10 border-emerald/20",
                            "Maintenance" => "bg-warning/10 border-warning/20",
                            _ => "bg-gray-500/10 border-gray-500/20"
                        };
                        let status_text = match node.status.as_str() {
                            "Online" => "text-emerald",
                            "Maintenance" => "text-warning",
                            _ => "text-gray-500"
                        };

                        rsx! {
                            GlassCard {
                                key: "{node.id}",
                                class: "!p-4 sm:!p-5 group hover:bg-white/5 transition-colors cursor-pointer".to_string(),
                                div {
                                    class: "flex items-start justify-between mb-4",
                                    div {
                                        class: "flex items-center gap-3 sm:gap-4",
                                        // Flag Image
                                        div {
                                            class: "w-10 h-10 sm:w-12 sm:h-12 rounded-xl overflow-hidden border border-white/10 group-hover:border-primary/50 transition-colors shadow-lg shrink-0",
                                            img {
                                                src: "{node.flag_url}",
                                                alt: "flag",
                                                class: "w-full h-full object-cover"
                                            }
                                        }
                                        div {
                                            h3 { class: "text-sm sm:text-base font-bold text-white group-hover:text-primary transition-colors", "{node.name}" }
                                            span { class: "text-[10px] sm:text-xs font-mono text-gray-500", "{node.location}" }
                                        }
                                    }

                                    // Status Badge
                                    div {
                                        class: "flex items-center gap-1.5 px-2 py-1 rounded-lg border {status_badge_bg}",
                                        div { class: "w-1.5 h-1.5 rounded-full {status_color} shadow-[0_0_6px_currentColor]" }
                                        span { class: "text-[9px] sm:text-[10px] font-bold uppercase {status_text}", "{node.status}" }
                                    }
                                }

                                // Server Info
                                div {
                                    class: "flex items-center gap-2 sm:gap-3 text-[10px] sm:text-xs font-mono text-gray-500 mb-4",
                                    span { class: "px-2 py-1 rounded bg-white/5 border border-white/5", "{node.id}" }
                                    span { "{node.latency}ms" }
                                }

                                // Protocol Tags
                                div {
                                    class: "flex flex-wrap gap-1.5 sm:gap-2",
                                    for protocol in node.protocols.iter() {
                                        span {
                                            key: "{protocol}",
                                            class: "px-2 py-0.5 rounded text-[9px] sm:text-[10px] font-bold uppercase bg-primary/10 text-primary border border-primary/20",
                                            "{protocol}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
