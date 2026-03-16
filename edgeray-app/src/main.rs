//! EdgeRay Client
//!
//! A cross-platform client built with Dioxus and Tauri.
//! Features a modern glassmorphism UI design.

use dioxus::prelude::*;
use edgeray_app::models::{PerAppMode, RoutingMode, ServerConfig, TunnelConfig};
use edgeray_app::{components, db, services};

use components::per_app_view::PerAppView;
use components::sidebar::MeshSafety;
use components::ui::Icon;
use components::{
    AssetManagerView, Dashboard, LogView, MeshDashboard, Page, ServerList, SettingsScreen,
    dashboard::ConnectionState, routing_view::RoutingView, server_add_modal::ServerAddModal,
};
use edgeray_app::ui::about_view::AboutView;
use edgeray_app::ui::adaptive_shell::{AdaptiveShell, UiMode};
use edgeray_app::ui::subscription_view::SubscriptionView;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    components::log_view::init_logging();
    dioxus::launch(App);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    components::log_view::init_logging();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut current_page = use_signal(|| Page::Dashboard);
    let mut connection_state = use_signal(|| ConnectionState::Disconnected);
    let selected_server_idx = use_signal(|| None::<usize>);
    let mut show_add_modal = use_signal(|| false);

    let mut mesh_safety = use_signal(|| MeshSafety::Secure);
    let mut ui_mode = use_signal(|| UiMode::Simple); // Default to Simple Mode for better UX

    // Initializing Intelligent Networking Components
    use_hook(|| {
        spawn(async move {
            db::init().await.expect("failed to init db");

            // Load persistent UI Mode
            if let Ok(settings) = db::get_settings().await {
                let mode = match settings.ui_mode.as_str() {
                    "pro" => UiMode::Pro,
                    _ => UiMode::Simple,
                };
                ui_mode.set(mode);
            }

            services::subscription_manager::SubscriptionManager::start_background_loop();

            #[cfg(not(target_arch = "wasm32"))]
            {
                // Init networking components
                let mut dialer = edgeray_app::networking::dialer::IspAwareDialer::new();
                match dialer.detect_isp().await {
                    Ok(isp) => log::info!(
                        "ISP-Aware Dialer initialized for: {} (ASN: {})",
                        isp.name,
                        isp.asn
                    ),
                    Err(e) => log::warn!("Failed to detect ISP: {}", e),
                }

                let _monitor = edgeray_app::networking::monitor::ConnectionMonitor::new();
                log::info!("Connection Monitor online");
            }
        });
    });

    let servers = use_resource(move || async move { db::list_servers().await.unwrap_or_default() });

    let active_server = use_memo(move || {
        let idx = *selected_server_idx.read();
        let list = servers.read();
        if let (Some(i), Some(s)) = (idx, list.as_ref()) {
            s.get(i).cloned()
        } else {
            None
        }
    });

    // Persist UI Mode changes
    use_effect(move || {
        let current_mode = *ui_mode.read();
        spawn(async move {
            if let Ok(mut settings) = db::get_settings().await {
                let mode_str = match current_mode {
                    UiMode::Simple => "simple",
                    UiMode::Pro => "pro",
                };
                if settings.ui_mode != mode_str {
                    settings.ui_mode = mode_str.to_string();
                    let _ = db::save_settings(settings).await;
                }
            }
        });
    });

    // Cycle mesh safety for demonstration
    use_effect(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
                loop {
                    interval.tick().await;
                    let current_safety = *mesh_safety.read();
                    let next_safety = match current_safety {
                        MeshSafety::Secure => MeshSafety::Compromised,
                        MeshSafety::Compromised => MeshSafety::Offline,
                        MeshSafety::Offline => MeshSafety::Secure,
                    };
                    mesh_safety.set(next_safety);
                }
            });
        }
    });

    let on_toggle = move |_| {
        let current = *connection_state.read();
        if current == ConnectionState::Disconnected {
            if let Some(server) = active_server.read().clone() {
                // Fetch settings before connecting
                spawn(async move {
                    let settings = db::get_settings().await.unwrap_or_default();

                    let routing_mode = match settings.routing_mode.as_str() {
                        "global" => RoutingMode::Global,
                        "direct" => RoutingMode::Direct,
                        "rule" => RoutingMode::Rule,
                        _ => RoutingMode::BypassLan, // Default fallback
                    };

                    let _config = TunnelConfig {
                        file_descriptor: None,
                        active_server: server.clone(),
                        tun_name: "edgeray0".to_string(),
                        tun_ip: "10.0.0.1".to_string(),
                        tun_cidr: 24,
                        tun_mtu: 1500,
                        routing_mode,
                        geodata_dir: None,
                        per_app_mode: PerAppMode::Global,
                        per_app_list: vec![],
                        sniffing: settings.sniffing,
                        dns_hijacking: settings.dns_hijacking,
                        lock_vpn: settings.lock_vpn,
                    };

                    log::info!(
                        "Connecting to: {} (Mode: {:?})",
                        server.remarks,
                        routing_mode
                    );

                    if settings.lock_vpn {
                        log::warn!("Kill Switch (Lock VPN) is ENABLED");
                    }

                    connection_state.set(ConnectionState::Connecting);

                    #[cfg(feature = "tauri")]
                    {
                        let args = serde_json::json!({
                            "config": config,
                        });
                        tauri_sys::core::invoke::<()>("connect_tunnel", &args).await;
                        log::info!("Tunnel connection requested");
                        connection_state.set(ConnectionState::Connected);
                    }
                    #[cfg(not(feature = "tauri"))]
                    {
                        // Mock connection delay
                        #[cfg(not(target_arch = "wasm32"))]
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        connection_state.set(ConnectionState::Connected);
                    }
                });
            }
        } else {
            connection_state.set(ConnectionState::Disconnected);
            spawn(async move {
                #[cfg(feature = "tauri")]
                {
                    tauri_sys::core::invoke::<()>("disconnect_tunnel", &serde_json::json!({}))
                        .await;
                }
            });
        }
    };

    let ping = use_memo(move || {
        if *connection_state.read() == ConnectionState::Connected {
            Some(42u32)
        } else {
            None
        }
    });

    // Traffic stats
    let mut speeds = use_signal(|| (0.0, 0.0)); // (upload, download) MB/s
    let mut bandwidth_history = use_signal(|| Vec::<f64>::with_capacity(30));

    use_effect(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
                let _last_upload = 0u64;
                let _last_download = 0u64;

                loop {
                    interval.tick().await;
                    if *connection_state.read() == ConnectionState::Connected {
                        let (up_speed, down_speed) = {
                            #[cfg(feature = "tauri")]
                            {
                                let (up, down) = tauri_sys::core::invoke::<(u64, u64)>(
                                    "get_stats",
                                    &serde_json::json!({}),
                                )
                                .await;
                                let u_speed =
                                    (up.saturating_sub(last_upload) as f64) / 1024.0 / 1024.0;
                                let d_speed =
                                    (down.saturating_sub(last_download) as f64) / 1024.0 / 1024.0;
                                last_upload = up;
                                last_download = down;
                                (u_speed, d_speed)
                            }
                            #[cfg(not(feature = "tauri"))]
                            {
                                (1.2, 3.5) // Mock stats
                            }
                        };

                        speeds.set((up_speed, down_speed));
                        let mut history = bandwidth_history.read().clone();
                        if history.len() >= 30 {
                            history.remove(0);
                        }
                        history.push(down_speed);
                        bandwidth_history.set(history);
                    }
                }
            });
        }
    });

    let (upload_speed, download_speed) = {
        let (up, down) = *speeds.read();
        (Some(up), Some(down))
    };

    rsx! {
        style { "{include_str!(\"../assets/styles.css\")}" }
        if *show_add_modal.read() {
            ServerAddModal {
                on_close: move |_| show_add_modal.set(false),
                on_save: move |configs: Vec<ServerConfig>| {
                    show_add_modal.set(false);
                     spawn(async move {
                         for config in configs {
                             let _ = db::save_server(config).await;
                         }
                         log::info!("Saved configs");
                     });
                },
            }
        } else {
            AdaptiveShell {
                current_page: current_page,
                on_navigate: move |page| current_page.set(page),
                mesh_safety: mesh_safety,
                ui_mode: ui_mode,



                match *current_page.read() {
                    Page::Dashboard => rsx! {
                        Dashboard {
                            active_server: active_server.read().clone(),
                            connection_state: *connection_state.read(),
                            on_toggle: on_toggle,
                            ping: *ping.read(),
                            upload_speed: upload_speed,
                            download_speed: download_speed,
                            bandwidth_history: bandwidth_history.cloned(),
                            ui_mode: *ui_mode.read(),
                        }
                    },
                    Page::Configs => rsx! {
                        ServersScreen {
                            servers: servers,
                            on_add: move |_| show_add_modal.set(true),
                        }
                    },
                    Page::SubscriptionGroups => rsx! {
                        SubscriptionView {
                            on_close: move |_| current_page.set(Page::Dashboard),
                        }
                    },
                    Page::Settings => rsx! {
                        SettingsScreen {
                            on_done: move |_| current_page.set(Page::Dashboard),
                            on_routing_rules: move |_| current_page.set(Page::RoutingRules),
                             on_assets: move |_| current_page.set(Page::Assets),
                             on_per_app_proxy: move |_| current_page.set(Page::PerAppProxy),
                             on_logs: move |_| current_page.set(Page::Logs),
                             on_firewall: move |_| current_page.set(Page::Firewall),
                             on_dns_tuning: move |_| current_page.set(Page::DnsTuning),
                             on_flow_tuning: move |_| current_page.set(Page::FlowJTuning),
                             on_stack_monitor: move |_| current_page.set(Page::StackMonitor),
                             on_advanced_tuning: move |_| current_page.set(Page::AdvancedTuning),
                             on_repo_click: move |_| current_page.set(Page::About),
                            on_policy_click: move |_| current_page.set(Page::About),
                         }
                    },
                    Page::AdvancedTuning => rsx! {
                        edgeray_app::ui::settings::advanced_tuning::AdvancedTuning {
                            // on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::Assets => rsx! {
                        AssetManagerView {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::Logs => rsx! {
                        LogView {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::Mesh => rsx! {
                        MeshDashboard {}
                    },
                    Page::RoutingRules => rsx! {
                        RoutingView {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::Firewall => rsx! {
                        edgeray_app::ui::diagnostics::firewall_view::FirewallView {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::DnsTuning => rsx! {
                        edgeray_app::ui::settings::dns_manager::DnsManager {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::FlowJTuning => rsx! {
                        edgeray_app::ui::settings::flow_j_pro::FlowJPro {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::StackMonitor => rsx! {
                        edgeray_app::ui::diagnostics::stack_monitor::StackMonitor {
                            on_back: move |_| current_page.set(Page::Settings),
                        }
                    },
                    Page::PerAppProxy => rsx! {
                        PerAppView {}
                    },
                    Page::About => rsx! {
                        AboutView {
                            on_back: move |_| current_page.set(Page::Settings),
                            on_repo_click: move |_| {
                                #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
                                {
                                    let _ = open::that("https://github.com/edgeray/edgeray");
                                }
                            },
                            on_policy_click: move |_| {
                                #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
                                {
                                    let _ = open::that("https://edgeray.io/privacy");
                                }
                            },
                        }
                    },
                    Page::Setup => rsx! {
                        edgeray_app::ui::pages::setup_page::SetupPage {
                            on_complete: move |(_, _)| current_page.set(Page::Dashboard),
                        }
                    },
                    Page::Shield => rsx! {
                        edgeray_app::ui::pages::shield_page::ShieldPage {
                            on_back: move |_| current_page.set(Page::Dashboard),
                        }
                    },
                    Page::Forensics => rsx! {
                        edgeray_app::ui::diagnostics::routing_canvas::RoutingCanvas {
                            nodes: vec![],
                            links: vec![],
                        }
                    },
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ServersScreenProps {
    servers: Resource<Vec<ServerConfig>>,
    on_add: EventHandler<()>,
}

#[component]
fn ServersScreen(props: ServersScreenProps) -> Element {
    let servers_list = props.servers.read();

    match servers_list.as_ref() {
        Some(list) if !list.is_empty() => {
            rsx! {
                ServerList {
                    servers: list.clone(),
                    on_select: move |server: ServerConfig| {
                        log::info!("Select server: {}", server.remarks);
                    },
                    on_edit: move |server: ServerConfig| {
                        log::info!("Edit server: {}", server.remarks);
                    },
                    on_delete: Some(EventHandler::new(move |server: ServerConfig| {
                        log::info!("Delete server: {}", server.remarks);
                    })),
                    on_share: Some(EventHandler::new(move |server: ServerConfig| {
                        log::info!("Share server: {}", server.remarks);
                    })),
                    on_ping_all: move |_| {
                        log::info!("Ping all servers");
                    },
                    on_add: props.on_add,
                }
            }
        }
        Some(_) => {
            rsx! {
                div {
                    class: "flex flex-col items-center gap-4 text-center transform scale-90 md:scale-100 transition-transform duration-500",
                    Icon { name: "dns", class: "text-6xl mb-4 opacity-50" }
                    p { class: "font-medium", "No servers configured" }
                    p { class: "text-sm text-white/30 mt-1", "Add a server to get started" }
                }
            }
        }
        None => {
            rsx! {
                div {
                    class: "flex items-center justify-center h-screen",
                    div {
                        class: "w-10 h-10 border-2 border-primary border-t-transparent rounded-full animate-spin"
                    }
                }
            }
        }
    }
}
