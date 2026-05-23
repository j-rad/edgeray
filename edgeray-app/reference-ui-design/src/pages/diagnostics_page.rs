use crate::components::GlassCard;
use dioxus::prelude::*;

#[component]
pub fn DiagnosticsPage() -> Element {
    let logs = vec![
        (
            "17:45:23",
            "INFO",
            "Connection established to TYO-092",
            "emerald",
        ),
        ("17:45:22", "DEBUG", "TLS handshake completed", "primary"),
        (
            "17:45:21",
            "INFO",
            "DNS resolved: api.example.com → 192.168.1.1",
            "gray",
        ),
        (
            "17:45:20",
            "WARN",
            "High latency detected: 245ms",
            "warning",
        ),
        (
            "17:45:19",
            "INFO",
            "Route matched: PROXY rule applied",
            "primary",
        ),
    ];

    rsx! {
        div {
            class: "animate-fade-in pb-12",

            div {
                class: "flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6 sm:mb-8",
                div {
                    h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "System Monitor" }
                    h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Diagnostics" }
                }

                div {
                    class: "flex gap-2",
                    button { class: "h-10 px-4 bg-white/5 border border-white/10 hover:bg-white/10 text-gray-300 rounded-xl text-xs font-bold uppercase tracking-wider transition-all active:scale-95", "Export Logs" }
                    button { class: "h-10 px-4 bg-primary/10 border border-primary/20 hover:bg-primary/20 text-primary rounded-xl text-xs font-bold uppercase tracking-wider transition-all active:scale-95", "Run Test" }
                }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-4 mb-6",

                // CPU
                GlassCard {
                    glow: "cyan".to_string(),
                    class: "!p-4".to_string(),
                    div {
                        class: "flex items-center justify-between mb-3",
                        span { class: "text-xs text-gray-400 uppercase tracking-wider", "CPU Usage" }
                        span { class: "text-lg font-mono font-bold text-primary", "24%" }
                    }
                    div { class: "h-2 bg-white/10 rounded-full overflow-hidden",
                        div { class: "h-full w-[24%] bg-gradient-to-r from-primary to-purple rounded-full" }
                    }
                }

                // Memory
                GlassCard {
                    glow: "purple".to_string(),
                    class: "!p-4".to_string(),
                    div {
                        class: "flex items-center justify-between mb-3",
                        span { class: "text-xs text-gray-400 uppercase tracking-wider", "Memory" }
                        span { class: "text-lg font-mono font-bold text-purple", "156 MB" }
                    }
                    div { class: "h-2 bg-white/10 rounded-full overflow-hidden",
                        div { class: "h-full w-[35%] bg-gradient-to-r from-purple to-emerald rounded-full" }
                    }
                }

                // Network
                GlassCard {
                    glow: "emerald".to_string(),
                    class: "!p-4".to_string(),
                    div {
                        class: "flex items-center justify-between mb-3",
                        span { class: "text-xs text-gray-400 uppercase tracking-wider", "Connections" }
                        span { class: "text-lg font-mono font-bold text-emerald", "147" }
                    }
                    div { class: "h-2 bg-white/10 rounded-full overflow-hidden",
                        div { class: "h-full w-[60%] bg-gradient-to-r from-emerald to-primary rounded-full" }
                    }
                }
            }

            // Logs Panel
            GlassCard {
                class: "!p-0 overflow-hidden".to_string(),
                div {
                    class: "flex items-center justify-between p-3 sm:p-4 border-b border-white/5",
                    h3 { class: "text-sm font-bold text-white", "Live Logs" }
                    div {
                        class: "flex items-center gap-2",
                        div { class: "w-2 h-2 rounded-full bg-emerald animate-pulse-fast" }
                        span { class: "text-[10px] text-gray-400 font-mono uppercase", "Streaming" }
                    }
                }

                div {
                    class: "max-h-[300px] overflow-y-auto no-scrollbar",
                    for (time, level, message, color) in &logs {
                        {
                            let level_class = match *color {
                                "emerald" => "bg-emerald/20 text-emerald",
                                "primary" => "bg-primary/20 text-primary",
                                "warning" => "bg-warning/20 text-warning",
                                _ => "bg-white/10 text-gray-400",
                            };
                            rsx! {
                                div {
                                    class: "flex items-start gap-3 px-3 sm:px-4 py-2 border-b border-white/5 hover:bg-white/5 transition-colors",
                                    span { class: "text-[10px] font-mono text-gray-500 shrink-0", "{time}" }
                                    span {
                                        class: "text-[9px] font-bold uppercase px-1.5 py-0.5 rounded shrink-0 {level_class}",
                                        "{level}"
                                    }
                                    span { class: "text-xs text-gray-300 font-mono", "{message}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
