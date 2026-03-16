use crate::components::ui::Icon;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FlowJProProps {
    pub on_back: EventHandler<()>,
}

#[component]
pub fn FlowJPro(props: FlowJProProps) -> Element {
    let mut fec_shards = use_signal(|| 10);
    let mut fec_ratio = use_signal(|| 20); // 20%
    let mut mtu_size = use_signal(|| 1350);
    let mut congestion_control = use_signal(|| "bbr".to_string());

    // New Pro Settings
    let mut port_count = use_signal(|| 1);
    let mut port_mode = use_signal(|| "static".to_string()); // static, dynamic
    let mut packet_padding = use_signal(|| "".to_string());
    let mut zero_rtt = use_signal(|| true);

    // Mock Telemetry Data for 64 ports
    // In production, this would come from a real-time signal
    let port_status = (0..64).map(|i| {
        if i < *port_count.read() {
            if i % 7 == 0 { 2 } // Red
            else if i % 3 == 0 { 1 } // Yellow
            else { 0 } // Green
        } else {
            3 // Inactive (Gray)
        }
    }).collect::<Vec<u8>>();

    rsx! {
        div {
            class: "flex flex-col h-full w-full max-w-4xl mx-auto px-4 py-8 overflow-y-auto custom-scrollbar",

            // Header
            header {
                class: "flex items-center gap-4 mb-8",
                 button {
                    class: "p-2 rounded-xl bg-white/10 hover:bg-white/20 transition-all",
                    onclick: move |_| props.on_back.call(()),
                    Icon { name: "arrow_back".to_string(), class: "text-white text-[20px]".to_string() }
                }
                div {
                    h2 { class: "text-2xl font-bold text-white tracking-tight", "Expert Flow-J Tuning" }
                    p { class: "text-sm text-slate-400 mt-1", "Advanced packet recovery and congestion management" }
                }
            }

            // Multiport Configuration (Pro)
            div {
                class: "mb-6 p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                h3 { class: "text-lg font-semibold text-white mb-6 flex items-center gap-2",
                    Icon { name: "hub".to_string(), class: "text-primary".to_string() }
                    "Flow-J Multiport"
                }

                div { class: "space-y-8",
                    // Port Count Slider
                    div { class: "space-y-3",
                        div { class: "flex justify-between",
                            span { class: "text-xs text-slate-400 font-bold uppercase", "Active Ports" }
                            span { class: "text-xs text-primary font-mono", "{port_count}" }
                        }
                        input {
                            "type": "range",
                            class: "w-full h-1.5 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary",
                            min: "1",
                            max: "64",
                            value: "{port_count}",
                            oninput: move |e| port_count.set(e.value().parse().unwrap_or(1))
                        }
                    }

                    // Port Mode Toggle
                    div { class: "flex items-center justify-between",
                        span { class: "text-xs text-slate-400 font-bold uppercase", "Distribution Mode" },
                        div { class: "flex bg-black/40 p-1 rounded-lg",
                            button {
                                class: format!("px-4 py-1.5 rounded-md text-xs font-bold transition-all {}", if *port_mode.read() == "static" { "bg-primary text-void shadow" } else { "text-slate-500 hover:text-white" }),
                                onclick: move |_| port_mode.set("static".to_string()),
                                "Static Range"
                            }
                            button {
                                class: format!("px-4 py-1.5 rounded-md text-xs font-bold transition-all {}", if *port_mode.read() == "dynamic" { "bg-primary text-void shadow" } else { "text-slate-500 hover:text-white" }),
                                onclick: move |_| port_mode.set("dynamic".to_string()),
                                "Dynamic Random"
                            }
                        }
                    }

                    // Port Telemetry Grid
                    div {
                        class: "p-4 bg-black/40 rounded-xl border border-white/5",
                        div { class: "text-[10px] text-slate-500 font-bold uppercase mb-3", "Port Activity Map" }
                        div { class: "grid grid-cols-16 gap-1",
                            for (i, status) in port_status.iter().enumerate() {
                                div {
                                    class: format!("aspect-square rounded-sm transition-colors duration-300 {}", match status {
                                        0 => "bg-emerald-500 shadow-[0_0_4px_rgba(16,185,129,0.5)]", // Green (Active)
                                        1 => "bg-yellow-500 shadow-[0_0_4px_rgba(234,179,8,0.5)]", // Yellow (Congested)
                                        2 => "bg-red-500 shadow-[0_0_4px_rgba(239,68,68,0.5)]", // Red (Lossy)
                                        _ => "bg-white/5", // Inactive
                                    }),
                                    title: format!("Port {}", i + 1)
                                }
                            }
                        }
                    }
                }
            }

            // H3/QUIC & Packet Tuning
            div {
                class: "mb-6 p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                h3 { class: "text-lg font-semibold text-white mb-6 flex items-center gap-2",
                    Icon { name: "speed".to_string(), class: "text-primary".to_string() }
                    "Transport & Obfuscation"
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // Congestion Control
                    div {
                        label { class: "block text-[10px] text-slate-500 uppercase font-bold mb-2", "Congestion Algorithm" }
                        select {
                            class: "w-full bg-black/20 border border-white/10 rounded-xl px-4 py-3 text-sm text-slate-200 focus:border-primary outline-none appearance-none",
                            value: "{congestion_control}",
                            onchange: move |e| congestion_control.set(e.value()),
                            option { value: "bbr", "BBR (Google / High Throughput)" }
                            option { value: "cubic", "CUBIC (Standard / Low CPU)" }
                            option { value: "newreno", "NewReno (Legacy / Stable)" }
                        }
                    }

                    // MTU
                    div {
                        label { class: "block text-[10px] text-slate-500 uppercase font-bold mb-2", "Target MTU" }
                        input {
                            class: "w-full bg-black/20 border border-white/10 rounded-xl px-4 py-3 text-sm text-slate-200 focus:border-primary outline-none",
                            "type": "number",
                            value: "{mtu_size}",
                            oninput: move |e| mtu_size.set(e.value().parse().unwrap_or(1350))
                        }
                    }

                    // 0-RTT Toggle
                    div {
                        class: "flex items-center justify-between p-3 rounded-xl bg-black/20 border border-white/10",
                        div {
                            span { class: "block text-sm font-bold text-slate-200", "0-RTT Handshake" }
                            span { class: "text-[10px] text-slate-500", "Faster connection resumption" }
                        }
                         button {
                            class: format!("w-12 h-6 rounded-full p-1 transition-colors {}", if *zero_rtt.read() { "bg-primary" } else { "bg-slate-700" }),
                            onclick: move |_| {
                                let val = *zero_rtt.read();
                                zero_rtt.set(!val);
                            },
                            div { class: format!("w-4 h-4 bg-white rounded-full shadow-md transform transition-transform {}", if *zero_rtt.read() { "translate-x-6" } else { "translate-x-0" }) }
                        }
                    }
                }

                // Packet Padding Editor
                div { class: "mt-6",
                    label { class: "block text-[10px] text-slate-500 uppercase font-bold mb-2", "Packet Padding (HEX/Regex)" }
                    div { class: "relative",
                        input {
                            class: "w-full bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-sm font-mono text-emerald-400 placeholder-white/20 focus:border-primary outline-none",
                            placeholder: "e.g. 16:AF:3B or ^(GET|POST)",
                            value: "{packet_padding}",
                            oninput: move |e| packet_padding.set(e.value())
                        }
                        div { class: "absolute right-3 top-3 text-[10px] text-slate-500 font-mono", "HEX" }
                    }
                    p { class: "text-[10px] text-slate-500 mt-2", "Randomizes packet length to evade DPI fingerprinting." }
                }
            }

            // FEC Settings (Existing, slightly styled)
            div {
                class: "p-6 rounded-3xl bg-white/5 border border-white/10 backdrop-blur-md",
                h3 { class: "text-lg font-semibold text-white mb-6 flex items-center gap-2",
                    Icon { name: "healing".to_string(), class: "text-primary".to_string() }
                    "Forward Error Correction (FEC)"
                }

                div {
                    class: "space-y-8",
                    // Shards
                    div {
                        class: "space-y-3",
                        div { class: "flex justify-between",
                            span { class: "text-xs text-slate-400 font-bold uppercase", "Data Shards" }
                            span { class: "text-xs text-primary font-mono", "{fec_shards}" }
                        }
                        input {
                            "type": "range",
                            class: "w-full h-1.5 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary",
                            min: "1",
                            max: "100",
                            value: "{fec_shards}",
                            oninput: move |e| fec_shards.set(e.value().parse().unwrap_or(10))
                        }
                    }

                    // Ratio
                    div {
                        class: "space-y-3",
                        div { class: "flex justify-between",
                            span { class: "text-xs text-slate-400 font-bold uppercase", "Redundancy Ratio" }
                            span { class: "text-xs text-primary font-mono", "{fec_ratio}%" }
                        }
                        input {
                            "type": "range",
                            class: "w-full h-1.5 bg-slate-700 rounded-lg appearance-none cursor-pointer accent-primary",
                            min: "0",
                            max: "100",
                            value: "{fec_ratio}",
                            oninput: move |e| fec_ratio.set(e.value().parse().unwrap_or(20))
                        }
                    }
                }
            }

            // Warning
            div {
                class: "mt-6 p-4 rounded-2xl bg-yellow-500/10 border border-yellow-500/20 flex gap-3",
                Icon { name: "warning".to_string(), class: "text-yellow-500 flex-shrink-0".to_string() }
                p { class: "text-xs text-yellow-500/80 leading-relaxed",
                    "Incorrect FEC or MTU settings can drastically decrease performance or cause connection drops. Only modify these if you understand the underlying network topology."
                }
            }

            // Save Button
            button {
                class: "mt-8 w-full py-4 rounded-2xl bg-primary text-white font-bold hover:bg-primary/90 transition-all shadow-lg shadow-primary/20",
                "Apply Expert Tuning"
            }
        }
    }
}
