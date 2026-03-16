use crate::components::import_view::ImportView;
use crate::components::ui::Icon;
use crate::models::{Protocol, ServerConfig};
use crate::ui::protocol_forms::{ProtocolSelector, ProtocolType, VlessForm};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ServerAddModalProps {
    on_close: EventHandler<()>,
    on_save: EventHandler<Vec<ServerConfig>>, // Returns a list of configs (1 for manual, N for import)
}

#[component]
pub fn ServerAddModal(props: ServerAddModalProps) -> Element {
    let mut mode = use_signal(|| "selection".to_string()); // selection, import, manual
    let mut selected_protocol = use_signal(|| ProtocolType::Vless);

    // VLESS Form State
    let vless_address = use_signal(String::new);
    let vless_port = use_signal(String::new);
    let vless_uuid = use_signal(String::new);
    let vless_flow = use_signal(String::new);
    let vless_reality = use_signal(|| true);

    let handle_save_manual = move |_| {
        // Construct ServerConfig based on protocol
        // This is a simplified example. In reality, you'd map fields to the ServerConfig struct.
        // For now, we'll create a dummy config to prove the flow.

        let config = ServerConfig {
            id: Some(uuid::Uuid::new_v4().to_string()),
            remarks: format!("VLESS-{}", vless_address.read()),
            protocol: Protocol::Vless,
            address: vless_address.read().clone(),
            port: vless_port.read().parse().unwrap_or(443),
            uuid: Some(vless_uuid.read().clone()),
            password: None,
            network: None,
            flow: Some(vless_flow.read().clone()),
            security: Some("reality".to_string()), // Simplified
            fingerprint: None,
            sni: None,
            host: None,
            path: None,
            pbk: None,
            sid: None,
            method: None,
            service_name: None,
            group: None,
            allow_insecure: Some(true),
        };

        props.on_save.call(vec![config]);
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm animate-in fade-in duration-200",
            div {
                class: "w-full max-w-2xl bg-white dark:bg-slate-900 rounded-2xl shadow-xl border border-white/20 dark:border-white/10 overflow-hidden flex flex-col max-h-[90vh]",

                // Header
                div {
                    class: "flex items-center justify-between p-4 border-b border-gray-100 dark:border-white/5",
                    h2 { class: "text-lg font-bold text-slate-800 dark:text-white",
                        match mode.read().as_str() {
                            "import" => "Import Configs",
                            "manual" => "Add Manual Config",
                            _ => "Add Server"
                        }
                    }
                    button {
                        class: "p-2 rounded-full hover:bg-gray-100 dark:hover:bg-white/5 transition-colors text-slate-500",
                        onclick: move |_| props.on_close.call(()),
                        Icon { name: "close", class: "text-lg" }
                    }
                }

                // Content
                div {
                    class: "p-6 overflow-y-auto",
                    match mode.read().as_str() {
                        "selection" => rsx! {
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                SelectionCard {
                                    icon: "download",
                                    title: "Import from Clipboard/File",
                                    desc: "Paste vless:// links or scan QR codes",
                                    onclick: move |_| mode.set("import".to_string())
                                }
                                SelectionCard {
                                    icon: "edit",
                                    title: "Manual Configuration",
                                    desc: "Configure protocol details manually",
                                    onclick: move |_| mode.set("manual".to_string())
                                }
                            }
                        },
                        "import" => rsx! {
                            ImportView {
                                on_back: move |_| mode.set("selection".to_string()),
                                on_import: move |configs| props.on_save.call(configs)
                            }
                        },
                        "manual" => rsx! {
                            div {
                                class: "flex flex-col gap-6",
                                button {
                                    class: "w-fit flex items-center gap-1 text-sm text-slate-500 hover:text-primary transition-colors",
                                    onclick: move |_| mode.set("selection".to_string()),
                                    Icon { name: "arrow_back", class: "text-lg" }
                                    "Back to Selection"
                                }

                                ProtocolSelector {
                                    selected: selected_protocol,
                                    on_change: move |p| selected_protocol.set(p)
                                }

                                match *selected_protocol.read() {
                                    ProtocolType::Vless => rsx! {
                                        VlessForm {
                                            address: vless_address,
                                            port: vless_port,
                                            uuid: vless_uuid,
                                            flow: vless_flow,
                                            reality: vless_reality,
                                            on_save: handle_save_manual
                                        }
                                    },
                                    _ => rsx! {
                                        div { class: "p-8 text-center text-slate-500", "Protocol form coming soon..." }
                                    }
                                }
                            }
                        },
                        _ => rsx! {}
                    }
                }
            }
        }
    }
}

#[component]
fn SelectionCard(icon: String, title: String, desc: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "flex flex-col items-center justify-center p-8 gap-4 rounded-xl border-2 border-dashed border-gray-200 dark:border-white/10 hover:border-primary/50 hover:bg-primary/5 transition-all text-center group",
            onclick: move |_| onclick.call(()),
            div {
                class: "p-4 rounded-full bg-primary/10 text-primary group-hover:scale-110 transition-transform",
                Icon { name: icon, class: "text-3xl" }
            }
            div {
                h3 { class: "font-bold text-slate-800 dark:text-white mb-1", "{title}" }
                p { class: "text-sm text-slate-500", "{desc}" }
            }
        }
    }
}
