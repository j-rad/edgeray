//! Import View Component
//!
//! Allows importing server configurations from:
//! - Clipboard (paste or auto-detect)
//! - QR Code Scan (requires camera access, platform-specific)
//! - File Import (JSON/TXT config files)
//! - Manual Configuration (via server editor)
//!
//! Glassmorphism design for premium look and feel

use super::ui::Icon;
use crate::models::{Protocol, ServerConfig};
use crate::parser::parse_share_link;
use dioxus::prelude::*;

#[component]
pub fn ImportView(
    on_back: EventHandler<()>,
    on_import: EventHandler<Vec<ServerConfig>>,
) -> Element {
    let mut manual_input = use_signal(String::new);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut parsed_configs = use_signal(Vec::<ServerConfig>::new);

    let mut handle_import = move |content: String| {
        error_msg.set(None);
        let mut configs = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match parse_share_link(line) {
                Ok(config) => configs.push(config),
                Err(e) => {
                    log::warn!("Failed to parse link '{}': {}", line, e);
                    if error_msg.read().is_none() {
                        error_msg.set(Some(format!("Error parsing link: {}", e)));
                    }
                }
            }
        }

        if configs.is_empty() && error_msg.read().is_none() {
            error_msg.set(Some("No valid links found".to_string()));
        } else {
            parsed_configs.set(configs);
        }
    };

    let mut do_read_clipboard = move || {
        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
        {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if let Ok(text) = clipboard.get_text() {
                    handle_import(text);
                } else {
                    error_msg.set(Some("Clipboard is empty or not accessible".to_string()));
                }
            } else {
                error_msg.set(Some("Failed to access clipboard".to_string()));
            }
        }
    };

    rsx! {
        div {
            class: "relative flex h-full min-h-screen w-full flex-col overflow-x-hidden bg-gradient-mesh font-display text-slate-900 dark:text-white antialiased",
            // Background Blobs
            div { class: "fixed top-[-10%] left-[-20%] w-[60%] h-[60%] rounded-full bg-blue-500/20 dark:bg-blue-600/20 blur-[120px] pointer-events-none z-0 mix-blend-multiply dark:mix-blend-screen" }
            div { class: "fixed top-[20%] right-[-20%] w-[50%] h-[50%] rounded-full bg-cyan-400/20 dark:bg-cyan-500/10 blur-[100px] pointer-events-none z-0 mix-blend-multiply dark:mix-blend-screen" }
            div { class: "fixed bottom-[-10%] left-[20%] w-[60%] h-[60%] rounded-full bg-purple-500/20 dark:bg-purple-600/20 blur-[120px] pointer-events-none z-0 mix-blend-multiply dark:mix-blend-screen" }

            // Header
            crate::components::ui::PageHeader {
                title: "Add Configuration".to_string(),
                left_action: Some(rsx! {
                     button {
                        class: "text-primary text-base font-medium active:opacity-70 transition-opacity hover:bg-white/10 rounded-lg px-2 py-1 -ml-2",
                        onclick: move |_| on_back.call(()),
                        "Cancel"
                    }
                })
            }

            // Main Content
            main {
                class: "flex-1 flex flex-col px-4 pt-4 pb-8 relative z-10",
                // Description
                div {
                    class: "py-6",
                    p {
                        class: "text-slate-600 dark:text-slate-300 text-sm font-medium text-center leading-relaxed drop-shadow-sm",
                        "Choose a method to import your server profile."
                    }
                }

                // Multi-line Import Input
                div {
                    class: "mb-8 relative group",
                    crate::components::ui::SectionHeader { title: "Multi-line Import".to_string() }

                    div { class: "absolute -inset-0.5 bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl blur opacity-0 group-hover:opacity-20 transition duration-500" }
                    div {
                        class: "relative flex items-center",
                        textarea {
                            class: "block w-full min-h-[120px] rounded-2xl border border-white/40 dark:border-white/10 bg-white/40 dark:bg-black/20 p-4 text-sm text-slate-900 dark:text-white shadow-xl shadow-blue-900/5 backdrop-blur-xl placeholder:text-slate-500/70 dark:placeholder:text-slate-400/70 focus:ring-2 focus:ring-primary/50 focus:border-primary/50 transition-all duration-300 hover:bg-white/50 dark:hover:bg-black/30 outline-none resize-none",
                            placeholder: "Paste one or more vmess://, vless://, or trojan:// links here...",
                            value: "{manual_input}",
                            oninput: move |e| {
                                let value = e.value();
                                manual_input.set(value.clone());
                                handle_import(value);
                            }
                        }
                        button {
                            class: "absolute right-3 bottom-3 flex items-center justify-center p-3 rounded-2xl bg-primary text-white hover:bg-primary/90 active:scale-95 transition-all shadow-lg",
                            "aria-label": "Paste",
                            onclick: move |_| do_read_clipboard(),
                            Icon { name: "content_paste".to_string(), class: "text-[20px]".to_string() }
                        }
                    }
                    p {
                        class: "mt-2.5 text-xs text-slate-500 dark:text-slate-500 px-2 font-medium",
                        "Each link should be on a new line. We'll automatically identify each profile."
                    }
                }


                // Import Options
                div {
                    class: "mb-8",
                    crate::components::ui::SectionHeader { title: "Import Options".to_string() }

                    crate::components::ui::GlassCard {
                        class: "flex flex-col overflow-hidden divide-y divide-white/20 dark:divide-white/5",
                        children: rsx! {
                            ImportOptionButton {
                                icon: "qr_code_scanner",
                                color: "blue",
                                title: "Scan QR Code",
                                description: "Use camera to scan server QR",
                                onclick: move |_| error_msg.set(Some("QR scanning requires camera access. Use clipboard paste or file import instead.".to_string()))
                            }
                            ImportOptionButton {
                                icon: "content_paste",
                                color: "purple",
                                title: "Import from Clipboard",
                                description: "Detect configuration from copy history",
                                onclick: move |_| do_read_clipboard()
                            }
                            ImportOptionButton {
                                icon: "snippet_folder",
                                color: "emerald",
                                title: "Import from File",
                                description: "Load JSON or TXT config file",
                                onclick: move |_| error_msg.set(Some("File import requires file system access. Paste your configuration directly in the text area above.".to_string()))
                            }
                        }
                    }
                }

                // Manual Configuration Option
                div {
                    class: "mb-6",
                    crate::components::ui::GlassCard {
                        class: "flex flex-col overflow-hidden",
                        children: rsx! {
                             ImportOptionButton {
                                icon: "tune",
                                color: "orange",
                                title: "Manual Configuration",
                                description: "Enter server details manually",
                                onclick: move |_| error_msg.set(Some("For manual configuration, paste a single server link in the text area above.".to_string()))
                            }
                        }
                    }
                }

                // Error Message
                if let Some(msg) = error_msg.read().as_ref() {
                    div {
                        class: "bg-red-500/10 border border-red-500/20 text-red-600 dark:text-red-300 text-xs p-3 rounded-xl mb-4 flex items-center gap-2 backdrop-blur-md",
                        Icon { name: "error".to_string(), class: "text-red-500".to_string() }
                        "{msg}"
                    }
                }

                // Parsed Configs
                if !parsed_configs.read().is_empty() {
                    div {
                        class: "space-y-3",
                        div {
                            class: "flex items-center justify-between text-xs text-slate-500 dark:text-slate-400 mb-2 px-1",
                            span { "Parsed {parsed_configs.read().len()} configs" }
                            button {
                                class: "text-primary font-bold cursor-pointer hover:text-primary/80 px-3 py-1 rounded-lg bg-primary/10 border border-primary/20 transition-all",
                                onclick: move |_| on_import.call(parsed_configs.read().clone()),
                                "Add All"
                            }
                        }
                        for config in parsed_configs.read().iter() {
                            PreviewCard { config: config.clone() }
                        }
                    }
                }

                // Supported Protocols Footer
                div {
                    class: "mt-auto pt-6 pb-2",
                    div {
                        class: "bg-white/20 dark:bg-white/5 rounded-xl p-3 border border-white/30 dark:border-white/5 backdrop-blur-md mx-auto max-w-[85%] shadow-sm",
                        p {
                            class: "text-center text-xs text-slate-500 dark:text-slate-400",
                            "Supported protocols:"
                            br {}
                            span {
                                class: "font-semibold text-slate-700 dark:text-slate-300 tracking-wide mt-1 block",
                                "VMess • VLESS • Trojan • Shadowsocks"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ImportOptionButtonProps {
    icon: &'static str,
    color: &'static str, // "blue", "purple", "emerald", "orange"
    title: &'static str,
    description: &'static str,
    onclick: EventHandler<()>,
}

#[component]
fn ImportOptionButton(props: ImportOptionButtonProps) -> Element {
    let (icon_from, icon_to, text_color, hover_text, shadow_color) = match props.color {
        "blue" => (
            "from-blue-500/10 to-blue-600/20",
            "group-hover:from-blue-500 group-hover:to-blue-600",
            "text-blue-600 dark:text-blue-400",
            "group-hover:text-blue-600 dark:group-hover:text-blue-400",
            "group-hover:shadow-blue-500/30",
        ),
        "purple" => (
            "from-purple-500/10 to-purple-600/20",
            "group-hover:from-purple-500 group-hover:to-purple-600",
            "text-purple-600 dark:text-purple-400",
            "group-hover:text-purple-600 dark:group-hover:text-purple-400",
            "group-hover:shadow-purple-500/30",
        ),
        "emerald" => (
            "from-emerald-500/10 to-emerald-600/20",
            "group-hover:from-emerald-500 group-hover:to-emerald-600",
            "text-emerald-600 dark:text-emerald-400",
            "group-hover:text-emerald-600 dark:group-hover:text-emerald-400",
            "group-hover:shadow-emerald-500/30",
        ),
        "orange" => (
            "from-orange-500/10 to-orange-600/20",
            "group-hover:from-orange-500 group-hover:to-orange-600",
            "text-orange-500 dark:text-orange-400",
            "group-hover:text-orange-600 dark:group-hover:text-orange-400",
            "group-hover:shadow-orange-500/30",
        ),
        _ => (
            "from-slate-500/10 to-slate-600/20",
            "group-hover:from-slate-500 group-hover:to-slate-600",
            "text-slate-600 dark:text-slate-400",
            "group-hover:text-slate-600 dark:group-hover:text-slate-400",
            "group-hover:shadow-slate-500/30",
        ),
    };

    rsx! {
        button {
            class: "flex items-center gap-4 px-5 py-4 w-full text-left hover:bg-white/40 dark:hover:bg-white/5 transition-all duration-200 active:bg-white/60 dark:active:bg-white/10 group",
            onclick: move |_| props.onclick.call(()),
            div {
                class: format!("flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-gradient-to-br {} {} group-hover:text-white transition-all duration-300 {} group-hover:shadow-lg shadow-sm border border-slate-200/20 dark:border-slate-500/20", icon_from, icon_to, shadow_color),
                Icon {
                    name: props.icon.to_string(),
                    class: text_color.to_string()
                }
            }
            div {
                class: "flex-1 min-w-0",
                p {
                    class: format!("text-base font-semibold text-slate-900 dark:text-white truncate {} transition-colors", hover_text),
                    "{props.title}"
                }
                p {
                    class: "text-xs text-slate-600 dark:text-slate-400 truncate mt-0.5",
                    "{props.description}"
                }
            }
            Icon { name: "chevron_right".to_string(), class: "text-slate-400/70 group-hover:text-primary transition-colors translate-x-0 group-hover:translate-x-1 duration-200".to_string() }
        }
    }
}

#[component]
fn PreviewCard(config: ServerConfig) -> Element {
    let (_protocol_text, variant) = match config.protocol {
        Protocol::Vmess => ("Vmess", "default"),
        Protocol::Vless => ("Vless", "default"),
        Protocol::Trojan => ("Trojan", "success"),
        Protocol::Shadowsocks => ("Shadowsocks", "warning"),
        Protocol::Hysteria2 => ("Hysteria2", "error"),
        Protocol::Flow => ("Flow", "default"),
    };

    rsx! {
        crate::components::ui::GlassCard {
            class: "p-4 flex items-center gap-3",
            children: rsx! {
                div {
                    class: "w-11 h-11 rounded-xl bg-gradient-to-br from-blue-50 to-white dark:from-slate-800 dark:to-slate-700/50 flex items-center justify-center shadow-inner border border-white/60 dark:border-white/5",
                    Icon { name: "dns".to_string(), class: "text-primary".to_string() }
                }
                div {
                    class: "flex-1 min-w-0",
                    div {
                        class: "flex items-center gap-2 mb-0.5",
                        crate::components::ui::Badge {
                            label: format!("{:?}", config.protocol),
                            variant: variant.to_string()
                        }
                        span {
                            class: "text-sm font-medium text-slate-800 dark:text-white truncate",
                            "{config.remarks}"
                        }
                    }
                    p {
                        class: "text-xs text-slate-500 dark:text-slate-400 truncate",
                        "{config.address}:{config.port}"
                    }
                }
            }
        }
    }
}
