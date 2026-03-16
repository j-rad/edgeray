//! Dashboard Component
//!
//! The main dashboard screen matching the v2ray-glass design.
//! Features: Power orb button, status badge, metrics panel, and traffic graph.

use super::ui::Icon;
use crate::models::ServerConfig;
use crate::ui::adaptive_shell::UiMode;
use dioxus::prelude::*;

/// Connection state for the dashboard
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

#[derive(Props, Clone, PartialEq)]
pub struct DashboardProps {
    pub active_server: Option<ServerConfig>,
    pub connection_state: ConnectionState,
    pub on_toggle: EventHandler<()>,
    #[props(default = None)]
    pub ping: Option<u32>,
    #[props(default = None)]
    pub upload_speed: Option<f64>,
    #[props(default = None)]
    pub download_speed: Option<f64>,
    #[props(default = Vec::new())]
    pub bandwidth_history: Vec<f64>, // Normalized 0-1 values for last 20 points
    #[props(default = UiMode::Simple)]
    pub ui_mode: UiMode,
}

fn generate_graph_path(data: &[f64], width: f64, height: f64) -> String {
    if data.is_empty() {
        return "M0,0".to_string();
    }

    let step_x = width / (data.len().max(2) - 1) as f64;
    let mut path_str = format!("M0 {:.1}", height - (data[0] * height));

    for (i, &val) in data.iter().skip(1).enumerate() {
        let x = (i + 1) as f64 * step_x;
        let y = height - (val * height);
        path_str.push_str(&format!(
            " C{:.1} {:.1}, {:.1} {:.1}, {:.1} {:.1}",
            x - step_x / 2.0,
            height - (data[i] * height),
            x - step_x / 2.0,
            y,
            x,
            y
        ));
    }

    path_str
}

fn generate_area_path(data: &[f64], width: f64, height: f64) -> String {
    if data.is_empty() {
        return String::new();
    }
    let line_path = generate_graph_path(data, width, height);
    format!(
        "{} L{:.1} {:.1} L0 {:.1} Z",
        line_path, width, height, height
    )
}

