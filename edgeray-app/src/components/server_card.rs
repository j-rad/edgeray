//! Server Card Component
//!
//! A glassmorphism server card matching the v2ray-glass design.
//! Features expandable actions, status indicators, and ping badges.

use super::ui::{GlassCard, Icon};
use crate::models::ServerConfig;
use dioxus::prelude::*;

/// Server status enum
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ServerStatus {
    Connected,
    Idle,
    Error,
}

/// Ping status for color coding
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(dead_code)]
pub enum PingStatus {
    Good,   // < 100ms
    Medium, // 100-300ms
    Poor,   // > 300ms
    Timeout,
}

#[allow(dead_code)]
impl PingStatus {
    pub fn from_ping(ping: Option<u32>) -> Self {
        match ping {
            Some(p) if p < 100 => PingStatus::Good,
            Some(p) if p < 300 => PingStatus::Medium,
            Some(_) => PingStatus::Poor,
            None => PingStatus::Timeout,
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            PingStatus::Good => "text-[#10b981] bg-[#10b981]/10 border-[#10b981]/20",
            PingStatus::Medium => "text-yellow-400 bg-yellow-400/10 border-yellow-400/20",
            PingStatus::Poor => "text-[#ef4444] bg-[#ef4444]/10 border-[#ef4444]/20",
            PingStatus::Timeout => "text-white/40 bg-white/5 border-white/5",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ServerCardProps {
    pub config: ServerConfig,
    pub active: bool,
    pub ping: Option<u32>,
    #[props(default = ServerStatus::Idle)]
    pub status: ServerStatus,
    #[props(default = false)]
    pub expanded: bool,
    #[props(default = None)]
    pub on_click: Option<EventHandler<()>>,
    #[props(default = None)]
    pub on_edit: Option<EventHandler<()>>,
    #[props(default = None)]
    pub on_share: Option<EventHandler<()>>,
    #[props(default = None)]
    pub on_delete: Option<EventHandler<()>>,
    #[props(default = None)]
    pub on_benchmark: Option<EventHandler<()>>,
    #[props(default = None)]
    pub speed_test_result: Option<f64>, // MB/s
    #[props(default = false)]
    pub is_benchmarking: bool,
}

#[component]
pub fn ServerCard(props: ServerCardProps) -> Element {
    let is_expanded = props.expanded;
    let mut is_sharing = use_signal(|| false);
    let _is_error = props.status == ServerStatus::Error;
    let ping_status = PingStatus::from_ping(props.ping);

    // Cyber accent border color based on protocol (matches reference design)
    let accent_border = match props.config.protocol {
        crate::models::Protocol::Vless => "border-l-4 border-l-purple-500",
        crate::models::Protocol::Vmess => "border-l-4 border-l-cyan-500",
        crate::models::Protocol::Trojan => "border-l-4 border-l-amber-500",
        _ => "border-l-4 border-l-primary",
    };

    // Card container class
    let card_class = if is_expanded {
        format!("{} !bg-white/10 !border-white/20", accent_border)
    } else {
        accent_border.to_string()
    };

    // Status dot color
    let dot_color = match props.status {
        ServerStatus::Connected => "bg-[#3b82f6] shadow-[0_0_10px_#2563eb]",
        ServerStatus::Error => "bg-[#ef4444]",
        ServerStatus::Idle => "bg-[#10b981]",
    };

    // Scale effect when expanded
    let container_class = if is_expanded { "mb-2 scale-[1.02]" } else { "" };

    let onclick = {
        let handler = props.on_click.clone();
        move |_| {
            if let Some(h) = &handler {
                h.call(());
            }
        }
    };

    let on_edit_click = {
        let handler = props.on_edit.clone();
        move |e: Event<MouseData>| {
            e.stop_propagation();
            if let Some(h) = &handler {
                h.call(());
            }
        }
    };

    let on_share_click = {
        let handler = props.on_share.clone(); // Keep original handler if needed
        move |e: Event<MouseData>| {
            e.stop_propagation();
            is_sharing.set(true);
            if let Some(h) = &handler {
                h.call(());
            }
        }
    };

    let on_delete_click = {
        let handler = props.on_delete.clone();
        move |e: Event<MouseData>| {
            e.stop_propagation();
            if let Some(h) = &handler {
                h.call(());
            }
        }
    };

    let on_benchmark_click = {
        let handler = props.on_benchmark.clone();
        move |e: Event<MouseData>| {
            e.stop_propagation();
            if let Some(h) = &handler {
                h.call(());
            }
        }
    };

    rsx! {
        div {
            class: "transition-all duration-300 {container_class}",
            GlassCard {
                class: card_class.to_string(),
                // Main card content
                div {
                    class: "p-4 flex items-center gap-4 cursor-pointer",
                    onclick: onclick,

                    // Status Dot with Ripple
                    div {
                        class: "shrink-0 relative flex items-center justify-center w-4 h-4",
                        div {
                            class: "w-2.5 h-2.5 rounded-full {dot_color} z-10 transition-colors duration-300"
                        }
                        if props.status == ServerStatus::Connected {
                            div {
                                class: "absolute inset-0 rounded-full bg-[#3b82f6] animate-ping opacity-40"
                            }
                        }
                    }

                    // Content
                    div {
                        class: "flex-1 min-w-0 flex flex-col gap-1",
                        div {
                            class: "flex items-center justify-between",
                            h4 {
                                class: format_args!(
                                    "font-semibold text-sm truncate pr-2 transition-colors {}",
                                    if is_expanded { "text-white" } else { "text-white/90" }
                                ),
                                "{props.config.remarks}"
                            }
                            span {
                                class: "text-[10px] font-bold text-white/40 bg-white/5 px-1.5 py-0.5 rounded border border-white/5 tracking-wide font-mono",
                                "{props.config.protocol:?}"
                            }
                        }

                        // Sub info
                        div {
                            class: "flex items-center gap-3 text-xs text-white/50",
                            if is_expanded {
                                div {
                                    class: "flex items-center gap-4 animate-in fade-in duration-300",
                                    if props.is_benchmarking {
                                         div { class: "flex items-center gap-2",
                                             div { class: "w-3 h-3 border-2 border-[#3b82f6] border-t-transparent rounded-full animate-spin" }
                                             span { class: "text-xs text-white/60", "Testing..." }
                                         }
                                    } else if let Some(speed) = props.speed_test_result {
                                         span {
                                            class: "flex items-center gap-1 text-white/80",
                                            Icon { name: "speed".to_string(), class: "text-[12px] text-[#10b981]".to_string() }
                                            "{speed:.1} MB/s"
                                        }
                                    } else {
                                        button {
                                            class: "flex items-center gap-1 text-[10px] bg-white/5 hover:bg-white/10 px-2 py-1 rounded border border-white/10 transition-colors",
                                            onclick: on_benchmark_click,
                                            Icon { name: "play_arrow".to_string(), class: "text-[12px]".to_string() }
                                            "Test Speed"
                                        }
                                    }
                                }
                            } else {
                                span {
                                    class: "truncate opacity-70 font-mono text-[11px]",
                                    "{props.config.address}:{props.config.port}"
                                }
                            }
                        }
                    }

                    // Ping / Expand Icon
                    div {
                        class: "shrink-0 flex flex-col items-end gap-0.5",
                        if is_expanded {
                            div {
                                class: "flex flex-col items-end",
                                Icon { name: "signal_cellular_alt".to_string(), class: "text-[#3b82f6] text-[20px]".to_string() }
                                span {
                                    class: "text-[10px] font-bold text-[#3b82f6] font-mono",
                                    if let Some(ping) = props.ping {
                                        "{ping}ms"
                                    } else {
                                        "N/A"
                                    }
                                }
                            }
                        } else {
                            div {
                                class: "flex items-center gap-1 px-2 py-1 rounded-lg border {ping_status.color_class()}",
                                span {
                                    class: "text-[10px] font-bold font-mono",
                                    if let Some(ping) = props.ping {
                                        "{ping}ms"
                                    } else {
                                        "Timeout"
                                    }
                                }
                            }
                        }
                    }
                }

                // Expanded Actions Strip
                if is_expanded {
                    div {
                        class: "grid grid-cols-3 divide-x divide-white/5 border-t border-white/5 bg-black/20 backdrop-blur-sm",
                        button {
                            class: "py-3.5 flex items-center justify-center gap-2 text-xs font-medium text-white/70 hover:bg-white/5 hover:text-white transition-colors bg-transparent border-none cursor-pointer",
                            onclick: on_share_click,
                            Icon { name: "qr_code".to_string(), class: "text-[18px]".to_string() }
                            "Share"
                        }
                        button {
                            class: "py-3.5 flex items-center justify-center gap-2 text-xs font-medium text-white/70 hover:bg-white/5 hover:text-white transition-colors bg-transparent border-none cursor-pointer",
                            onclick: on_edit_click,
                            Icon { name: "edit".to_string(), class: "text-[18px]".to_string() }
                            "Edit"
                        }
                        button {
                            class: "py-3.5 flex items-center justify-center gap-2 text-xs font-medium text-[#ef4444]/80 hover:bg-[#ef4444]/10 hover:text-[#ef4444] transition-colors bg-transparent border-none cursor-pointer",
                            onclick: on_delete_click,
                            Icon { name: "delete".to_string(), class: "text-[18px]".to_string() }
                            "Delete"
                        }
                    }
                }
            }

            // Share Overlay
            if *is_sharing.read() {
                crate::components::qr_share::QrShare {
                    config: props.config.clone(),
                    on_close: move |_| is_sharing.set(false),
                }
            }
        }
    }
}

/// Compact server card for the dashboard (non-expandable)
#[derive(Props, Clone, PartialEq)]
pub struct CompactServerCardProps {
    pub config: ServerConfig,
    pub active: bool,
    pub ping: Option<u32>,
    #[props(default = ServerStatus::Idle)]
    pub status: ServerStatus,
    #[props(default = None)]
    pub on_click: Option<EventHandler<()>>,
}

#[component]
pub fn CompactServerCard(props: CompactServerCardProps) -> Element {
    let ping_status = PingStatus::from_ping(props.ping);

    let card_class = if props.active {
        "ring-2 ring-[#3b82f6] ring-offset-2 ring-offset-transparent"
    } else {
        ""
    };

    let dot_color = match props.status {
        ServerStatus::Connected => "bg-[#3b82f6] shadow-[0_0_10px_#2563eb]",
        ServerStatus::Error => "bg-[#ef4444]",
        ServerStatus::Idle => "bg-[#10b981]",
    };

    let onclick = {
        let handler = props.on_click.clone();
        move |_| {
            if let Some(h) = &handler {
                h.call(());
            }
        }
    };

    rsx! {
        div {
            class: "cursor-pointer transition-transform hover:scale-[1.02] active:scale-[0.98]",
            onclick: onclick,
            GlassCard {
                class: card_class.to_string(),
                div {
                    class: "p-4 flex items-center gap-4",

                    // Status Dot
                    div {
                        class: "shrink-0 relative flex items-center justify-center w-4 h-4",
                        div {
                            class: "w-2.5 h-2.5 rounded-full {dot_color} z-10"
                        }
                        if props.status == ServerStatus::Connected {
                            div {
                                class: "absolute inset-0 rounded-full bg-[#3b82f6] animate-ping opacity-40"
                            }
                        }
                    }

                    // Content
                    div {
                        class: "flex-1 min-w-0 flex flex-col gap-1",
                        h4 {
                            class: "font-semibold text-sm truncate text-white/90",
                            "{props.config.remarks}"
                        }
                        div {
                            class: "flex items-center gap-2 text-xs text-white/50",
                            span {
                                class: "text-[10px] font-bold text-white/40 bg-white/5 px-1.5 py-0.5 rounded border border-white/5 tracking-wide font-mono",
                                "{props.config.protocol:?}"
                            }
                            span {
                                class: "truncate opacity-70 font-mono text-[11px]",
                                "{props.config.address}:{props.config.port}"
                            }
                        }
                    }

                    // Ping Badge
                    div {
                        class: "shrink-0 flex items-center gap-1 px-2 py-1 rounded-lg border {ping_status.color_class()}",
                        span {
                            class: "text-[10px] font-bold font-mono",
                            if let Some(ping) = props.ping {
                                "{ping}ms"
                            } else {
                                "Timeout"
                            }
                        }
                    }
                }
            }
        }
    }
}
