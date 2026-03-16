// edgeray-app/src/ui/settings/rule_hierarchy.rs

use crate::components::ui::Icon;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
struct RuleItem {
    id: String,
    name: String,
    priority: usize,
    action: String, // "proxy", "direct", "block"
}

#[component]
pub fn RuleHierarchy() -> Element {
    // Mock rules
    let mut rules = use_signal(|| {
        vec![
            RuleItem {
                id: "rule-1".to_string(),
                name: "Block Ads".to_string(),
                priority: 0,
                action: "block".to_string(),
            },
            RuleItem {
                id: "rule-2".to_string(),
                name: "Local LAN Direct".to_string(),
                priority: 1,
                action: "direct".to_string(),
            },
            RuleItem {
                id: "rule-3".to_string(),
                name: "GeoIP CN Direct".to_string(),
                priority: 2,
                action: "direct".to_string(),
            },
            RuleItem {
                id: "rule-4".to_string(),
                name: "Global Proxy".to_string(),
                priority: 3,
                action: "proxy".to_string(),
            },
        ]
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // Header
            header {
                class: "flex items-center gap-4 mb-8",
                 div {
                    class: "p-3 rounded-2xl bg-primary/20 text-primary",
                    Icon { name: "toc".to_string(), class: "text-[24px]".to_string() }
                }
                div {
                    h2 { class: "text-2xl font-bold text-white tracking-tight", "Rule Hierarchy" }
                    p { class: "text-sm text-slate-400 mt-1", "Manage routing priority (top to bottom)" }
                }
            }

            // Rules List
            div {
                class: "space-y-3",
                for (idx, rule) in rules.read().iter().enumerate() {
                    div {
                        class: "group flex items-center gap-4 p-4 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 transition-all",
                        key: "{rule.id}",

                        // Drag Handle
                        div {
                            class: "text-slate-500 cursor-move hover:text-white",
                             Icon { name: "drag_indicator".to_string(), class: "".to_string() }
                        }

                        // Priority Badge
                        div {
                            class: "w-8 h-8 rounded-full bg-black/30 flex items-center justify-center text-xs font-mono font-bold text-slate-400 border border-white/5",
                            "{idx + 1}"
                        }

                        // Rule Info
                        div {
                            class: "flex-1",
                            h3 { class: "text-sm font-semibold text-white", "{rule.name}" }
                            div { class: "flex items-center gap-2 mt-1",
                                {
                                    let badge_color = match rule.action.as_str() {
                                        "block" => "bg-red-500/20 text-red-400",
                                        "direct" => "bg-green-500/20 text-green-400",
                                        _ => "bg-blue-500/20 text-blue-400"
                                    };
                                    rsx! {
                                        span { class: "text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded-full {badge_color}", "{rule.action}" }
                                    }
                                }
                            }
                        }

                        // Actions (Reorder buttons since D&D is complex in basic Dioxus without libraries)
                        div {
                            class: "flex flex-col gap-1 opacity-0 group-hover:opacity-100 transition-opacity",
                            button {
                                class: "p-1 rounded bg-white/5 hover:bg-white/20 text-slate-400 hover:text-white disabled:opacity-30",
                                disabled: idx == 0,
                                onclick: move |_| {
                                    let mut r = rules.read().clone();
                                    r.swap(idx, idx - 1);
                                    rules.set(r);
                                },
                                Icon { name: "arrow_drop_up".to_string(), class: "text-[16px]".to_string() }
                            }
                             button {
                                class: "p-1 rounded bg-white/5 hover:bg-white/20 text-slate-400 hover:text-white disabled:opacity-30",
                                disabled: idx == rules.read().len() - 1,
                                onclick: move |_| {
                                    let mut r = rules.read().clone();
                                    r.swap(idx, idx + 1);
                                    rules.set(r);
                                },
                                Icon { name: "arrow_drop_down".to_string(), class: "text-[16px]".to_string() }
                            }
                        }

                        // Edit/Delete
                        button {
                            class: "p-2 rounded-xl text-slate-500 hover:text-white hover:bg-white/10 transition-colors",
                             Icon { name: "edit".to_string(), class: "text-[18px]".to_string() }
                        }
                    }
                }
            }

            // Add Button
            button {
                class: "mt-4 flex items-center justify-center gap-2 w-full py-4 rounded-2xl border-2 border-dashed border-white/10 text-slate-400 hover:border-primary/50 hover:text-primary hover:bg-primary/5 transition-all",
                Icon { name: "add".to_string(), class: "".to_string() }
                "Add New Rule"
            }
        }
    }
}
