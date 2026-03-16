use crate::components::ui::Icon;
use crate::ui::forms::Button;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ImportWizardProps {
    pub on_import: EventHandler<Vec<String>>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn ImportWizard(props: ImportWizardProps) -> Element {
    let mut raw_links = use_signal(|| String::new());
    let mut show_qr_scanner = use_signal(|| false);

    let links_list = move || {
        raw_links
            .read()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect::<Vec<String>>()
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-md",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "w-full max-w-2xl mx-4 overflow-hidden rounded-3xl bg-slate-900 border border-white/10 shadow-2xl",
                onclick: move |e| e.stop_propagation(),

                div {
                    class: "p-8",
                    // Header
                    div {
                        class: "flex items-center justify-between mb-8",
                        div {
                            h3 { class: "text-2xl font-bold text-white", "Import Sources" }
                            p { class: "text-sm text-slate-400 mt-1", "Paste multiple share links or scan a QR code" }
                        }
                        button {
                            class: "p-2 rounded-full bg-white/5 hover:bg-white/10 transition-colors",
                            onclick: move |_| props.on_close.call(()),
                            Icon { name: "close".to_string(), class: "text-white text-[24px]".to_string() }
                        }
                    }

                    // Content
                    div {
                        class: "space-y-6",

                        // QR Action
                        div {
                            class: "flex gap-4 p-4 rounded-2xl bg-primary/5 border border-primary/20 items-center justify-between",
                            div {
                                class: "flex items-center gap-3",
                                div {
                                    class: "p-3 rounded-xl bg-primary/10",
                                    Icon { name: "qr_code_scanner".to_string(), class: "text-primary text-[24px]".to_string() }
                                }
                                div {
                                    div { class: "text-sm font-bold text-white", "Scan QR Code" }
                                    div { class: "text-xs text-slate-400", "Import using your device camera" }
                                }
                            }
                            Button {
                                variant: Some("primary".to_string()),
                                class: "rounded-xl px-4 py-2 text-xs",
                                onclick: move |_| show_qr_scanner.set(true),
                                "Launch Scanner"
                            }
                        }

                        // Text Area for multi-line
                        div {
                            label { class: "block text-xs font-bold text-slate-400 uppercase mb-2", "Share Links (One per line)" }
                            textarea {
                                class: "w-full h-48 px-4 py-4 rounded-2xl bg-white/5 border border-white/10 text-white placeholder:text-slate-600 focus:outline-none focus:ring-2 focus:ring-primary/50 font-mono text-sm resize-none custom-scrollbar",
                                placeholder: "vmess://...\nvless://...\ntrojan://...",
                                value: "{raw_links}",
                                oninput: move |e| raw_links.set(e.value())
                            }
                            div {
                                class: "mt-2 flex justify-between items-center",
                                span { class: "text-[10px] text-slate-500", "Detected {links_list().len()} links" }
                                button {
                                    class: "text-[10px] font-bold text-primary hover:underline uppercase",
                                    onclick: move |_| {
                                        // Clipboard access is usually async and browser-specific,
                                        // but for this UI we just show the button.
                                        // Interaction would be handled by the platform.
                                    },
                                    "Paste from Clipboard"
                                }
                            }
                        }
                    }

                    // Footer Actions
                    div {
                        class: "flex gap-4 mt-10",
                        Button {
                            class: "flex-1 rounded-2xl",
                            onclick: move |_| props.on_close.call(()),
                            "Discard"
                        }
                        Button {
                            variant: Some("primary".to_string()),
                            class: "flex-1 rounded-2xl",
                            disabled: links_list().is_empty(),
                            onclick: move |_| {
                                let links = links_list();
                                if !links.is_empty() {
                                    props.on_import.call(links);
                                }
                            },
                            "Import Now"
                        }
                    }
                }
            }
        }

        if *show_qr_scanner.read() {
            QRScannerOverlay {
                on_result: move |result: String| {
                    let mut current = raw_links.read().clone();
                    if !current.is_empty() && !current.ends_with('\n') {
                        current.push('\n');
                    }
                    current.push_str(&result);
                    raw_links.set(current);
                    show_qr_scanner.set(false);
                },
                on_close: move |_| show_qr_scanner.set(false)
            }
        }
    }
}

#[component]
fn QRScannerOverlay(on_result: EventHandler<String>, on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-[60] flex flex-col items-center justify-center bg-black",

            div {
                class: "w-full max-w-sm aspect-square border-2 border-primary/50 relative overflow-hidden",
                // Scanner Animation
                div { class: "absolute inset-x-0 h-0.5 bg-primary/50 shadow-[0_0_15px_rgba(59,130,246,0.5)] animate-scan" }

                // Camera placeholder
                div {
                    class: "absolute inset-0 flex flex-col items-center justify-center text-slate-500",
                    Icon { name: "videocam_off".to_string(), class: "text-[64px] mb-4".to_string() }
                    p { class: "text-sm", "Camera interface inhibited in terminal" }
                }
            }

            button {
                class: "mt-12 px-8 py-3 rounded-full bg-white/10 text-white font-bold hover:bg-white/20 transition-colors uppercase tracking-widest text-sm",
                onclick: move |_| on_close.call(()),
                "Close Scanner"
            }

            // Mock result button for testing
            button {
                class: "mt-4 text-[10px] text-slate-500",
                onclick: move |_| on_result.call("vless://test-from-qr@example.com:443#QR_Import".to_string()),
                "(Mock QR Detection)"
            }
        }
    }
}
