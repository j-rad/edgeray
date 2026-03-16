use crate::components::ui::Icon;
use crate::db;
use crate::models::Subscription;
use crate::services;
use crate::ui::components::import_wizard::ImportWizard;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SubscriptionViewProps {
    pub on_close: EventHandler<()>,
}

#[component]
pub fn SubscriptionView(props: SubscriptionViewProps) -> Element {
    let mut subscriptions =
        use_resource(move || async move { db::list_subscriptions().await.unwrap_or_default() });

    let mut syncing_id = use_signal(|| Option::<String>::None);
    let mut sync_progress = use_signal(|| 0.0f32);
    let mut show_import_wizard = use_signal(|| false);
    let mut editing_subscription = use_signal(|| Option::<Subscription>::None);

    let on_sync = move |id: String| {
        syncing_id.set(Some(id.clone()));
        sync_progress.set(0.1);

        spawn(async move {
            // Simulated granular progress while the service works
            for i in 2..=9 {
                sync_progress.set(i as f32 / 10.0);
                #[cfg(not(target_arch = "wasm32"))]
                tokio::time::sleep(std::time::Duration::from_millis(150)).await;
            }

            let result =
                services::subscription_manager::SubscriptionManager::sync_subscription(&id).await;

            match result {
                Ok(count) => log::info!("Synced {} nodes for {}", count, id),
                Err(e) => log::error!("Sync failed: {}", e),
            }

            // Finalize
            sync_progress.set(1.0);
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;

            syncing_id.set(None);
            sync_progress.set(0.0);

            // Refresh list
            subscriptions.restart();
        });
    };

    let subs_list = subscriptions.read().clone();

    // Calculate stats from real data
    let (total_nodes, active_subs, failed_syncs) = subs_list
        .as_ref()
        .map(|list| {
            let total: usize = list.iter().map(|s| s.node_count).sum();
            let active = list.iter().filter(|s| s.enabled).count();
            let failed = list
                .iter()
                .filter(|s| !s.enabled && s.last_update.is_some())
                .count();
            (total.to_string(), active.to_string(), failed.to_string())
        })
        .unwrap_or_else(|| ("0".to_string(), "0".to_string(), "0".to_string()));

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-6xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // Header - Specialized for technical source management
            header {
                class: "flex items-center justify-between mb-10",
                div {
                    h2 { class: "text-3xl font-black text-white tracking-tight flex items-center gap-3",
                        Icon { name: "terminal".to_string(), class: "text-primary text-[32px]".to_string() }
                        "Node Sources"
                    }
                    p { class: "text-sm text-slate-400 mt-1", "Manage server subscriptions and technical configuration endpoints" }
                }
                button {
                    class: "group flex items-center justify-center rounded-full p-2.5 bg-white/5 hover:bg-white/10 transition-all active:scale-95 border border-white/10",
                    onclick: move |_| props.on_close.call(()),
                    Icon { name: "close".to_string(), class: "text-white text-[24px]".to_string() }
                }
            }

            // Technical Stats Grid
            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-6 mb-10",
                TechStatCard { label: "TOTAL NODES", value: total_nodes, icon: "dns", color: "text-blue-400" }
                TechStatCard { label: "ACTIVE SOURCES", value: active_subs, icon: "terminal", color: "text-emerald-400" }
                TechStatCard { label: "FAILED LINKS", value: failed_syncs, icon: "terminal", color: "text-rose-400" }
            }

            // Sources Management Area
            div {
                class: "bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-xl",

                // Table Header / Actions
                div {
                    class: "flex items-center justify-between p-6 border-b border-white/10",
                    div {
                        h3 { class: "text-sm font-black uppercase tracking-[0.2em] text-slate-500", "Config Registry" }
                    }
                    div {
                        class: "flex gap-3",
                        Button {
                            class: "px-4 py-2 text-xs font-bold uppercase tracking-widest bg-white/5 hover:bg-white/10 text-slate-300 rounded-xl border border-white/5",
                            onclick: move |_| subscriptions.restart(),
                            Icon { name: "refresh".to_string(), class: "text-[16px]".to_string() }
                            span { class: "ml-2", "Reload" }
                        }
                        Button {
                            variant: Some("primary".to_string()),
                            class: "px-4 py-2 text-xs font-bold uppercase tracking-widest rounded-xl",
                            onclick: move |_| show_import_wizard.set(true),
                            Icon { name: "add".to_string(), class: "text-[16px]".to_string() }
                            span { class: "ml-2", "Import Source" }
                        }
                    }
                }

                // Subscription Table
                if let Some(list) = subs_list.clone() {
                    if list.is_empty() {
                         div {
                            class: "flex flex-col items-center justify-center p-24 text-center",
                            div {
                                class: "p-6 rounded-full bg-primary/5 mb-6",
                                Icon { name: "cloud_off".to_string(), class: "text-[64px] text-slate-600".to_string() }
                            }
                            h4 { class: "text-xl font-bold text-white mb-2", "No Sources Configured" }
                            p { class: "text-slate-400 max-w-sm mb-8", "Add your first server subscription URL to begin propagating nodes to the mesh." }
                            Button {
                                variant: Some("primary".to_string()),
                                class: "px-8 py-3 rounded-2xl",
                                onclick: move |_| show_import_wizard.set(true),
                                "Initialize First Source"
                            }
                        }
                    } else {
                        div {
                            class: "overflow-x-auto",
                            table {
                                class: "w-full text-left border-collapse",
                                thead {
                                    class: "bg-white/5",
                                    tr {
                                        th { class: "px-6 py-4 text-[10px] font-black uppercase tracking-widest text-slate-500", "Source Name" }
                                        th { class: "px-6 py-4 text-[10px] font-black uppercase tracking-widest text-slate-500", "Endpoint URL" }
                                        th { class: "px-6 py-4 text-[10px] font-black uppercase tracking-widest text-slate-500", "Node Count" }
                                        th { class: "px-6 py-4 text-[10px] font-black uppercase tracking-widest text-slate-500", "Sync Status" }
                                        th { class: "px-6 py-4 text-[10px] font-black uppercase tracking-widest text-slate-500 text-right", "Actions" }
                                    }
                                }
                                tbody {
                                    for sub in list {
                                        SubscriptionRow {
                                            key: "{sub.id}",
                                            sub: sub.clone(),
                                            is_syncing: syncing_id.read().as_ref() == Some(&sub.id),
                                            sync_progress: *sync_progress.read(),
                                            on_sync: {
                                                let id = sub.id.clone();
                                                let mut on_sync = on_sync;
                                                move |_| on_sync(id.clone())
                                            },
                                            on_edit: {
                                                let sub_clone = sub.clone();
                                                move |_| editing_subscription.set(Some(sub_clone.clone()))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "flex items-center justify-center p-20",
                        div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-primary" }
                    }
                }
            }
        }

        // Import Wizard Modal
        if *show_import_wizard.read() {
            ImportWizard {
                on_import: move |links: Vec<String>| {
                    spawn(async move {
                        for url in links {
                            let mut sub = Subscription::default();
                            sub.name = format!("Imported Source ({})", &url[..std::cmp::min(10, url.len())]);
                            sub.urls = vec![url];
                            if let Err(e) = db::save_subscription(sub).await {
                                log::error!("Failed to save subscription: {}", e);
                            }
                        }
                        subscriptions.restart();
                    });
                    show_import_wizard.set(false);
                },
                on_close: move |_| show_import_wizard.set(false)
            }
        }

        // Edit Modal
        if let Some(sub) = editing_subscription.read().clone() {
            EditSourceModal {
                subscription: sub,
                on_save: move |updated_sub: Subscription| {
                    spawn(async move {
                        if let Err(e) = db::save_subscription(updated_sub).await {
                            log::error!("Failed to update subscription: {}", e);
                        }
                        subscriptions.restart();
                    });
                    editing_subscription.set(None);
                },
                on_close: move |_| editing_subscription.set(None)
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SubscriptionRowProps {
    sub: Subscription,
    is_syncing: bool,
    sync_progress: f32,
    on_sync: EventHandler<()>,
    on_edit: EventHandler<()>,
}

#[component]
fn SubscriptionRow(props: SubscriptionRowProps) -> Element {
    let last_update = props
        .sub
        .last_update
        .map(|_| "Updated recently".to_string())
        .unwrap_or_else(|| "Never synced".to_string());

    let status_class = if props.sub.enabled {
        "bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]"
    } else {
        "bg-slate-700"
    };

    rsx! {
        tr {
            class: "border-t border-white/5 hover:bg-white/5 transition-colors group",
            td {
                class: "px-6 py-5",
                div {
                    class: "flex items-center gap-3",
                    div { class: "w-2 h-2 rounded-full {status_class}" }
                    span { class: "font-bold text-white", "{props.sub.name}" }
                }
            }
            td {
                class: "px-6 py-5",
                span { class: "text-sm text-slate-400 font-mono opacity-60 group-hover:opacity-100 transition-opacity",
                    "{props.sub.urls.first().cloned().unwrap_or_default()}"
                }
            }
            td {
                class: "px-6 py-5",
                div {
                    class: "flex items-center gap-2",
                    Icon { name: "layers".to_string(), class: "text-slate-500 text-[16px]".to_string() }
                    span { class: "text-sm font-bold text-slate-300", "{props.sub.node_count}" }
                }
            }
            td {
                class: "px-6 py-5",
                div {
                    class: "flex flex-col gap-1.5",
                    if props.is_syncing {
                        div {
                            class: "w-32 h-1.5 bg-white/10 rounded-full overflow-hidden",
                            div {
                                class: "h-full bg-primary transition-all duration-300",
                                style: "width: {props.sync_progress * 100.0}%"
                            }
                        }
                        span { class: "text-[10px] font-bold text-primary flex items-center gap-1",
                            Icon { name: "sync".to_string(), class: "animate-spin text-[10px]".to_string() }
                            "PROPAGATING MESH..."
                        }
                    } else {
                        span { class: "text-xs font-bold text-slate-500", "{last_update}" }
                        if props.sub.node_count == 0 {
                            span { class: "text-[9px] text-rose-400 font-bold flex items-center gap-1",
                                Icon { name: "warning".to_string(), class: "text-[12px]".to_string() }
                                "EMPTY SOURCE"
                            }
                        }
                    }
                }
            }
            td {
                class: "px-6 py-5 text-right",
                div {
                    class: "flex items-center justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity",
                    button {
                        class: "p-2 rounded-lg bg-white/5 hover:bg-primary/20 hover:text-primary transition-all",
                        title: "Manual Sync",
                        onclick: move |_| props.on_sync.call(()),
                        Icon { name: "sync".to_string(), class: "text-[18px]".to_string() }
                    }
                    button {
                        class: "p-2 rounded-lg bg-white/5 hover:bg-white/10 text-white transition-all",
                        title: "Edit Configuration",
                        onclick: move |_| props.on_edit.call(()),
                        Icon { name: "settings".to_string(), class: "text-[18px]".to_string() }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TechStatCardProps {
    label: &'static str,
    value: String,
    icon: &'static str,
    color: &'static str,
}

#[component]
fn TechStatCard(props: TechStatCardProps) -> Element {
    rsx! {
        div {
            class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-sm relative overflow-hidden group",
            div { class: "absolute top-0 right-0 p-8 opacity-[0.03] group-hover:opacity-[0.07] transition-opacity",
                Icon { name: props.icon.to_string(), class: "text-[96px] text-white".to_string() }
            }
            div {
                class: "relative z-10",
                div {
                    class: "flex items-center gap-2 mb-4",
                    div { class: format!("p-2 rounded-lg bg-white/5 {}", props.color),
                        Icon { name: props.icon.to_string(), class: "text-[16px]".to_string() }
                    }
                    span { class: "text-[10px] font-black uppercase tracking-[0.2em] text-slate-500", "{props.label}" }
                }
                p { class: "text-3xl font-black text-white", "{props.value}" }
            }
        }
    }
}

#[component]
fn Button(
    children: Element,
    onclick: EventHandler<MouseEvent>,
    variant: Option<String>,
    class: Option<String>,
    disabled: Option<bool>,
) -> Element {
    let base = "flex items-center justify-center transition-all active:scale-95 disabled:opacity-50 disabled:active:scale-100 cursor-pointer";
    let variant_styles = match variant.as_deref() {
        Some("primary") => "bg-primary text-white hover:bg-primary/90 shadow-lg shadow-primary/20",
        _ => "bg-white/10 text-white hover:bg-white/20",
    };
    let extra = class.unwrap_or_default();

    rsx! {
        button {
            class: "{base} {variant_styles} {extra}",
            onclick: move |e| {
                 if !disabled.unwrap_or(false) {
                     onclick.call(e);
                 }
            },
            disabled: disabled.unwrap_or(false),
            {children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct EditSourceModalProps {
    subscription: Subscription,
    on_save: EventHandler<Subscription>,
    on_close: EventHandler<()>,
}

#[component]
fn EditSourceModal(props: EditSourceModalProps) -> Element {
    let mut name = use_signal(|| props.subscription.name.clone());
    let mut url = use_signal(|| props.subscription.urls.first().cloned().unwrap_or_default());
    let mut enabled = use_signal(|| props.subscription.enabled);

    let is_enabled = *enabled.read();
    let enabled_bg = if is_enabled {
        "bg-primary"
    } else {
        "bg-slate-700"
    };
    let dot_translate = if is_enabled {
        "translate-x-6"
    } else {
        "translate-x-0"
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "w-full max-w-lg mx-4 p-8 rounded-3xl bg-slate-900 border border-white/10 shadow-2xl",
                onclick: move |e| e.stop_propagation(),

                h3 { class: "text-2xl font-bold text-white mb-8", "Edit Node Source" }

                div {
                    class: "space-y-6",
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Label" }
                        input {
                            class: "w-full px-4 py-3 rounded-2xl bg-white/5 border border-white/10 text-white placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/50",
                            value: "{name}",
                            oninput: move |e| name.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Endpoint URL" }
                        input {
                            class: "w-full px-4 py-3 rounded-2xl bg-white/5 border border-white/10 text-white placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/50 font-mono text-sm",
                            value: "{url}",
                            oninput: move |e| url.set(e.value())
                        }
                    }
                    div {
                        class: "flex items-center justify-between p-4 rounded-2xl bg-white/5 border border-white/10",
                        div {
                            div { class: "text-sm font-bold text-white", "Active Propagation" }
                            div { class: "text-xs text-slate-500", "Include this source in auto-updates" }
                        }
                        button {
                            class: "w-12 h-6 rounded-full transition-colors relative flex items-center px-1 {enabled_bg}",
                            onclick: move |_| enabled.set(!enabled()),
                            div {
                                class: "w-4 h-4 bg-white rounded-full transition-transform {dot_translate}"
                            }
                        }
                    }
                }

                div {
                    class: "flex gap-4 mt-10",
                    Button {
                        class: "flex-1 px-6 py-3 rounded-2xl font-bold",
                        onclick: move |_| props.on_close.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: Some("primary".to_string()),
                        class: "flex-1 px-6 py-3 rounded-2xl font-bold",
                        onclick: move |_| {
                            let mut sub = props.subscription.clone();
                            sub.name = name.read().clone();
                            sub.urls = vec![url.read().clone()];
                            sub.enabled = *enabled.read();
                            props.on_save.call(sub);
                        },
                        "Apply Changes"
                    }
                }
            }
        }
    }
}
