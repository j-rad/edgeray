//! Routing View Component
//!
//! Renders the routing settings and rules management screens.
use dioxus::prelude::*;

use crate::components::ui::Icon;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rule {
    pub id: usize,
    pub rule_type: String, // "Domain", "IP-CIDR", "Process"
    pub content: String,
    pub target: String, // e.g., "Direct", "Proxy-US"
}

#[component]
pub fn RoutingView(on_back: EventHandler<()>) -> Element {
    let mut show_advanced_rules = use_signal(|| false);

    if *show_advanced_rules.read() {
        rsx! {
            AdvancedRulesScreen {
                on_back: move |_| show_advanced_rules.set(false),
            }
        }
    } else {
        rsx! {
            RoutingSettingsScreen {
                on_back: on_back,
                on_manage_rules: move |_| show_advanced_rules.set(true),
            }
        }
    }
}

#[component]
fn RoutingSettingsScreen(on_back: EventHandler<()>, on_manage_rules: EventHandler<()>) -> Element {
    let routing_mode = use_signal(|| "rule_based".to_string());
    let bypass_lan = use_signal(|| true);
    let bypass_mainland = use_signal(|| true);
    let block_ads = use_signal(|| false);

    rsx! {
        div {
            class: "relative flex flex-col w-full max-w-md lg:max-w-3xl mx-auto h-full flex-grow overflow-x-hidden z-10",

            crate::components::ui::PageHeader {
                title: "Routing Settings".to_string(),
                left_action: Some(rsx! {
                     button {
                        class: "flex items-center justify-center p-2 -ml-2 text-primary hover:bg-white/10 rounded-full transition-colors",
                         onclick: move |_| on_back.call(()),
                        Icon { name: "arrow_back_ios_new".to_string(), class: "text-lg".to_string() }
                    }
                })
            }

            main {
                class: "flex-1 flex flex-col p-4 lg:p-6 gap-6 pb-32 lg:pb-24",
                // Routing Mode
                section {
                    div {
                        class: "flex items-center gap-2 mb-3 px-1",
                        Icon { name: "alt_route".to_string(), class: "text-primary text-xl drop-shadow-sm".to_string() }
                        crate::components::ui::SectionHeader { title: "Routing Mode".to_string() }
                    }
                    div {
                        class: "p-1.5 rounded-2xl glass-inset backdrop-blur-md shadow-inner bg-black/40 border border-white/5",
                        div {
                            class: "grid grid-cols-3 gap-1",
                            RoutingModeButton { label: "Global", icon: "public", mode: "global".to_string(), current_mode: routing_mode }
                            RoutingModeButton { label: "Rule-Based", icon: "rule", mode: "rule_based".to_string(), current_mode: routing_mode }
                            RoutingModeButton { label: "Direct", icon: "near_me", mode: "direct".to_string(), current_mode: routing_mode }
                        }
                    }
                }

                // Quick Rules
                section {
                     div {
                        class: "flex items-center gap-2 mb-3 px-1",
                        Icon { name: "toggle_on".to_string(), class: "text-primary text-xl drop-shadow-sm".to_string() }
                        crate::components::ui::SectionHeader { title: "Quick Rules".to_string() }
                    }
                    crate::components::ui::GlassCard {
                        class: "divide-y divide-gray-200/40 dark:divide-white/5",
                        children: rsx! {
                            crate::components::ui::ToggleItem { label: "Bypass LAN".to_string(), sublabel: Some("Skip proxy for local network".to_string()), checked: bypass_lan }
                            crate::components::ui::ToggleItem { label: "Bypass Iran".to_string(), sublabel: Some("Direct connection for IR IPs/Domains".to_string()), checked: bypass_mainland }
                            crate::components::ui::ToggleItem { label: "Block Ads".to_string(), sublabel: Some("Filter common ad domains".to_string()), checked: block_ads }
                        }
                    }
                }

                // Custom Rules
                section {
                    div {
                        class: "flex items-center justify-between mb-3 px-1",
                        div {
                            class: "flex items-center gap-2",
                            Icon { name: "edit_note".to_string(), class: "text-primary text-xl drop-shadow-sm".to_string() }
                            crate::components::ui::SectionHeader { title: "Custom Rules".to_string() }
                        }
                        button {
                            class: "text-xs font-bold text-primary hover:text-primary/70 transition-colors bg-white/40 dark:bg-white/10 px-3 py-1.5 rounded-lg backdrop-blur-md border border-white/40 shadow-sm",
                            onclick: move |_| on_manage_rules.call(()),
                            "Manage"
                        }
                    }
                     crate::components::ui::GlassCard {
                        class: "p-4",
                        children: rsx! {
                            label { class: "block text-sm font-bold mb-2 text-white/80", "Add Custom Domains" }
                            div {
                                class: "relative group",
                                textarea {
                                    class: "w-full glass-inset rounded-xl p-3 text-sm focus:ring-2 focus:ring-primary/50 focus:border-primary/30 outline-none transition-all resize-none placeholder-gray-500 text-white bg-black/40 backdrop-blur-sm border border-transparent focus:border-primary/30",
                                    placeholder: "example.com, google.com...",
                                    rows: "4"
                                }
                            }
                        }
                    }
                }
            }
             // Apply Changes Button
            div {
                class: "fixed bottom-0 left-0 right-0 p-4 lg:p-6 bg-void/80 backdrop-blur-2xl border-t border-white/5 max-w-md lg:max-w-3xl mx-auto z-20 shadow-glow-cyan/5",
                crate::components::ui::PrimaryButton {
                    onclick: move |_| (),
                    label: "Apply Changes".to_string(),
                    icon: Some("save".to_string()),
                }
            }
        }
    }
}

