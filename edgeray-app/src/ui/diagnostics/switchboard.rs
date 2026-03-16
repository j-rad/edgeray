//! Switchboard - TLS Connection Forensics
//!
//! tls-tunnel style diagnostics showing real-time local port → remote TLS SNI mappings,
//! uTLS fingerprints, and handshake forensics.

use crate::components::ui::{GlassCard, Icon, PageHeader, SectionHeader};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// TLS connection information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TlsConnectionInfo {
    pub id: String,
    pub local_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub sni: String,
    pub tls_version: String,
    pub cipher_suite: String,
    pub utls_fingerprint: String,
    pub state: ConnectionState,
    pub handshake_duration_ms: u32,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub established_at: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionState {
    Handshaking,
    Established,
    Closing,
    Closed,
}

impl ConnectionState {
    fn color(&self) -> &'static str {
        match self {
            Self::Handshaking => "text-yellow-600 dark:text-yellow-400",
            Self::Established => "text-green-600 dark:text-green-400",
            Self::Closing => "text-orange-600 dark:text-orange-400",
            Self::Closed => "text-gray-600 dark:text-gray-400",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Handshaking => "Handshaking",
            Self::Established => "Established",
            Self::Closing => "Closing",
            Self::Closed => "Closed",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Handshaking => "sync",
            Self::Established => "check_circle",
            Self::Closing => "pending",
            Self::Closed => "cancel",
        }
    }
}

#[derive(Debug, Clone, Default)]
struct SwitchboardStats {
    total_connections: u32,
    active_connections: u32,
    total_bytes_sent: u64,
    total_bytes_received: u64,
    avg_handshake_ms: u32,
}

