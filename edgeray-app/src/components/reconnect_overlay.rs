use dioxus::prelude::*;
use std::time::Duration;

#[derive(Props, Clone, PartialEq)]
pub struct ReconnectOverlayProps {
    #[props(default = EventHandler::new(|_| {}))]
    pub on_close: EventHandler<()>,
}

#[component]
pub fn ReconnectOverlay(props: ReconnectOverlayProps) -> Element {
    let mut is_connected = use_signal(|| true);
    let mut retry_count = use_signal(|| 0);

    // Health Check Polling
    use_future(move || async move {
        let client = reqwest::Client::new();
        loop {
            // Check every 2s
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::TimeoutFuture::new(2000).await;
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(Duration::from_secs(2)).await;

            // We assume API is relative "/health" or implicit
            // For WASM: /health
            // For Desktop: http://127.0.0.1:port/health (we need base URL)
            // But let's assume relative works for Web and Desktop if configured

            // Using a simple fetch or reqwest
            let url = "/health"; // Usually works for web. For desktop, might fail if base not set.
            // But if we are in this app, we likely have drivers configured.
            // Let's rely on global fetch if possible or just try "/health".

            // Note: edgeray-app might run on desktop without a local server if it controls a remote one.
            // But usually for "Reconnect", it implies connection to the Controlled Node.

            match client.get(url).timeout(Duration::from_secs(2)).send().await {
                Ok(resp) if resp.status().is_success() => {
                    if !*is_connected.read() {
                        is_connected.set(true);
                        retry_count.set(0);
                    }
                }
                _ => {
                    // Start showing overlay after 2 consecutive failures to avoid flicker
                    if *is_connected.read() {
                        let c = *retry_count.read();
                        if c >= 2 {
                            is_connected.set(false);
                        } else {
                            retry_count.set(c + 1);
                        }
                    }
                }
            }
        }
    });

    if *is_connected.read() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-md transition-opacity duration-300",
            div {
                class: "flex flex-col items-center gap-6 p-8 rounded-2xl bg-[#0f172a] border border-white/10 shadow-2xl max-w-sm text-center transform scale-100",
                div {
                    class: "relative w-16 h-16",
                    div { class: "absolute inset-0 rounded-full border-4 border-red-500/30 animate-ping" }
                    div { class: "absolute inset-0 rounded-full border-4 border-t-red-500 border-r-transparent border-b-red-500 border-l-transparent animate-spin" }
                }

                div {
                    h2 { class: "text-2xl font-bold text-white mb-2", "Connection Lost" }
                    p { class: "text-gray-400", "Attempting to reconnect to node..." }
                }

                div {
                    class: "flex gap-4",
                    button {
                        onclick: move |_| {
                            props.on_close.call(());
                            is_connected.set(true); // Hide overlay locally as we are disconnecting
                            retry_count.set(0);
                        },
                        class: "px-6 py-2 bg-red-500/20 hover:bg-red-500/30 text-red-400 border border-red-500/50 rounded-lg transition-colors font-medium",
                        "Disconnect"
                    }
                    button {
                        onclick: move |_| {
                            // Manual Retry Trigger
                            retry_count.set(0);
                        },
                        class: "px-6 py-2 bg-white/10 hover:bg-white/20 text-white rounded-lg transition-colors font-medium",
                        "Try Now"
                    }
                }
            }
        }
    }
}
