use crate::components::GlassCard;
use crate::constants::get_nodes;
use dioxus::prelude::*;

#[component]
pub fn CoreDashboard() -> Element {
    let mut is_active = use_signal(|| true);
    let nodes = get_nodes();
    let active_node = &nodes[0];

    let glow_class = if is_active() {
        "opacity-100"
    } else {
        "opacity-20"
    };
    let ring1_class = if is_active() {
        "animate-spin-slow"
    } else {
        "opacity-20"
    };
    let ring2_class = if is_active() {
        "animate-spin-reverse-slow"
    } else {
        "opacity-20"
    };
    let ring3_class = if is_active() {
        "animate-pulse-fast"
    } else {
        ""
    };
    let power_bg_class = if is_active() {
        "bg-primary/20 shadow-[0_0_20px_rgba(34,211,238,0.4)] backdrop-blur-md"
    } else {
        "bg-white/5"
    };
    let power_icon_class = if is_active() {
        "text-primary"
    } else {
        "text-gray-500"
    };
    let status_text_class = if is_active() {
        "text-white text-shadow-glow"
    } else {
        "text-gray-600"
    };
    let stroke_w = if is_active() { "2.5" } else { "2" };

    rsx! {
        div {
            class: "animate-fade-in pb-10",

            // SVG Gradient Definitions
            svg {
                class: "absolute w-0 h-0",
                defs {
                    linearGradient { id: "colorDown", x1: "0", y1: "0", x2: "0", y2: "1",
                        stop { offset: "5%", stop_color: "#22d3ee", stop_opacity: "0.6" }
                        stop { offset: "95%", stop_color: "#22d3ee", stop_opacity: "0" }
                    }
                    linearGradient { id: "colorUp", x1: "0", y1: "0", x2: "0", y2: "1",
                        stop { offset: "5%", stop_color: "#bc00ff", stop_opacity: "0.6" }
                        stop { offset: "95%", stop_color: "#bc00ff", stop_opacity: "0" }
                    }
                    linearGradient { id: "colorPing", x1: "0", y1: "0", x2: "0", y2: "1",
                        stop { offset: "5%", stop_color: "#00ffa3", stop_opacity: "0.6" }
                        stop { offset: "95%", stop_color: "#00ffa3", stop_opacity: "0" }
                    }
                    linearGradient { id: "colorJitter", x1: "0", y1: "0", x2: "0", y2: "1",
                        stop { offset: "5%", stop_color: "#d4ff00", stop_opacity: "0.6" }
                        stop { offset: "95%", stop_color: "#d4ff00", stop_opacity: "0" }
                    }
                }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-12 gap-6 lg:gap-8 items-center min-h-[calc(100vh-140px)]",

                // Left Col: Reactor Core
                div {
                    class: "col-span-1 lg:col-span-5 flex justify-center py-6 lg:py-0",
                    div {
                        class: "relative w-[260px] h-[260px] sm:w-[320px] sm:h-[320px] flex items-center justify-center",

                        // Background Glow
                        div { class: "absolute inset-0 rounded-full bg-primary/20 blur-[60px] sm:blur-[80px] transition-opacity duration-700 {glow_class}" }

                        // Outer Ring
                        div {
                            class: "absolute inset-0 rounded-full border border-white/5 flex items-center justify-center",
                            div { class: "absolute inset-2 border border-dashed border-white/10 rounded-full opacity-50" }
                        }

                        // Rotating Rings
                        div { class: "absolute inset-3 sm:inset-4 rounded-full border border-transparent border-t-primary/30 border-l-primary/10 {ring1_class}" }
                        div { class: "absolute inset-8 sm:inset-10 rounded-full border border-transparent border-r-purple/40 border-b-purple/10 {ring2_class}" }
                        div { class: "absolute inset-12 sm:inset-16 rounded-full border-[1px] border-white/5 {ring3_class}" }

                        // Core Button
                        button {
                            class: "relative w-36 h-36 sm:w-48 sm:h-48 rounded-full glass-panel flex flex-col items-center justify-center z-10 transition-all duration-500 active:scale-95 group hover:border-primary/50 hover:shadow-[0_0_50px_rgba(34,211,238,0.2)] overflow-hidden",
                            onclick: move |_| is_active.set(!is_active()),

                            // Glass Reflection
                            div { class: "absolute top-0 left-[10%] w-[80%] h-[40%] bg-gradient-to-b from-white/10 to-transparent rounded-[50%] opacity-50 pointer-events-none" }

                            div {
                                class: "relative z-10 flex flex-col items-center",
                                div {
                                    class: "p-3 sm:p-4 rounded-full mb-1 sm:mb-2 transition-all duration-500 {power_bg_class}",
                                    svg {
                                        class: "w-6 h-6 sm:w-8 sm:h-8 transition-all duration-500 {power_icon_class}",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "{stroke_w}",
                                        view_box: "0 0 24 24",
                                        path { d: "M12 2v10M18.4 6.6a9 9 0 1 1-12.77 0" }
                                    }
                                }
                                div {
                                    class: "flex flex-col items-center gap-0.5 sm:gap-1",
                                    span { class: "text-[8px] sm:text-[9px] font-mono uppercase tracking-[0.2em] text-gray-400", "System State" }
                                    span {
                                        class: "text-base sm:text-lg font-mono font-bold tracking-widest transition-colors duration-300 {status_text_class}",
                                        if is_active() {"ONLINE"} else {"STBY"}
                                    }
                                }
                            }
                        }
                    }
                }

                // Right Col: Metrics
                div {
                    class: "col-span-1 lg:col-span-7 flex flex-col gap-3 sm:gap-4 w-full",

                    // Bandwidth Row
                    div {
                        class: "grid grid-cols-2 gap-3",

                        // Downlink Card
                        GlassCard {
                            glow: "cyan".to_string(),
                            class: "relative !p-0 h-28 sm:h-32 overflow-hidden".to_string(),
                            div {
                                class: "relative z-10 p-3 sm:p-4",
                                div {
                                    class: "flex justify-between items-center mb-0.5 sm:mb-1",
                                    span {
                                        class: "text-[9px] sm:text-[10px] font-bold uppercase text-primary flex items-center gap-1.5 tracking-widest",
                                        span { class: "w-1.5 h-1.5 rounded-sm bg-primary shadow-[0_0_5px_var(--primary)]" }
                                        "Downlink"
                                    }
                                    svg { class: "w-3 h-3 text-primary/50", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", polyline { points: "22,12 18,12 15,21 9,3 6,12 2,12" } }
                                }
                                div {
                                    class: "flex items-baseline gap-1.5",
                                    span { class: "text-xl sm:text-2xl font-mono font-bold text-white tracking-tighter", "842.5" }
                                    span { class: "text-[9px] sm:text-[10px] font-mono text-gray-400", "Mbps" }
                                }
                            }
                            // Area Chart
                            div {
                                class: "absolute inset-0 top-10 opacity-40",
                                svg {
                                    class: "w-full h-full",
                                    view_box: "0 0 200 60",
                                    preserve_aspect_ratio: "none",
                                    defs {
                                        linearGradient { id: "downGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                            stop { offset: "0%", stop_color: "#22d3ee", stop_opacity: "0.6" }
                                            stop { offset: "100%", stop_color: "#22d3ee", stop_opacity: "0" }
                                        }
                                    }
                                    path {
                                        d: "M0,30 Q20,15 40,25 T80,20 T120,35 T160,18 T200,28 L200,60 L0,60 Z",
                                        fill: "url(#downGrad)"
                                    }
                                    path {
                                        d: "M0,30 Q20,15 40,25 T80,20 T120,35 T160,18 T200,28",
                                        fill: "none",
                                        stroke: "#22d3ee",
                                        stroke_width: "2"
                                    }
                                }
                            }
                        }

                        // Uplink Card
                        GlassCard {
                            glow: "purple".to_string(),
                            class: "relative !p-0 h-28 sm:h-32 overflow-hidden".to_string(),
                            div {
                                class: "relative z-10 p-3 sm:p-4",
                                div {
                                    class: "flex justify-between items-center mb-0.5 sm:mb-1",
                                    span {
                                        class: "text-[9px] sm:text-[10px] font-bold uppercase text-purple flex items-center gap-1.5 tracking-widest",
                                        span { class: "w-1.5 h-1.5 rounded-sm bg-purple shadow-[0_0_5px_var(--purple)]" }
                                        "Uplink"
                                    }
                                    svg { class: "w-3 h-3 text-purple/50", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", polyline { points: "22,12 18,12 15,21 9,3 6,12 2,12" } }
                                }
                                div {
                                    class: "flex items-baseline gap-1.5",
                                    span { class: "text-xl sm:text-2xl font-mono font-bold text-white tracking-tighter", "124.8" }
                                    span { class: "text-[9px] sm:text-[10px] font-mono text-gray-400", "Mbps" }
                                }
                            }
                            // Area Chart
                            div {
                                class: "absolute inset-0 top-10 opacity-40",
                                svg {
                                    class: "w-full h-full",
                                    view_box: "0 0 200 60",
                                    preserve_aspect_ratio: "none",
                                    defs {
                                        linearGradient { id: "upGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                            stop { offset: "0%", stop_color: "#bc00ff", stop_opacity: "0.6" }
                                            stop { offset: "100%", stop_color: "#bc00ff", stop_opacity: "0" }
                                        }
                                    }
                                    path {
                                        d: "M0,35 Q25,28 50,32 T100,25 T150,38 T200,30 L200,60 L0,60 Z",
                                        fill: "url(#upGrad)"
                                    }
                                    path {
                                        d: "M0,35 Q25,28 50,32 T100,25 T150,38 T200,30",
                                        fill: "none",
                                        stroke: "#bc00ff",
                                        stroke_width: "2"
                                    }
                                }
                            }
                        }
                    }

                    // Latency Row
                    div {
                        class: "grid grid-cols-2 gap-3",

                        // Ping Card
                        GlassCard {
                            glow: "emerald".to_string(),
                            class: "relative !p-0 h-20 sm:h-24 overflow-hidden".to_string(),
                            div {
                                class: "relative z-10 p-2.5 sm:p-3",
                                div {
                                    class: "flex justify-between items-center mb-0.5",
                                    span {
                                        class: "text-[8px] sm:text-[9px] font-bold uppercase text-emerald flex items-center gap-1.5 tracking-widest",
                                        span { class: "w-1.5 h-1.5 rounded-sm bg-emerald shadow-[0_0_5px_var(--emerald)]" }
                                        "Ping"
                                    }
                                    svg { class: "w-2.5 h-2.5 text-emerald/50", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M5 12.55a11 11 0 0 1 14.08 0M8.53 16.11a6 6 0 0 1 6.95 0M12 20h.01" } }
                                }
                                div {
                                    class: "flex items-baseline gap-1.5",
                                    span { class: "text-lg sm:text-xl font-mono font-bold text-white tracking-tighter", "{active_node.latency}" }
                                    span { class: "text-[9px] sm:text-[10px] font-mono text-gray-500", "ms" }
                                }
                            }
                            // Area Chart
                            div {
                                class: "absolute inset-0 top-8 opacity-30",
                                svg {
                                    class: "w-full h-full",
                                    view_box: "0 0 200 40",
                                    preserve_aspect_ratio: "none",
                                    defs {
                                        linearGradient { id: "pingGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                            stop { offset: "0%", stop_color: "#00ffa3", stop_opacity: "0.6" }
                                            stop { offset: "100%", stop_color: "#00ffa3", stop_opacity: "0" }
                                        }
                                    }
                                    path {
                                        d: "M0,20 Q30,12 60,18 T120,15 T180,22 T200,18 L200,40 L0,40 Z",
                                        fill: "url(#pingGrad)"
                                    }
                                    path {
                                        d: "M0,20 Q30,12 60,18 T120,15 T180,22 T200,18",
                                        fill: "none",
                                        stroke: "#00ffa3",
                                        stroke_width: "2"
                                    }
                                }
                            }
                        }

                        // Jitter Card
                        GlassCard {
                            glow: "none".to_string(),
                            class: "relative !p-0 h-20 sm:h-24 overflow-hidden".to_string(),
                            div {
                                class: "relative z-10 p-2.5 sm:p-3",
                                div {
                                    class: "flex justify-between items-center mb-0.5",
                                    span {
                                        class: "text-[8px] sm:text-[9px] font-bold uppercase text-warning flex items-center gap-1.5 tracking-widest",
                                        span { class: "w-1.5 h-1.5 rounded-sm bg-warning shadow-[0_0_5px_var(--warning)]" }
                                        "Jitter"
                                    }
                                    svg { class: "w-2.5 h-2.5 text-warning/50", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", polyline { points: "22,12 18,12 15,21 9,3 6,12 2,12" } }
                                }
                                div {
                                    class: "flex items-baseline gap-1.5",
                                    span { class: "text-lg sm:text-xl font-mono font-bold text-white tracking-tighter", "{active_node.jitter}" }
                                    span { class: "text-[9px] sm:text-[10px] font-mono text-gray-500", "ms" }
                                }
                            }
                            // Step Chart (Jitter style)
                            div {
                                class: "absolute inset-0 top-8 opacity-30",
                                svg {
                                    class: "w-full h-full",
                                    view_box: "0 0 200 40",
                                    preserve_aspect_ratio: "none",
                                    defs {
                                        linearGradient { id: "jitterGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                            stop { offset: "0%", stop_color: "#d4ff00", stop_opacity: "0.6" }
                                            stop { offset: "100%", stop_color: "#d4ff00", stop_opacity: "0" }
                                        }
                                    }
                                    path {
                                        d: "M0,25 L20,25 L20,18 L40,18 L40,22 L60,22 L60,15 L80,15 L80,28 L100,28 L100,20 L120,20 L120,25 L140,25 L140,18 L160,18 L160,22 L180,22 L180,20 L200,20 L200,40 L0,40 Z",
                                        fill: "url(#jitterGrad)"
                                    }
                                    path {
                                        d: "M0,25 L20,25 L20,18 L40,18 L40,22 L60,22 L60,15 L80,15 L80,28 L100,28 L100,20 L120,20 L120,25 L140,25 L140,18 L160,18 L160,22 L180,22 L180,20 L200,20",
                                        fill: "none",
                                        stroke: "#d4ff00",
                                        stroke_width: "2"
                                    }
                                }
                            }
                        }
                    }

                    // Connection Card
                    div {
                        class: "w-full",
                        GlassCard {
                            class: "group flex items-center justify-between !p-3 sm:!p-4".to_string(),
                            div {
                                class: "flex items-center gap-4 sm:gap-5",
                                div {
                                    class: "relative w-12 h-12 sm:w-16 sm:h-16 shrink-0",
                                    div { class: "absolute inset-[-3px] sm:inset-[-4px] rounded-[18px] border border-dashed border-white/20 animate-spin-slow" }
                                    div {
                                        class: "w-full h-full rounded-2xl overflow-hidden border border-white/10 group-hover:border-primary/50 transition-colors relative z-10 shadow-lg",
                                        img {
                                            src: "{active_node.flag_url}",
                                            alt: "flag",
                                            class: "w-full h-full object-cover grayscale group-hover:grayscale-0 transition-all duration-500"
                                        }
                                    }
                                }

                                div {
                                    class: "min-w-0",
                                    div {
                                        class: "flex items-center gap-2 mb-0.5 sm:mb-1",
                                        h3 { class: "text-sm sm:text-base font-bold text-white group-hover:text-primary transition-colors truncate", "{active_node.name}" }
                                        if is_active() {
                                            div { class: "w-1.5 h-1.5 sm:w-2 sm:h-2 rounded-full bg-emerald shadow-[0_0_8px_var(--emerald)] animate-pulse-fast shrink-0" }
                                        }
                                    }
                                    div {
                                        class: "flex items-center gap-2 sm:gap-3",
                                        span { class: "text-[9px] sm:text-[10px] font-mono bg-white/5 text-gray-400 px-1.5 py-0.5 rounded border border-white/5 group-hover:border-primary/20 group-hover:text-primary transition-colors", "{active_node.id}" }
                                        span {
                                            class: "text-[10px] sm:text-xs font-mono text-gray-500 flex items-center gap-1 group-hover:text-emerald transition-colors",
                                            svg { class: "w-2.5 h-2.5", fill: "currentColor", view_box: "0 0 24 24", path { d: "M13 10V3L4 14h7v7l9-11h-7z" } }
                                            "{active_node.latency}ms"
                                        }
                                    }
                                }
                            }
                            div {
                                class: "h-8 w-8 sm:h-10 sm:w-10 rounded-full bg-white/5 flex items-center justify-center group-hover:bg-primary/20 group-hover:text-primary transition-all duration-300 shrink-0",
                                svg {
                                    class: "w-4 h-4 text-gray-500 group-hover:text-primary",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    path { d: "M9 5l7 7-7 7" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
