use crate::components::GlassCard;
use dioxus::prelude::*;

#[derive(Clone)]
struct AppInfo {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    icon_type: &'static str,
    allowed: bool,
}

#[allow(dead_code)]
struct ThreatInfo {
    id: u32,
    domain: &'static str,
    time: &'static str,
    source: &'static str,
}

fn get_apps() -> Vec<AppInfo> {
    vec![
        AppInfo {
            id: "1",
            name: "Chrome Browser",
            category: "Web",
            icon_type: "chrome",
            allowed: true,
        },
        AppInfo {
            id: "2",
            name: "Spotify",
            category: "Media",
            icon_type: "music",
            allowed: true,
        },
        AppInfo {
            id: "3",
            name: "Instagram",
            category: "Social",
            icon_type: "image",
            allowed: false,
        },
        AppInfo {
            id: "4",
            name: "WhatsApp",
            category: "Messaging",
            icon_type: "message",
            allowed: true,
        },
        AppInfo {
            id: "5",
            name: "Netflix",
            category: "Video",
            icon_type: "video",
            allowed: true,
        },
        AppInfo {
            id: "6",
            name: "System Core",
            category: "OS",
            icon_type: "phone",
            allowed: true,
        },
        AppInfo {
            id: "7",
            name: "Slack",
            category: "Work",
            icon_type: "message",
            allowed: true,
        },
        AppInfo {
            id: "8",
            name: "Zoom",
            category: "Work",
            icon_type: "video",
            allowed: false,
        },
    ]
}

fn get_threats() -> Vec<ThreatInfo> {
    vec![
        ThreatInfo {
            id: 1,
            domain: "analytics.google...",
            time: "14:02:44",
            source: "Chrome",
        },
        ThreatInfo {
            id: 2,
            domain: "graph.facebook.c...",
            time: "14:01:20",
            source: "Instagram",
        },
        ThreatInfo {
            id: 3,
            domain: "tracking.adjust.com",
            time: "13:59:12",
            source: "Background",
        },
        ThreatInfo {
            id: 4,
            domain: "ads.doubleclick.net",
            time: "13:55:01",
            source: "Chrome",
        },
        ThreatInfo {
            id: 5,
            domain: "telemetry.tiktok.a...",
            time: "13:42:33",
            source: "TikTok",
        },
    ]
}

fn render_app_icon(icon_type: &str) -> Element {
    match icon_type {
        "chrome" => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "10" }
                circle { cx: "12", cy: "12", r: "4" }
                path { d: "M21.17 8H12M3.95 6.06L8.54 14M10.88 21.94L15.46 14" }
            }
        },
        "music" => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                path { d: "M9 18V5l12-2v13" }
                circle { cx: "6", cy: "18", r: "3" }
                circle { cx: "18", cy: "16", r: "3" }
            }
        },
        "image" => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                rect { x: "3", y: "3", width: "18", height: "18", rx: "2", ry: "2" }
                circle { cx: "8.5", cy: "8.5", r: "1.5" }
                polyline { points: "21 15 16 10 5 21" }
            }
        },
        "message" => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                path { d: "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z" }
            }
        },
        "video" => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                polygon { points: "23 7 16 12 23 17 23 7" }
                rect { x: "1", y: "5", width: "15", height: "14", rx: "2", ry: "2" }
            }
        },
        "phone" => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                rect { x: "5", y: "2", width: "14", height: "20", rx: "2", ry: "2" }
                line { x1: "12", y1: "18", x2: "12.01", y2: "18" }
            }
        },
        _ => rsx! {
            svg { class: "w-[18px] h-[18px] sm:w-6 sm:h-6", fill: "none", stroke: "currentColor", stroke_width: "1.5", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "10" }
            }
        },
    }
}

