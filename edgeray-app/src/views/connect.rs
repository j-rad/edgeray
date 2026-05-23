//! Connection View for EdgeRay
//!
//! Handles the high-level connection logic, ISP analysis, and
//! adaptive transport selection based on network conditions.

use crate::components::dashboard::ConnectionState;
use dioxus::prelude::*;
use log::{info, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum TransportType {
    Standard,
    WebRTC,
    GhostMode,
}

#[component]
pub fn ConnectView() -> Element {
    let mut conn_state = use_signal(|| ConnectionState::Disconnected);
    let mut transport = use_signal(|| TransportType::Standard);
    let mut isp_info = use_signal(|| "Ready to Connect".to_string());
    let mut analyzing = use_signal(|| false);

    let handle_connect = move |_| {
        spawn(async move {
            info!("Initiating Connection Adaptive Handshake...");
            conn_state.set(ConnectionState::Connecting);

            // Step 1: Start ISP Analysis (Mocks ProbeTransport)
            auto_select_transport(transport, isp_info, analyzing).await;

            // Step 2: Establish Tunnel
            tokio::time::sleep(std::time::Duration::from_millis(800)).await;
            conn_state.set(ConnectionState::Connected);
            info!("Connection established via {:?}", *transport.read());
        });
    };

    rsx! {
        div {
            class: "flex flex-col items-center justify-center space-y-6 p-8 glass rounded-3xl",

            div {
                class: "text-center",
                h2 { class: "text-2xl font-bold text-white", "Adaptive Core" }
                if *analyzing.read() {
                    p { class: "text-primary text-sm animate-pulse flex items-center justify-center gap-2", 
                        span { class: "material-symbols-outlined animate-spin", "radar" }
                        "{isp_info}"
                    }
                } else {
                    p { class: "text-slate-400 text-sm", "{isp_info}" }
                }
            }

            button {
                class: format!(
                    "px-8 py-4 rounded-2xl font-bold transition-all transform hover:scale-105 active:scale-95 {}",
                    if *conn_state.read() == ConnectionState::Connected {
                        "bg-emerald-500/20 text-emerald-400 border border-emerald-500/50"
                    } else {
                        "bg-primary/20 text-primary border border-primary/50"
                    }
                ),
                onclick: handle_connect,
                "{conn_state.read().label()}"
            }

            if *transport.read() == TransportType::WebRTC {
                div {
                    class: "px-4 py-2 bg-amber-500/10 border border-amber-500/30 rounded-xl text-amber-400 text-xs animate-pulse",
                    "WebRTC Stealth Mode Active"
                }
            }
        }
    }
}

/// Automatically selects the best transport based on ISP/Network probes.
pub async fn auto_select_transport(
    mut transport_signal: Signal<TransportType>,
    mut info_signal: Signal<String>,
    mut analyzing_signal: Signal<bool>,
) {
    info!("Performing ProbeTransport ISP analysis...");
    analyzing_signal.set(true);
    info_signal.set("Analyzing Network Interference...".to_string());

    // In a real implementation, this would call the gRPC probe service
    // let probe_results = rustray_client.probe().await?;

    // Mock logic for demonstration:
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let is_censored = true; // Mock detection
    if is_censored {
        warn!("Censorship detected. Switching to WebRTC Stealth Transport.");
        transport_signal.set(TransportType::WebRTC);
        info_signal.set("Network: Restricted (Stealth Enabled)".to_string());
    } else {
        transport_signal.set(TransportType::Standard);
        info_signal.set("Network: Open (Standard Mode)".to_string());
    }
    analyzing_signal.set(false);
}
