use crate::components::GlassCard;
use crate::constants::get_tracer_steps;
use crate::types::TracerStatus;
use dioxus::prelude::*;

#[component]
pub fn TracerPage() -> Element {
    let steps = get_tracer_steps();

    rsx! {
        div {
            class: "animate-fade-in pb-12",

            div {
                class: "flex flex-col md:flex-row md:items-end justify-between gap-4 mb-6 sm:mb-8",
                div {
                    h2 { class: "text-[10px] sm:text-xs font-bold uppercase tracking-[0.2em] text-gray-500 mb-1", "Network Analysis" }
                    h1 { class: "text-xl sm:text-2xl font-bold text-white tracking-tight", "Packet Tracer" }
                }

                div {
                    class: "flex items-center gap-3",
                    input {
                        r#type: "text",
                        placeholder: "Enter domain or IP...",
                        class: "h-10 px-4 bg-white/5 border border-white/10 rounded-xl text-sm text-white placeholder:text-gray-500 focus:border-primary/50 focus:outline-none transition-colors w-48 sm:w-64"
                    }
                    button {
                        class: "h-10 px-4 bg-primary/10 border border-primary/20 hover:bg-primary/20 text-primary rounded-xl flex items-center gap-2 text-xs font-bold uppercase tracking-wider transition-all active:scale-95",
                        svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M22 2L11 13M22 2l-7 20-4-9-9-4 20-7z" } }
                        "Trace"
                    }
                }
            }

            // Trace Steps
            div {
                class: "relative",

                // Vertical line
                div { class: "absolute left-5 sm:left-7 top-0 bottom-0 w-px bg-gradient-to-b from-primary via-purple to-emerald" }

                div {
                    class: "space-y-4",
                    for (idx, step) in steps.iter().enumerate() {
                        {
                            let indicator_class = match step.status {
                                TracerStatus::Success => "border-emerald/50 shadow-[0_0_20px_rgba(0,255,163,0.3)]",
                                TracerStatus::Active => "border-primary/50 shadow-[0_0_20px_rgba(34,211,238,0.3)] animate-pulse-fast",
                                _ => "border-white/10",
                            };
                            let num_class = match step.status {
                                TracerStatus::Success => "text-emerald",
                                TracerStatus::Active => "text-primary",
                                _ => "text-gray-500",
                            };
                            let status_class = match step.status {
                                TracerStatus::Success => "bg-emerald/20 text-emerald",
                                TracerStatus::Active => "bg-primary/20 text-primary",
                                _ => "bg-white/10 text-gray-400",
                            };
                            let status_text = match step.status {
                                TracerStatus::Success => "Success",
                                TracerStatus::Active => "Active",
                                TracerStatus::Pending => "Pending",
                                TracerStatus::Warning => "Warning",
                            };
                            let glow = match step.color.as_str() {
                                "primary" => "cyan",
                                "cyber-purple" => "purple",
                                "success-emerald" => "emerald",
                                _ => "none",
                            };

                            rsx! {
                                div {
                                    class: "relative flex gap-4 sm:gap-6",

                                    // Step indicator
                                    div {
                                        class: "relative z-10 w-10 h-10 sm:w-14 sm:h-14 rounded-full glass-panel flex items-center justify-center shrink-0 {indicator_class}",
                                        span { class: "text-sm sm:text-lg font-bold {num_class}", "{idx + 1}" }
                                    }

                                    // Step content
                                    GlassCard {
                                        glow: glow.to_string(),
                                        class: "flex-1 !p-3 sm:!p-4".to_string(),

                                        div {
                                            class: "flex items-start justify-between mb-2",
                                            div {
                                                h3 { class: "font-bold text-sm sm:text-base text-white", "{step.title}" }
                                                p { class: "text-[10px] sm:text-xs text-gray-400", "{step.subtitle}" }
                                            }
                                            div {
                                                class: "px-2 py-1 rounded-md text-[9px] font-bold uppercase tracking-wider {status_class}",
                                                "{status_text}"
                                            }
                                        }

                                        div {
                                            class: "flex flex-wrap gap-3",
                                            for detail in &step.details {
                                                {
                                                    let color_class = detail.color.as_deref().unwrap_or("text-white");
                                                    rsx! {
                                                        div {
                                                            class: "flex items-center gap-2",
                                                            span { class: "text-[10px] text-gray-500", "{detail.label}:" }
                                                            span { class: "text-xs font-mono {color_class}", "{detail.value}" }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
