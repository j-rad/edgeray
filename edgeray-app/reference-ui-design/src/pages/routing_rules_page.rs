use crate::components::GlassCard;
use dioxus::prelude::*;

#[component]
pub fn RoutingRulesPage() -> Element {
    let rules = vec![
        ("Direct", "*.local, 192.168.*", "emerald", true),
        ("Proxy", "*.google.com, *.youtube.com", "primary", true),
        ("Block", "*.ads.*, *.tracking.*", "purple", true),
        ("Reject", "*.malware.*", "warning", false),
    ];

    rsx! {
        div {
            class: "animate-fade-in pb-12",

            div {
                class: "flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6 sm:mb-8",
                div {
                    h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Traffic Control" }
                    h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Routing Rules" }
                }

                button {
                    class: "h-10 px-4 bg-primary/10 border border-primary/20 hover:bg-primary/20 text-primary rounded-xl flex items-center gap-2 text-xs font-bold uppercase tracking-wider transition-all active:scale-95",
                    svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M12 5v14m-7-7h14" } }
                    "Add Rule"
                }
            }

            div {
                class: "space-y-3",
                for (action, pattern, color, enabled) in &rules {
                    {
                        let glow_val = if *enabled { *color } else { "none" };
                        let icon_bg = match *color {
                            "emerald" => "bg-emerald/20",
                            "primary" => "bg-primary/20",
                            "purple" => "bg-purple/20",
                            _ => "bg-warning/20",
                        };
                        let icon_color = match *color {
                            "emerald" => "text-emerald",
                            "primary" => "text-primary",
                            "purple" => "text-purple",
                            _ => "text-warning",
                        };
                        let status_class = if *enabled { "text-emerald" } else { "text-gray-500" };
                        let toggle_bg = if *enabled { "bg-emerald/30" } else { "bg-white/10" };
                        let toggle_dot = if *enabled { "translate-x-4 bg-emerald" } else { "bg-gray-500" };

                        rsx! {
                            GlassCard {
                                glow: glow_val.to_string(),
                                class: "!p-3 sm:!p-4".to_string(),

                                div {
                                    class: "flex items-center justify-between",
                                    div {
                                        class: "flex items-center gap-4",
                                        div {
                                            class: "w-10 h-10 rounded-xl flex items-center justify-center {icon_bg}",
                                            svg {
                                                class: "w-5 h-5 {icon_color}",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                view_box: "0 0 24 24",
                                                match *action {
                                                    "Direct" => rsx! { path { d: "M5 12h14M12 5l7 7-7 7" } },
                                                    "Proxy" => rsx! { path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" } },
                                                    "Block" => rsx! { path { d: "M18.36 6.64A9 9 0 0 1 5.64 18.36M18.36 6.64L5.64 18.36M18.36 6.64A9 9 0 0 0 5.64 18.36" } },
                                                    _ => rsx! { path { d: "M18 6L6 18M6 6l12 12" } }
                                                }
                                            }
                                        }
                                        div {
                                            h4 { class: "font-bold text-sm text-white", "{action}" }
                                            p { class: "text-[10px] text-gray-500 font-mono", "{pattern}" }
                                        }
                                    }

                                    div {
                                        class: "flex items-center gap-3",
                                        span { class: "text-[10px] uppercase tracking-wider {status_class}", if *enabled {"Active"} else {"Disabled"} }
                                        div {
                                            class: "w-10 h-6 rounded-full p-1 cursor-pointer {toggle_bg}",
                                            div { class: "w-4 h-4 rounded-full shadow-lg transition-transform {toggle_dot}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Rule Priority Info
            div {
                class: "mt-6 p-4 bg-white/5 rounded-xl border border-white/10",
                div {
                    class: "flex items-start gap-3",
                    svg { class: "w-5 h-5 text-primary shrink-0 mt-0.5", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", circle { cx: "12", cy: "12", r: "10" } path { d: "M12 16v-4m0-4h.01" } }
                    div {
                        h4 { class: "text-sm font-bold text-white mb-1", "Rule Priority" }
                        p { class: "text-xs text-gray-400", "Rules are evaluated from top to bottom. Drag to reorder priority. The first matching rule will be applied." }
                    }
                }
            }
        }
    }
}
