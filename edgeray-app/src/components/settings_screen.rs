//! Settings Screen Component
//!
//! Renders the main application settings UI.

use crate::components::gestures::{Ripple, use_back_handler};
use crate::components::ui::Icon;
use crate::components::ui::glass;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SettingsScreenProps {
    on_done: EventHandler<()>,
    on_routing_rules: EventHandler<()>,
    on_per_app_proxy: EventHandler<()>,
    on_assets: EventHandler<()>,
    on_logs: EventHandler<()>,
    on_firewall: EventHandler<()>,
    on_dns_tuning: EventHandler<()>,
    on_flow_tuning: EventHandler<()>,
    on_stack_monitor: EventHandler<()>,
    on_repo_click: EventHandler<()>,
    on_policy_click: EventHandler<()>,
    on_advanced_tuning: EventHandler<()>,
}

#[component]
pub fn SettingsScreen(props: SettingsScreenProps) -> Element {
    let mut theme = use_signal(|| "system".to_string());
    let mut start_on_boot = use_signal(|| false);
    let mut allow_insecure = use_signal(|| false);
    let mut mode = use_signal(|| "rule".to_string());

    // Advanced State
    let mut lock_vpn = use_signal(|| false);
    let mut dns_hijacking = use_signal(|| true);
    let mut sniffing = use_signal(|| true);
    let mut doh_url = use_signal(|| "https://1.1.1.1/dns-query".to_string());
    let mut auto_update = use_signal(|| false);
    let mut active_core = use_signal(|| "rustray".to_string());
    let mut xray_ver = use_signal(|| "v25.12.8".to_string());
    let rustray_ver = use_signal(|| "v0.1.0".to_string());
    let mut sb_ver = use_signal(|| "v1.12.14".to_string());

    // Load settings
    let settings_resource =
        use_resource(move || async move { crate::db::get_settings().await.unwrap_or_default() });

    use_effect(move || {
        if let Some(s) = settings_resource.read().as_ref() {
            theme.set(s.theme.clone());
            start_on_boot.set(s.start_on_boot);
            allow_insecure.set(s.allow_insecure);
            mode.set(s.routing_mode.clone());
            lock_vpn.set(s.lock_vpn);
            dns_hijacking.set(s.dns_hijacking);
            sniffing.set(s.sniffing);
            doh_url.set(s.doh_url.clone());
            auto_update.set(s.auto_update);
            active_core.set(s.active_core.clone());
            xray_ver.set(s.rustray_version.clone());
            sb_ver.set(s.singbox_version.clone());
        }
    });

    // Handle back button on mobile
    let on_done_handler = props.on_done.clone();
    use_back_handler(move || {
        on_done_handler.call(());
    });

    let save_settings = move || {
        let current_ui_mode = settings_resource
            .read()
            .as_ref()
            .map(|s| s.ui_mode.clone())
            .unwrap_or_else(|| "simple".to_string());
        let new_settings = crate::models::AppSettings {
            theme: theme.read().clone(),
            ui_mode: current_ui_mode,
            start_on_boot: *start_on_boot.read(),
            allow_insecure: *allow_insecure.read(),
            routing_mode: mode.read().clone(),
            sniffing: *sniffing.read(),
            dns_hijacking: *dns_hijacking.read(),
            lock_vpn: *lock_vpn.read(),
            doh_url: doh_url.read().clone(),
            auto_update: *auto_update.read(),
            active_core: active_core.read().clone(),
            rustray_version: xray_ver.read().clone(),
            singbox_version: sb_ver.read().clone(),
            fec_data_shards: 10,
            fec_parities: 3,
            mqtt_heartbeat_interval: 30,
            fingerprint_rotation_interval: 3600,
            fakedns: crate::models::FakeDnsConfig::default(),
        };
        spawn(async move {
            let _ = crate::db::save_settings(new_settings).await;
        });
    };

    rsx! {
        div {
            class: "w-full max-w-md lg:max-w-2xl mx-auto flex flex-col pb-10 lg:pb-6 z-10 relative",


            crate::components::ui::PageHeader {
                title: "Settings".to_string(),
                right_action: Some(rsx! {
                    button {
                        class: "text-primary font-semibold text-base hover:opacity-80 transition-opacity",
                        onclick: move |_| props.on_done.call(()),
                        "Done"
                    }
                })
            }

            div {
                class: "px-4 lg:px-6 py-6 space-y-6",

                // General Section
                SettingsSection {
                    title: "General",
                    div {
                        class: "relative p-3 border-b border-white/20 dark:border-white/5",
                        div {
                            class: "flex h-9 w-full items-center justify-center rounded-xl bg-gray-200/50 dark:bg-black/20 p-1 backdrop-blur-md shadow-inner",
                            ThemeButton { label: "System", theme_mode: "system".to_string(), current_theme: theme, on_change: save_settings }
                            ThemeButton { label: "Dark", theme_mode: "dark".to_string(), current_theme: theme, on_change: save_settings }
                            ThemeButton { label: "Light", theme_mode: "light".to_string(), current_theme: theme, on_change: save_settings }
                        }
                    }
                    ToggleItem { label: "Start on Boot".to_string(), checked: start_on_boot, on_change: save_settings }
                    ToggleItem { label: "Allow Insecure".to_string(), checked: allow_insecure, on_change: save_settings }
                    SettingItem {
                        label: "Battery Optimization".to_string(),
                        right_element: rsx! {
                            button {
                                class: "px-3 py-1 bg-white/5 hover:bg-white/10 rounded-lg text-xs transition-colors text-white",
                                onclick: move |_| {
                                    // Placeholder for now as tauri_sys might not be present
                                    log::info!("User clicked Battery Optimization Disable");
                                },
                                "Disable"
                            }
                        },
                        last_item: true
                    }
                }

                // Connection Section
                SettingsSection {
                    title: "Connection",
                    div {
                        class: "relative p-3 border-b border-white/20 dark:border-white/5",
                        div {
                            class: "flex h-9 w-full items-center justify-center rounded-xl bg-gray-200/50 dark:bg-black/20 p-1 backdrop-blur-md shadow-inner",
                             ModeButton { label: "Rule", mode: "rule".to_string(), current_mode: mode, on_change: save_settings }
                             ModeButton { label: "Global", mode: "global".to_string(), current_mode: mode, on_change: save_settings }
                             ModeButton { label: "Direct", mode: "direct".to_string(), current_mode: mode, on_change: save_settings }
                        }
                    }
                    NavigateItem { label: "Routing Rules".to_string(), value: "Default".to_string(), onclick: move |_| props.on_routing_rules.call(()) }
                    NavigateItem { label: "Routing Assets".to_string(), value: "v2.0".to_string(), onclick: move |_| props.on_assets.call(()) }
                    NavigateItem { label: "Per-App Proxy".to_string(), value: "Off".to_string(), onclick: move |_| props.on_per_app_proxy.call(()) }
                    ToggleItem { label: "Sniffing".to_string(), sublabel: Some("Override destination IP".to_string()), checked: sniffing, on_change: save_settings }

                    // Advanced Connection Settings
                    ToggleItem { label: "DNS Hijacking".to_string(), sublabel: Some("Prevent DNS Leaks".to_string()), checked: dns_hijacking, on_change: save_settings }
                    ToggleItem { label: "Lock VPN".to_string(), sublabel: Some("Block traffic on failure".to_string()), checked: lock_vpn, on_change: save_settings }
                    NavigateItem { label: "DNS & Sniffing".to_string(), value: "Advanced".to_string(), onclick: move |_| props.on_dns_tuning.call(()) }
                    NavigateItem { label: "Flow-J Expert Tuning".to_string(), value: "FEC/CC".to_string(), onclick: move |_| props.on_flow_tuning.call(()) }


                    if *dns_hijacking.read() {
                        div {
                            class: "px-4 py-3.5 border-t border-white/5",
                            span { class: "text-xs text-gray-400 block mb-1", "DoH Endpoint" }
                             input {
                                class: "w-full bg-transparent text-sm text-gray-200 placeholder-gray-500 focus:outline-none font-mono",
                                value: "{doh_url}",
                                oninput: move |e| { doh_url.set(e.value()); save_settings(); }
                            }
                        }
                    }
                }

                // Subscription & Support
                 SettingsSection {
                    title: "Subscription & Support",
                    ToggleItem { label: "Auto Update".to_string(), sublabel: Some("Update subscription daily".to_string()), checked: auto_update, on_change: save_settings }
                    NavigateItem { label: "View Logs".to_string(), value: "Debug".to_string(), onclick: move |_| props.on_logs.call(()) }
                }

                // Core Engine section
                SettingsSection {
                    title: "Core Engine",
                    div {
                        class: "relative p-3",
                        div {

                            class: "flex h-auto w-full flex-col gap-2 rounded-xl bg-black/40 p-2 backdrop-blur-md shadow-inner border border-white/5",

                            // Rustray Option (Main)
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-white/5 transition-colors",
                                label {
                                    class: "flex items-center gap-3 cursor-pointer grow",
                                    input {
                                        "type": "radio",
                                        name: "core_select",
                                        value: "rustray",
                                        checked: *active_core.read() == "rustray",
                                        onchange: move |_| { active_core.set("rustray".to_string()); save_settings(); },
                                        class: "accent-primary w-4 h-4"
                                    }
                                    div {
                                        class: "flex flex-col",
                                        span { class: "text-sm font-bold text-white", "Rustray-core (Recommended)" }
                                        span { class: "text-[10px] text-gray-500", "{rustray_ver}" }
                                    }
                                }
                                button {
                                    class: "px-2 py-1 text-[10px] font-bold text-primary bg-primary/10 rounded hover:bg-primary/20",
                                    onclick: move |_| {}, // Rustray is embedded, no update needed usually
                                    "Built-in"
                                }
                            }

                            // Xray Option
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-white/5 transition-colors opacity-70",
                                label {
                                    class: "flex items-center gap-3 cursor-pointer grow",
                                    input {
                                        "type": "radio",
                                        name: "core_select",
                                        value: "xray",
                                        checked: *active_core.read() == "xray",
                                        onchange: move |_| { active_core.set("xray".to_string()); save_settings(); },
                                        class: "accent-primary w-4 h-4"
                                    }
                                    div {
                                        class: "flex flex-col",
                                        span { class: "text-sm font-bold text-white", "Xray-core" }
                                        span { class: "text-[10px] text-gray-500", "{xray_ver}" }
                                    }
                                }
                                button {
                                    class: "px-2 py-1 text-[10px] font-bold text-primary bg-primary/10 rounded hover:bg-primary/20",
                                    onclick: move |_| {
                                        spawn(async move {
                                            use crate::drivers::DriverFactory;
                                            let driver = DriverFactory::local();
                                            match driver.update_core("xray".to_string()).await {
                                                Ok(v) => {
                                                    xray_ver.set(v);
                                                    let mut s = settings_resource.read().clone().unwrap_or_default();
                                                    s.rustray_version = xray_ver.read().clone();
                                                    let _ = crate::db::save_settings(s).await;
                                                },
                                                Err(e) => log::error!("Failed to update Xray: {}", e),
                                            }
                                        });
                                    },
                                    "Update"
                                }
                            }

                            // Sing-box Option
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-white/5 transition-colors opacity-70",
                                label {
                                    class: "flex items-center gap-3 cursor-pointer grow",
                                    input {
                                        "type": "radio",
                                        name: "core_select",
                                        value: "sing-box",
                                        checked: *active_core.read() == "sing-box",
                                        onchange: move |_| { active_core.set("sing-box".to_string()); save_settings(); },
                                        class: "accent-primary w-4 h-4"
                                    }
                                    div {
                                        class: "flex flex-col",
                                        span { class: "text-sm font-bold text-white", "Sing-box" }
                                        span { class: "text-[10px] text-gray-500", "{sb_ver}" }
                                    }
                                }
                                button {
                                    class: "px-2 py-1 text-[10px] font-bold text-primary bg-primary/10 rounded hover:bg-primary/20",
                                    onclick: move |_| {
                                        spawn(async move {
                                            use crate::drivers::DriverFactory;
                                            let driver = DriverFactory::local();
                                            match driver.update_core("sing-box".to_string()).await {
                                                Ok(v) => {
                                                    sb_ver.set(v);
                                                    let mut s = settings_resource.read().clone().unwrap_or_default();
                                                    s.singbox_version = sb_ver.read().clone();
                                                    let _ = crate::db::save_settings(s).await;
                                                },
                                                Err(e) => log::error!("Failed to update Sing-box: {}", e),
                                            }
                                        });
                                    },
                                    "Update"
                                }
                            }
                        }
                    }
                    div {
                        class: "px-4 pb-3 flex flex-col gap-3",
                        NavigateItem {
                            label: "Advanced Core Tuning".to_string(),
                            value: "FEC/FakeDNS".to_string(),
                            onclick: move |_| props.on_advanced_tuning.call(())
                        }
                        p { class: "text-[10px] text-gray-500 dark:text-gray-400 text-center italic", "Restart required to apply core changes" }
                    }
                }


                // About section
                SettingsSection {
                    title: "About",
                    div {
                        class: "relative flex items-center justify-between px-4 py-3.5 border-b border-white/5 bg-white/[0.02]",
                        span { class: "text-white text-base font-normal", "Version" }
                        span { class: "text-gray-400 text-base font-bold", "v1.4.2 (Build 204)" }
                    }

                    SettingItem {
                        label: "Check for Updates".to_string(),
                        right_element: rsx! {
                            button {
                                class: "px-3 py-1 bg-white/5 hover:bg-white/10 rounded-lg text-xs transition-colors text-white",
                                onclick: move |_| { spawn(async move { log::info!("Checking updates..."); }); },
                                "Check"
                            }
                        }
                    }
                    SettingItem {
                         label: "Self Diagnostic".to_string(),
                         right_element: rsx! { span { class: "text-xs text-green-500 font-mono", "Healthy" } }
                    }
                    NavigateItem { label: "Firewall Rules".to_string(), value: "nftables".to_string(), onclick: move |_| props.on_firewall.call(()) }
                    NavigateItem { label: "Userspace Stack".to_string(), value: "Stats".to_string(), onclick: move |_| props.on_stack_monitor.call(()) }

                    ExternalLinkItem { label: "Github Repository".to_string(), icon: "code".to_string(), onclick: move |_| props.on_repo_click.call(()) }

                    ExternalLinkItem { label: "Privacy Policy".to_string(), icon: "policy".to_string(), onclick: move |_| props.on_policy_click.call(()), last_item: true }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SettingsSectionProps {
    title: String,
    children: Element,
}
#[component]
fn SettingsSection(props: SettingsSectionProps) -> Element {
    rsx! {
        div { class: "space-y-2",
            crate::components::ui::SectionHeader { title: props.title }
            div {
                class: "{glass::PANEL} rounded-3xl overflow-hidden divide-y divide-white/5",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ThemeButtonProps {
    label: String,
    theme_mode: String,
    current_theme: Signal<String>,
    on_change: EventHandler<()>,
}
#[component]
fn ThemeButton(mut props: ThemeButtonProps) -> Element {
    let is_checked = *props.current_theme.read() == props.theme_mode;
    rsx! {
        label {
            class: "group flex cursor-pointer h-full grow items-center justify-center overflow-hidden rounded-[0.5rem] has-[:checked]:bg-white/80 dark:has-[:checked]:bg-white/10 has-[:checked]:shadow-sm has-[:checked]:backdrop-blur-sm transition-all duration-300",
            span {
                class: "text-xs font-medium text-gray-600 dark:text-gray-400 group-has-[:checked]:text-gray-900 dark:group-has-[:checked]:text-white transition-colors",
                "{props.label}"
            }
            input {
                class: "hidden",
                "type": "radio",
                name: "theme_select",
                value: "{props.theme_mode}",
                checked: is_checked,
                onchange: move |_| { props.current_theme.set(props.theme_mode.clone()); props.on_change.call(()); },
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ModeButtonProps {
    label: String,
    mode: String,
    current_mode: Signal<String>,
    on_change: EventHandler<()>,
}
#[component]
fn ModeButton(mut props: ModeButtonProps) -> Element {
    let is_checked = *props.current_mode.read() == props.mode;
    rsx! {
        label {
            class: "group flex cursor-pointer h-full grow items-center justify-center overflow-hidden rounded-[0.5rem] has-[:checked]:bg-white/80 dark:has-[:checked]:bg-white/10 has-[:checked]:shadow-sm has-[:checked]:backdrop-blur-sm transition-all duration-300",
            span {
                class: "text-xs font-medium text-gray-600 dark:text-gray-400 group-has-[:checked]:text-gray-900 dark:group-has-[:checked]:text-white transition-colors",
                "{props.label}"
            }
            input {
                class: "hidden",
                "type": "radio",
                name: "mode_select",
                value: "{props.mode}",
                checked: is_checked,
                onchange: move |_| { props.current_mode.set(props.mode.clone()); props.on_change.call(()); },
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ToggleItemProps {
    label: String,
    #[props(default = None)]
    sublabel: Option<String>,
    checked: Signal<bool>,
    on_change: EventHandler<()>,
    #[props(default = false)]
    last_item: bool,
}
#[component]
fn ToggleItem(mut props: ToggleItemProps) -> Element {
    let is_checked = *props.checked.read();
    rsx! {
        div {
            class: "p-5 flex items-center justify-between hover:bg-white/10 transition-colors group cursor-pointer",
            onclick: move |_| {
                let current = *props.checked.read();
                props.checked.set(!current);
                props.on_change.call(());
            },
            div {
                class: "flex items-center space-x-4",
                div {
                    if let Some(sublabel) = &props.sublabel {
                        div {
                            span { class: "font-semibold text-base text-gray-200 group-hover:text-white transition-colors", "{props.label}" }
                            p { class: "text-[10px] text-gray-500 font-mono mt-0.5", "{sublabel}" }
                        }
                    } else {
                        span { class: "font-semibold text-base text-gray-200 group-hover:text-white transition-colors", "{props.label}" }
                    }
                }
            }
            // Glowing toggle switch - purple when active
            button {
                class: format!(
                    "relative w-12 h-7 rounded-full transition-all duration-300 {}",
                    if is_checked { "bg-cyber shadow-cyber" } else { "bg-white/10" }
                ),
                div {
                    class: format!(
                        "absolute top-1 size-5 bg-white rounded-full transition-all duration-300 shadow-sm {}",
                        if is_checked { "left-6" } else { "left-1" }
                    )
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NavigateItemProps {
    label: String,
    value: String,
    onclick: EventHandler<()>,
}

#[component]
fn NavigateItem(props: NavigateItemProps) -> Element {
    rsx! {
        Ripple {
            class: "w-full border-b border-white/20 dark:border-white/5 hover:bg-white/10 transition-colors".to_string(),
            onclick: Some(EventHandler::new(move |_| props.onclick.call(()))),
            div {
                class: "flex items-center justify-between px-4 py-3.5 text-left group",
                span { class: "text-gray-200 text-base font-semibold select-none group-hover:text-primary transition-colors", "{props.label}" }
                div {
                    class: "flex items-center gap-2",
                    span { class: "text-sm text-gray-400 select-none", "{props.value}" }
                    Icon { name: "chevron_right".to_string(), class: "text-gray-500 group-hover:text-primary transition-colors text-[20px]".to_string() }
                }
            }
        }
    }
}
#[derive(Props, Clone, PartialEq)]
struct ExternalLinkItemProps {
    label: String,
    icon: String,
    onclick: EventHandler<()>,
    #[props(default = false)]
    last_item: bool,
}
#[component]
fn ExternalLinkItem(props: ExternalLinkItemProps) -> Element {
    rsx! {
        Ripple {
            class: format!("relative block hover:bg-white/10 transition-colors group {}", if !props.last_item { "border-b border-white/20 dark:border-white/5" } else { "" }),
            onclick: Some(EventHandler::new(move |_| props.onclick.call(()))),
            div {
                class: "flex items-center justify-between px-4 py-3.5 cursor-pointer",
                div {
                    class: "flex items-center gap-3",
                    Icon { name: props.icon.clone(), class: "text-gray-400 group-hover:text-primary transition-colors".to_string() }
                    span { class: "text-gray-200 text-base font-semibold select-none group-hover:text-white transition-colors", "{props.label}" }
                }
                Icon { name: "open_in_new".to_string(), class: "text-gray-400 group-hover:text-primary transition-colors text-[20px]".to_string() }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SettingItemProps {
    label: String,
    #[props(default = None)]
    sublabel: Option<String>,
    #[props(default = None)]
    icon: Option<String>,
    #[props(default)]
    onclick: Option<EventHandler<()>>,
    #[props(default = rsx! { })]
    right_element: Element,
    #[props(default = false)]
    last_item: bool,
}

#[component]
fn SettingItem(props: SettingItemProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: format!("relative flex items-center justify-between px-4 py-3.5 transition-colors group {}", if !props.last_item { "border-b border-white/20 dark:border-white/5" } else { "" }),
            div {
                class: "flex items-center gap-3",
                if let Some(icon) = &props.icon {
                    Icon { name: icon.clone(), class: "text-gray-500 dark:text-gray-400".to_string() }
                }
                div {
                    class: "flex flex-col",
                    span { class: "text-gray-200 text-base font-semibold", "{props.label}" }
                    if let Some(sub) = &props.sublabel {
                        span { class: "text-xs text-gray-500", "{sub}" }
                    }
                }
            }

            div {
                class: "flex items-center gap-2",
                 {props.right_element}
                 if onclick.is_some() {
                     button {
                         class: "absolute inset-0 w-full h-full cursor-pointer bg-transparent",
                         onclick: move |_| if let Some(h) = &onclick { h.call(()) }
                     }
                 }
            }
        }
    }
}