#[component]
pub fn Dashboard(props: DashboardProps) -> Element {
    let _connected = props.connection_state == ConnectionState::Connected;
    let _connecting = props.connection_state == ConnectionState::Connecting;
    let connected = props.connection_state == ConnectionState::Connected;
    let connecting = props.connection_state == ConnectionState::Connecting;
    let (graph_width, graph_height) = (478.0, 150.0);
    let mut is_scanning = use_signal(|| false);

    // Log Streamer for Pro Mode
    let mut logs = use_signal(Vec::<String>::new);
    use_effect(move || {
        if props.ui_mode == UiMode::Pro {
             spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
                loop {
                    interval.tick().await;
                    let entries = crate::components::log_view::get_log_entries();
                    let last_logs = entries.iter().rev().take(5).map(|e| format!("{} [{:?}] {}", e.timestamp, e.level, e.message)).collect();
                    logs.set(last_logs);
                }
            });
        }
    });

    let server_name = props
        .active_server
        .as_ref()
        .map_or("No Server Selected", |s| &s.remarks);
    let server_location = props
        .active_server
        .as_ref()
        .map_or("Select a server to connect", |s| &s.address);

    rsx! {
        div {
            class: "flex flex-col min-h-full pb-24 md:pb-0 relative",



            // Header
            header {
                class: "flex flex-col gap-6 px-6 pt-6 safe-area-top shrink-0 z-10 md:flex-row md:items-center md:justify-between md:pt-8 md:px-10",

                // Left: System Identity
                div {
                    class: "flex items-center justify-between w-full md:w-auto",
                    div {
                        class: "flex flex-col",
                        span { class: "text-[10px] font-black text-primary/60 uppercase tracking-[0.4em] mb-1", "Operational Unit" }
                        h1 { class: "text-2xl font-black tracking-tighter text-white drop-shadow-glow-cyan", "DASHBOARD" }
                    }
                    div { class: "flex items-center gap-3",
                        div {
                            class: "size-12 flex items-center justify-center rounded-2xl glass-button border-glow-cyan/20 cursor-pointer",
                            onclick: move |_| is_scanning.set(true),
                            Icon { name: "qr_code_scanner", class: "text-xl text-primary" }
                        }
                        div { class: "md:hidden size-12 flex items-center justify-center rounded-2xl glass-button border-glow-cyan/20",
                            Icon { name: "notifications", class: "text-xl text-primary animate-pulse-slow" }
                        }
                    }
                }

                // QR Scanner Overlay
                if *is_scanning.read() {
                    crate::components::qr_scanner::QrScanner {
                        on_close: move |_| is_scanning.set(false),
                        on_scan: move |data| {
                            log::info!("Scanned QR: {}", data);
                            is_scanning.set(false);
                            let script = format!("showDXToast('QR Scanned', 'Content: {}', 'success', 3000)", data);
                            let _ = dioxus::document::eval(&script);
                        }
                    }
                }

                // Right: Active Node
                div {
                    class: "flex flex-col items-center gap-4 md:flex-row md:gap-8",
                    // Status badge
                    div {
                        class: format!(
                            "flex items-center gap-3 rounded-2xl glass-panel px-6 py-3 border {} shadow-glow-neon/5",
                            if connected { "border-primary/20 bg-primary/5" } else { "border-white/5" }
                        ),
                        div {
                            class: format!(
                                "size-2.5 rounded-full {} transition-all duration-500",
                                if connected { "bg-primary shadow-glow-cyan-intense animate-pulse" } else { "bg-slate-600" }
                            )
                        }
                        span { class: "text-white font-black text-[10px] uppercase tracking-widest", if connected { "System Encrypted" } else { "Link Offline" } }
                    }
                    // Location info
                    div {
                        class: "text-center md:text-right flex flex-col items-center md:items-end gap-1",
                        h2 { class: "text-2xl font-black leading-none tracking-tighter text-white uppercase", "{server_name}" }
                        div {
                            class: "flex items-center gap-2 text-primary/50 text-[11px] font-bold font-mono tracking-tight",
                            Icon { name: "location_on", class: "text-xs" }
                            span { "{server_location}" }
                        }
                    }
                }
            }

            // Main Grid Content - power orb left, telemetry panel right on desktop
            div {
                class: format!(
                    "flex-1 p-6 md:p-8 grid grid-cols-1 {} gap-8 items-center transition-all duration-500",
                    if props.ui_mode == UiMode::Pro { "xl:grid-cols-2" } else { "place-content-center" }
                ),

                // Power Core Area (left on desktop)
                div {
                    class: "flex flex-col items-center justify-center relative z-10 py-8 xl:py-0",

                    // Multi-stage status ring - changes based on connection state
                    // 3-Layer Rotating Ring System per reference design
                    div {
                        class: "absolute inset-0 flex items-center justify-center pointer-events-none",

                        // Background Glow
                        if connected || connecting {
                            div { class: format!("absolute inset-0 rounded-full bg-primary/20 blur-[60px] transition-opacity duration-700 {}", if connected { "opacity-100" } else { "opacity-40" }) }
                        }

                        // Outer decorative ring
                        div {
                            class: "absolute inset-0 rounded-full border border-white/5 flex items-center justify-center",
                            div { class: "absolute inset-2 border border-dashed border-white/10 rounded-full opacity-50" }
                        }

                        // Ring 1: Outermost - cyan accent, slow clockwise rotation
                        div {
                            class: format!(
                                "absolute inset-3 lg:inset-4 rounded-full border border-transparent {} {}",
                                if connected { "border-t-cyan-400/30 border-l-cyan-400/10 animate-spin-slow" } else { "border-t-white/5 border-l-white/5" },
                                if connecting { "animate-pulse" } else { "" }
                            )
                        }

                        // Ring 2: Middle - purple accent, slow counter-clockwise rotation
                        div {
                            class: format!(
                                "absolute inset-8 lg:inset-10 rounded-full border border-transparent {}",
                                if connected { "border-r-purple-500/40 border-b-purple-500/10 animate-spin-reverse-slow" } else { "border-r-white/5 border-b-white/5" }
                            )
                        }

                        // Ring 3: Inner - subtle pulse for active state
                        div {
                            class: format!(
                                "absolute inset-12 lg:inset-16 rounded-full border border-white/5 {}",
                                if connected { "animate-pulse-fast" } else { "" }
                            )
                        }

                        // Core SVG ring for status indicator
                        svg {
                            class: format!(
                                "size-52 lg:size-60 {}",
                                if connecting { "animate-spin-slow" } else { "" }
                            ),
                            view_box: "0 0 200 200",
                            fill: "none",

                            // Define gradient for status arc
                            defs {
                                linearGradient {
                                    id: "cyberGradient",
                                    x1: "0%", y1: "0%", x2: "100%", y2: "0%",
                                    stop { offset: "0%", stop_color: "#22d3ee" }    // cyan
                                    stop { offset: "100%", stop_color: "#bc00ff" } // purple
                                }
                            }

                            // Base circle
                            circle {
                                cx: "100", cy: "100", r: "90",
                                stroke: "#1e293b",
                                stroke_width: "6",
                                fill: "none"
                            }

                            // Status arc - gradient stroked
                            if connected || connecting {
                                circle {
                                    cx: "100", cy: "100", r: "90",
                                    stroke: "url(#cyberGradient)",
                                    stroke_width: "3",
                                    stroke_linecap: "round",
                                    stroke_dasharray: if connected { "565" } else { "200 365" },
                                    stroke_dashoffset: if connected { "0" } else { "0" },
                                    fill: "none",
                                    transform: "rotate(-90 100 100)"
                                }
                            }
                        }

                        // 3-Ring Pulse Effect System (reference design pattern)
                        // Ring 1: Innermost - primary color, fast pulse
                        if connected {
                            div { class: "size-48 lg:size-56 rounded-full border-2 border-primary/30 absolute animate-ping-slow" }
                        }
                        // Ring 2: Middle - subtle glow
                        if connected {
                            div { class: "size-56 lg:size-64 rounded-full border border-neon/20 absolute animate-ping-slower" }
                        }
                        // Ring 3: Outermost - faint decorative
                        if connected {
                            div { class: "size-64 lg:size-72 rounded-full border border-white/5 absolute animate-[ping_5s_cubic-bezier(0,0,0.2,1)_infinite_2s] opacity-20" }
                        }
                        // Connecting state - amber pulsing ring
                        if connecting {
                            div { class: "size-52 lg:size-60 rounded-full border-2 border-amber-500/40 absolute animate-pulse" }
                        }
                    }

                    // Power Orb Button
                    button {
                        class: "relative group cursor-pointer transition-all duration-500 active:scale-90 z-20 animate-float",
                        onclick: move |_| props.on_toggle.call(()),

                        // Enhanced Glow system
                        div {
                            class: format!(
                                "absolute inset-[-40px] rounded-full blur-[80px] transition-all duration-700 {}",
                                if connecting { "bg-purple-600 opacity-30 animate-pulse-slow" }
                                else if connected { "bg-primary opacity-30 animate-glow-breathe" }
                                else { "bg-slate-800 opacity-10 group-hover:opacity-20" }
                            )
                        }

                        // Main plasma orb
                        div {
                            class: format!(
                                "relative flex size-44 md:size-52 lg:size-60 items-center justify-center rounded-full transition-all duration-700 border-2 shadow-2xl overflow-hidden {}",
                                if connecting { "bg-void border-purple-500/40 shadow-glow-purple-intense scale-105" }
                                else if connected { "bg-void border-primary/40 shadow-glow-cyan-intense scale-110" }
                                else { "bg-void border-white/10 shadow-inner" }
                            ),

                            // Plasma effects inside
                                div {
                                    class: format!(
                                        "absolute inset-0 opacity-40 mix-blend-screen animate-spin-slow {}",
                                        if connecting { "bg-gradient-to-tr from-purple-600 to-transparent" }
                                        else { "bg-gradient-to-tr from-primary to-transparent" }
                                    )
                                }

                            // Power icon
                            Icon {
                                name: if connecting { "sync" } else { "bolt" },
                                class: format!(
                                    "text-[64px] lg:text-[80px] z-10 transition-all duration-500 {}",
                                    if connecting { "animate-spin text-purple-400 filter drop-shadow-[0_0_15px_rgba(191,0,255,0.8)]" }
                                    else if connected { "text-primary filter drop-shadow-[0_0_20px_rgba(0,240,255,0.9)]" }
                                    else { "text-slate-600 group-hover:text-slate-400" }
                                )
                            }
                        }
                    }

                    // Action text
                    p {
                        class: "mt-10 lg:mt-12 text-[11px] font-black text-primary/40 uppercase tracking-[0.5em] animate-pulse-slow",
                        if connecting { "ESTABLISHING NEURAL LINK..." } else if connected { "SYSTEM FULLY OPERATIONAL" } else { "INITIALIZE CORE ENGINE" }
                    }
                }

                // Telemetry Panel (Only in Pro Mode)
                if props.ui_mode == UiMode::Pro {
                    div {
                        class: "glass-panel rounded-[2rem] p-8 xl:min-h-[400px] transition-all duration-500 border-white/5 shadow-2xl relative overflow-hidden group/panel animate-fade-in-right",

                        // Background grid inside panel
                        div { class: "absolute inset-0 bg-grid-pattern opacity-10 pointer-events-none" }

                        // Panel header
                        div { class: "relative z-10 flex items-center justify-between mb-8",
                            div { class: "flex flex-col",
                                span { class: "text-[10px] font-black text-slate-500 uppercase tracking-widest", "Signal Processor" }
                                span { class: "text-lg font-black text-white tracking-tighter uppercase", "Live Telemetry" }
                            }
                            div { class: "size-10 rounded-xl glass flex items-center justify-center border border-white/10",
                                Icon { name: "analytics", class: "text-primary" }
                            }
                        }

                        // Metrics row
                        div {
                            class: "relative z-10 grid grid-cols-3 gap-6 mb-8",
                            // Sparklines embedded in tiles
                            MetricTile {
                                label: "Latency",
                                value: props.ping.map(|p| p.to_string()).unwrap_or("--".to_string()),
                                unit: "ms".to_string(),
                                icon: "speed".to_string(),
                                history: vec![120.0, 115.0, 110.0, 125.0, 118.0, 122.0] // Mock sparkline
                            }

                            // Jitter Tile
                            MetricTile {
                                label: "Jitter",
                                value: "12".to_string(),
                                unit: "ms".to_string(),
                                icon: "graphic_eq".to_string(),
                                history: vec![5.0, 12.0, 8.0, 4.0, 15.0, 10.0]
                            }

                            // Loss Tile
                            MetricTile {
                                label: "Packet Loss",
                                value: "0.1".to_string(),
                                unit: "%".to_string(),
                                icon: "security_update_good".to_string(),
                                history: vec![0.0, 0.0, 0.2, 0.0, 0.1, 0.0],
                                is_alert: true
                            }
                        }

                        // Speed row
                        div {
                            class: "relative z-10 flex items-center justify-between px-4 mb-8",
                            SpeedIndicator { direction: "Inbound", speed: props.download_speed.unwrap_or(0.0) }
                            SpeedIndicator { direction: "Outbound", speed: props.upload_speed.unwrap_or(0.0), is_upload: true }
                        }

                        // Chart
                        div {
                            class: "relative z-10 h-32 lg:h-40 w-full overflow-hidden rounded-2xl glass-inset border-white/5",
                            svg {
                                class: "absolute bottom-0 left-0 right-0 h-full w-full opacity-100",
                                preserve_aspect_ratio: "none",
                                view_box: "0 0 {graph_width} {graph_height}",
                                xmlns: "http://www.w3.org/2000/svg",
                                defs {
                                    linearGradient {
                                        id: "chartGradient",
                                        x1: "0", y1: "0", x2: "0", y2: "1",
                                        stop { offset: "0%", stop_color: "#00f0ff", stop_opacity: "0.2" }
                                        stop { offset: "100%", stop_color: "#bc00ff", stop_opacity: "0" }
                                    }
                                }
                                path {
                                    d: generate_area_path(&props.bandwidth_history, graph_width, graph_height),
                                    fill: "url(#chartGradient)",
                                }
                                path {
                                    d: generate_graph_path(&props.bandwidth_history, graph_width, graph_height),
                                    fill: "none",
                                    stroke: "#00f0ff",
                                    stroke_width: "3",
                                    stroke_linecap: "round",
                                    filter: "drop-shadow(0 0 10px rgba(0, 240, 255, 0.4))"
                                }
                            }
                        }

                        // Raw Log Streamer
                        div {
                            class: "mt-6 p-4 rounded-xl bg-black/40 border border-white/5 font-mono text-[10px] text-gray-400 h-32 overflow-hidden flex flex-col justify-end",
                            div { class: "mb-2 text-primary font-bold uppercase tracking-widest", "Raw Log Stream" }
                            for log in logs.read().iter() {
                                div { class: "truncate", "{log}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MetricTile(
    label: String,
    value: String,
    unit: String,
    icon: String,
    #[props(default = Vec::new())]
    history: Vec<f64>,
    #[props(default = false)]
    is_alert: bool
) -> Element {
    // Generate simple sparkline path
    let spark_path = if !history.is_empty() {
        let max = history.iter().cloned().fold(0.0/0.0, f64::max).max(1.0);
        let min = history.iter().cloned().fold(0.0/0.0, f64::min).min(0.0);
        let range = if max - min == 0.0 { 1.0 } else { max - min };
        let width = 50.0;
        let height = 15.0;
        let step = width / (history.len().max(2) - 1) as f64;

        let mut d = format!("M0 {:.1}", height - ((history[0] - min) / range * height));
        for (i, val) in history.iter().skip(1).enumerate() {
             d.push_str(&format!(" L{:.1} {:.1}", (i + 1) as f64 * step, height - ((val - min) / range * height)));
        }
        d
    } else {
        String::new()
    };

    let val_color = if is_alert { "text-red-500" } else { "text-emerald-600 dark:text-emerald-400" };

    rsx! {
        div {
            class: "flex flex-col items-center gap-0.5 sm:gap-1 p-2 sm:p-3 rounded-xl sm:rounded-2xl glass-card transition-colors hover:bg-white/40 dark:hover:bg-white/10 group relative overflow-hidden",
            span { class: "text-[8px] sm:text-[10px] font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider group-hover:text-primary transition-colors", "{label}" }
            div {
                class: format!("flex items-center gap-0.5 sm:gap-1 {}", val_color),
                Icon { class: "text-[12px] sm:text-[16px]", name: icon }
                span { class: "text-sm sm:text-lg font-bold font-mono text-slate-700 dark:text-slate-200", "{value}" }
                span { class: "text-[8px] sm:text-[10px] font-medium opacity-70 text-slate-500 dark:text-slate-400", "{unit}" }
            }

            if !history.is_empty() {
                div {
                    class: "w-full h-4 mt-1 opacity-50",
                    svg {
                        view_box: "0 0 50 15",
                        preserve_aspect_ratio: "none",
                        class: "w-full h-full",
                        path {
                            d: "{spark_path}",
                            fill: "none",
                            stroke: if is_alert { "red" } else { "currentColor" },
                            stroke_width: "1.5"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SpeedIndicatorProps {
    direction: String,
    speed: f64,
    #[props(default = false)]
    is_upload: bool,
}

#[component]
fn SpeedIndicator(props: SpeedIndicatorProps) -> Element {
    let (icon, icon_bg, speed_class, text_class, main_speed_class) = if props.is_upload {
        (
            "upload",
            "bg-slate-500/10 dark:bg-white/5",
            "text-slate-500 dark:text-slate-300",
            "text-slate-500",
            "text-sm sm:text-lg",
        )
    } else {
        (
            "download",
            "bg-blue-500/10 dark:bg-blue-400/10",
            "text-primary",
            "text-slate-500",
            "text-lg sm:text-2xl",
        )
    };
    let speed_formatted = format!("{:.1}", props.speed);

    rsx! {
        div {
            class: format!("flex items-center gap-2 sm:gap-3 {}", if props.is_upload { "text-right flex-row-reverse" } else { "" }),
             div {
                class: format!("p-1.5 sm:p-2 rounded-full border border-blue-500/20 {}", icon_bg),
                Icon { class: format!("text-primary text-xs sm:text-sm block {}", speed_class), name: icon.to_string() }
            }
            div {
                class: format!("flex flex-col {}", if props.is_upload { "items-end" } else { "" }),
                span { class: format!("text-[8px] sm:text-[10px] font-medium uppercase {}", text_class), "{props.direction}" }
                div {
                    class: "flex items-baseline gap-0.5 sm:gap-1",
                    span { class: format!("font-bold font-mono text-slate-800 dark:text-white {}", main_speed_class), "{speed_formatted}" }
                    span { class: "text-[10px] sm:text-xs text-slate-500 font-medium", "MB/s" }
                }
            }
        }
    }
}
