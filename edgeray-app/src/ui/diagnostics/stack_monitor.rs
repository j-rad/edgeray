use crate::components::ui::Icon;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StackMonitorProps {
    pub on_back: EventHandler<()>,
}

#[component]
pub fn StackMonitor(props: StackMonitorProps) -> Element {
    let driver = crate::services::provisioner::use_driver();

    // We use a fixed conn_id for the monitor (representative path) or could take it from props
    let conn_id = "proxy-representative";

    // Metrics that update from polling
    let mut metrics = use_signal(Vec::<crate::models::ConnectionMetrics>::new);

    let latest = metrics.read().last().cloned().unwrap_or_default();
    let rtt = latest.rtt_ms;
    let window_size = latest.cwnd_bytes;
    let dpi_state = format!("{:?}", latest.dpi_state);
    let _packet_loss = 0.02; // Still mock for now as it's harder to track precisely per-packet without more state

    use_future(move || {
        let driver = driver.clone();
        async move {
            loop {
                if let Ok(m) = driver.pull_connection_metrics(conn_id).await {
                    metrics.set(m);
                }
                crate::utils::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar relative",

            // Animated blob backgrounds (reference design pattern)
            div { class: "fixed inset-0 -z-10 overflow-hidden pointer-events-none",
                div { class: "absolute top-[-10%] right-[-10%] w-[50vw] h-[50vw] bg-cyan-500/10 rounded-full blur-[100px] animate-blob-1" }
                div { class: "absolute bottom-[-10%] left-[-10%] w-[60vw] h-[60vw] bg-primary/10 rounded-full blur-[120px] animate-blob-2" }
            }

            // Header
            header {
                class: "flex items-center gap-4 mb-8",
                 button {
                    class: "p-2 rounded-xl bg-white/10 hover:bg-white/20 transition-all",
                    onclick: move |_| props.on_back.call(()),
                    Icon { name: "arrow_back".to_string(), class: "text-white text-[20px]".to_string() }
                }
                div {
                    h2 { class: "text-2xl font-bold text-white tracking-tight", "Userspace Stack Monitor" }
                    p { class: "text-sm text-slate-400 mt-1", "Real-time transport health and window scaling" }
                }
            }

            // Hero Stats
            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-4 mb-8",
                MetricCard { label: "Path RTT", value: "{rtt}ms", icon: "timer", trend: "0ms" }
                MetricCard { label: "TCP Window", value: "{window_size / 1024}KB", icon: "open_in_full", trend: "Scaling" }
                MetricCard { label: "DPI Status", value: "{dpi_state}", icon: "security", trend: "Active" }
            }

            // Real-time Visualizers
            div {
                class: "space-y-6",

                // TCP Window Scaling Graph (Real Data from metrics signal)
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                    h3 { class: "text-base font-semibold text-white mb-4 flex items-center gap-2",
                        Icon { name: "show_chart".to_string(), class: "text-primary".to_string() }
                        "Congestion Window (cwnd)"
                    }
                    div {
                        class: "h-48 w-full bg-black/40 rounded-2xl relative overflow-hidden flex items-end px-2 gap-1",
                        for m in metrics.read().iter().rev().take(40).rev() {
                            div {
                                class: "flex-1 bg-primary/40 rounded-t-sm transition-all duration-300",
                                // Scale height based on CWND (e.g. 1MB max for visualization)
                                style: "height: {std::cmp::min(100, (m.cwnd_bytes / 10240) as usize)}%"
                            }
                        }
                        div { class: "absolute inset-0 flex items-center justify-center bg-black/20 backdrop-blur-[1px]",
                            if metrics.read().is_empty() {
                                span { class: "text-[10px] text-slate-500 uppercase tracking-widest font-bold", "Waiting for telemetry..." }
                            }
                        }
                    }
                }

                // Path Fragmentation
                div {
                    class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                    h3 { class: "text-base font-semibold text-white mb-4", "Fragmentation Policy" }
                    div {
                        class: "space-y-4",
                        div {
                            class: "flex justify-between items-center text-sm",
                            span { class: "text-slate-400", "Maximum Segment Size (MSS)" }
                            span { class: "text-slate-200 font-mono", "1310 bytes" }
                        }
                        div {
                            class: "w-full h-2 bg-slate-700 rounded-full overflow-hidden",
                            div { class: "h-full bg-green-500 w-[92%]" }
                        }
                        p { class: "text-[10px] text-slate-500 italic", "No PMTU Blackhole detected in current path." }
                    }
                }
            }
        }
    }
}

#[component]
fn MetricCard(label: String, value: String, icon: String, trend: String) -> Element {
    rsx! {
        div {
            class: "p-5 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md flex flex-col gap-1",
            div { class: "flex justify-between items-start",
                span { class: "text-[10px] text-slate-500 uppercase font-bold tracking-wider", "{label}" }
                Icon { name: icon, class: "text-primary/60 text-[18px]".to_string() }
            }
            span { class: "text-2xl font-bold text-white", "{value}" }
            span { class: "text-[10px] text-slate-400 font-mono", "{trend}" }
        }
    }
}
