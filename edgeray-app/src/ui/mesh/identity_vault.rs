// edgeray-app/src/ui/mesh/identity_vault.rs
use crate::components::ui::Icon;
use dioxus::prelude::*;

#[component]
pub fn IdentityVault() -> Element {
    let mut locked = use_signal(|| true); // Start locked
    let mut key_visible = use_signal(|| false);
    let mut pin = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 custom-scrollbar",

            // Header
            header {
                class: "flex items-center gap-4 mb-8",
                div {
                    class: "p-3 rounded-2xl bg-primary/20 text-primary",
                    Icon { name: "shield_lock".to_string(), class: "text-[24px]".to_string() }
                }
                div {
                    h2 { class: "text-2xl font-bold text-white tracking-tight", "Identity Vault" }
                    p { class: "text-sm text-slate-400 mt-1", "Secure Mesh Identity & Key Management" }
                }
            }

            if *locked.read() {
                // Lock Screen
                div {
                     class: "flex flex-col items-center justify-center h-[60vh] space-y-8",
                     div {
                         class: "p-8 rounded-full bg-slate-800/50 border border-slate-700 backdrop-blur-md shadow-2xl animate-pulse",
                         Icon { name: "lock".to_string(), class: "text-[48px] text-primary".to_string() }
                     }
                     div {
                         class: "text-center",
                         h3 { class: "text-xl font-semibold text-white", "Identity Locked" }
                         p { class: "text-sm text-slate-400 mt-2", "Enter PIN or use Biometrics to access keys" }
                     }

                     // Mock PIN Pad
                     div {
                         class: "grid grid-cols-3 gap-4 w-64",
                         for i in 1..=9 {
                             button {
                                 class: "w-16 h-16 rounded-full bg-slate-800 hover:bg-slate-700 text-xl font-bold text-white transition-all active:scale-95 flex items-center justify-center",
                                 onclick: move |_| {
                                     let current = pin.read().clone();
                                     if current.len() < 4 {
                                          pin.set(format!("{}{}", current, i));
                                          if pin.read().len() == 4 {
                                              // Auto unlock unlock mock
                                              locked.set(false);
                                          }
                                     }
                                 },
                                 "{i}"
                             }
                         }
                         button { class: "col-start-2 w-16 h-16 rounded-full bg-slate-800 hover:bg-slate-700 text-xl font-bold text-white flex items-center justify-center", onclick: move |_| pin.set("".to_string()), "C" }
                     }

                     div {
                         class: "flex gap-2 justify-center",
                         for i in 0..4 {
                             div {
                                 class: if pin.read().len() > i {
                                     "w-3 h-3 rounded-full bg-primary"
                                 } else {
                                     "w-3 h-3 rounded-full bg-slate-700"
                                 }
                             }
                         }
                     }
                }
            } else {
                // Unlocked View
                div {
                    class: "space-y-6 animate-fade-in",

                    // Node ID
                    div {
                        class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                        h3 { class: "text-sm font-semibold text-slate-400 mb-2 uppercase tracking-wide", "Public Node Identity" }
                        div {
                            class: "flex items-center gap-4 bg-black/30 p-4 rounded-xl font-mono text-primary truncate border border-primary/20",
                            Icon { name: "fingerprint".to_string(), class: "text-primary".to_string() }
                            span { "12D3KooWJ..." } // Mock ID
                             button {
                                class: "ml-auto hover:text-white text-slate-500",
                                Icon { name: "content_copy".to_string(), class: "".to_string() }
                            }
                        }
                    }

                    // Keypair
                    div {
                        class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                        div { class: "flex justify-between items-center mb-4",
                            h3 { class: "text-sm font-semibold text-slate-400 uppercase tracking-wide", "ED25519 Private Key" }
                            button {
                                class: "text-xs px-3 py-1 rounded-full bg-slate-800 hover:bg-slate-700 text-white transition-colors",
                                onclick: move |_| {
                                    let v = *key_visible.read();
                                    key_visible.set(!v);
                                },
                                if *key_visible.read() { "Hide" } else { "Reveal" }
                            }
                        }

                         div {
                            class: "font-mono text-sm break-all p-4 rounded-xl bg-red-900/10 border border-red-500/20 text-red-300 relative",
                            if *key_visible.read() {
                                "priv_ed25519_..." // Mock key
                            } else {
                                "•••••••••••••••••••••••••••••••••••••••••••••••••••••"
                            }
                            div {
                                class: "absolute top-2 right-2",
                                Icon { name: "lock".to_string(), class: "text-red-500/50".to_string() }
                            }
                        }
                         p { class: "text-xs text-red-400/60 mt-2 flex items-center gap-1",
                            Icon { name: "warning".to_string(), class: "text-[14px]".to_string() }
                            "Never share this key. It controls your mesh identity."
                        }
                    }

                    // Actions
                    div {
                        class: "grid grid-cols-2 gap-4",
                         button {
                            class: "p-4 rounded-2xl bg-slate-800 hover:bg-slate-700 text-white font-medium transition-all flex items-center justify-center gap-2",
                             Icon { name: "refresh".to_string(), class: "".to_string() }
                            "Rotate Keys"
                        }
                        button {
                            class: "p-4 rounded-2xl bg-slate-800 hover:bg-slate-700 text-white font-medium transition-all flex items-center justify-center gap-2",
                            onclick: move |_| { locked.set(true); pin.set("".to_string()); },
                             Icon { name: "lock".to_string(), class: "".to_string() }
                            "Lock Vault"
                        }
                    }
                }
            }
        }
    }
}
