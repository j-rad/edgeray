use crate::components::GlassCard;
use dioxus::prelude::*;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div {
            class: "animate-fade-in pb-12",

            div {
                class: "mb-6 sm:mb-8",
                h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Preferences" }
                h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Settings" }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-2 gap-4",

                // Appearance
                GlassCard {
                    class: "!p-4 sm:!p-6",
                    h3 { class: "text-sm font-bold text-white mb-4 flex items-center gap-2",
                        svg { class: "w-4 h-4 text-primary", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", circle { cx: "12", cy: "12", r: "5" } path { d: "M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" } }
                        "Appearance"
                    }
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between p-3 bg-white/5 rounded-lg",
                            span { class: "text-sm text-gray-300", "Theme" }
                            div {
                                class: "flex gap-2",
                                button { class: "w-8 h-8 rounded-lg bg-gray-900 border-2 border-primary" }
                                button { class: "w-8 h-8 rounded-lg bg-gray-100 border border-white/20" }
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-3 bg-white/5 rounded-lg",
                            span { class: "text-sm text-gray-300", "Compact Mode" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-white/10 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-gray-500 shadow-lg" }
                            }
                        }
                    }
                }

                // Notifications
                GlassCard {
                    class: "!p-4 sm:!p-6",
                    h3 { class: "text-sm font-bold text-white mb-4 flex items-center gap-2",
                        svg { class: "w-4 h-4 text-purple", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9M13.73 21a2 2 0 0 1-3.46 0" } }
                        "Notifications"
                    }
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-sm text-gray-300", "Connection Alerts" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-emerald/30 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-emerald translate-x-4 shadow-lg" }
                            }
                        }
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-sm text-gray-300", "Speed Warnings" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-emerald/30 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-emerald translate-x-4 shadow-lg" }
                            }
                        }
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-sm text-gray-300", "Update Reminders" }
                            div { class: "w-10 h-6 rounded-full p-1 bg-white/10 cursor-pointer",
                                div { class: "w-4 h-4 rounded-full bg-gray-500 shadow-lg" }
                            }
                        }
                    }
                }

                // About
                GlassCard {
                    glow: "cyan",
                    class: "!p-4 sm:!p-6 lg:col-span-2",
                    div {
                        class: "flex items-center gap-4 mb-4",
                        div {
                            class: "w-12 h-12 rounded-xl bg-gradient-to-br from-primary/20 to-purple/20 flex items-center justify-center border border-white/10",
                            svg { class: "w-6 h-6 text-primary", fill: "rgba(34,211,238,0.2)", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" } }
                        }
                        div {
                            h3 { class: "font-bold text-white", "EdgeRay Pro" }
                            p { class: "text-xs text-gray-400", "Version 2.0.0 • Built with Rust & Dioxus" }
                        }
                    }
                    div {
                        class: "flex flex-wrap gap-2",
                        button { class: "px-3 py-2 bg-white/5 border border-white/10 rounded-lg text-xs text-gray-400 hover:text-white hover:border-white/20 transition-colors", "Check for Updates" }
                        button { class: "px-3 py-2 bg-white/5 border border-white/10 rounded-lg text-xs text-gray-400 hover:text-white hover:border-white/20 transition-colors", "View Changelog" }
                        button { class: "px-3 py-2 bg-white/5 border border-white/10 rounded-lg text-xs text-gray-400 hover:text-white hover:border-white/20 transition-colors", "Export Config" }
                    }
                }
            }
        }
    }
}
