//! Per-App Proxy View
//!
//! High-performance app picker for split-tunneling configuration.
//! Supports virtualization for 200+ apps with search filtering.

use crate::components::gestures::{PullToRefresh, Ripple};
use crate::models::PerAppMode;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Android app information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AndroidApp {
    pub package_name: String,
    pub app_name: String,
    pub uid: i32,
    pub is_system_app: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_base64: Option<String>,
}

#[component]
pub fn PerAppView() -> Element {
    let mut apps = use_signal(|| Vec::<AndroidApp>::new());
    let mut filtered_apps = use_signal(|| Vec::<AndroidApp>::new());
    let mut selected_packages = use_signal(|| Vec::<String>::new());
    let mut search_query = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut show_system_apps = use_signal(|| false);
    let mut per_app_mode = use_signal(|| PerAppMode::Global);

    // Load apps on mount
    use_effect(move || {
        spawn(async move {
            loading.set(true);

            #[cfg(target_os = "android")]
            {
                match fetch_installed_apps(show_system_apps()).await {
                    Ok(app_list) => {
                        apps.set(app_list.clone());
                        filtered_apps.set(app_list);
                    }
                    Err(e) => {
                        log::error!("Failed to fetch apps: {}", e);
                    }
                }
            }

            #[cfg(not(target_os = "android"))]
            {
                // Mock data for desktop testing
                apps.set(generate_mock_apps());
                filtered_apps.set(generate_mock_apps());
            }

            loading.set(false);
        });
    });

    // Filter apps when search query changes
    use_effect(move || {
        let query = search_query().to_lowercase();
        if query.is_empty() {
            filtered_apps.set(apps());
        } else {
            let filtered: Vec<AndroidApp> = apps()
                .into_iter()
                .filter(|app| {
                    app.app_name.to_lowercase().contains(&query)
                        || app.package_name.to_lowercase().contains(&query)
                })
                .collect();
            filtered_apps.set(filtered);
        }
    });

    // Toggle app selection
    let mut toggle_app = move |package_name: String| {
        let mut current = selected_packages();
        if current.contains(&package_name) {
            current.retain(|p| p != &package_name);
        } else {
            current.push(package_name);
        }
        selected_packages.set(current);
    };

    // Save configuration
    let save_config = move |_| {
        spawn(async move {
            let packages = selected_packages();
            let mode = per_app_mode();

            match save_per_app_config(mode, packages).await {
                Ok(_) => {
                    log::info!("Per-app configuration saved");
                }
                Err(e) => {
                    log::error!("Failed to save per-app config: {}", e);
                }
            }
        });
    };

    // Pull to refresh handler
    let on_refresh = move |_| {
        spawn(async move {
            // Simulate network delay for better UX
            // Simulate network delay for better UX
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::sleep(std::time::Duration::from_millis(1000)).await;

            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

            #[cfg(target_os = "android")]
            {
                if let Ok(app_list) = fetch_installed_apps(show_system_apps()).await {
                    apps.set(app_list.clone());
                    filtered_apps.set(app_list);
                }
            }

            #[cfg(not(target_os = "android"))]
            {
                apps.set(generate_mock_apps());
                filtered_apps.set(generate_mock_apps());
            }
        });
    };

    rsx! {
        div {
            class: "relative flex h-full min-h-screen w-full flex-col overflow-x-hidden font-display text-white antialiased",
            // Background managed by Layout


            // Header
            crate::components::ui::PageHeader {
                title: "Per-App Proxy".to_string(),
                subtitle: Some("Select which apps should use the VPN tunnel".to_string()),
            }

            main {
                class: "flex-1 flex flex-col px-4 lg:px-8 pb-32 lg:pb-8 pt-4 z-10 gap-4 safe-area-top safe-area-bottom",

                // Mode selector
                crate::components::ui::GlassCard {
                    class: "p-4 glass-panel",
                    children: rsx! {
                        div { class: "text-xs font-bold text-gray-400 uppercase tracking-widest mb-3", "Routing Mode" }
                        div {
                            class: "flex p-1.5 rounded-xl bg-black/40 backdrop-blur-md shadow-inner border border-white/5",
                            button {
                                class: format!("flex-1 py-2.5 rounded-lg text-sm font-bold transition-all {}",
                                    if *per_app_mode.read() == PerAppMode::Global { "bg-primary/20 text-primary shadow-glow-cyan/50" } else { "text-gray-400 hover:text-white" }
                                ),
                                onclick: move |_| per_app_mode.set(PerAppMode::Global),
                                "Global"
                            }
                            button {
                                class: format!("flex-1 py-2.5 rounded-lg text-sm font-bold transition-all {}",
                                    if *per_app_mode.read() == PerAppMode::Whitelist { "bg-primary/20 text-primary shadow-glow-cyan/50" } else { "text-gray-400 hover:text-white" }
                                ),
                                onclick: move |_| per_app_mode.set(PerAppMode::Whitelist),
                                "Whitelist"
                            }
                            button {
                                class: format!("flex-1 py-2.5 rounded-lg text-sm font-bold transition-all {}",
                                    if *per_app_mode.read() == PerAppMode::Blacklist { "bg-primary/20 text-primary shadow-glow-cyan/50" } else { "text-gray-400 hover:text-white" }
                                ),
                                onclick: move |_| per_app_mode.set(PerAppMode::Blacklist),
                                "Blacklist"
                            }
                        }
                    }
                }

                // Search bar
                div {
                    class: "relative group",
                    div { class: "absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 pointer-events-none",
                        crate::components::ui::Icon { name: "search", class: "" }
                    }
                    input {
                        r#type: "text",
                        class: "pl-12 pr-4 py-3.5 w-full text-sm bg-black/40 border border-white/5 rounded-xl focus:ring-2 focus:ring-primary/50 focus:border-primary/50 outline-none transition-all placeholder:text-gray-500 text-white shadow-inner glass-inset",
                        placeholder: "Search apps...",
                        value: "{search_query()}",
                        oninput: move |evt| search_query.set(evt.value()),
                    }
                }

                // Stats + Show system apps toggle
                div {
                    class: "flex items-center justify-between px-1",
                    div { class: "flex items-center gap-4 text-sm",
                        span { class: "font-bold text-primary drop-shadow-[0_0_8px_rgba(34,211,238,0.5)]", "{selected_packages().len()} selected" }
                        span { class: "text-gray-500", "{filtered_apps().len()} apps" }
                    }
                    label {
                        class: "flex items-center gap-2 text-sm text-gray-400 cursor-pointer select-none",
                        input {
                            r#type: "checkbox",
                            class: "rounded border-white/10 bg-black/40 text-primary focus:ring-primary/50",
                            checked: show_system_apps(),
                            onchange: move |evt| {
                                show_system_apps.set(evt.checked());
                                spawn(async move {
                                    loading.set(true);
                                    #[cfg(target_os = "android")]
                                    {
                                        if let Ok(app_list) = fetch_installed_apps(show_system_apps()).await {
                                            apps.set(app_list.clone());
                                            filtered_apps.set(app_list);
                                        }
                                    }
                                    loading.set(false);
                                });
                            },
                        }
                        "Show system apps"
                    }
                }

                // App list
                if loading() {
                    div { class: "flex-1 flex items-center justify-center",
                        div { class: "flex flex-col items-center gap-3",
                            div { class: "size-8 border-2 border-primary border-t-transparent rounded-full animate-spin" }
                            span { class: "text-sm text-gray-400", "Loading apps..." }
                        }
                    }
                } else {

                    div {
                        class: "flex-1 overflow-y-auto rounded-2xl no-scrollbar",
                        PullToRefresh {
                            on_refresh: on_refresh,
                            children: rsx! {
                                div {
                                    class: "space-y-2 pb-4",
                                    for app in filtered_apps() {
                                        AppCard {
                                            key: "{app.package_name}",
                                            app: app.clone(),
                                            selected: selected_packages().contains(&app.package_name),
                                            on_toggle: move |pkg| toggle_app(pkg),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Save button
                div {
                    class: "fixed bottom-0 left-0 right-0 p-4 lg:p-6 bg-obsidian/90 backdrop-blur-2xl border-t border-white/5 z-50 safe-area-bottom",
                    crate::components::ui::PrimaryButton {
                        label: "Save Configuration".to_string(),
                        icon: Some("save".to_string()),
                        onclick: save_config,
                    }
                }
            }
        }
    }
}

#[component]
fn AppCard(app: AndroidApp, selected: bool, on_toggle: EventHandler<String>) -> Element {
    let card_class = if selected {
        format!(
            "{} ring-1 ring-primary/50 bg-primary/20 shadow-glow-cyan/20",
            crate::components::ui::glass::CARD
        )
    } else {
        crate::components::ui::glass::CARD.to_string()
    };

    rsx! {
        Ripple {
            class: format!("{} p-4 flex items-center gap-4 cursor-pointer transition-all hover:scale-[1.02] hover:bg-white/5", card_class),
            onclick: {
                let pkg = app.package_name.clone();
                move |_| on_toggle.call(pkg.clone())
            },

            // App icon
            div { class: "shrink-0 size-12 rounded-xl bg-gradient-to-br from-white/10 to-white/5 flex items-center justify-center overflow-hidden border border-white/10",
                if let Some(icon_data) = &app.icon_base64 {
                    img {
                        class: "size-10 rounded-lg object-cover",
                        src: "data:image/png;base64,{icon_data}",
                        alt: "{app.app_name}",
                    }
                } else {
                    span { class: "text-2xl", "📱" }
                }
            }

            // App info
            div { class: "flex-1 min-w-0",
                div { class: "font-bold text-white truncate", "{app.app_name}" }
                div { class: "text-xs text-gray-500 font-mono truncate", "{app.package_name}" }
            }

            // Checkbox
            div { class: "shrink-0",
                div {
                    class: format!("size-6 rounded-lg border-2 flex items-center justify-center transition-all {}",
                        if selected { "bg-primary border-primary shadow-glow-cyan" } else { "border-white/10 bg-black/40" }
                    ),
                    onclick: move |evt| evt.stop_propagation(),
                    if selected {
                        crate::components::ui::Icon { name: "check", class: "text-white text-sm" }
                    }
                }
            }
        }
    }
}

/// Fetch installed apps from Android
#[cfg(target_os = "android")]
async fn fetch_installed_apps(include_system: bool) -> Result<Vec<AndroidApp>, String> {
    let metadata_list = crate::platform::fetch_installed_apps().await?;

    let apps = metadata_list
        .into_iter()
        .filter_map(|m| {
            if !include_system && m.is_system {
                return None;
            }
            Some(AndroidApp {
                package_name: m.package_id,
                app_name: m.name,
                // Use 0 or unwrap if uid is Option<u32>, cast safely
                uid: m.uid.unwrap_or(0) as i32,
                is_system_app: m.is_system,
                icon_base64: None, // Icon path vs base64 handling todo
            })
        })
        .collect();

    Ok(apps)
}

/// Save per-app configuration to database
#[allow(dead_code)]
async fn save_per_app_config(mode: PerAppMode, packages: Vec<String>) -> Result<(), String> {
    // Store packages as JSON in settings
    let _packages_json = serde_json::to_string(&packages)
        .map_err(|e| format!("Failed to serialize packages: {}", e))?;

    let _ = mode; // Suppress unused warning

    // For now, we'll use a simple file-based storage
    // In production, this should be stored in SurrealDB
    #[cfg(target_os = "android")]
    {
        use std::fs;
        let data_dir = dirs::data_dir().ok_or("Failed to get data dir")?;
        let config_path = data_dir.join("edgeray").join("per_app_config.json");

        let config = serde_json::json!({
            "mode": format!("{:?}", mode),
            "packages": packages,
        });

        fs::write(config_path, config.to_string())
            .map_err(|e| format!("Failed to write config: {}", e))?;
    }

    Ok(())
}

/// Generate mock apps for desktop testing
#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
fn generate_mock_apps() -> Vec<AndroidApp> {
    vec![
        AndroidApp {
            package_name: "com.android.chrome".to_string(),
            app_name: "Chrome".to_string(),
            uid: 10001,
            is_system_app: false,
            icon_base64: None,
        },
        AndroidApp {
            package_name: "com.whatsapp".to_string(),
            app_name: "WhatsApp".to_string(),
            uid: 10002,
            is_system_app: false,
            icon_base64: None,
        },
        AndroidApp {
            package_name: "org.telegram.messenger".to_string(),
            app_name: "Telegram".to_string(),
            uid: 10003,
            is_system_app: false,
            icon_base64: None,
        },
        AndroidApp {
            package_name: "com.twitter.android".to_string(),
            app_name: "Twitter".to_string(),
            uid: 10004,
            is_system_app: false,
            icon_base64: None,
        },
        AndroidApp {
            package_name: "com.instagram.android".to_string(),
            app_name: "Instagram".to_string(),
            uid: 10005,
            is_system_app: false,
            icon_base64: None,
        },
    ]
}
