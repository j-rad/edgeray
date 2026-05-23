use crate::components::GlassCard;
use dioxus::prelude::*;

#[component]
pub fn SetupPage() -> Element {
    rsx! {
        div {
            class: "animate-fade-in pb-12",

            div {
                class: "mb-6 sm:mb-8",
                h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Configuration" }
                h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Tuning & Config" }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-4",

                // Protocol Settings
                GlassCard {
                    glow: "cyan",
                    class: "!p-4 sm:!p-6",
                    h3 { class: "text-sm font-bold text-white mb-4 flex items-center gap-2",
                        svg { class: "w-4 h-4 text-primary", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" } }
                        "Protocol Settings"
                    }
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between p-3 bg-white/5 rounded-lg border border-white/5",
                            span { class: "text-sm text-gray-300", "Default Protocol" }
                            select {
                                class: "bg-transparent text-primary text-sm font-mono border-none focus:outline-none cursor-pointer",
                                option { "VLESS + Reality" }
                                option { "Trojan" }
                                option { "VMess" }
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-3 bg-white/5 rounded-lg border border-white/5",
                            span { class: "text-sm text-gray-300", "Encryption" }
                            span { class: "text-sm font-mono text-emerald", "AES-256-GCM" }
                        }
                    }
                }

                // Performance Tuning
                GlassCard {
                    glow: "purple",
                    class: "!p-4 sm:!p-6",
                    h3 { class: "text-sm font-bold text-white mb-4 flex items-center gap-2",
                        svg { class: "w-4 h-4 text-purple", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M13 2L3 14h9l-1 8 10-12h-9l1-8z" } }
                        "Performance Tuning"
                    }
                    div {
                        class: "space-y-4",
                        div {
                            div { class: "flex justify-between text-xs mb-1",
                                span { class: "text-gray-400", "MTU Size" }
                                span { class: "text-primary font-mono", "1400" }
                            }
                            div { class: "h-2 bg-white/10 rounded-full overflow-hidden",
                                div { class: "h-full w-[70%] bg-gradient-to-r from-primary to-purple rounded-full" }
                            }
                        }
                        div {
                            div { class: "flex justify-between text-xs mb-1",
                                span { class: "text-gray-400", "Buffer Size" }
                                span { class: "text-purple font-mono", "64KB" }
                            }
                            div { class: "h-2 bg-white/10 rounded-full overflow-hidden",
                                div { class: "h-full w-[50%] bg-gradient-to-r from-purple to-emerald rounded-full" }
                            }
                        }
                    }
                }

                // DNS Configuration
                GlassCard {
                    class: "!p-4 sm:!p-6",
                    h3 { class: "text-sm font-bold text-white mb-4", "DNS Configuration" }
                    div {
                        class: "space-y-2",
                        div {
                            class: "flex items-center gap-3 p-3 bg-white/5 rounded-lg border border-white/5",
                            div { class: "w-2 h-2 rounded-full bg-emerald" }
                            span { class: "text-sm text-gray-300 flex-1", "Primary DNS" }
                            span { class: "text-xs font-mono text-gray-400", "1.1.1.1" }
                        }
                        div {
                            class: "flex items-center gap-3 p-3 bg-white/5 rounded-lg border border-white/5",
                            div { class: "w-2 h-2 rounded-full bg-primary" }
                            span { class: "text-sm text-gray-300 flex-1", "Secondary DNS" }
                            span { class: "text-xs font-mono text-gray-400", "8.8.8.8" }
                        }
                    }
                }

                // Advanced Options
                GlassCard {
                    class: "!p-4 sm:!p-6",
                    h3 { class: "text-sm font-bold text-white mb-4", "Advanced Options" }
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-sm text-gray-300", "TCP Fast Open" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-emerald/30 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-emerald translate-x-4 shadow-lg" }
                            }
                        }
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-sm text-gray-300", "Multiplex" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-emerald/30 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-emerald translate-x-4 shadow-lg" }
                            }
                        }
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-sm text-gray-300", "QUIC" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-white/10 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-gray-500 shadow-lg" }
                            }
                        }
                    }
                }
            }
        }
    }
}
