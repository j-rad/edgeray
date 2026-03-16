//! Routing Manager UI
//!
//! Rule management interface for domain/IP routing with balancers.

use crate::components::forms::{
    Button, ButtonVariant, FormCard, Select, TextInput,
};
use crate::components::ui::Icon;
use dioxus::prelude::*;

/// Routing rule data structure
#[derive(Clone, Debug, PartialEq, Default)]
pub struct RoutingRule {
    pub id: String,
    pub enabled: bool,
    pub name: String,
    pub rule_type: RuleType,
    pub domains: Vec<String>,
    pub ips: Vec<String>,
    pub ports: String,
    pub outbound: String,
    pub priority: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum RuleType {
    #[default]
    Field,
    Balancer,
}

/// Props for the RoutingManager component
#[derive(Props, Clone, PartialEq)]
pub struct RoutingManagerProps {
    /// Current routing rules
    #[props(default = Vec::new())]
    pub rules: Vec<RoutingRule>,
    /// Available outbound tags
    #[props(default = vec!["proxy".to_string(), "direct".to_string(), "block".to_string()])]
    pub outbounds: Vec<String>,
    /// Save handler
    pub on_save: EventHandler<Vec<RoutingRule>>,
    /// Back navigation handler
    pub on_back: EventHandler<()>,
}

/// Routing rules management UI
#[component]
pub fn RoutingManager(props: RoutingManagerProps) -> Element {
    let rules = use_signal(|| props.rules.clone());
    let mut editing_rule = use_signal(|| None::<usize>);
    let mut show_add_form = use_signal(|| false);

    // Domain strategy signal
    let domain_strategy = use_signal(|| "AsIs".to_string());

    rsx! {
        div {
            class: "min-h-screen bg-[#050a10] text-white",

            // Header
            header {
                class: "flex items-center gap-4 p-6 border-b border-white/10 sticky top-0 bg-[#050a10]/95 backdrop-blur-xl z-10",
                button {
                    class: "size-10 flex items-center justify-center rounded-xl bg-white/5 hover:bg-white/10 transition-colors",
                    onclick: move |_| props.on_back.call(()),
                    Icon { name: "arrow_back", class: "" }
                }
                div {
                    h1 { class: "text-xl font-bold", "Routing Rules" }
                    p { class: "text-sm text-slate-400", "Manage domain and IP routing" }
                }
                div { class: "flex-1" }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| show_add_form.set(true),
                    icon: Some("add".to_string()),
                    "Add Rule"
                }
            }

            main {
                class: "p-6 space-y-6 max-w-4xl mx-auto",

                // Domain Strategy
                FormCard {
                    title: Some("Domain Resolution".to_string()),
                    Select {
                        label: "Domain Strategy".to_string(),
                        value: domain_strategy,
                        options: vec![
                            ("AsIs".to_string(), "AsIs - Use domain directly".to_string()),
                            ("IPIfNonMatch".to_string(), "IPIfNonMatch - Resolve if no domain match".to_string()),
                            ("IPOnDemand".to_string(), "IPOnDemand - Always resolve".to_string()),
                        ],
                        help: Some("How to handle domain matching vs IP matching".to_string()),
                    }
                }

                // Rules List
                if rules.read().is_empty() {
                    div {
                        class: "flex flex-col items-center justify-center py-16 text-center",
                        Icon { name: "alt_route", class: "text-6xl text-slate-600 mb-4" }
                        h3 { class: "text-lg font-medium text-white mb-2", "No Routing Rules" }
                        p { class: "text-slate-400 max-w-sm", "Add rules to customize how traffic is routed. By default, all traffic goes through the proxy." }
                    }
                } else {
                    div {
                        class: "space-y-3",
                        for (i, rule) in rules.read().iter().enumerate() {
                            RuleCard {
                                rule: rule.clone(),
                                outbounds: props.outbounds.clone(),
                                on_edit: move |_| editing_rule.set(Some(i)),
                                on_delete: {
                                    let mut rules = rules.clone();
                                    move |_| {
                                        let mut r = rules.read().clone();
                                        r.remove(i);
                                        rules.set(r);
                                    }
                                },
                                on_toggle: {
                                    let mut rules = rules.clone();
                                    move |enabled| {
                                        let mut r = rules.read().clone();
                                        if let Some(rule) = r.get_mut(i) {
                                            rule.enabled = enabled;
                                        }
                                        rules.set(r);
                                    }
                                },
                            }
                        }
                    }
                }

                // Preset Rules
                FormCard {
                    title: Some("Quick Presets".to_string()),
                    description: Some("Common routing configurations".to_string()),
                    div {
                        class: "grid grid-cols-2 gap-3",
                        PresetButton {
                            label: "Bypass LAN".to_string(),
                            icon: "home".to_string(),
                            description: "Private IPs direct".to_string(),
                            on_click: {
                                let mut rules = rules.clone();
                                move |_| {
                                    let mut r = rules.read().clone();
                                    r.push(RoutingRule {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        enabled: true,
                                        name: "Bypass LAN".to_string(),
                                        rule_type: RuleType::Field,
                                        domains: vec![],
                                        ips: vec![
                                            "geoip:private".to_string(),
                                            "127.0.0.0/8".to_string(),
                                        ],
                                        ports: String::new(),
                                        outbound: "direct".to_string(),
                                        priority: 100,
                                    });
                                    rules.set(r);
                                }
                            },
                        }
                        PresetButton {
                            label: "Bypass China".to_string(),
                            icon: "language".to_string(),
                            description: "CN domains/IPs direct".to_string(),
                            on_click: {
                                let mut rules = rules.clone();
                                move |_| {
                                    let mut r = rules.read().clone();
                                    r.push(RoutingRule {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        enabled: true,
                                        name: "Bypass China".to_string(),
                                        rule_type: RuleType::Field,
                                        domains: vec!["geosite:cn".to_string()],
                                        ips: vec!["geoip:cn".to_string()],
                                        ports: String::new(),
                                        outbound: "direct".to_string(),
                                        priority: 90,
                                    });
                                    rules.set(r);
                                }
                            },
                        }
                        PresetButton {
                            label: "Block Ads".to_string(),
                            icon: "block".to_string(),
                            description: "Advertising domains".to_string(),
                            on_click: {
                                let mut rules = rules.clone();
                                move |_| {
                                    let mut r = rules.read().clone();
                                    r.push(RoutingRule {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        enabled: true,
                                        name: "Block Ads".to_string(),
                                        rule_type: RuleType::Field,
                                        domains: vec!["geosite:category-ads-all".to_string()],
                                        ips: vec![],
                                        ports: String::new(),
                                        outbound: "block".to_string(),
                                        priority: 80,
                                    });
                                    rules.set(r);
                                }
                            },
                        }
                        PresetButton {
                            label: "Media Streaming".to_string(),
                            icon: "play_circle".to_string(),
                            description: "Netflix, YouTube etc.".to_string(),
                            on_click: {
                                let mut rules = rules.clone();
                                move |_| {
                                    let mut r = rules.read().clone();
                                    r.push(RoutingRule {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        enabled: true,
                                        name: "Media Streaming".to_string(),
                                        rule_type: RuleType::Field,
                                        domains: vec![
                                            "geosite:netflix".to_string(),
                                            "geosite:youtube".to_string(),
                                            "geosite:google".to_string(),
                                        ],
                                        ips: vec![],
                                        ports: String::new(),
                                        outbound: "proxy".to_string(),
                                        priority: 70,
                                    });
                                    rules.set(r);
                                }
                            },
                        }
                    }
                }

                // Save Button
                div {
                    class: "flex justify-end pt-4",
                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: move |_| props.on_save.call(rules.read().clone()),
                        icon: Some("save".to_string()),
                        "Save Rules"
                    }
                }
            }

            // Add Rule Modal
            if *show_add_form.read() {
                RuleModal {
                    rule: None,
                    outbounds: props.outbounds.clone(),
                    on_save: {
                        let mut rules = rules.clone();
                        let mut show_add_form = show_add_form.clone();
                        move |rule: RoutingRule| {
                            let mut r = rules.read().clone();
                            r.push(rule);
                            rules.set(r);
                            show_add_form.set(false);
                        }
                    },
                    on_cancel: move |_| show_add_form.set(false),
                }
            }
        }
    }
}

