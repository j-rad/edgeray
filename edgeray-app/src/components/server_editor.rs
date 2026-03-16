use crate::components::ui::Icon;
use crate::models::ServerConfig;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ServerEditorProps {
    pub server: ServerConfig,
    pub on_save: EventHandler<ServerConfig>,
    pub on_cancel: EventHandler<()>,
    #[props(default = None)]
    pub on_delete: Option<EventHandler<ServerConfig>>,
    #[props(default = None)]
    pub on_share: Option<EventHandler<ServerConfig>>,
}

#[component]
pub fn ServerEditor(props: ServerEditorProps) -> Element {
    let mut config = use_signal(|| props.server.clone());

    rsx! {
        div {
            class: "relative z-10 flex h-full min-h-screen w-full flex-col overflow-x-hidden max-w-md mx-auto shadow-2xl glass-container",
             header {
                class: "sticky top-0 z-50 flex items-center justify-between bg-white/70 dark:bg-slate-900/70 backdrop-blur-xl px-4 py-3 border-b border-white/40 dark:border-white/10 transition-all duration-300 shadow-sm",
                button {
                    class: "group flex items-center justify-center rounded-full p-2 hover:bg-white/10 transition-colors",
                    onclick: move |_| props.on_cancel.call(()),
                    Icon { name: "arrow_back_ios_new".to_string(), class: "text-primary text-[24px]".to_string() }
                }
                h2 { class: "text-base font-bold leading-tight tracking-tight text-slate-800 dark:text-slate-100 drop-shadow-sm", "Edit Configuration" }
                button {
                    class: "flex items-center justify-center rounded-full py-1 px-3 bg-transparent hover:bg-white/10 transition-colors",
                    onclick: move |_| props.on_save.call(config.read().clone()),
                    p { class: "text-primary text-base font-bold leading-normal", "Save" }
                }
            }
            main {
                class: "flex-1 px-4 py-6 space-y-6",
                // Core Info
                Section {
                    title: "Core Info",
                    InfoRow { label: "Remarks".to_string(), value: config.read().remarks.clone(), on_change: move |v| config.write().remarks = v }
                    InfoRow { label: "Address".to_string(), value: config.read().address.clone(), on_change: move |v| config.write().address = v }
                    InfoRow { label: "Port".to_string(), value: config.read().port, on_change: move |v: String| if let Ok(p) = v.parse::<u16>() { config.write().port = p } }
                }

                // Authentication
                Section {
                    title: "Authentication",
                    div {
                        class: "flex flex-col px-4 py-3 border-b border-slate-200/50 dark:border-white/5 last:border-0",
                        div {
                            class: "flex justify-between items-center mb-1",
                            label { class: "text-base font-medium text-slate-800 dark:text-slate-200", "UUID" }
                            button {
                                class: "text-primary hover:bg-primary/10 rounded-full p-1 transition-colors backdrop-blur-sm",
                                title: "Generate New UUID",
                                Icon { name: "autorenew".to_string(), class: "text-[20px]".to_string() }
                            }
                        }
                        div {
                            class: "relative",
                            input {
                                class: format!("w-full px-3 py-2 rounded-xl text-sm font-mono tracking-tight shadow-inner {}", crate::components::ui::glass::INPUT),
                                value: "{config.read().uuid.clone().unwrap_or_default()}",
                                onchange: move |e| config.write().uuid = Some(e.value()),
                            }
                        }
                    }
                    InfoRow { label: "AlterId".to_string(), value: "0".to_string(), on_change: |_| {} }
                    SelectRow { label: "Security".to_string(), value: config.read().security.clone().unwrap_or_default() }
                }

                 // Transport Settings
                Section {
                    title: "Transport Settings",
                    div {
                        class: "flex flex-col px-4 py-3 border-b border-slate-200/50 dark:border-white/5 last:border-0",
                        label { class: "text-base font-medium text-slate-800 dark:text-slate-200 mb-3", "Network" }
                        div {
                            class: "flex w-full rounded-xl bg-slate-200/40 dark:bg-black/20 p-1 shadow-inner backdrop-blur-sm",
                            // This should be a component
                            button { class: "flex-1 rounded-lg bg-white/90 dark:bg-primary shadow-sm py-1.5 text-sm font-semibold text-slate-900 dark:text-white transition-all transform scale-100", "TCP" }
                            button { class: "flex-1 rounded-lg bg-transparent py-1.5 text-sm font-medium text-slate-500 dark:text-slate-400", "WS" }
                            button { class: "flex-1 rounded-lg bg-transparent py-1.5 text-sm font-medium text-slate-500 dark:text-slate-400", "gRPC" }
                        }
                    }
                    ToggleRow { label: "TLS".to_string(), sublabel: "Transport Layer Security".to_string(), checked: config.read().security.as_deref() == Some("tls") }
                    InfoRow { label: "SNI".to_string(), value: config.read().sni.clone().unwrap_or_default(), on_change: move |v| config.write().sni = Some(v) }
                }

                // Action Buttons
                section {
                    class: "pt-2 pb-8 space-y-3",
                    button {
                        class: format!("flex w-full items-center justify-center gap-2 rounded-2xl p-4 text-base font-bold text-primary active:scale-[0.98] transition-all hover:bg-white/20 dark:hover:bg-slate-700/60 ring-1 ring-white/20 dark:ring-white/5 {}", crate::components::ui::glass::PANEL),
                        onclick: move |_| {
                            if let Some(h) = &props.on_share {
                                h.call(config.read().clone())
                            }
                        },
                        Icon { name: "ios_share".to_string(), class: "text-[20px]".to_string() }
                        "Share Configuration"
                    }
                    button {
                        class: "flex w-full items-center justify-center gap-2 rounded-2xl bg-red-500/10 dark:bg-red-500/20 p-4 text-base font-bold text-red-600 dark:text-red-400 shadow-glass border border-red-500/10 dark:border-red-500/10 active:scale-[0.98] transition-all hover:bg-red-500/20 dark:hover:bg-red-500/30 backdrop-blur-md",
                        onclick: move |_| {
                            if let Some(h) = &props.on_delete {
                                h.call(config.read().clone())
                            }
                        },
                        Icon { name: "delete".to_string(), class: "text-[20px]".to_string() }
                        "Delete Server"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SectionProps {
    title: String,
    children: Element,
}
#[component]
fn Section(props: SectionProps) -> Element {
    rsx! {
        section {
            crate::components::ui::SectionHeader { title: props.title }
            div {
                class: format!("flex flex-col overflow-hidden rounded-2xl mix-blend-normal {}", crate::components::ui::glass::PANEL),
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct InfoRowProps<T: std::fmt::Display + Clone + PartialEq + 'static> {
    label: String,
    value: T,
    on_change: EventHandler<String>,
}
#[component]
fn InfoRow<T: std::fmt::Display + Clone + PartialEq + 'static>(props: InfoRowProps<T>) -> Element {
    rsx! {
        div {
            class: "flex items-center px-4 py-1 border-b border-slate-200/50 dark:border-white/5 last:border-0 group hover:bg-white/10 transition-colors",
            label { class: "w-24 shrink-0 text-base font-medium text-slate-800 dark:text-slate-200", "{props.label}" }
            input {
                class: "flex-1 border-none bg-transparent py-3 pl-2 pr-0 text-right text-base text-slate-700 dark:text-slate-200 placeholder:text-slate-400/70 focus:ring-0",
                value: "{props.value}",
                onchange: move |e| props.on_change.call(e.value()),
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct SelectRowProps {
    label: String,
    value: String,
}
#[component]
fn SelectRow(props: SelectRowProps) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between px-4 py-3 border-b border-slate-200/50 dark:border-white/5 last:border-0 hover:bg-white/10 cursor-pointer group transition-colors",
            label { class: "text-base font-medium text-slate-800 dark:text-slate-200", "{props.label}" }
            div {
                class: "flex items-center gap-1 text-slate-500 dark:text-slate-400",
                span { class: "text-base font-medium", "{props.value}" }
                Icon { name: "chevron_right".to_string(), class: "text-[20px] text-slate-400/80".to_string() }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ToggleRowProps {
    label: String,
    sublabel: String,
    checked: bool,
}
#[component]
fn ToggleRow(props: ToggleRowProps) -> Element {
    rsx! {
         div {
            class: "flex items-center justify-between px-4 py-3 border-b border-slate-200/50 dark:border-white/5 last:border-0 group hover:bg-white/10 transition-colors",
            div {
                class: "flex flex-col",
                label { class: "text-base font-medium text-slate-800 dark:text-slate-200", "{props.label}" }
                span { class: "text-xs text-slate-500 dark:text-slate-400/80", "{props.sublabel}" }
            }
            div {
                class: "relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in",
                input {
                    class: "toggle-checkbox absolute block w-7 h-7 rounded-full bg-white border-4 appearance-none cursor-pointer border-slate-200 dark:border-slate-600 checked:right-0 checked:border-primary transition-all duration-300 shadow-sm",
                    "type": "checkbox",
                    checked: props.checked,
                }
                label { class: "toggle-label block overflow-hidden h-7 rounded-full bg-slate-200/60 dark:bg-slate-700/60 cursor-pointer transition-colors duration-300 backdrop-blur-sm" }
            }
        }
    }
}
