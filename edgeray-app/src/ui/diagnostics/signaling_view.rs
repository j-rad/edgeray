// edgeray-app/src/ui/diagnostics/signaling_view.rs
use crate::components::ui::Icon;
use dioxus::prelude::*;

#[component]
pub fn SignalingView() -> Element {
    // Mock signal data for now - will connect to backend driver
    let signals = use_signal(|| {
        vec![
            SignalState {
                id: "Direct-QUIC".to_string(),
                status: "Connected".to_string(),
                latency: 45,
                direction: "Peer-to-Peer".to_string(),
            },
            SignalState {
                id: "Relay-MQTT".to_string(),
                status: "Signaling".to_string(),
                latency: 120,
                direction: "Relayed".to_string(),
            },
            SignalState {
                id: "Orchestrator".to_string(),
                status: "Syncing".to_string(),
                latency: 200,
                direction: "Cloud".to_string(),
            },
        ]
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // Header
            header {
                class: "flex items-center gap-4 mb-8",
                div {
                    class: "p-3 rounded-2xl bg-primary/20 text-primary",
                    Icon { name: "hub".to_string(), class: "text-[24px]".to_string() }
                }
                div {
                    h2 { class: "text-2xl font-bold text-white tracking-tight", "Mesh Signaling" }
                    p { class: "text-sm text-slate-400 mt-1", "Real-time mesh network state visualization" }
                }
            }

            // Signal Grid
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                for signal in signals.read().iter() {
                    SignalCard { signal: signal.clone() }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct SignalState {
    id: String,
    status: String,
    latency: u32,
    direction: String,
}

#[component]
fn SignalCard(signal: SignalState) -> Element {
    let status_color = match signal.status.as_str() {
        "Connected" => "text-green-400 bg-green-400/10",
        "Signaling" => "text-yellow-400 bg-yellow-400/10",
        "Syncing" => "text-blue-400 bg-blue-400/10",
        _ => "text-gray-400 bg-gray-400/10",
    };

    rsx! {
        div {
            class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md hover:bg-white/10 transition-all",
            div {
                class: "flex justify-between items-start mb-4",
                div {
                    h3 { class: "text-lg font-semibold text-white", "{signal.id}" }
                    p { class: "text-xs text-slate-400", "{signal.direction}" }
                }
                div {
                    class: "px-3 py-1 rounded-full text-xs font-bold {status_color}",
                    "{signal.status}"
                }
            }

            div {
                class: "flex items-center gap-2 mt-4",
                Icon { name: "speed".to_string(), class: "text-slate-400 text-[16px]".to_string() }
                span { class: "text-sm text-slate-300 font-mono", "{signal.latency} ms" }
            }
        }
    }
}