/// Single rule card display
#[derive(Props, Clone, PartialEq)]
struct RuleCardProps {
    rule: RoutingRule,
    outbounds: Vec<String>,
    on_edit: EventHandler<()>,
    on_delete: EventHandler<()>,
    on_toggle: EventHandler<bool>,
}

#[component]
fn RuleCard(props: RuleCardProps) -> Element {
    let rule = &props.rule;
    let outbound_class = match rule.outbound.as_str() {
        "direct" => "bg-emerald-500/20 text-emerald-400",
        "block" | "blackhole" => "bg-red-500/20 text-red-400",
        _ => "bg-blue-500/20 text-blue-400",
    };

    // Clone handlers to avoid borrowing props in closures while rule borrows props
    let on_toggle = props.on_toggle.clone();
    let on_edit = props.on_edit.clone();
    let on_delete = props.on_delete.clone();
    let is_enabled = rule.enabled;

    rsx! {
        div {
            class: format!(
                "flex items-center gap-4 p-4 rounded-xl glass-card transition-all {}",
                if rule.enabled { "" } else { "opacity-50" }
            ),

            // Toggle
            button {
                class: format!(
                    "size-6 rounded-full border-2 transition-colors {}",
                    if rule.enabled { "bg-primary border-primary" } else { "border-slate-500" }
                ),
                onclick: move |_| on_toggle.call(!is_enabled),
                if rule.enabled {
                    Icon { name: "check", class: "text-white text-sm" }
                }
            }

            // Rule info
            div {
                class: "flex-1 min-w-0",
                div {
                    class: "flex items-center gap-2 mb-1",
                    span { class: "font-medium text-white truncate", "{rule.name}" }
                    span {
                        class: format!("px-2 py-0.5 text-[10px] font-bold uppercase rounded {}", outbound_class),
                        "{rule.outbound}"
                    }
                }
                div {
                    class: "text-xs text-slate-400 truncate",
                    if !rule.domains.is_empty() {
                        span { "{rule.domains.len()} domains" }
                    }
                    if !rule.domains.is_empty() && !rule.ips.is_empty() {
                        span { " • " }
                    }
                    if !rule.ips.is_empty() {
                        span { "{rule.ips.len()} IPs" }
                    }
                }
            }

            // Actions
            div {
                class: "flex items-center gap-1",
                button {
                    class: "size-8 rounded-lg hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition-colors",
                    onclick: move |_| on_edit.call(()),
                    Icon { name: "edit", class: "text-lg" }
                }
                button {
                    class: "size-8 rounded-lg hover:bg-red-500/20 flex items-center justify-center text-slate-400 hover:text-red-400 transition-colors",
                    onclick: move |_| on_delete.call(()),
                    Icon { name: "delete", class: "text-lg" }
                }
            }
        }
    }
}

