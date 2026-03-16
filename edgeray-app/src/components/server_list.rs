//! Server List Component
//!
//! Renders the main server list screen.
use crate::components::gestures::{PullToRefresh, SwipeableCard};
use crate::components::ui::Icon;
use crate::models::ServerConfig;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ServerListProps {
    pub servers: Vec<ServerConfig>,
    pub on_select: EventHandler<ServerConfig>,
    pub on_edit: EventHandler<ServerConfig>,
    #[props(default = None)]
    pub on_share: Option<EventHandler<ServerConfig>>,
    #[props(default = None)]
    pub on_delete: Option<EventHandler<ServerConfig>>,
    pub on_ping_all: EventHandler<()>,
    pub on_add: EventHandler<()>,
    #[props(default = None)]
    pub connected_server_id: Option<String>,
    #[props(default = None)]
    pub pings: Option<std::collections::HashMap<String, u32>>,
}

#[component]
pub fn ServerList(props: ServerListProps) -> Element {
    let connected_server = props
        .servers
        .iter()
        .find(|s| Some(s.id.clone().unwrap_or_default()) == props.connected_server_id);
    let _available_servers = props
        .servers
        .iter()
        .filter(|s| Some(s.id.clone().unwrap_or_default()) != props.connected_server_id)
        .collect::<Vec<_>>();
    let connected_count = if props.connected_server_id.is_some() {
        1
    } else {
        0
    };

    // Virtual scrolling state for large lists (1000+ nodes)
    const PAGE_SIZE: usize = 50;
    let mut current_page = use_signal(|| 0usize);

    // Search and filter state (reference design pattern)
    let mut search_query = use_signal(|| String::new());
    let mut is_search_focused = use_signal(|| false);
    let mut active_filter = use_signal(|| "All".to_string());

    // Filter servers by search query and protocol filter
    let filtered_servers: Vec<&ServerConfig> = props
        .servers
        .iter()
        .filter(|s| Some(s.id.clone().unwrap_or_default()) != props.connected_server_id)
        .filter(|s| {
            let query = search_query.read().to_lowercase();
            if query.is_empty() {
                true
            } else {
                s.remarks.to_lowercase().contains(&query)
                    || s.address.to_lowercase().contains(&query)
            }
        })
        .filter(|s| {
            let f = active_filter.read();
            match f.as_str() {
                "All" => true,
                proto => format!("{:?}", s.protocol).to_uppercase() == proto,
            }
        })
        .collect();

    let total_pages = (filtered_servers.len() + PAGE_SIZE - 1) / PAGE_SIZE;

    // Calculate visible range
    let start_idx = *current_page.read() * PAGE_SIZE;
    let end_idx = (start_idx + PAGE_SIZE).min(filtered_servers.len());
    let visible_servers = &filtered_servers[start_idx..end_idx];

    rsx! {
        div {
            class: "w-full h-full flex flex-col font-display text-white antialiased",


            div { class: "w-full h-6 shrink-0" } // Spacer

            // Standard Page Header
            crate::components::ui::PageHeader {
                title: "Servers".to_string(),
                left_action: Some(rsx! {
                    div {
                        class: "text-xs font-medium text-slate-500 dark:text-slate-400 mix-blend-plus-lighter",
                        "{props.servers.len()} Servers • {connected_count} Connected"
                    }
                }),
                right_action: Some(rsx! {
                    div {
                        class: "flex items-center gap-3",
                        button {
                            "aria-label": "Ping All",
                            class: "flex items-center justify-center size-10 rounded-full bg-white/40 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 backdrop-blur-md border border-white/40 dark:border-white/10 transition-all shadow-sm group",
                            onclick: move |_| props.on_ping_all.call(()),
                            Icon { name: "bolt".to_string(), class: "text-slate-600 dark:text-slate-300 group-hover:text-primary transition-colors".to_string()}
                        }
                        crate::components::ui::PrimaryButton {
                            label: "Add".to_string(),
                            icon: Some("add".to_string()),
                            onclick: move |_| props.on_add.call(()),
                        }
                    }
                })
            }

            // Search Bar (reference design pattern)
            div { class: "sticky top-0 z-20 glass border-b border-white/5 px-4 lg:px-8 py-3 backdrop-blur-3xl",
                div { class: "flex items-center space-x-4 max-w-4xl mx-auto w-full",
                    div { class: "flex-1 relative",
                        div { class: "absolute left-3 top-1/2 -translate-y-1/2 text-gray-400",
                            Icon { name: "search".to_string(), class: "text-sm".to_string() }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Search servers...",
                            class: "w-full glass-input rounded-xl py-2.5 pl-10 pr-4 text-sm focus:ring-1 focus:ring-cyber/50 transition-all placeholder:text-gray-500 dark:placeholder:text-gray-600",
                            value: "{search_query}",
                            oninput: move |e| search_query.set(e.value().clone()),
                            onfocus: move |_| is_search_focused.set(true),
                            onblur: move |_| is_search_focused.set(false)
                        }
                    }
                    button {
                        class: "p-2.5 bg-neon/10 rounded-xl text-neon border border-neon/20 hover:bg-neon/20 transition-colors",
                        onclick: move |_| props.on_ping_all.call(()),
                        Icon { name: "bolt".to_string(), class: "text-lg".to_string() }
                    }
                }

                // Protocol Filters (shown when search focused or has content)
                if *is_search_focused.read() || !search_query.read().is_empty() {
                    div { class: "overflow-hidden mt-2 max-w-4xl mx-auto",
                        div { class: "flex space-x-2 py-2 overflow-x-auto no-scrollbar",
                            for filter_name in ["All", "VLESS", "VMESS", "TROJAN", "SHADOWSOCKS"] {
                                {
                                    let is_active = *active_filter.read() == filter_name;
                                    let btn_class = if is_active {
                                        "px-4 py-1.5 rounded-full text-xs font-bold whitespace-nowrap transition-all border bg-primary/20 border-primary text-primary shadow-glow-cyan"
                                    } else {
                                        "px-4 py-1.5 rounded-full text-xs whitespace-nowrap transition-colors border glass-button text-gray-400 border-white/10"
                                    };
                                    let filter_val = filter_name.to_string();
                                    rsx! {
                                        button {
                                            class: "{btn_class}",
                                            onclick: move |_| {
                                                active_filter.set(filter_val.clone());
                                                current_page.set(0); // Reset to first page on filter change
                                            },
                                            "{filter_name}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            main {
                class: "flex-1 overflow-y-auto px-4 lg:px-8 pb-24 lg:pb-8 pt-2 no-scrollbar z-10",
                if let Some(server) = connected_server {
                    CurrentSessionCard {
                        server: server.clone(),
                        on_edit: props.on_edit.clone(),
                        on_share: props.on_share.clone(),
                        on_delete: props.on_delete.clone()
                    }
                }

                crate::components::ui::SectionHeader {
                    title: "Available Servers".to_string(),
                    action: Some(rsx!{
                        button { class: "text-[10px] font-semibold text-primary hover:text-white hover:bg-primary transition-all bg-primary/10 border border-primary/20 px-2.5 py-1 rounded-md backdrop-blur-sm", "Sort by Ping" }
                    })
                }

                div {
                    class: "flex flex-col gap-3",
                    PullToRefresh {
                        on_refresh: props.on_ping_all.clone(),
                        children: rsx! {
                            div {
                                class: "flex flex-col lg:grid lg:grid-cols-2 xl:grid-cols-3 gap-3 animate-entrance-up stagger-entrance",
                                for server in visible_servers {
                                    SwipeableCard {
                                        key: "{server.id.clone().unwrap_or_default()}",
                                        on_delete: {
                                            let s = (*server).clone();
                                            let h = props.on_delete.clone();
                                            move |_| {
                                                if let Some(handler) = &h {
                                                    handler.call(s.clone());
                                                }
                                            }
                                        },
                                        children: rsx! {
                                            AvailableServerRow {
                                                server: (*server).clone(),
                                                ping: props.pings.as_ref().and_then(|p| p.get(server.id.as_ref().unwrap_or(&"".to_string())).cloned()),
                                                on_select: props.on_select.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Pagination controls for large lists
                if total_pages > 1 {
                    div {
                        class: "flex items-center justify-center gap-2 mt-6 mb-4",
                        button {
                            class: "px-3 py-1.5 rounded-lg text-xs font-medium bg-white/40 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed border border-white/40 dark:border-white/10 transition-all",
                            disabled: *current_page.read() == 0,
                            onclick: move |_| {
                                let current = *current_page.read();
                                if current > 0 {
                                    current_page.set(current - 1);
                                }
                            },
                            "Previous"
                        }
                        span {
                            class: "text-xs font-medium text-slate-600 dark:text-slate-400 px-3",
                            "Page {*current_page.read() + 1} of {total_pages}"
                        }
                        button {
                            class: "px-3 py-1.5 rounded-lg text-xs font-medium bg-white/40 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 disabled:opacity-50 disabled:cursor-not-allowed border border-white/40 dark:border-white/10 transition-all",
                            disabled: *current_page.read() >= total_pages - 1,
                            onclick: move |_| {
                                let current = *current_page.read();
                                if current < total_pages - 1 {
                                    current_page.set(current + 1);
                                }
                            },
                            "Next"
                        }
                    }
                }
            }

        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CurrentSessionCardProps {
    server: ServerConfig,
    on_edit: EventHandler<ServerConfig>,
    on_share: Option<EventHandler<ServerConfig>>,
    on_delete: Option<EventHandler<ServerConfig>>,
}

#[component]
fn CurrentSessionCard(props: CurrentSessionCardProps) -> Element {
    let server_for_share = props.server.clone();
    let server_for_edit = props.server.clone();
    let server_for_delete = props.server.clone();
    rsx! {
        div {
            class: "mb-5",
            crate::components::ui::SectionHeader { title: "Current Session".to_string() }
            crate::components::ui::GlassCard {
                class: "relative group overflow-hidden border-primary/20 shadow-glow-cyan/20",
                div {
                    class: "flex items-center gap-4 p-5 relative z-10",
                    div {
                        class: "shrink-0 relative flex items-center justify-center size-4",
                        div { class: "size-3 rounded-full bg-primary z-10 shadow-glow ring-2 ring-white/30 dark:ring-black/20" }
                        div { class: "absolute inset-0 rounded-full bg-primary animate-ping opacity-30 scale-150" }
                    }
                    div {
                        class: "shrink-0 size-12 rounded-2xl bg-gradient-to-br from-primary/10 to-purple/5 flex items-center justify-center text-primary shadow-inner border border-primary/20",
                        Icon { name: "public".to_string(), class: "drop-shadow-glow-cyan".to_string() }
                    }
                    div {
                        class: "flex-1 min-w-0",
                        div {
                            class: "flex items-center justify-between mb-1.5",
                            h4 { class: "font-bold text-base truncate pr-2 text-white tracking-tight drop-shadow-md", "{props.server.remarks}" }
                            crate::components::ui::Badge { label: format!("{:?}", props.server.protocol).to_uppercase() }
                        }
                    }
                }
                div {
                    class: "flex border-t border-white/5 divide-x divide-white/5 bg-black/40 backdrop-blur-md relative z-10",
                    button {
                        class: "flex-1 py-3.5 flex items-center justify-center gap-2 text-xs font-semibold hover:bg-white/40 dark:hover:bg-white/5 transition-colors text-slate-700 dark:text-slate-200 group/btn",
                        onclick: move |_| {
                            if let Some(h) = &props.on_share {
                                h.call(server_for_share.clone())
                            }
                        },
                        Icon { name: "share".to_string(), class: "text-[18px] group-hover/btn:scale-110 transition-transform".to_string() }
                        "Share"
                    }
                    button {
                        class: "flex-1 py-3.5 flex items-center justify-center gap-2 text-xs font-semibold hover:bg-white/40 dark:hover:bg-white/5 transition-colors text-slate-700 dark:text-slate-200 group/btn",
                        onclick: move |_| {
                            props.on_edit.call(server_for_edit.clone())
                        },
                        Icon { name: "edit".to_string(), class: "text-[18px] group-hover/btn:scale-110 transition-transform".to_string() }
                        "Edit"
                    }
                    button {
                        class: "flex-1 py-3.5 flex items-center justify-center gap-2 text-xs font-semibold hover:bg-red-500/10 dark:hover:bg-red-500/20 transition-colors text-red-500 group/btn",
                        onclick: move |_| {
                            if let Some(h) = &props.on_delete {
                                h.call(server_for_delete.clone())
                            }
                        },
                        Icon { name: "power_settings_new".to_string(), class: "text-[18px] group-hover/btn:scale-110 transition-transform".to_string() }
                        "Stop"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AvailableServerRowProps {
    server: ServerConfig,
    ping: Option<u32>,
    on_select: EventHandler<ServerConfig>,
}

#[component]
fn AvailableServerRow(props: AvailableServerRowProps) -> Element {
    let server = props.server.clone();
    let select_handler = props.on_select.clone();

    // Protocol-based left border color (matching reference Nodes.tsx)
    let protocol_border = match format!("{:?}", server.protocol).to_uppercase().as_str() {
        "VLESS" => "border-l-primary",
        "VMESS" => "border-l-purple-500",
        "TROJAN" => "border-l-pink-500",
        "SHADOWSOCKS" => "border-l-amber-500",
        _ => "border-l-primary",
    };

    // Ping styling with neon glow for fast connections
    let (ping_color, ping_text, ping_glow) = match props.ping {
        Some(p) if p < 100 => (
            "text-neon",
            p.to_string(),
            "drop-shadow-[0_0_5px_rgba(0,240,255,0.5)]",
        ),
        Some(p) if p < 200 => ("text-emerald-400", p.to_string(), ""),
        Some(p) if p < 500 => ("text-yellow-400", p.to_string(), ""),
        Some(_) => ("text-red-400", "Timeout".to_string(), ""),
        None => ("text-gray-500", "--".to_string(), ""),
    };

    // Online status
    let is_online = props.ping.is_some() && props.ping.unwrap_or(0) < 1000;

    rsx! {
        div {
            class: format!("{} rounded-2xl p-5 flex items-center justify-between cursor-pointer transition-all hover:scale-[1.02] hover:-translate-y-0.5 group border-l-4 hover:shadow-glow-cyan/20 {}", crate::components::ui::glass::CARD, protocol_border),
            onclick: move |_| select_handler.call(server.clone()),

            div { class: "flex items-center space-x-4",
                div {
                    h3 { class: "font-bold text-lg text-white group-hover:text-primary transition-colors tracking-tight", "{server.remarks}" }
                    div { class: "flex items-center space-x-2 text-xs text-gray-400 mt-1",
                        span { class: "px-1.5 py-0.5 rounded bg-white/20 dark:bg-white/5 font-mono border border-white/20 dark:border-white/5", {format!("{:?}", server.protocol).to_uppercase()} }
                        span { class: "flex items-center",
                            Icon { name: "public".to_string(), class: "text-[10px] mr-1 text-cyan-400".to_string() }
                            if !server.address.is_empty() {
                                "{server.address}"
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-col items-end",
                span { class: format!("text-xl font-mono font-bold {} {}", ping_color, ping_glow),
                    "{ping_text}"
                    if ping_text != "--" && ping_text != "Timeout" {
                        span { class: "text-xs text-slate-500 dark:text-gray-500 ml-1 font-normal", "ms" }
                    }
                }
                if is_online {
                    span { class: "text-[10px] text-emerald-500 flex items-center mt-1 font-bold tracking-wider",
                        div { class: "size-1.5 rounded-full bg-emerald-500 animate-pulse mr-1.5" }
                        "ONLINE"
                    }
                } else {
                    span { class: "text-[10px] text-red-500 font-bold tracking-wider opacity-70", "OFFLINE" }
                }
            }
        }
    }
}

#[component]
fn BottomNav() -> Element {
    rsx! {
        nav {
            class: "h-[84px] border-t border-white/40 dark:border-white/10 flex items-start justify-around pt-3 shrink-0 pb-5 z-20 backdrop-blur-xl bg-white/70 dark:bg-slate-900/60 shadow-[0_-10px_40px_rgba(0,0,0,0.05)] dark:shadow-none",
            BottomNavItem { label: "Home", icon: "dashboard".to_string() }
            BottomNavItem { label: "Servers", icon: "dns".to_string(), active: true }
            BottomNavItem { label: "Logs", icon: "history".to_string() }
            BottomNavItem { label: "Settings", icon: "settings".to_string() }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BottomNavItemProps {
    label: String,
    icon: String,
    #[props(default = false)]
    active: bool,
}

#[component]
fn BottomNavItem(props: BottomNavItemProps) -> Element {
    let (icon_class, text_class) = if props.active {
        (
            "text-primary drop-shadow-[0_0_8px_rgba(139,92,246,0.5)]",
            "text-primary",
        )
    } else {
        (
            "text-slate-500 dark:text-slate-400 group-hover:text-primary",
            "text-slate-500 dark:text-slate-400 group-hover:text-primary",
        )
    };

    rsx! {
        button {
            class: "flex flex-col items-center gap-1.5 w-16 group",
            Icon { name: props.icon, class: format!("transition-colors {}", icon_class) }
            span { class: format!("text-[10px] font-medium transition-colors {}", text_class), "{props.label}" }
        }
    }
}
