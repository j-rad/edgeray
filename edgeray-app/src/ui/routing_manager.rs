use super::forms::{Button, Card};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct RouteRule {
    pub id: usize,
    pub domain_suffix: String,
    pub target: String, // "direct", "proxy", "block"
    pub active: bool,
}

#[component]
pub fn RoutingChecklist(rules: Signal<Vec<RouteRule>>, on_add: EventHandler<()>) -> Element {
    rsx! {
        Card {
            title: "Routing Rules",
            extra: rsx! {
                Button {
                    onclick: move |_| on_add.call(()),
                    "Add Rule"
                }
            },
            children: rsx! {
                div {
                    class: "space-y-4",
                    for rule in rules.read().iter() {
                        div {
                            key: "{rule.id}",
                            class: "flex items-center gap-4 p-3 border rounded bg-gray-50 dark:bg-gray-800/50",
                            div { class: "flex-1 font-mono text-sm", "{rule.domain_suffix}" }
                            div {
                                class: "px-2 py-1 rounded text-xs font-bold uppercase " .to_string() + match rule.target.as_str() {
                                    "proxy" => "bg-green-100 text-green-800",
                                    "block" => "bg-red-100 text-red-800",
                                    _ => "bg-gray-200 text-gray-800"
                                },
                                "{rule.target}"
                            }
                            // Logic to toggle active or delete would go here
                        }
                    }
                }
            }
        }
    }
}