#[component]
fn AdvancedRulesScreen(on_back: EventHandler<()>) -> Element {
    let mut rules = use_signal(|| {
        vec![
            Rule {
                id: 1,
                rule_type: "Domain".to_string(),
                content: "geosite:category-ir".to_string(),
                target: "Direct".to_string(),
            },
            Rule {
                id: 2,
                rule_type: "IP-CIDR".to_string(),
                content: "192.168.0.0/16".to_string(),
                target: "Direct".to_string(),
            },
            Rule {
                id: 3,
                rule_type: "Domain".to_string(),
                content: "netflix.com".to_string(),
                target: "Proxy-US".to_string(),
            },
        ]
    });
    let mut show_new_rule_sheet = use_signal(|| false);

    rsx! {
        div {
            class: "relative flex h-full min-h-screen w-full flex-col text-white group/design-root overflow-x-hidden font-display z-10 max-w-3xl mx-auto",

             crate::components::ui::PageHeader {
                title: "Routing Rules".to_string(),
                left_action: Some(rsx! {
                    button {
                        class: "text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200 transition-colors p-2 -ml-2",
                        onclick: move |_| on_back.call(()),
                         Icon { name: "arrow_back".to_string(), class: "text-2xl".to_string() }
                    }
                }),
                right_action: Some(rsx! {
                     button {
                        class: "text-primary hover:text-primary/80 transition-colors p-2 -mr-2",
                        onclick: move |_| show_new_rule_sheet.set(true),
                        Icon { name: "add".to_string(), class: "text-2xl".to_string() }
                    }
                })
            }

            // Main Content
            main {
                class: "flex-1 flex flex-col p-4 gap-4 pb-32",
                div {
                    class: "flex items-center justify-between px-1",
                    p { class: "text-slate-500 dark:text-slate-400 text-sm font-medium uppercase tracking-wider pl-1", "Active Rules ({rules.read().len()})" }
                    button { class: "text-primary text-sm font-medium hover:underline pr-1", "Reorder" }
                }
                for rule in rules.read().iter() {
                    RuleCard { rule: rule.clone() }
                }
            }

            // New Rule Sheet
            if *show_new_rule_sheet.read() {
                NewRuleSheet {
                    on_close: move |_| show_new_rule_sheet.set(false),
                    on_save: move |new_rule| {
                        let mut new_rules = rules.read().clone();
                        new_rules.push(new_rule);
                        rules.set(new_rules);
                        show_new_rule_sheet.set(false);
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RoutingModeButtonProps {
    label: String,
    icon: String,
    mode: String,
    current_mode: Signal<String>,
}
#[component]
fn RoutingModeButton(mut props: RoutingModeButtonProps) -> Element {
    let is_checked = *props.current_mode.read() == props.mode;
    rsx! {
        label {
            class: "cursor-pointer group relative",
            input {
                class: "peer sr-only",
                "type": "radio",
                name: "routing_mode",
                value: "{props.mode}",
                checked: is_checked,
                onchange: move |_| props.current_mode.set(props.mode.clone()),
            }
            div {
                class: "flex flex-col items-center justify-center py-3 px-2 rounded-xl transition-all duration-300 text-gray-400 font-bold text-sm peer-checked:bg-primary/20 peer-checked:text-primary peer-checked:shadow-glow-cyan peer-checked:scale-[1.02] hover:bg-white/5",
                Icon { name: props.icon, class: "mb-1 text-xl".to_string() }
                span { class: "text-xs", "{props.label}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct RuleCardProps {
    rule: Rule,
}
#[component]
fn RuleCard(props: RuleCardProps) -> Element {
    let (icon, color_class, variant) = match props.rule.rule_type.as_str() {
        "Domain" => ("language", "text-primary", "default"),
        "IP-CIDR" => ("lan", "text-emerald-400", "success"),
        "Process" => ("memory", "text-amber-500", "warning"),
        _ => ("help", "text-gray-400", "neutral"),
    };
    rsx! {
        crate::components::ui::GlassCard {
            class: "flex flex-col overflow-hidden hover:border-primary/50 transition-all hover:scale-[1.01] hover:shadow-glow-cyan/20 group",
            children: rsx! {
                 div {
                    class: "flex items-center p-4 gap-4",
                    div {
                        class: format!("flex shrink-0 size-12 items-center justify-center rounded-full bg-gradient-to-br from-primary/10 to-primary/5 backdrop-blur-md border border-white/20 shadow-inner {}", color_class),
                        Icon { name: icon.to_string(), class: "".to_string() }
                    }
                    div {
                        class: "flex flex-1 flex-col overflow-hidden",
                        div {
                            class: "flex items-center gap-2",
                            crate::components::ui::Badge { label: props.rule.rule_type.clone(), variant: variant.to_string() }
                            span { class: "text-[10px] font-bold text-gray-400", "{props.rule.target}" }
                        }
                        p { class: "text-base font-bold truncate mt-1 font-mono text-white tracking-tight", "{props.rule.content}" }
                    }
                    button {
                        class: "flex size-9 shrink-0 items-center justify-center rounded-full text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 bg-white/30 dark:bg-white/5 hover:bg-white/60 dark:hover:bg-white/10 border border-white/20 backdrop-blur-md transition-all shadow-sm",
                        Icon { name: "drag_handle".to_string(), class: "".to_string() }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NewRuleSheetProps {
    on_close: EventHandler<()>,
    on_save: EventHandler<Rule>,
}
#[component]
fn NewRuleSheet(props: NewRuleSheetProps) -> Element {
    let selected_type = use_signal(|| "Domain".to_string());
    let mut content = use_signal(String::new);
    let mut target = use_signal(|| "Direct".to_string());

    let handle_save = move |_| {
        let new_rule = Rule {
            id: 0, // Should be generated
            rule_type: selected_type.read().clone(),
            content: content.read().clone(),
            target: target.read().clone(),
        };
        props.on_save.call(new_rule);
    };

    rsx! {
        div {
            class: "fixed inset-x-0 bottom-0 z-30 flex flex-col rounded-t-3xl bg-obsidian/95 border-t border-white/10 shadow-[0_-8px_40px_rgba(0,240,255,0.1)] backdrop-blur-3xl transform transition-transform duration-300 ease-out",
            div {
                class: "w-full flex justify-center pt-3 pb-1",
                div { class: "h-1.5 w-12 rounded-full bg-gray-300/50 dark:bg-gray-600/50 backdrop-blur-sm" }
            }
            div {
                class: "flex items-center justify-between px-5 pb-2 pt-1",
                button { class: "text-sm font-medium text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 transition-colors", onclick: move |_| props.on_close.call(()), "Cancel" }
                h3 { class: "text-base font-bold bg-clip-text text-transparent bg-gradient-to-r from-slate-900 to-slate-700 dark:from-white dark:to-slate-300", "New Rule" }
                button { class: "text-sm font-bold text-primary hover:text-primary/80 transition-colors", onclick: handle_save, "Save" }
            }
            div {
                class: "p-5 flex flex-col gap-6",
                div {
                    class: "bg-gray-200/30 dark:bg-black/20 p-1.5 rounded-xl flex items-center backdrop-blur-md shadow-inner border border-white/10 dark:border-white/5",
                    RuleTypeButton { label: "Domain".to_string(), selected: selected_type }
                    RuleTypeButton { label: "IP".to_string(), selected: selected_type }
                    RuleTypeButton { label: "Process".to_string(), selected: selected_type }
                }
                // Match Content
                div {
                    class: "flex flex-col gap-2",
                    label { class: "text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 pl-1", "Match Content" }
                    div {
                        class: "relative group",
                        textarea {
                            class: "relative w-full rounded-xl bg-white/40 dark:bg-black/10 border border-white/40 dark:border-white/10 p-3.5 text-base font-mono text-gray-900 dark:text-gray-100 placeholder-gray-400/70 focus:border-primary/50 focus:ring-0 focus:outline-none focus:bg-white/60 dark:focus:bg-black/30 backdrop-blur-md resize-none shadow-sm transition-all",
                            placeholder: "example.com, google.com",
                            rows: "3",
                            value: "{content}",
                            oninput: move |e| content.set(e.value()),
                        }
                    }
                }
                // Target Proxy
                div {
                    class: "flex flex-col gap-2",
                     label { class: "text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 pl-1", "Target Proxy" }
                    div {
                        class: "relative group",
                        select {
                            class: "relative w-full appearance-none rounded-xl bg-black/40 border border-white/10 p-3.5 pr-10 text-base text-white focus:border-primary/50 focus:ring-0 focus:outline-none focus:bg-black/60 backdrop-blur-md shadow-sm transition-all",
                            onchange: move |e| target.set(e.value()),
                            option { class: "bg-obsidian text-white", value: "Direct", "Direct" }
                            option { class: "bg-obsidian text-white", value: "Proxy", "Proxy" }
                            option { class: "bg-obsidian text-white", value: "Block", "Block" }
                        }
                        div {
                            class: "pointer-events-none absolute inset-y-0 right-0 flex items-center px-3.5 text-gray-500 z-10",
                            Icon { name: "expand_more".to_string(), class: "".to_string() }
                        }
                    }
                }
            }
        }
    }
}
#[derive(Props, Clone, PartialEq)]
struct RuleTypeButtonProps {
    label: String,
    selected: Signal<String>,
}
#[component]
fn RuleTypeButton(mut props: RuleTypeButtonProps) -> Element {
    let is_selected = *props.selected.read() == props.label;
    rsx! {
        button {
            class: format!("flex-1 py-2 text-sm rounded-lg transition-all text-center {}",
                if is_selected {
                    "font-bold bg-primary/20 text-primary shadow-glow-cyan ring-1 ring-primary/50 backdrop-blur-sm"
                } else {
                    "font-bold text-gray-500 hover:text-gray-300"
                }
            ),
            onclick: move |_| props.selected.set(props.label.clone()),
            "{props.label}"
        }
    }
}
