use super::forms::{Button, Card, Input, Switch};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ProtocolType {
    Vless,
    Vmess,
    Trojan,
    Shadowsocks,
}

#[component]
pub fn VlessForm(
    address: Signal<String>,
    port: Signal<String>,
    uuid: Signal<String>,
    flow: Signal<String>,
    reality: Signal<bool>,
    on_save: EventHandler<()>,
) -> Element {
    rsx! {
        Card {
            title: "VLESS Configuration",
            children: rsx! {
                Input { label: "Address", value: address, placeholder: "example.com" }
                Input { label: "Port", value: port, placeholder: "443" }
                Input { label: "UUID", value: uuid, placeholder: "uuid-v4" }
                Input { label: "Flow", value: flow, placeholder: "xtls-rprx-vision" }
                Switch { label: "Enable Reality", checked: reality }
            },
            actions: rsx! {
                Button {
                    variant: "primary",
                    onclick: move |_| on_save.call(()),
                    "Save Configuration"
                }
            }
        }
    }
}

#[component]
pub fn ProtocolSelector(
    selected: Signal<ProtocolType>,
    on_change: EventHandler<ProtocolType>,
) -> Element {
    rsx! {
        div {
            class: "flex gap-4 mb-6",
            Button {
                variant: if *selected.read() == ProtocolType::Vless { "primary" } else { "default" },
                onclick: move |_| on_change.call(ProtocolType::Vless),
                "VLESS"
            }
            Button {
                variant: if *selected.read() == ProtocolType::Vmess { "primary" } else { "default" },
                onclick: move |_| on_change.call(ProtocolType::Vmess),
                "VMess"
            }
            Button {
                variant: if *selected.read() == ProtocolType::Trojan { "primary" } else { "default" },
                onclick: move |_| on_change.call(ProtocolType::Trojan),
                "Trojan"
            }
        }
    }
}
