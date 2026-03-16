use crate::components::Page;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MeshSafety {
    Secure,
    Compromised,
    Offline,
}

#[component]
pub fn Sidebar(
    current_page: Signal<Page>,
    on_navigate: EventHandler<Page>,
    mesh_safety: Signal<MeshSafety>,

    /// UI Mode switch handler
    #[props(default)]
    on_toggle_mode: Option<EventHandler<()>>,
    #[props(default = false)] is_drawer: bool,
) -> Element {
    // Navigation Sections configuration
    let nav_sections = vec![
        (
            "CORE",
            vec![
                (Page::Dashboard, "dashboard", "Dashboard"),
                (Page::Configs, "server", "Nodes"),
            ],
        ),
        (
            "ENGINE",
            vec![
                (Page::Firewall, "shield", "Shield"),
                (Page::RoutingRules, "merge", "Rules"),
                (Page::Logs, "activity", "Tracer"),
            ],
        ),
        (
            "DEBUG",
            vec![
                (Page::Mesh, "network", "Mesh"),
                (Page::StackMonitor, "chart", "Stats"),
            ],
        ),
        ("SYSTEM", vec![(Page::Settings, "cog", "Settings")]),
    ];

    let safety_color = match *mesh_safety.read() {
        MeshSafety::Secure => "bg-emerald shadow-[0_0_12px_#00ffa3]",
        MeshSafety::Compromised => "bg-warning shadow-[0_0_12px_var(--warning)]",
        MeshSafety::Offline => "bg-red-500 shadow-[0_0_12px_var(--red-500)]",
    };

    let status_text = match *mesh_safety.read() {
        MeshSafety::Secure => "System Nominal",
        MeshSafety::Compromised => "Security Alert",
        MeshSafety::Offline => "Offline",
    };

    // Main container style to match reference glass panel feel
    // Note: The parent AdaptiveShell handles the "aside" positioning and width transitions (w-20 -> w-72) for desktop.
    // We strictly fill the available space.
    rsx! {
        div {
            class: "flex flex-col h-full w-full",

            // --- Header / Logo ---
            div {
                class: "shrink-0 min-h-[70px] pt-safe pb-3 flex items-center px-5 transition-all duration-300 border-b border-white/5",
                div {
                    class: format!("flex items-center gap-3 overflow-hidden {}", if is_drawer { "" } else { "group-hover:justify-start justify-center" }),

                    // Logo Icon with neon glow
                    div {
                        class: "w-9 h-9 rounded-xl bg-gradient-to-br from-primary/20 to-purple/10 border border-primary/20 flex-shrink-0 shadow-lg shadow-glow-cyan group-child cursor-pointer hover:border-primary/50 hover:shadow-glow-cyan-intense transition-all duration-300 flex items-center justify-center relative",
                        div { class: "absolute inset-0 bg-primary/20 opacity-0 group-hover/logo:opacity-100 transition-opacity rounded-xl blur-md" }
                        crate::components::ui::Icon {
                            name: "shield".to_string(), // Using shield as logo icon for now, or we can use custom SVG content if needed
                            class: "relative z-10 text-primary".to_string(),
                            size: "md".to_string(),
                            glow: "cyan".to_string(),
                        }
                    }

                    // Text (Hidden on collapsed desktop until hover)
                    div {
                        class: format!(
                            "flex flex-col justify-center transition-all duration-300 overflow-hidden whitespace-nowrap {}",
                            if is_drawer { "opacity-100 w-auto" } else { "w-0 opacity-0 group-hover:w-auto group-hover:opacity-100 pl-1" }
                        ),
                        // Using div for text layout to prevent layout shifts
                        div { class: "flex items-baseline gap-0.5 leading-none",
                             span { class: "text-base font-bold text-white tracking-tight", "EDGE" }
                             span { class: "text-base font-bold text-primary text-glow-cyan", "RAY" }
                        }
                        span { class: "text-[9px] text-purple-400 font-mono tracking-[0.2em] uppercase mt-0.5", "Pro Suite" }
                    }
                }
            }

            // --- Scrollable Navigation Content ---
            div {
                class: "flex-1 overflow-y-auto py-6 px-4 space-y-8 no-scrollbar",

                for (section_name, items) in nav_sections {
                    nav {
                        class: "flex flex-col gap-1.5",

                        // Section Header
                        if !items.is_empty() {
                            div {
                                class: format!(
                                    "px-3 mb-2 flex items-center gap-2 text-[10px] text-slate-500 uppercase tracking-widest font-black whitespace-nowrap overflow-hidden transition-all duration-300 {}",
                                    if is_drawer { "opacity-100" } else { "opacity-0 group-hover:opacity-60 h-0 group-hover:h-auto group-hover:mb-2" }
                                ),
                                "{section_name}"
                            }
                        }

                        for (page, icon, label) in items {
                            NavItem {
                                page: page,
                                icon: icon,
                                label: label,
                                current_page: current_page,
                                on_navigate: on_navigate,
                                is_drawer: is_drawer,
                            }
                        }
                    }
                }
            }



            // --- Simple Mode Toggle (Desktop only or Drawer) ---
            if let Some(handler) = on_toggle_mode {
                div {
                    class: format!(
                        "px-4 pb-2 transition-all duration-300 {}",
                         if is_drawer { "opacity-100" } else { "opacity-0 group-hover:opacity-100" }
                    ),
                    button {
                        class: "w-full flex items-center gap-3 px-4 py-3 rounded-xl bg-white/5 hover:bg-white/10 border border-white/5 text-slate-400 hover:text-white transition-all text-xs font-medium uppercase tracking-wider",
                        onclick: move |_event: Event<MouseData>| handler.call(()),
                        crate::components::ui::Icon { name: "arrow_back", size: "sm" }
                        "Simple Mode"
                    }
                }
            }

            // --- Status Footer ---
            div {
                class: format!(
                    "shrink-0 px-6 pt-6 pb-safe border-t border-white/5 flex items-center transition-all duration-300 bg-void/60 backdrop-blur-md {}",
                    if is_drawer { "" } else { "justify-center group-hover:justify-start" }
                ),
                // Status Indicator
                div {
                    class: "relative w-12 h-12 rounded-2xl glass border border-white/10 transition-all duration-500 shrink-0 flex items-center justify-center",
                    div {
                        class: format!("w-3 h-3 rounded-full transition-colors duration-500 brightness-125 {}", safety_color)
                    }
                    if *mesh_safety.read() == MeshSafety::Secure {
                         div { class: "absolute inset-0 rounded-2xl bg-emerald/10 animate-pulse-slow" }
                    }
                }

                // Status Text
                div {
                    class: format!(
                        "ml-4 flex flex-col transition-all duration-500 whitespace-nowrap overflow-hidden text-left {}",
                        if is_drawer { "w-auto opacity-100" } else { "w-0 opacity-0 group-hover:w-auto group-hover:opacity-100" }
                    ),
                    span { class: "text-[11px] font-black text-white/90 uppercase tracking-widest", "{status_text}" }
                    span { class: "text-[10px] text-primary/60 font-mono font-bold tracking-tight", "ENGINE v2.4.0" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NavItemProps {
    page: Page,
    icon: &'static str,
    label: &'static str,
    current_page: Signal<Page>,
    on_navigate: EventHandler<Page>,
    is_drawer: bool,
}

#[component]
fn NavItem(props: NavItemProps) -> Element {
    let is_active = *props.current_page.read() == props.page;
    let click_handler = move |_| props.on_navigate.call(props.page);

    // "Technical" items get the Purple Neon treatment, others get Cyan Neon.
    let is_technical = ["network", "chart", "activity", "merge", "sliders"].contains(&props.icon);

    // Using new custom_theme.css utility classes: .shadow-glow-purple, .shadow-glow-cyan, .text-glow-purple, .text-glow-cyan
    let (color_style, active_text, bg_bar) = if is_technical {
        (
            "text-purple-400 shadow-[0_0_15px_rgba(192,132,252,0.4)]", // intense glow
            "text-purple-400",
            "bg-purple-500",
        )
    } else {
        (
            "text-primary shadow-[0_0_15px_rgba(34,211,238,0.4)]",
            "text-primary",
            "bg-primary",
        )
    };

    let btn_base = "group/item relative flex items-center h-[52px] px-4 rounded-2xl transition-all duration-300 active:scale-95 cursor-pointer outline-none select-none";
    let btn_justify = if props.is_drawer {
        ""
    } else {
        "justify-center group-hover:justify-start"
    };

    let btn_active = if is_active {
        "bg-white/5 border border-white/10 shadow-glow-cyan/10"
    } else {
        "text-slate-500 hover:text-white border border-transparent"
    };

    rsx! {
        button {
            class: format!("{} {} {}", btn_base, btn_justify, btn_active),
            onclick: click_handler,
            title: if !props.is_drawer { props.label } else { "" },

            // Active Indicator Line
            if is_active {
                div { class: format!("absolute left-0 top-1/2 -translate-y-1/2 w-1 h-6 {} rounded-r-sm {} animate-pulse-fast", bg_bar, color_style) }
            }

            // Icon
            div {
                class: format!("relative z-10 shrink-0 transition-transform duration-300 {}", if is_active { active_text } else { "group-hover/item:scale-110" }),
                crate::components::ui::Icon {
                    name: props.icon.to_string(),
                    size: "md".to_string(),
                    class: if is_active { "stroke-[2.5px]".to_string() } else { "".to_string() }
                }
            }

            // Label
            span {
                class: format!(
                    "ml-3 text-sm font-medium whitespace-nowrap transition-all duration-300 overflow-hidden leading-none {}",
                    if props.is_drawer { "w-auto opacity-100" } else { "w-0 opacity-0 group-hover:w-auto group-hover:opacity-100" }
                ),
                "{props.label}"
            }
        }
    }
}

// Removed render_icon function as it is no longer used
