use crate::components::ui::Icon;
use crate::models::Subscription;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SubscriptionCardProps {
    pub subscription: Subscription,
    pub on_sync: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    #[props(default = 0.0)]
    pub sync_progress: f32,
    #[props(default = false)]
    pub is_syncing: bool,
}

#[component]
pub fn SubscriptionCard(props: SubscriptionCardProps) -> Element {
    let last_update = if let Some(ts) = props.subscription.last_update {
        // Simplified timestamp display
        format!(
            "Synced {}m ago",
            (chrono::Utc::now().timestamp() as u64 - ts) / 60
        )
    } else {
        "Never synced".to_string()
    };

    rsx! {
        div {
            class: "relative overflow-hidden group rounded-3xl bg-white/10 dark:bg-white/5 border border-white/20 dark:border-white/10 p-5 backdrop-blur-md transition-all duration-300 hover:shadow-glass-lg hover:-translate-y-1",

            // Sync Progress Overlay
            if props.is_syncing {
                div {
                    class: "absolute bottom-0 left-0 h-1 bg-primary/60 transition-all duration-300 ease-out",
                    style: "width: {props.sync_progress * 100.0}%"
                }
            }

            div {
                class: "flex items-start justify-between",
                div {
                    class: "flex items-center gap-3",
                    div {
                        class: "p-2.5 rounded-2xl bg-primary/10",
                        Icon { name: "rss_feed".to_string(), class: "text-[24px] text-primary".to_string() }
                    }
                    div {
                        h4 { class: "font-bold text-slate-900 dark:text-white leading-tight", "{props.subscription.name}" }
                        p { class: "text-xs text-slate-500 dark:text-slate-400 mt-0.5", "{last_update}" }
                    }
                }
                div {
                    class: "flex gap-2",
                    button {
                        class: "p-2 rounded-xl bg-white/40 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 transition-colors",
                        onclick: move |_| props.on_edit.call(()),
                        Icon { name: "settings".to_string(), class: "text-[18px] text-slate-500 dark:text-slate-400".to_string() }
                    }
                    button {
                        class: "p-2 rounded-xl bg-primary/10 hover:bg-primary/20 transition-colors group/btn",
                        onclick: move |_| props.on_sync.call(()),
                        disabled: props.is_syncing,
                        Icon {
                            name: if props.is_syncing { "sync".to_string() } else { "refresh".to_string() },
                            class: format!("text-[18px] text-primary {}", if props.is_syncing { "animate-spin" } else { "group-hover/btn:rotate-180 transition-transform duration-500" })
                        }
                    }
                }
            }

            div {
                class: "mt-4 flex items-center justify-between",
                div {
                    class: "flex flex-wrap gap-1.5",
                    for tag in &props.subscription.filter_tags {
                        span {
                            class: "px-2 py-0.5 rounded-lg bg-white/30 dark:bg-white/5 text-[10px] font-bold text-slate-600 dark:text-slate-300 border border-white/20 dark:border-white/10",
                            "{tag}"
                        }
                    }
                }
                div {
                    class: "text-right",
                    p { class: "text-xs font-bold text-slate-900 dark:text-white", "{props.subscription.node_count}" }
                    p { class: "text-[10px] text-slate-500 dark:text-slate-400 leading-none", "Nodes" }
                }
            }
        }
    }
}
