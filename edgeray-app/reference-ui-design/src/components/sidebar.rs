use crate::types::NavigationTab;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct SidebarProps {
    pub active_tab: NavigationTab,
    pub on_tab_change: EventHandler<NavigationTab>,
    pub is_vpn_active: bool,
    pub is_open: bool,
    pub on_close: EventHandler,
    pub is_desktop_expanded: bool,
    pub on_desktop_toggle: EventHandler,
}

fn render_icon(icon_name: &str, is_active: bool) -> Element {
    let stroke_w = if is_active { "2.5" } else { "2" };
    match icon_name {
        "dashboard" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                rect { x: "3", y: "3", width: "7", height: "7" }
                rect { x: "14", y: "3", width: "7", height: "7" }
                rect { x: "14", y: "14", width: "7", height: "7" }
                rect { x: "3", y: "14", width: "7", height: "7" }
            }
        },
        "server" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                rect { x: "2", y: "2", width: "20", height: "8", rx: "2", ry: "2" }
                rect { x: "2", y: "14", width: "20", height: "8", rx: "2", ry: "2" }
                line { x1: "6", y1: "6", x2: "6.01", y2: "6" }
                line { x1: "6", y1: "18", x2: "6.01", y2: "18" }
            }
        },
        "shield" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
            }
        },
        "network" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                circle { cx: "12", cy: "5", r: "3" }
                circle { cx: "5", cy: "19", r: "3" }
                circle { cx: "19", cy: "19", r: "3" }
                line { x1: "12", y1: "8", x2: "12", y2: "12" }
                path { d: "M12 12l-7 7M12 12l7 7" }
            }
        },
        "chart" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                line { x1: "12", y1: "20", x2: "12", y2: "10" }
                line { x1: "18", y1: "20", x2: "18", y2: "4" }
                line { x1: "6", y1: "20", x2: "6", y2: "16" }
            }
        },
        "activity" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                polyline { points: "22,12 18,12 15,21 9,3 6,12 2,12" }
            }
        },
        "merge" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                circle { cx: "18", cy: "18", r: "3" }
                circle { cx: "6", cy: "6", r: "3" }
                path { d: "M6 21V9a9 9 0 0 0 9 9" }
            }
        },
        "sliders" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                line { x1: "4", y1: "21", x2: "4", y2: "14" }
                line { x1: "4", y1: "10", x2: "4", y2: "3" }
                line { x1: "12", y1: "21", x2: "12", y2: "12" }
                line { x1: "12", y1: "8", x2: "12", y2: "3" }
                line { x1: "20", y1: "21", x2: "20", y2: "16" }
                line { x1: "20", y1: "12", x2: "20", y2: "3" }
                line { x1: "1", y1: "14", x2: "7", y2: "14" }
                line { x1: "9", y1: "8", x2: "15", y2: "8" }
                line { x1: "17", y1: "16", x2: "23", y2: "16" }
            }
        },
        "cog" => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "3" }
                path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
            }
        },
        _ => rsx! {
            svg { class: "w-5 h-5", fill: "none", stroke: "currentColor", stroke_width: "{stroke_w}", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "10" }
            }
        },
    }
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    // Define navigation sections
    let nav_items = [
        (NavigationTab::Core, "dashboard", "Dashboard"),
        (NavigationTab::Nodes, "server", "Nodes"),
        (NavigationTab::Mesh, "shield", "Shield"),
    ];

    let lab_items = [
        (NavigationTab::Topology, "network", "Mesh Topology"),
        (NavigationTab::Diagnostics, "chart", "Diagnostics"),
        (NavigationTab::Tracer, "activity", "Packet Tracer"),
        (NavigationTab::Routing, "merge", "Routing Rules"),
        (NavigationTab::Setup, "sliders", "Tuning & Config"),
    ];

    let sys_items = [(NavigationTab::Settings, "cog", "Settings")];

    let sidebar_translate = if props.is_open {
        "translate-x-0"
    } else {
        "-translate-x-full lg:translate-x-0"
    };
    let sidebar_width = if props.is_desktop_expanded {
        "lg:w-[260px]"
    } else {
        "lg:w-[80px]"
    };
    let header_justify = if props.is_desktop_expanded {
        "justify-between"
    } else {
        "lg:justify-center"
    };
    let logo_hidden = if props.is_desktop_expanded {
        ""
    } else {
        "lg:hidden lg:opacity-0 lg:w-0 overflow-hidden"
    };
    let label_hidden = if props.is_desktop_expanded {
        ""
    } else {
        "lg:hidden lg:opacity-0 lg:w-0"
    };
    let footer_center = if props.is_desktop_expanded {
        ""
    } else {
        "lg:justify-center"
    };

    let mobile_backdrop_class = if props.is_open {
        "opacity-100 pointer-events-auto"
    } else {
        "opacity-0 pointer-events-none"
    };

    let on_close = props.on_close;
    let on_desktop_toggle = props.on_desktop_toggle;

    let vpn_dot_class = if props.is_vpn_active {
        "bg-emerald shadow-[0_0_12px_#00ffa3]"
    } else {
        "bg-slate-700"
    };
    let vpn_status_text = if props.is_vpn_active {
        "System Nominal"
    } else {
        "Idle"
    };

    rsx! {
        // Mobile Backdrop
        div {
            class: "fixed inset-0 bg-black/60 backdrop-blur-md z-40 transition-opacity duration-300 lg:hidden {mobile_backdrop_class}",
            onclick: move |_| on_close.call(())
        }

        // Sidebar
        aside {
            class: "fixed top-0 left-0 bottom-0 z-50 glass-panel border-r-0 border-white/5 flex flex-col pt-safe transition-all duration-300 ease-[cubic-bezier(0.2,0.8,0.2,1)] w-[260px] {sidebar_translate} {sidebar_width}",

            // Desktop Toggle Button
            button {
                class: "hidden lg:flex absolute -right-3 top-24 w-6 h-6 bg-white/10 backdrop-blur-md border border-white/20 rounded-full items-center justify-center text-gray-300 hover:text-white hover:border-primary/50 hover:bg-primary/20 transition-all z-50 shadow-lg",
                onclick: move |_| on_desktop_toggle.call(()),
                if props.is_desktop_expanded {
                    svg { class: "w-3.5 h-3.5", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M15 18l-6-6 6-6" } }
                } else {
                    svg { class: "w-3.5 h-3.5", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M9 18l6-6-6-6" } }
                }
            }

            // Header / Logo
            div {
                class: "h-[80px] flex items-center px-6 {header_justify}",
                div {
                    class: "flex items-center gap-3",
                    div {
                        class: "w-10 h-10 rounded-xl bg-gradient-to-br from-white/10 to-transparent border border-white/10 p-0.5 flex-shrink-0 shadow-lg group cursor-pointer hover:border-primary/50 transition-colors",
                        div {
                            class: "w-full h-full bg-black/20 rounded-[10px] flex items-center justify-center relative overflow-hidden backdrop-blur-sm",
                            div { class: "absolute inset-0 bg-primary/20 opacity-0 group-hover:opacity-100 transition-opacity" }
                            svg { class: "w-5 h-5 text-primary relative z-10", fill: "rgba(34, 211, 238, 0.2)", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24",
                                path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            }
                        }
                    }
                    div {
                        class: "flex flex-col justify-center transition-all duration-300 {logo_hidden}",
                        span { class: "text-base font-bold text-white tracking-tight leading-none drop-shadow-md",
                            "EDGE"
                            span { class: "text-primary", "RAY" }
                        }
                        span { class: "text-[9px] text-gray-400 font-mono tracking-[0.25em] mt-1", "PRO SUITE" }
                    }
                }
                // Mobile Close
                button {
                    class: "lg:hidden p-2 text-gray-400 hover:text-white active:scale-95 transition-transform",
                    onclick: move |_| props.on_close.call(()),
                    svg { class: "w-6 h-6", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M18 6L6 18M6 6l12 12" } }
                }
            }

            // Scrollable Content
            div {
                class: "flex-1 overflow-y-auto py-6 px-3 space-y-8 no-scrollbar",

                // Primary Nav
                nav {
                    class: "flex flex-col gap-2",
                    div { class: "px-4 mb-2 text-[10px] text-gray-400 uppercase tracking-widest font-bold {label_hidden}", "Main Module" }
                    for (tab, icon, label) in nav_items.iter() {
                        {render_nav_item(*tab, icon, label, props.active_tab, "primary", props.is_desktop_expanded, props.on_tab_change, props.on_close)}
                    }
                }

                // Labs / Technical
                nav {
                    class: "flex flex-col gap-2",
                    div { class: "px-4 mb-2 flex items-center gap-2 text-[10px] text-gray-400 uppercase tracking-widest font-bold {label_hidden}", "Technical" }
                    for (tab, icon, label) in lab_items.iter() {
                        {render_nav_item(*tab, icon, label, props.active_tab, "purple", props.is_desktop_expanded, props.on_tab_change, props.on_close)}
                    }
                }

                // System
                nav {
                    class: "flex flex-col gap-2",
                    div { class: "px-4 mb-2 flex items-center gap-2 text-[10px] text-gray-400 uppercase tracking-widest font-bold {label_hidden}", "System" }
                    for (tab, icon, label) in sys_items.iter() {
                        {render_nav_item(*tab, icon, label, props.active_tab, "primary", props.is_desktop_expanded, props.on_tab_change, props.on_close)}
                    }
                }
            }

            // Status Footer
            div {
                class: "p-6 border-t border-white/5 flex items-center {footer_center}",
                div {
                    class: "relative w-2 h-2 rounded-full transition-colors duration-500 {vpn_dot_class}",
                    if props.is_vpn_active {
                        div { class: "absolute inset-0 rounded-full bg-emerald animate-ping opacity-75 duration-1000" }
                    }
                }
                span { class: "ml-3 text-[10px] font-mono uppercase tracking-widest text-gray-500 transition-all duration-300 {label_hidden}", "{vpn_status_text}" }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_nav_item(
    tab: NavigationTab,
    icon: &str,
    label: &str,
    active_tab: NavigationTab,
    color_class: &str,
    is_expanded: bool,
    on_tab_change: EventHandler<NavigationTab>,
    on_close: EventHandler,
) -> Element {
    let is_active = active_tab == tab;

    let (color_style, active_text, bg_bar) = if color_class == "purple" {
        (
            "text-purple shadow-[0_0_15px_var(--purple)]",
            "text-purple drop-shadow-[0_0_8px_rgba(188,0,255,0.8)]",
            "bg-purple",
        )
    } else {
        (
            "text-primary shadow-[0_0_15px_var(--primary)]",
            "text-primary drop-shadow-[0_0_8px_rgba(34,211,238,0.8)]",
            "bg-primary",
        )
    };

    let btn_base = "group relative flex items-center p-3 rounded-xl transition-all duration-300 active:scale-95";
    let btn_justify = if is_expanded { "" } else { "lg:justify-center" };
    let btn_active = if is_active {
        "bg-white/10 text-white shadow-inner border border-white/5"
    } else {
        "text-gray-400 hover:bg-white/5 hover:text-white hover:border hover:border-white/5"
    };
    let btn_class = format!("{btn_base} {btn_justify} {btn_active}");

    let icon_class = if is_active {
        format!("transition-all duration-300 {active_text}")
    } else {
        "transition-all duration-300 group-hover:text-gray-200".to_string()
    };
    let label_hidden = if is_expanded {
        ""
    } else {
        "lg:hidden lg:opacity-0 lg:w-0"
    };

    let tab_clone = tab;
    let icon_name = icon.to_string();
    let label_str = label.to_string();

    rsx! {
        button {
            key: "{tab:?}",
            class: "{btn_class}",
            onclick: move |_| {
                on_tab_change.call(tab_clone);
                on_close.call(());
            },

            if is_active {
                div { class: "absolute left-0 top-1/2 -translate-y-1/2 w-1 h-8 {bg_bar} rounded-r-full {color_style} animate-pulse-fast" }
            }

            div {
                class: "relative z-10 {icon_class}",
                {render_icon(&icon_name, is_active)}
            }

            span { class: "ml-3 text-sm font-medium whitespace-nowrap transition-all duration-300 {label_hidden}", "{label_str}" }

            if !is_expanded {
                div { class: "hidden lg:block absolute left-full ml-4 px-3 py-1.5 bg-black/60 backdrop-blur-xl border border-white/10 rounded-lg text-xs text-white opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-50 shadow-xl", "{label_str}" }
            }
        }
    }
}