/// Preset button for quick rule addition
#[derive(Props, Clone, PartialEq)]
struct PresetButtonProps {
    label: String,
    icon: String,
    description: String,
    on_click: EventHandler<()>,
}

#[component]
fn PresetButton(props: PresetButtonProps) -> Element {
    rsx! {
        button {
            class: "flex items-center gap-3 p-3 rounded-xl bg-white/5 hover:bg-white/10 border border-white/10 text-left transition-colors",
            onclick: move |_| props.on_click.call(()),
            div {
                class: "size-10 rounded-lg bg-primary/20 flex items-center justify-center shrink-0",
                Icon { name: props.icon, class: "text-primary" }
            }
            div {
                div { class: "font-medium text-white text-sm", "{props.label}" }
                div { class: "text-xs text-slate-400", "{props.description}" }
            }
        }
    }
}

/// Rule edit/create modal
#[derive(Props, Clone, PartialEq)]
struct RuleModalProps {
    rule: Option<RoutingRule>,
    outbounds: Vec<String>,
    on_save: EventHandler<RoutingRule>,
    on_cancel: EventHandler<()>,
}

#[component]
fn RuleModal(props: RuleModalProps) -> Element {
    let existing = props.rule.clone();

    let name = use_signal(|| {
        existing
            .as_ref()
            .map(|r| r.name.clone())
            .unwrap_or_default()
    });
    let mut domains_text = use_signal(|| {
        existing
            .as_ref()
            .map(|r| r.domains.join("\n"))
            .unwrap_or_default()
    });
    let mut ips_text = use_signal(|| {
        existing
            .as_ref()
            .map(|r| r.ips.join("\n"))
            .unwrap_or_default()
    });
    let outbound = use_signal(|| {
        existing
            .as_ref()
            .map(|r| r.outbound.clone())
            .unwrap_or_else(|| "proxy".to_string())
    });

    let save = move |_| {
        let domains: Vec<String> = domains_text
            .read()
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let ips: Vec<String> = ips_text
            .read()
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let rule = RoutingRule {
            id: existing
                .as_ref()
                .map(|r| r.id.clone())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            enabled: true,
            name: name.read().clone(),
            rule_type: RuleType::Field,
            domains,
            ips,
            ports: String::new(),
            outbound: outbound.read().clone(),
            priority: 50,
        };
        props.on_save.call(rule);
    };

    rsx! {
        // Modal backdrop
        div {
            class: "fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4",
            onclick: move |_| props.on_cancel.call(()),

            // Modal content
            div {
                class: "w-full max-w-lg glass-card p-6 rounded-2xl space-y-4",
                onclick: move |e: Event<MouseData>| e.stop_propagation(),

                h2 { class: "text-xl font-bold text-white", "Add Routing Rule" }

                TextInput {
                    label: "Rule Name".to_string(),
                    value: name,
                    placeholder: "My Custom Rule".to_string(),
                    required: true,
                }

                div {
                    class: "space-y-1.5",
                    label { class: "text-sm font-medium text-slate-300", "Domains (one per line)" }
                    textarea {
                        class: "w-full px-4 py-3 rounded-xl bg-white/5 border border-white/10 text-white placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-primary/50 resize-none",
                        rows: 4,
                        placeholder: "geosite:google\ndomain:example.com",
                        value: "{domains_text.read()}",
                        oninput: move |e: Event<FormData>| domains_text.set(e.value().clone()),
                    }
                }

                div {
                    class: "space-y-1.5",
                    label { class: "text-sm font-medium text-slate-300", "IPs (one per line)" }
                    textarea {
                        class: "w-full px-4 py-3 rounded-xl bg-white/5 border border-white/10 text-white placeholder:text-slate-500 focus:outline-none focus:ring-2 focus:ring-primary/50 resize-none",
                        rows: 3,
                        placeholder: "geoip:us\n192.168.0.0/16",
                        value: "{ips_text.read()}",
                        oninput: move |e: Event<FormData>| ips_text.set(e.value().clone()),
                    }
                }

                Select {
                    label: "Outbound".to_string(),
                    value: outbound,
                    options: props.outbounds.iter().map(|o| (o.clone(), o.clone())).collect(),
                }

                div {
                    class: "flex justify-end gap-3 pt-4 border-t border-white/10",
                    Button {
                        variant: ButtonVariant::Ghost,
                        on_click: move |_| props.on_cancel.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: save,
                        "Save Rule"
                    }
                }
            }
        }
    }
}
