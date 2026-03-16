use crate::ui::forms::{Card, Input};
use dioxus::prelude::*;

#[component]
pub fn AdvancedTuning() -> Element {
    // Top-level signals for form state
    let fec_shards = use_signal(|| "10".to_string());
    let fec_parities = use_signal(|| "3".to_string());
    let mqtt_heartbeat = use_signal(|| "30".to_string());
    let fingerprint_interval = use_signal(|| "3600".to_string());

    // FakeDNS signals
    let fakedns_pool = use_signal(|| "198.18.0.0/16".to_string());
    let fakedns_size = use_signal(|| "65536".to_string());
    let fakedns_max = use_signal(|| "65535".to_string());

    rsx! {
        div {
            class: "space-y-6",

            header {
                class: "mb-8",
                h2 { class: "text-2xl font-bold text-white", "Advanced Core Tuning" }
                p { class: "text-slate-400 text-sm mt-1", "Fine-tune the Rustray engine for specialized network conditions." }
            }

            Card {
                title: Some("Forward Error Correction (FEC)".to_string()),
                class: Some("bg-slate-900/50 border-white/5".to_string()),
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Data Shards" }
                        Input {
                            value: fec_shards,
                            placeholder: Some("10".to_string()),
                            class: Some("bg-white/5 border-white/10".to_string()),
                        }
                        p { class: "text-[10px] text-slate-500 mt-2", "Number of data packets per FEC block." }
                    }
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Parity Shards" }
                        Input {
                            value: fec_parities,
                            placeholder: Some("3".to_string()),
                            class: Some("bg-white/5 border-white/10".to_string()),
                        }
                        p { class: "text-[10px] text-slate-500 mt-2", "Redundancy packets for error recovery (higher = more stable under loss)." }
                    }
                }
            }

            Card {
                title: Some("Mesh Connectivity (MQTT)".to_string()),
                class: Some("bg-slate-900/50 border-white/5".to_string()),
                div {
                    class: "space-y-4",
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Heartbeat Interval (seconds)" }
                        Input {
                            value: mqtt_heartbeat,
                            placeholder: Some("30".to_string()),
                            class: Some("bg-white/5 border-white/10".to_string()),
                        }
                        p { class: "text-[10px] text-slate-500 mt-2", "Frequency of heartbeat signals to keep the mesh persistent." }
                    }
                }
            }

            Card {
                title: Some("Stealth & Fingerprinting (REALITY)".to_string()),
                class: Some("bg-slate-900/50 border-white/5".to_string()),
                div {
                    class: "space-y-4",
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Fingerprint Rotation (seconds)" }
                        Input {
                            value: fingerprint_interval,
                            placeholder: Some("3600".to_string()),
                            class: Some("bg-white/5 border-white/10".to_string()),
                        }
                        p { class: "text-[10px] text-slate-500 mt-2", "Interval for rotating TLS browser fingerprints to avoid detection." }
                    }
                }
            }

            Card {
                title: Some("FakeDNS (Bypass & Speed)".to_string()),
                class: Some("bg-slate-900/50 border-white/5".to_string()),
                div {
                    class: "space-y-6",
                    div {
                        label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "IP Pool" }
                        Input {
                            value: fakedns_pool,
                            placeholder: Some("198.18.0.0/16".to_string()),
                            class: Some("bg-white/5 border-white/10 font-mono text-xs".to_string()),
                        }
                        p { class: "text-[10px] text-slate-500 mt-2", "Internal IP range for fake address mapping (used for DNS sniffing)." }
                    }
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                        div {
                            label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Pool Size" }
                            Input {
                                value: fakedns_size,
                                placeholder: Some("65536".to_string()),
                                class: Some("bg-white/5 border-white/10".to_string()),
                            }
                        }
                        div {
                            label { class: "block text-xs font-bold text-slate-500 uppercase mb-2", "Max LRU Entries" }
                            Input {
                                value: fakedns_max,
                                placeholder: Some("65535".to_string()),
                                class: Some("bg-white/5 border-white/10".to_string()),
                            }
                        }
                    }
                }
            }

            div {
                class: "flex justify-end gap-3 pt-6",
                button {
                    class: "px-6 py-2.5 rounded-xl bg-white/5 hover:bg-white/10 text-white font-bold transition-all",
                    "Reset Defaults"
                }
                button {
                    class: "px-6 py-2.5 rounded-xl bg-primary hover:bg-primary-light text-white font-bold shadow-lg shadow-primary/20 transition-all",
                    "Apply settings"
                }
            }
        }
    }
}
