//! About Page Component
//!
//! Displays application information, version details, and acknowledgements.

use crate::components::ui::Icon;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AboutViewProps {
    pub on_back: EventHandler<()>,
    pub on_repo_click: EventHandler<()>,
    pub on_policy_click: EventHandler<()>,
}

#[component]
pub fn AboutView(props: AboutViewProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // Header
            header {
                class: "flex items-center justify-between mb-10",
                button {
                    class: "group flex items-center justify-center rounded-full p-2.5 bg-white/10 hover:bg-white/20 transition-all active:scale-95",
                    onclick: move |_| props.on_back.call(()),
                    Icon { name: "arrow_back".to_string(), class: "text-white text-[24px]".to_string() }
                }
                h2 { class: "text-2xl font-bold text-white tracking-tight", "About EdgeRay" }
                div { class: "w-10" }
            }

            // Logo & Version
            div {
                class: "flex flex-col items-center justify-center py-10",
                div {
                    class: "w-24 h-24 rounded-3xl bg-gradient-to-br from-primary to-purple-600 flex items-center justify-center shadow-2xl mb-6",
                    Icon { name: "bolt".to_string(), class: "text-white text-[48px]".to_string() }
                }
                h1 { class: "text-3xl font-black text-white tracking-tight mb-1", "EdgeRay" }
                p { class: "text-slate-400 font-mono text-sm", "v1.0.0" }
            }

            // Info Cards
            div {
                class: "space-y-4",

                // Description Card
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-sm",
                    p {
                        class: "text-slate-300 text-center leading-relaxed",
                        "EdgeRay is a next-generation VPN client with advanced stealth protocols, "
                        "mesh networking capabilities, and premium privacy protection. "
                        "Built with Rust for maximum performance and security."
                    }
                }

                // Features List
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-sm",
                    h3 { class: "text-xs font-bold uppercase tracking-widest text-slate-500 mb-4", "Key Features" }
                    ul {
                        class: "space-y-3",
                        FeatureItem { icon: "security", label: "REALITY & Vision stealth protocols" }
                        FeatureItem { icon: "hub", label: "Peer-to-peer mesh networking" }
                        FeatureItem { icon: "speed", label: "Zero-copy userspace TCP/IP stack" }
                        FeatureItem { icon: "dns", label: "Advanced DNS hijacking & sniffing" }
                        FeatureItem { icon: "tune", label: "Flow-J forward error correction" }
                    }
                }

                // Links
                div {
                    class: "flex flex-col rounded-3xl bg-white/5 border border-white/10 backdrop-blur-sm overflow-hidden divide-y divide-white/10",
                    button {
                        class: "flex items-center gap-4 px-6 py-4 hover:bg-white/5 transition-colors",
                        onclick: move |_| props.on_repo_click.call(()),
                        Icon { name: "code".to_string(), class: "text-primary text-[24px]".to_string() }
                        span { class: "text-white font-medium", "View Source Code" }
                        Icon { name: "open_in_new".to_string(), class: "text-slate-500 ml-auto".to_string() }
                    }
                    button {
                        class: "flex items-center gap-4 px-6 py-4 hover:bg-white/5 transition-colors",
                        onclick: move |_| props.on_policy_click.call(()),
                        Icon { name: "policy".to_string(), class: "text-primary text-[24px]".to_string() }
                        span { class: "text-white font-medium", "Privacy Policy" }
                        Icon { name: "open_in_new".to_string(), class: "text-slate-500 ml-auto".to_string() }
                    }
                }

                // License & Credits
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-sm",
                    h3 { class: "text-xs font-bold uppercase tracking-widest text-slate-500 mb-4", "License" }
                    p {
                        class: "text-slate-400 text-sm text-center",
                        "EdgeRay is open source software licensed under the Apache 2.0 License. "
                        "© 2024-2026 EdgeRay Contributors. All rights reserved."
                    }
                }
            }

            // Footer
            div {
                class: "mt-8 text-center",
                p { class: "text-slate-600 text-xs", "Made with ❤️ using Rust & Dioxus" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct FeatureItemProps {
    icon: &'static str,
    label: &'static str,
}

#[component]
fn FeatureItem(props: FeatureItemProps) -> Element {
    rsx! {
        li {
            class: "flex items-center gap-3",
            Icon { name: props.icon.to_string(), class: "text-primary text-[20px]".to_string() }
            span { class: "text-slate-300 text-sm", "{props.label}" }
        }
    }
}
