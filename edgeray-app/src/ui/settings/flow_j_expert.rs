// edgeray-app/src/ui/settings/flow_j_expert.rs
use crate::components::ui::Icon;
use dioxus::prelude::*;

#[component]
pub fn FlowJExpertSettings() -> Element {
    // Mock settings state
    let mut fec_data_shards = use_signal(|| 10);
    let mut fec_parity_shards = use_signal(|| 3);
    let mut mqtt_obfuscate = use_signal(|| true);
    let mut fingerprint = use_signal(|| "chrome".to_string());

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // Header
            header {
                class: "flex items-center gap-4 mb-8",
                div {
                    class: "p-3 rounded-2xl bg-primary/20 text-primary",
                    Icon { name: "psychology".to_string(), class: "text-[24px]".to_string() }
                }
                div {
                    h2 { class: "text-2xl font-bold text-white tracking-tight", "Flow-J Expert Tuning" }
                    p { class: "text-sm text-slate-400 mt-1", "Advanced FEC, Carrier & Stealth customization" }
                }
            }

            // FEC Pro Config
            section {
                class: "mb-8",
                h3 { class: "text-lg font-semibold text-white mb-4", "Forward Error Correction (FEC)" }
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md space-y-6",

                    // Data Shards
                    div {
                        class: "space-y-2",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center gap-1.5",
                                label { class: "text-sm text-slate-300", "Data Shards" }
                                TooltipIcon { tooltip: "Original data fragments. Higher values improve throughput but increase bandwidth. Typical: 8-12 for streaming." }
                            }
                            span { class: "text-sm font-mono text-primary", "{fec_data_shards}" }
                        }
                        input {
                            "type": "range",
                            "min": "1",
                            "max": "20",
                            class: "w-full accent-primary",
                            value: "{fec_data_shards}",
                            oninput: move |evt| fec_data_shards.set(evt.value().parse().unwrap_or(10))
                        }
                    }

                    // Parity Shards
                    div {
                        class: "space-y-2",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center gap-1.5",
                                label { class: "text-sm text-slate-300", "Parity Shards" }
                                TooltipIcon { tooltip: "Redundancy fragments for packet recovery. Rule: can recover from N lost packets where N ≤ parity count. Higher = more overhead." }
                            }
                            span { class: "text-sm font-mono text-primary", "{fec_parity_shards}" }
                        }
                        input {
                            "type": "range",
                            "min": "0",
                            "max": "10",
                            class: "w-full accent-primary",
                            value: "{fec_parity_shards}",
                            oninput: move |evt| fec_parity_shards.set(evt.value().parse().unwrap_or(3))
                        }
                    }

                    // Visual Ratio
                    div {
                        class: "flex gap-1 h-3 rounded-full overflow-hidden mt-4",
                        for _ in 0..*fec_data_shards.read() {
                             div { class: "flex-1 bg-blue-500/50" }
                        }
                        for _ in 0..*fec_parity_shards.read() {
                             div { class: "flex-1 bg-yellow-500/50" }
                        }
                    }
                    p { class: "text-[10px] text-slate-500 italic text-center", "Blue: Data, Yellow: Redundancy (Recovery)" }
                }
            }

            // MQTT Advanced
            section {
                class: "mb-8",
                h3 { class: "text-lg font-semibold text-white mb-4", "MQTT Carrier Control" }
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md space-y-4",

                    div {
                        class: "flex items-center justify-between",
                        div {
                            h4 { class: "text-sm font-medium text-slate-200", "Topic Obfuscation" }
                            p { class: "text-xs text-slate-400 max-w-[80%]", "Encrypt topic names to prevent DPI classification" }
                        }
                        div {
                            class: "relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in",
                            input {
                                "type": "checkbox",
                                class: "toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer start-0 checked:start-6 checked:bg-primary transition-all duration-300",
                                checked: *mqtt_obfuscate.read(),
                                onchange: move |_| {
                        let new_val = !*mqtt_obfuscate.read();
                        mqtt_obfuscate.set(new_val);
                    }
                            }
                            label {  class: "toggle-label block overflow-hidden h-6 rounded-full bg-slate-700 cursor-pointer" }
                        }
                    }
                }
            }

             // Reality Fingerprint
            section {
                h3 { class: "text-lg font-semibold text-white mb-4", "Reality Fingerprint" }
                 div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                        FingerprintOption { id: "chrome", name: "Chrome", active: *fingerprint.read() == "chrome", onclick: move |_| fingerprint.set("chrome".to_string()) }
                        FingerprintOption { id: "firefox", name: "Firefox", active: *fingerprint.read() == "firefox", onclick: move |_| fingerprint.set("firefox".to_string()) }
                        FingerprintOption { id: "safari", name: "Safari", active: *fingerprint.read() == "safari", onclick: move |_| fingerprint.set("safari".to_string()) }
                         FingerprintOption { id: "ios", name: "iOS", active: *fingerprint.read() == "ios", onclick: move |_| fingerprint.set("ios".to_string()) }
                    }
                }
            }
        }
    }
}

#[component]
fn FingerprintOption(id: String, name: String, active: bool, onclick: EventHandler<()>) -> Element {
    let bg_class = if active {
        "bg-primary/20 border-primary text-white"
    } else {
        "bg-white/5 border-transparent text-slate-400 hover:bg-white/10"
    };

    rsx! {
        button {
            class: "flex flex-col items-center justify-center p-4 rounded-2xl border transition-all {bg_class}",
            onclick: move |_| onclick.call(()),
             Icon { name: "fingerprint".to_string(), class: "mb-2".to_string() }
            span { class: "text-sm font-medium", "{name}" }
        }
    }
}

/// Inline tooltip icon with hover documentation
#[component]
fn TooltipIcon(tooltip: String) -> Element {
    let mut show_tooltip = use_signal(|| false);

    rsx! {
        div {
            class: "relative inline-block",
            onmouseenter: move |_| show_tooltip.set(true),
            onmouseleave: move |_| show_tooltip.set(false),
            Icon {
                name: "help_outline".to_string(),
                class: "text-slate-500 text-[14px] cursor-help hover:text-slate-300 transition-colors".to_string()
            }
            if *show_tooltip.read() {
                div {
                    class: "absolute z-50 bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-2 text-xs text-slate-200 bg-slate-800 border border-white/10 rounded-lg shadow-xl max-w-xs whitespace-normal font-normal",
                    "{tooltip}"
                    // Tooltip arrow
                    div {
                        class: "absolute top-full left-1/2 -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-slate-800"
                    }
                }
            }
        }
    }
}