#[component]
pub fn Switchboard() -> Element {
    let mut connections = use_signal(|| Vec::<TlsConnectionInfo>::new());
    let mut stats = use_signal(|| SwitchboardStats::default());
    let mut loading = use_signal(|| false);
    let mut auto_refresh = use_signal(|| true);
    let mut filter_state = use_signal(|| None::<ConnectionState>);
    let mut selected_connection = use_signal(|| None::<String>);

    // Load connections on mount
    use_effect(move || {
        spawn(async move {
            loading.set(true);

            #[cfg(not(target_arch = "wasm32"))]
            {
                match load_tls_connections().await {
                    Ok(conns) => {
                        connections.set(conns.clone());
                        stats.set(calculate_switchboard_stats(&conns));
                    }
                    Err(e) => log::error!("Failed to load TLS connections: {}", e),
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let mock_conns = generate_mock_connections();
                connections.set(mock_conns.clone());
                stats.set(calculate_switchboard_stats(&mock_conns));
            }

            loading.set(false);
        });
    });

    // Auto-refresh every 2 seconds
    use_effect(move || {
        if auto_refresh() {
            #[cfg(not(target_arch = "wasm32"))]
            {
                spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
                    loop {
                        interval.tick().await;
                        if !auto_refresh() {
                            break;
                        }

                        if let Ok(conns) = load_tls_connections().await {
                            connections.set(conns.clone());
                            stats.set(calculate_switchboard_stats(&conns));
                        }
                    }
                });
            }
        }
    });

    let filtered_connections = use_memo(move || {
        let conns = connections();
        if let Some(state) = filter_state() {
            conns.into_iter().filter(|c| c.state == state).collect()
        } else {
            conns
        }
    });

    rsx! {
        div {
            class: "relative flex h-full min-h-screen w-full flex-col overflow-x-hidden font-display text-slate-900 dark:text-white antialiased",

            // Background
            div { class: "fixed inset-0 bg-[#f8fafc] dark:bg-[#020617] -z-20" }
            div { class: "fixed top-[-20%] left-[-20%] w-[60vw] h-[60vw] bg-cyan-400/20 dark:bg-cyan-600/20 rounded-full blur-[120px] pointer-events-none -z-10 mix-blend-multiply dark:mix-blend-screen animate-pulse" }
            div { class: "fixed bottom-[-20%] right-[-20%] w-[60vw] h-[60vw] bg-teal-400/20 dark:bg-teal-600/20 rounded-full blur-[120px] pointer-events-none -z-10 mix-blend-multiply dark:mix-blend-screen" }

            PageHeader {
                title: "Switchboard".to_string(),
                subtitle: Some("TLS connection forensics and port mapping".to_string()),
            }

            main {
                class: "flex-1 flex flex-col px-4 lg:px-8 pb-8 pt-4 z-10 gap-4",

                // Stats overview
                div {
                    class: "grid grid-cols-2 md:grid-cols-5 gap-3",
                    SwitchboardStatCard {
                        label: "Total Connections",
                        value: format!("{}", stats().total_connections),
                        icon: "hub",
                        color: "text-blue-600 dark:text-blue-400"
                    }
                    SwitchboardStatCard {
                        label: "Active",
                        value: format!("{}", stats().active_connections),
                        icon: "link",
                        color: "text-green-600 dark:text-green-400"
                    }
                    SwitchboardStatCard {
                        label: "Sent",
                        value: format_bytes(stats().total_bytes_sent),
                        icon: "upload",
                        color: "text-orange-600 dark:text-orange-400"
                    }
                    SwitchboardStatCard {
                        label: "Received",
                        value: format_bytes(stats().total_bytes_received),
                        icon: "download",
                        color: "text-purple-600 dark:text-purple-400"
                    }
                    SwitchboardStatCard {
                        label: "Avg Handshake",
                        value: format!("{}ms", stats().avg_handshake_ms),
                        icon: "speed",
                        color: "text-cyan-600 dark:text-cyan-400"
                    }
                }

                // State filter
                GlassCard {
                    class: "p-4",
                    children: rsx! {
                        div { class: "flex items-center justify-between mb-3",
                            SectionHeader {
                                title: "Connection States".to_string(),
                                icon: Some("filter_list".to_string())
                            }
                            label {
                                class: "flex items-center gap-2 text-sm text-slate-500 dark:text-gray-400 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded border-white/20 bg-white/5 text-primary focus:ring-primary/50",
                                    checked: auto_refresh(),
                                    onchange: move |evt| auto_refresh.set(evt.checked()),
                                }
                                "Auto-refresh"
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2",
                            StateChip {
                                state: None,
                                active: filter_state().is_none(),
                                onclick: move |_| filter_state.set(None),
                            }
                            for state in [
                                ConnectionState::Handshaking,
                                ConnectionState::Established,
                                ConnectionState::Closing,
                                ConnectionState::Closed,
                            ] {
                                StateChip {
                                    state: Some(state),
                                    active: filter_state() == Some(state),
                                    onclick: move |_| filter_state.set(Some(state)),
                                }
                            }
                        }
                    }
                }

                // Connection list
                div {
                    SectionHeader {
                        title: format!("TLS Connections ({})", filtered_connections().len()),
                        icon: Some("list".to_string())
                    }
                }

                if loading() {
                    div { class: "flex-1 flex items-center justify-center",
                        div { class: "flex flex-col items-center gap-3",
                            div { class: "size-8 border-2 border-primary border-t-transparent rounded-full animate-spin" }
                            span { class: "text-sm text-slate-500 dark:text-gray-400", "Loading connections..." }
                        }
                    }
                } else {
                    div {
                        class: "flex-1 overflow-y-auto space-y-2 custom-scrollbar",
                        for conn in filtered_connections() {
                            ConnectionCard {
                                key: "{conn.id}",
                                connection: conn.clone(),
                                selected: selected_connection() == Some(conn.id.clone()),
                                on_select: move |id: String| {
                                    if selected_connection() == Some(id.clone()) {
                                        selected_connection.set(None);
                                    } else {
                                        selected_connection.set(Some(id));
                                    }
                                },
                            }
                        }
                        if filtered_connections().is_empty() {
                            div { class: "flex flex-col items-center justify-center py-12 text-center",
                                Icon { name: "link_off", class: "text-6xl text-gray-500 mb-4" }
                                p { class: "text-lg font-semibold text-slate-900 dark:text-white", "No connections" }
                                p { class: "text-sm text-slate-500 dark:text-gray-400 mt-1", "Establish a connection to see diagnostics" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SwitchboardStatCard(label: String, value: String, icon: String, color: String) -> Element {
    rsx! {
        GlassCard {
            class: "p-4 flex items-center gap-3",
            children: rsx! {
                div { class: "p-2 rounded-lg bg-white/10 dark:bg-black/20",
                    Icon { name: icon, class: format!("{} text-xl", color) }
                }
                div { class: "flex-1 min-w-0",
                    div { class: "text-xs text-slate-500 dark:text-gray-400 uppercase tracking-wide", "{label}" }
                    div { class: format!("text-lg font-bold truncate {}", color), "{value}" }
                }
            }
        }
    }
}

#[component]
fn StateChip(state: Option<ConnectionState>, active: bool, onclick: EventHandler<()>) -> Element {
    let (label, icon, color) = if let Some(st) = state {
        (st.label(), st.icon(), st.color())
    } else {
        ("All", "filter_list", "text-slate-600 dark:text-gray-400")
    };

    rsx! {
        button {
            class: format!(
                "px-3 py-2 rounded-lg text-sm font-medium transition-all flex items-center gap-2 {}",
                if active {
                    "bg-white dark:bg-white/10 shadow-sm ring-2 ring-primary/50"
                } else {
                    "bg-white/50 dark:bg-black/20 hover:bg-white dark:hover:bg-white/5"
                }
            ),
            onclick: move |_| onclick.call(()),
            Icon { name: icon.to_string(), class: format!("text-base {}", color) }
            span { class: color, "{label}" }
        }
    }
}

#[component]
fn ConnectionCard(
    connection: TlsConnectionInfo,
    selected: bool,
    on_select: EventHandler<String>,
) -> Element {
    let uptime = format_uptime(connection.established_at);

    rsx! {
        GlassCard {
            class: format!(
                "p-4 cursor-pointer transition-all {}",
                if selected { "ring-2 ring-primary/50 bg-primary/5" } else { "hover:bg-white/10 dark:hover:bg-white/5" }
            ),
            children: rsx! {
                div {
                    onclick: move |_| on_select.call(connection.id.clone()),

                    // Header row
                    div { class: "flex items-center justify-between mb-3",
                        div { class: "flex items-center gap-2",
                            Icon { name: connection.state.icon().to_string(), class: format!("text-xl {}", connection.state.color()) }
                            span { class: "font-mono text-sm font-semibold text-slate-900 dark:text-white",
                                ":{connection.local_port} → {connection.remote_host}:{connection.remote_port}"
                            }
                        }
                        span {
                            class: format!("text-[10px] px-2 py-0.5 rounded-full font-medium {}",
                                match connection.state {
                                    ConnectionState::Established => "bg-green-500/20 text-green-600 dark:text-green-400",
                                    ConnectionState::Handshaking => "bg-yellow-500/20 text-yellow-600 dark:text-yellow-400",
                                    ConnectionState::Closing => "bg-orange-500/20 text-orange-600 dark:text-orange-400",
                                    ConnectionState::Closed => "bg-gray-500/20 text-gray-600 dark:text-gray-400",
                                }
                            ),
                            "{connection.state.label()}"
                        }
                    }

                    // SNI and TLS info
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-2 mb-3",
                        InfoRow { label: "SNI", value: connection.sni.clone(), icon: "dns" }
                        InfoRow { label: "TLS Version", value: connection.tls_version.clone(), icon: "security" }
                        InfoRow { label: "Cipher", value: connection.cipher_suite.clone(), icon: "lock" }
                        InfoRow { label: "Handshake", value: format!("{}ms", connection.handshake_duration_ms), icon: "speed" }
                    }

                    // uTLS Fingerprint
                    div { class: "mb-3",
                        div { class: "text-xs font-bold text-slate-500 dark:text-gray-400 uppercase tracking-wide mb-1", "uTLS Fingerprint" }
                        div { class: "font-mono text-xs text-slate-700 dark:text-gray-300 bg-black/10 dark:bg-white/5 px-3 py-2 rounded-lg break-all",
                            "{connection.utls_fingerprint}"
                        }
                    }

                    // Stats row
                    div { class: "flex items-center justify-between text-xs text-slate-500 dark:text-gray-400",
                        div { class: "flex items-center gap-4",
                            span { "⬆ {format_bytes(connection.bytes_sent)}" }
                            span { "⬇ {format_bytes(connection.bytes_received)}" }
                        }
                        span { "⏱ {uptime}" }
                    }
                }

                // Expanded details
                if selected {
                    div { class: "mt-4 pt-4 border-t border-white/10",
                        div { class: "text-xs font-bold text-slate-500 dark:text-gray-400 uppercase tracking-wide mb-2", "Connection Details" }
                        div { class: "grid grid-cols-2 gap-2 text-xs",
                            DetailItem { label: "Connection ID", value: connection.id.clone() }
                            DetailItem { label: "Local Port", value: format!("{}", connection.local_port) }
                            DetailItem { label: "Remote Host", value: connection.remote_host.clone() }
                            DetailItem { label: "Remote Port", value: format!("{}", connection.remote_port) }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InfoRow(label: String, value: String, icon: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-2",
            Icon { name: icon, class: "text-sm text-slate-500 dark:text-gray-400" }
            div { class: "flex-1 min-w-0",
                div { class: "text-[10px] text-slate-500 dark:text-gray-400 uppercase", "{label}" }
                div { class: "text-xs font-semibold text-slate-900 dark:text-white truncate", "{value}" }
            }
        }
    }
}

#[component]
fn DetailItem(label: String, value: String) -> Element {
    rsx! {
        div {
            div { class: "text-[10px] text-slate-500 dark:text-gray-400 uppercase mb-0.5", "{label}" }
            div { class: "text-xs font-mono text-slate-900 dark:text-white truncate", "{value}" }
        }
    }
}

fn calculate_switchboard_stats(connections: &[TlsConnectionInfo]) -> SwitchboardStats {
    let total_connections = connections.len() as u32;
    let active_connections = connections
        .iter()
        .filter(|c| {
            matches!(
                c.state,
                ConnectionState::Established | ConnectionState::Handshaking
            )
        })
        .count() as u32;
    let total_bytes_sent: u64 = connections.iter().map(|c| c.bytes_sent).sum();
    let total_bytes_received: u64 = connections.iter().map(|c| c.bytes_received).sum();
    let avg_handshake_ms = if total_connections > 0 {
        connections
            .iter()
            .map(|c| c.handshake_duration_ms)
            .sum::<u32>()
            / total_connections
    } else {
        0
    };

    SwitchboardStats {
        total_connections,
        active_connections,
        total_bytes_sent,
        total_bytes_received,
        avg_handshake_ms,
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}KB", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

fn format_uptime(timestamp: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let diff = now.saturating_sub(timestamp);

    if diff < 60 {
        format!("{}s", diff)
    } else if diff < 3600 {
        format!("{}m {}s", diff / 60, diff % 60)
    } else {
        format!("{}h {}m", diff / 3600, (diff % 3600) / 60)
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_tls_connections() -> Result<Vec<TlsConnectionInfo>, String> {
    // This would call the driver's diagnostic API
    // For now, return mock data
    Ok(generate_mock_connections())
}

fn generate_mock_connections() -> Vec<TlsConnectionInfo> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    vec![
        TlsConnectionInfo {
            id: uuid::Uuid::new_v4().to_string(),
            local_port: 54321,
            remote_host: "cdn.example.com".to_string(),
            remote_port: 443,
            sni: "cdn.example.com".to_string(),
            tls_version: "TLS 1.3".to_string(),
            cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
            utls_fingerprint: "771,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-21,29-23-24,0".to_string(),
            state: ConnectionState::Established,
            handshake_duration_ms: 45,
            bytes_sent: 524288,
            bytes_received: 2097152,
            established_at: now - 120,
        },
        TlsConnectionInfo {
            id: uuid::Uuid::new_v4().to_string(),
            local_port: 54322,
            remote_host: "api.service.io".to_string(),
            remote_port: 443,
            sni: "api.service.io".to_string(),
            tls_version: "TLS 1.3".to_string(),
            cipher_suite: "TLS_CHACHA20_POLY1305_SHA256".to_string(),
            utls_fingerprint: "771,4865-4867-4866-49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-51-57-47-53,0-23-65281-10-11-35-16-5-51-43-13-45-28-21,29-23-24-25-256-257,0".to_string(),
            state: ConnectionState::Handshaking,
            handshake_duration_ms: 0,
            bytes_sent: 0,
            bytes_received: 0,
            established_at: now - 5,
        },
        TlsConnectionInfo {
            id: uuid::Uuid::new_v4().to_string(),
            local_port: 54323,
            remote_host: "secure.bank.com".to_string(),
            remote_port: 443,
            sni: "secure.bank.com".to_string(),
            tls_version: "TLS 1.2".to_string(),
            cipher_suite: "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256".to_string(),
            utls_fingerprint: "769,49195-49199-52393-52392-49196-49200-49162-49161-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13,29-23-24,0".to_string(),
            state: ConnectionState::Established,
            handshake_duration_ms: 67,
            bytes_sent: 1048576,
            bytes_received: 4194304,
            established_at: now - 300,
        },
    ]
}