#[component]
pub fn ShieldPage() -> Element {
    let mut apps = use_signal(get_apps);
    let mut search_term = use_signal(String::new);
    let threats = get_threats();

    rsx! {
        div {
            class: "animate-fade-in pb-12",

            // Header
            div {
                class: "flex justify-between items-end mb-6 sm:mb-8",
                div {
                    h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Privacy Guard" }
                    h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Active Shield" }
                }
                div {
                    class: "hidden md:flex items-center gap-2 px-3 py-1.5 rounded-lg bg-emerald/10 border border-emerald/20",
                    svg { class: "w-3.5 h-3.5 text-emerald", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                        path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                    }
                    span { class: "text-xs font-bold text-emerald uppercase tracking-wide", "Protection Active" }
                }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-12 gap-4 lg:gap-6",

                // Left Col: App Matrix (8 cols)
                div {
                    class: "lg:col-span-8 flex flex-col gap-4",

                    // Search Bar
                    div {
                        class: "relative group",
                        div {
                            class: "absolute inset-y-0 left-0 pl-3 sm:pl-4 flex items-center pointer-events-none",
                            svg { class: "w-4 h-4 text-gray-500 group-focus-within:text-primary transition-colors", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                                circle { cx: "11", cy: "11", r: "8" }
                                line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Search protected apps...",
                            value: "{search_term}",
                            oninput: move |e| search_term.set(e.value()),
                            class: "w-full h-10 sm:h-12 pl-10 sm:pl-12 pr-4 bg-surface/50 border border-white/10 rounded-xl sm:rounded-2xl text-xs sm:text-sm text-white placeholder-gray-500 focus:outline-none focus:border-primary/50 focus:bg-surface/80 transition-all backdrop-blur-sm"
                        }
                    }

                    // App List
                    div {
                        class: "grid gap-2 sm:gap-3",
                        {
                            let search = search_term().to_lowercase();
                            let filtered_apps: Vec<(usize, AppInfo)> = {
                                let apps_guard = apps.read();
                                apps_guard.iter().enumerate()
                                    .filter(|(_, app)| app.name.to_lowercase().contains(&search))
                                    .map(|(idx, app)| (idx, app.clone()))
                                    .collect()
                            };

                            rsx! {
                                for (idx, app) in filtered_apps.iter() {
                                    {
                                        let allowed = app.allowed;
                                        let icon_bg = if allowed { "bg-white/5 text-white border border-white/10" } else { "bg-black/40 text-gray-600 border border-white/5 grayscale" };
                                        let name_class = if allowed { "text-white" } else { "text-gray-500" };
                                        let status_text = if allowed { "Tunneling" } else { "Bypassed" };
                                        let status_class = if allowed { "text-emerald" } else { "text-gray-600" };
                                        let toggle_bg = if allowed { "bg-purple/20 border-purple/50 shadow-[0_0_15px_rgba(188,0,255,0.3)]" } else { "bg-white/5 border-white/10" };
                                        let toggle_knob_class = if allowed { "bg-purple translate-x-4 sm:translate-x-5 shadow-[0_0_10px_var(--purple)]" } else { "bg-gray-400 translate-x-0" };
                                        let icon_type = app.icon_type;
                                        let idx = *idx;

                                        rsx! {
                                            GlassCard {
                                                key: "{app.id}",
                                                class: "!p-2.5 sm:!p-3 flex items-center justify-between group hover:bg-white/5 transition-colors".to_string(),
                                                div {
                                                    class: "flex items-center gap-3 sm:gap-4",
                                                    div {
                                                        class: "w-9 h-9 sm:w-12 sm:h-12 rounded-lg sm:rounded-xl flex items-center justify-center transition-all {icon_bg}",
                                                        {render_app_icon(icon_type)}
                                                    }
                                                    div {
                                                        h3 { class: "text-xs sm:text-sm font-bold transition-colors {name_class}", "{app.name}" }
                                                        div {
                                                            class: "flex items-center gap-2",
                                                            span { class: "text-[9px] sm:text-[10px] font-mono text-gray-500 uppercase tracking-wider", "{app.category}" }
                                                            if !allowed {
                                                                span {
                                                                    class: "text-[8px] sm:text-[9px] font-bold text-red-500 flex items-center gap-1",
                                                                    svg { class: "w-2 h-2", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                                                                        rect { x: "3", y: "11", width: "18", height: "11", rx: "2", ry: "2" }
                                                                        path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                                                                    }
                                                                    "BLOCKED"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }

                                                div {
                                                    class: "flex items-center gap-3 sm:gap-4",
                                                    span { class: "text-[9px] font-mono uppercase tracking-widest hidden sm:block {status_class}", "{status_text}" }
                                                    div {
                                                        class: "w-9 h-5 sm:w-11 sm:h-6 rounded-full p-0.5 transition-all duration-300 cursor-pointer flex items-center border {toggle_bg}",
                                                        onclick: move |_| {
                                                            apps.with_mut(|a| {
                                                                a[idx].allowed = !a[idx].allowed;
                                                            });
                                                        },
                                                        div { class: "w-3.5 h-3.5 sm:w-4 sm:h-4 rounded-full shadow-md transition-all duration-300 {toggle_knob_class}" }
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

                // Right Col: DNS HUD & Threats (4 cols)
                div {
                    class: "lg:col-span-4 flex flex-col gap-4 sm:gap-6",

                    // DNS HUD Card
                    GlassCard {
                        class: "relative overflow-hidden !p-3 sm:!p-5".to_string(),
                        div {
                            class: "flex justify-between items-start mb-4 relative z-10",
                            div {
                                div {
                                    class: "flex items-center gap-2 mb-1",
                                    svg { class: "w-3.5 h-3.5 sm:w-4 sm:h-4 text-emerald", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                                        circle { cx: "12", cy: "12", r: "10" }
                                        line { x1: "2", y1: "12", x2: "22", y2: "12" }
                                        path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                                    }
                                    h3 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-widest text-emerald", "DNSCrypt" }
                                }
                                span { class: "text-lg sm:text-xl font-bold text-white", "Cloudflare Security" }
                            }
                            div {
                                class: "text-right",
                                div { class: "text-[9px] sm:text-[10px] text-gray-400 uppercase tracking-widest mb-1", "Latency" }
                                div {
                                    class: "text-lg sm:text-xl font-mono font-bold text-primary",
                                    "14"
                                    span { class: "text-xs sm:text-sm text-gray-500 ml-1", "ms" }
                                }
                            }
                        }

                        // Status
                        div {
                            class: "flex items-center gap-2 sm:gap-3 mb-6 relative z-10",
                            div { class: "w-1.5 h-1.5 sm:w-2 sm:h-2 rounded-full bg-emerald shadow-[0_0_10px_var(--emerald)] animate-pulse-fast" }
                            span { class: "text-[10px] sm:text-xs text-gray-300 font-mono", "Encrypted / DoH" }
                        }

                        // Chart Background
                        div {
                            class: "absolute inset-x-0 bottom-0 h-16 opacity-30 pointer-events-none",
                            svg {
                                class: "w-full h-full",
                                view_box: "0 0 200 40",
                                preserve_aspect_ratio: "none",
                                defs {
                                    linearGradient { id: "dnsGrad", x1: "0", y1: "0", x2: "0", y2: "1",
                                        stop { offset: "0%", stop_color: "#22d3ee", stop_opacity: "0.5" }
                                        stop { offset: "100%", stop_color: "#22d3ee", stop_opacity: "0" }
                                    }
                                }
                                path {
                                    d: "M0,20 Q30,15 60,18 T120,14 T180,20 T200,16 L200,40 L0,40 Z",
                                    fill: "url(#dnsGrad)"
                                }
                                path {
                                    d: "M0,20 Q30,15 60,18 T120,14 T180,20 T200,16",
                                    fill: "none",
                                    stroke: "#22d3ee",
                                    stroke_width: "2"
                                }
                            }
                        }
                    }

                    // Threat Log
                    div {
                        class: "flex flex-col gap-2 sm:gap-3 flex-1",
                        div {
                            class: "flex items-center justify-between px-1",
                            h3 {
                                class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 flex items-center gap-2",
                                svg { class: "w-3.5 h-3.5", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                                    path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                                    path { d: "M12 8v4M12 16h.01" }
                                }
                                "Threat Log"
                            }
                            span { class: "text-[9px] sm:text-[10px] font-mono text-gray-600 bg-white/5 px-2 py-0.5 rounded", "24h" }
                        }

                        div {
                            class: "flex-1 space-y-2",
                            for threat in threats.iter() {
                                div {
                                    key: "{threat.id}",
                                    class: "p-2.5 sm:p-3 rounded-xl bg-black/40 border border-white/5 flex gap-3 hover:border-red-500/30 transition-colors group",
                                    div {
                                        class: "mt-0.5 w-7 h-7 sm:w-8 sm:h-8 rounded-lg bg-red-500/10 flex items-center justify-center text-red-500 border border-red-500/20 group-hover:shadow-[0_0_15px_rgba(239,68,68,0.2)] transition-all shrink-0",
                                        svg { class: "w-3.5 h-3.5 sm:w-4 sm:h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                                            path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                                            path { d: "M12 8v4M12 16h.01" }
                                        }
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        div {
                                            class: "flex justify-between items-center mb-0.5",
                                            span { class: "text-[11px] sm:text-xs font-bold text-gray-300 truncate max-w-[120px]", "{threat.domain}" }
                                            span { class: "text-[8px] sm:text-[9px] font-mono text-gray-600", "{threat.time}" }
                                        }
                                        div {
                                            class: "flex items-center gap-1.5",
                                            span { class: "text-[8px] sm:text-[9px] uppercase tracking-wider text-red-400 font-bold", "BLOCKED" }
                                            span { class: "w-0.5 h-2 bg-gray-700" }
                                            span { class: "text-[8px] sm:text-[9px] text-gray-500", "{threat.source}" }
                                        }
                                    }
                                }
                            }

                            button {
                                class: "w-full py-2.5 sm:py-3 mt-2 text-[9px] sm:text-[10px] font-bold uppercase tracking-widest text-gray-500 hover:text-white border border-dashed border-white/10 hover:border-white/30 rounded-xl transition-all",
                                "View Full History"
                            }
                        }
                    }
                }
            }
        }
    }
}
