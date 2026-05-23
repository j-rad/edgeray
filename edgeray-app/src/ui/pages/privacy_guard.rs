//! Privacy Guard - InviZible Pro Style Per-App Firewall
//!
//! Advanced per-app proxy control with UID-based routing and virtualized scrolling.

use crate::components::ui::{GlassCard, Icon, PageHeader, PrimaryButton};
use crate::models::PerAppMode;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Per-app routing rule stored in SurrealDB
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerAppRule {
    pub id: String,
    pub package_name: String,
    pub app_name: String,
    pub uid: i32,
    pub enabled: bool,
    pub mode: PerAppMode,
    pub created_at: u64,
    pub updated_at: u64,
}

/// App metadata with extended information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppMetadata {
    pub package_name: String,
    pub app_name: String,
    pub uid: i32,
    pub is_system_app: bool,
    pub version_name: Option<String>,
    pub version_code: Option<i32>,
    pub install_time: Option<u64>,
    pub update_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_base64: Option<String>,
    pub data_usage_bytes: u64,
}

#[component]
pub fn PrivacyGuard() -> Element {
    let mut apps = use_signal(|| Vec::<AppMetadata>::new());
    let mut filtered_apps = use_signal(|| Vec::<AppMetadata>::new());
    let mut selected_packages = use_signal(|| Vec::<String>::new());
    let mut search_query = use_signal(|| String::new());
    let mut loading = use_signal(|| false);
    let mut show_system_apps = use_signal(|| false);
    let mut per_app_mode = use_signal(|| PerAppMode::Global);
    let mut sort_by = use_signal(|| SortMode::Name);
    let mut stats = use_signal(|| AppStats::default());

    // Load apps and rules on mount
    use_effect(move || {
        spawn(async move {
            loading.set(true);

            // Load existing rules from SurrealDB
            #[cfg(not(target_arch = "wasm32"))]
            {
                match load_per_app_rules().await {
                    Ok(rules) => {
                        let packages: Vec<String> = rules
                            .iter()
                            .filter(|r| r.enabled)
                            .map(|r| r.package_name.clone())
                            .collect();
                        selected_packages.set(packages);

                        if let Some(first_rule) = rules.first() {
                            per_app_mode.set(first_rule.mode);
                        }
                    }
                    Err(e) => log::error!("Failed to load per-app rules: {}", e),
                }
            }

            // Fetch installed apps
            #[cfg(target_os = "android")]
            {
                match fetch_installed_apps_with_metadata(show_system_apps()).await {
                    Ok(app_list) => {
                        apps.set(app_list.clone());
                        filtered_apps.set(app_list.clone());
                        stats.set(calculate_stats(&app_list));
                    }
                    Err(e) => log::error!("Failed to fetch apps: {}", e),
                }
            }

            #[cfg(not(target_os = "android"))]
            {
                let mock_apps = generate_mock_apps_extended();
                apps.set(mock_apps.clone());
                filtered_apps.set(mock_apps.clone());
                stats.set(calculate_stats(&mock_apps));
            }

            loading.set(false);
        });
    });

    // Filter and sort apps when search query or sort mode changes
    use_effect(move || {
        let query = search_query().to_lowercase();
        let mut filtered: Vec<AppMetadata> = if query.is_empty() {
            apps()
        } else {
            apps()
                .into_iter()
                .filter(|app| {
                    app.app_name.to_lowercase().contains(&query)
                        || app.package_name.to_lowercase().contains(&query)
                })
                .collect()
        };

        // Apply sorting
        match sort_by() {
            SortMode::Name => filtered.sort_by(|a, b| a.app_name.cmp(&b.app_name)),
            SortMode::DataUsage => {
                filtered.sort_by(|a, b| b.data_usage_bytes.cmp(&a.data_usage_bytes))
            }
            SortMode::InstallDate => filtered.sort_by(|a, b| {
                b.install_time
                    .unwrap_or(0)
                    .cmp(&a.install_time.unwrap_or(0))
            }),
        }

        filtered_apps.set(filtered);
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

    // Save configuration to SurrealDB
    let save_config = move |_| {
        spawn(async move {
            let packages = selected_packages();
            let mode = per_app_mode();
            let app_list = apps();

            match save_per_app_rules_to_db(mode, packages, app_list).await {
                Ok(_) => {
                    log::info!("✓ Per-app configuration saved to SurrealDB");
                }
                Err(e) => {
                    log::error!("✗ Failed to save per-app config: {}", e);
                }
            }
        });
    };

    rsx! {
        div {
            class: "relative flex h-full min-h-screen w-full flex-col overflow-x-hidden font-display text-slate-900 dark:text-white antialiased",

            // Background gradients
            div { class: "fixed inset-0 bg-[#f8fafc] dark:bg-[#020617] -z-20" }
            div { class: "fixed top-[-20%] left-[-20%] w-[60vw] h-[60vw] bg-blue-400/20 dark:bg-blue-600/20 rounded-full blur-[120px] pointer-events-none -z-10 mix-blend-multiply dark:mix-blend-screen animate-pulse" }
            div { class: "fixed bottom-[-20%] right-[-20%] w-[60vw] h-[60vw] bg-purple-400/20 dark:bg-purple-600/20 rounded-full blur-[120px] pointer-events-none -z-10 mix-blend-multiply dark:mix-blend-screen" }

            PageHeader {
                title: "Privacy Guard".to_string(),
                subtitle: Some("InviZible-style per-app firewall control".to_string()),
            }

            main {
                class: "flex-1 flex flex-col px-4 lg:px-8 pb-24 lg:pb-8 pt-4 z-10 gap-4",

                // Stats overview
                div {
                    class: "grid grid-cols-2 md:grid-cols-4 gap-3",
                    StatCard { label: "Total Apps", value: format!("{}", stats().total_apps), icon: "apps" }
                    StatCard { label: "Selected", value: format!("{}", selected_packages().len()), icon: "check_circle" }
                    StatCard { label: "System Apps", value: format!("{}", stats().system_apps), icon: "settings" }
                    StatCard { label: "Data Usage", value: format_bytes(stats().total_data_usage), icon: "data_usage" }
                }

                // Mode selector
                GlassCard {
                    class: "p-4",
                    children: rsx! {
                        div { class: "text-xs font-bold text-slate-500 dark:text-gray-400 uppercase tracking-widest mb-3", "Routing Mode" }
                        div {
                            class: "flex p-1.5 rounded-xl bg-slate-200/50 dark:bg-black/30 backdrop-blur-md shadow-inner border border-white/10",
                            button {
                                class: format!("flex-1 py-2.5 rounded-lg text-sm font-semibold transition-all {}",
                                    if *per_app_mode.read() == PerAppMode::Global { "bg-white dark:bg-white/10 text-primary shadow-sm" } else { "text-slate-500 dark:text-gray-400 hover:text-slate-700 dark:hover:text-white" }
                                ),
                                onclick: move |_| per_app_mode.set(PerAppMode::Global),
                                "Global (All Apps)"
                            }
                            button {
                                class: format!("flex-1 py-2.5 rounded-lg text-sm font-semibold transition-all {}",
                                    if *per_app_mode.read() == PerAppMode::Whitelist { "bg-white dark:bg-white/10 text-primary shadow-sm" } else { "text-slate-500 dark:text-gray-400 hover:text-slate-700 dark:hover:text-white" }
                                ),
                                onclick: move |_| per_app_mode.set(PerAppMode::Whitelist),
                                "Whitelist (Only Selected)"
                            }
                            button {
                                class: format!("flex-1 py-2.5 rounded-lg text-sm font-semibold transition-all {}",
                                    if *per_app_mode.read() == PerAppMode::Blacklist { "bg-white dark:bg-white/10 text-primary shadow-sm" } else { "text-slate-500 dark:text-gray-400 hover:text-slate-700 dark:hover:text-white" }
                                ),
                                onclick: move |_| per_app_mode.set(PerAppMode::Blacklist),
                                "Blacklist (Exclude Selected)"
                            }
                        }
                        p { class: "text-xs text-slate-500 dark:text-gray-400 mt-3",
                            match *per_app_mode.read() {
                                PerAppMode::Global => "All apps use VPN tunnel",
                                PerAppMode::Whitelist => "Only selected apps use VPN, others bypass",
                                PerAppMode::Blacklist => "Selected apps bypass VPN, others use tunnel",
                            }
                        }
                    }
                }

                // Search and controls
                div {
                    class: "flex flex-col md:flex-row gap-3",
                    div {
                        class: "flex-1 relative group",
                        div { class: "absolute left-4 top-1/2 -translate-y-1/2 text-slate-400 dark:text-gray-500 pointer-events-none",
                            Icon { name: "search", class: "" }
                        }
                        input {
                            r#type: "text",
                            class: format!("{} pl-12 pr-4 py-3.5 w-full text-sm focus:ring-2 focus:ring-primary/50 outline-none transition-all", crate::components::ui::glass::CARD),
                            placeholder: "Search apps by name or package...",
                            value: "{search_query()}",
                            oninput: move |evt| search_query.set(evt.value()),
                        }
                    }
                    select {
                        class: format!("{} px-4 py-3.5 text-sm focus:ring-2 focus:ring-primary/50 outline-none transition-all", crate::components::ui::glass::CARD),
                        onchange: move |evt| {
                            sort_by.set(match evt.value().as_str() {
                                "data" => SortMode::DataUsage,
                                "date" => SortMode::InstallDate,
                                _ => SortMode::Name,
                            });
                        },
                        option { value: "name", "Sort by Name" }
                        option { value: "data", "Sort by Data Usage" }
                        option { value: "date", "Sort by Install Date" }
                    }
                }

                // Controls row
                div {
                    class: "flex items-center justify-between px-1",
                    div { class: "flex items-center gap-4 text-sm",
                        span { class: "font-semibold text-primary", "{selected_packages().len()} selected" }
                        span { class: "text-slate-500 dark:text-gray-500", "{filtered_apps().len()} apps" }
                    }
                    label {
                        class: "flex items-center gap-2 text-sm text-slate-500 dark:text-gray-400 cursor-pointer",
                        input {
                            r#type: "checkbox",
                            class: "rounded border-white/20 bg-white/5 text-primary focus:ring-primary/50",
                            checked: show_system_apps(),
                            onchange: move |evt| {
                                show_system_apps.set(evt.checked());
                                spawn(async move {
                                    loading.set(true);
                                    #[cfg(target_os = "android")]
                                    {
                                        if let Ok(app_list) = fetch_installed_apps_with_metadata(show_system_apps()).await {
                                            apps.set(app_list.clone());
                                            filtered_apps.set(app_list.clone());
                                            stats.set(calculate_stats(&app_list));
                                        }
                                    }
                                    loading.set(false);
                                });
                            },
                        }
                        "Show system apps"
                    }
                }

                // Virtualized app list
                if loading() {
                    div { class: "flex-1 flex items-center justify-center",
                        div { class: "flex flex-col items-center gap-3",
                            div { class: "size-8 border-2 border-primary border-t-transparent rounded-full animate-spin" }
                            span { class: "text-sm text-slate-500 dark:text-gray-400", "Loading apps..." }
                        }
                    }
                } else {
                    VirtualizedAppList {
                        apps: filtered_apps(),
                        selected_packages: selected_packages(),
                        on_toggle: move |pkg| toggle_app(pkg),
                    }
                }

                // Save button (fixed at bottom)
                div {
                    class: "fixed bottom-0 left-0 right-0 p-4 lg:p-6 bg-white/40 dark:bg-slate-900/40 backdrop-blur-2xl border-t border-white/40 dark:border-white/5 z-20",
                    PrimaryButton {
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
fn StatCard(label: String, value: String, icon: String) -> Element {
    rsx! {
        GlassCard {
            class: "p-4 flex items-center gap-3",
            children: rsx! {
                div { class: "p-2 rounded-lg bg-primary/10",
                    Icon { name: icon, class: "text-primary text-xl" }
                }
                div { class: "flex-1 min-w-0",
                    div { class: "text-xs text-slate-500 dark:text-gray-400 uppercase tracking-wide", "{label}" }
                    div { class: "text-lg font-bold text-slate-900 dark:text-white truncate", "{value}" }
                }
            }
        }
    }
}

#[component]
fn VirtualizedAppList(
    apps: Vec<AppMetadata>,
    selected_packages: Vec<String>,
    on_toggle: EventHandler<String>,
) -> Element {
    // Simple virtualization: render all for now, optimize later with windowing
    rsx! {
        div {
            class: "flex-1 overflow-y-auto rounded-2xl space-y-2 custom-scrollbar",
            for app in apps {
                AppCardExtended {
                    key: "{app.package_name}",
                    app: app.clone(),
                    selected: selected_packages.contains(&app.package_name),
                    on_toggle: move |pkg| on_toggle.call(pkg),
                }
            }
        }
    }
}

#[component]
fn AppCardExtended(app: AppMetadata, selected: bool, on_toggle: EventHandler<String>) -> Element {
    let card_class = if selected {
        format!(
            "{} ring-2 ring-primary/50 bg-primary/5",
            crate::components::ui::glass::CARD
        )
    } else {
        crate::components::ui::glass::CARD.to_string()
    };

    rsx! {
        div {
            class: format!("{} p-4 flex items-center gap-4 cursor-pointer transition-all hover:scale-[1.01] hover:bg-white/10 dark:hover:bg-white/5", card_class),
            onclick: move |_| on_toggle.call(app.package_name.clone()),

            // App icon
            div { class: "shrink-0 size-14 rounded-xl bg-gradient-to-br from-slate-100 to-slate-200 dark:from-slate-700 dark:to-slate-800 flex items-center justify-center overflow-hidden border border-white/20 dark:border-white/10",
                if let Some(icon_data) = &app.icon_base64 {
                    img {
                        class: "size-12 rounded-lg object-cover",
                        src: "data:image/png;base64,{icon_data}",
                        alt: "{app.app_name}",
                    }
                } else {
                    span { class: "text-3xl", "📱" }
                }
            }

            // App info
            div { class: "flex-1 min-w-0",
                div { class: "font-semibold text-slate-900 dark:text-white truncate", "{app.app_name}" }
                div { class: "text-xs text-slate-500 dark:text-gray-400 font-mono truncate", "{app.package_name}" }
                div { class: "flex items-center gap-3 mt-1.5",
                    if app.is_system_app {
                        span { class: "text-[10px] px-2 py-0.5 rounded-full bg-amber-500/20 text-amber-600 dark:text-amber-400 font-medium", "SYSTEM" }
                    }
                    span { class: "text-[10px] text-slate-500 dark:text-gray-500", "UID: {app.uid}" }
                    if app.data_usage_bytes > 0 {
                        span { class: "text-[10px] text-slate-500 dark:text-gray-500", "{format_bytes(app.data_usage_bytes)}" }
                    }
                }
            }

            // Checkbox
            div { class: "shrink-0",
                div {
                    class: format!("size-6 rounded-lg border-2 flex items-center justify-center transition-all {}",
                        if selected { "bg-primary border-primary" } else { "border-slate-300 dark:border-white/20 bg-white/10" }
                    ),
                    onclick: move |evt| evt.stop_propagation(),
                    if selected {
                        Icon { name: "check", class: "text-white text-sm" }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortMode {
    Name,
    DataUsage,
    InstallDate,
}

#[derive(Debug, Clone, Default)]
struct AppStats {
    total_apps: usize,
    system_apps: usize,
    total_data_usage: u64,
}

fn calculate_stats(apps: &[AppMetadata]) -> AppStats {
    AppStats {
        total_apps: apps.len(),
        system_apps: apps.iter().filter(|a| a.is_system_app).count(),
        total_data_usage: apps.iter().map(|a| a.data_usage_bytes).sum(),
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

// Database operations
#[cfg(not(target_arch = "wasm32"))]
async fn load_per_app_rules() -> Result<Vec<PerAppRule>, String> {
    use crate::db;

    let db = db::get_db().await.map_err(|e| e.to_string())?;
    let rules: Vec<PerAppRule> = db
        .select("per_app_rules")
        .await
        .map_err(|e| format!("Failed to query rules: {}", e))?;

    Ok(rules)
}

#[cfg(not(target_arch = "wasm32"))]
async fn save_per_app_rules_to_db(
    mode: PerAppMode,
    packages: Vec<String>,
    apps: Vec<AppMetadata>,
) -> Result<(), String> {
    use crate::db;

    let db = db::get_db().await.map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Delete all existing rules
    let _: Vec<PerAppRule> = db
        .delete("per_app_rules")
        .await
        .map_err(|e| format!("Failed to delete old rules: {}", e))?;

    // Create new rules for selected packages
    for package in packages {
        if let Some(app) = apps.iter().find(|a| a.package_name == package) {
            let rule = PerAppRule {
                id: uuid::Uuid::new_v4().to_string(),
                package_name: app.package_name.clone(),
                app_name: app.app_name.clone(),
                uid: app.uid,
                enabled: true,
                mode,
                created_at: now,
                updated_at: now,
            };

            let _: Option<PerAppRule> = db
                .create(("per_app_rules", rule.id.as_str()))
                .content(rule)
                .await
                .map_err(|e| format!("Failed to create rule: {}", e))?;
        }
    }

    Ok(())
}

#[cfg(target_arch = "wasm32")]
async fn save_per_app_rules_to_db(
    _mode: PerAppMode,
    _packages: Vec<String>,
    _apps: Vec<AppMetadata>,
) -> Result<(), String> {
    Ok(())
}

// Platform-specific app fetching
#[cfg(target_os = "android")]
async fn fetch_installed_apps_with_metadata(
    include_system: bool,
) -> Result<Vec<AppMetadata>, String> {
    // JNI bridge implementation would go here
    // For now, return mock data
    Ok(generate_mock_apps_extended())
}

fn generate_mock_apps_extended() -> Vec<AppMetadata> {
    vec![
        AppMetadata {
            package_name: "com.android.chrome".to_string(),
            app_name: "Chrome".to_string(),
            uid: 10001,
            is_system_app: false,
            version_name: Some("120.0.6099.144".to_string()),
            version_code: Some(609914400),
            install_time: Some(1704067200),
            update_time: Some(1706745600),
            icon_base64: None,
            data_usage_bytes: 524288000, // 500 MB
        },
        AppMetadata {
            package_name: "com.whatsapp".to_string(),
            app_name: "WhatsApp".to_string(),
            uid: 10002,
            is_system_app: false,
            version_name: Some("2.24.1.78".to_string()),
            version_code: Some(242478),
            install_time: Some(1704067200),
            update_time: Some(1706745600),
            icon_base64: None,
            data_usage_bytes: 1073741824, // 1 GB
        },
        AppMetadata {
            package_name: "org.telegram.messenger".to_string(),
            app_name: "Telegram".to_string(),
            uid: 10003,
            is_system_app: false,
            version_name: Some("10.5.2".to_string()),
            version_code: Some(38950),
            install_time: Some(1704067200),
            update_time: Some(1706745600),
            icon_base64: None,
            data_usage_bytes: 314572800, // 300 MB
        },
        AppMetadata {
            package_name: "com.android.settings".to_string(),
            app_name: "Settings".to_string(),
            uid: 1000,
            is_system_app: true,
            version_name: Some("14".to_string()),
            version_code: Some(34),
            install_time: Some(1704067200),
            update_time: Some(1706745600),
            icon_base64: None,
            data_usage_bytes: 10485760, // 10 MB
        },
        AppMetadata {
            package_name: "com.google.android.gms".to_string(),
            app_name: "Google Play Services".to_string(),
            uid: 10004,
            is_system_app: true,
            version_name: Some("24.01.14".to_string()),
            version_code: Some(240114000),
            install_time: Some(1704067200),
            update_time: Some(1706745600),
            icon_base64: None,
            data_usage_bytes: 209715200, // 200 MB
        },
    ]
}
