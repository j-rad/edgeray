//! Subscription View Component
//!
//! Renders the subscription management and upgrade plan screen.

use crate::components::ui::Icon;
use dioxus::prelude::*;

#[component]
pub fn SubscriptionView(on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "relative z-10 flex h-full min-h-screen w-full flex-col overflow-x-hidden max-w-md lg:max-w-2xl mx-auto shadow-gw-glow glass-panel animate-slide-up no-scrollbar",

            crate::components::ui::PageHeader {
                title: "Subscription".to_string(),
                left_action: Some(rsx! {
                    button {
                        class: "group flex items-center justify-center rounded-full p-2 hover:bg-white/10 transition-all",
                        onclick: move |_| on_close.call(()),
                        Icon { name: "close".to_string(), class: "text-gray-400 group-hover:text-white transition-colors text-[24px]".to_string() }
                    }
                }),
                right_action: Some(rsx! {
                    button {
                        class: "flex items-center justify-center rounded-full py-1 px-3 bg-transparent hover:bg-white/10 transition-colors border border-transparent hover:border-primary/30",
                        p { class: "text-primary text-sm font-bold leading-normal text-glow-cyan", "Restore" }
                    }
                })
            }

            // Main Content
            main {
                class: "flex-1 px-6 py-8 space-y-8 stagger-entrance",

                // Active Subscription Card
                section {
                    div {
                        class: "glass-panel-hover rounded-3xl p-6 relative overflow-hidden group hover:shadow-glow-cyan transition-all duration-300 border border-white/5",

                        // Background Ambient
                        div { class: "absolute top-0 right-0 w-64 h-64 bg-primary/5 rounded-full blur-[80px] -translate-y-1/2 translate-x-1/2" }

                        div {
                            class: "relative z-10",
                            div {
                                class: "flex justify-between items-start mb-6",
                                div {
                                    div {
                                        class: "flex items-center gap-2 mb-2",
                                        crate::components::ui::Badge { label: "Active".to_string(), variant: "success".to_string() }
                                        span { class: "text-xs text-gray-400 font-medium tracking-wide", "Basic Plan" }
                                    }
                                    h1 { class: "text-3xl font-bold tracking-tight text-white drop-shadow-md", "Monthly" }
                                }
                                div {
                                    class: "flex items-center justify-center w-12 h-12 rounded-2xl bg-gradient-to-br from-white/10 to-white/5 backdrop-blur-md border border-white/10 shadow-inner group-hover:scale-110 transition-transform duration-300",
                                    Icon { name: "rocket_launch".to_string(), class: "text-[24px] text-primary drop-shadow-lg".to_string() }
                                }
                            }
                            div {
                                class: "space-y-6",
                                // Expiry & Auto-renewal
                                    div {
                                        class: "flex justify-between items-end p-4 rounded-xl bg-black/40 border border-white/5 backdrop-blur-sm",
                                        div {
                                            p { class: "text-[10px] text-gray-500 uppercase tracking-widest font-bold mb-1", "Expires on" }
                                            p { class: "text-lg font-medium font-mono text-white tracking-tight", "2024-12-15" }
                                        }
                                        div {
                                            class: "text-right",
                                            p { class: "text-[10px] text-gray-500 uppercase tracking-widest font-bold mb-1", "Auto-renewal" }
                                            div {
                                                class: "flex items-center justify-end gap-1.5",
                                                span { class: "text-base font-medium text-emerald-400 text-glow-emerald", "On" }
                                                Icon { name: "check_circle".to_string(), class: "text-[18px] text-emerald-400".to_string() }
                                            }
                                        }
                                    }
                                // Traffic Usage
                                div {
                                    div {
                                        class: "flex justify-between text-xs text-gray-400 mb-2 px-1 font-medium",
                                        span { "Traffic Used" }
                                        span { class: "font-mono text-primary", "85%" }
                                    }
                                    div {
                                        class: "w-full bg-black/40 rounded-full h-2.5 overflow-hidden backdrop-blur-sm border border-white/5",
                                        div {
                                            class: "bg-gradient-to-r from-primary to-purple-500 h-full rounded-full shadow-[0_0_15px_rgba(0,240,255,0.5)] relative",
                                            style: "width: 85%",
                                            div { class: "absolute right-0 top-0 bottom-0 w-[2px] bg-white/50 blur-[1px]" } // Leading edge shine
                                        }
                                    }
                                    div {
                                        class: "flex justify-between text-[10px] text-gray-500 mt-2 font-mono px-1",
                                        span { "425 GB" }
                                        span { "500 GB" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Upgrade Plan Section
                section {
                    crate::components::ui::SectionHeader {
                        title: "Upgrade Plan".to_string(),
                        action: Some(rsx! {
                            div {
                                class: "flex p-1 bg-black/40 rounded-xl backdrop-blur-md border border-white/5 shadow-inner",
                                button { class: "px-4 py-1.5 rounded-lg text-xs font-bold bg-white/10 text-white shadow-sm transition-all border border-white/5", "Monthly" }
                                button { class: "px-4 py-1.5 rounded-lg text-xs font-bold text-gray-500 hover:text-white transition-all hover:bg-white/5", "Yearly" }
                            }
                        })
                    }

                    div {
                        class: "space-y-4 pt-2",
                        PlanOption { plan_name: "Basic".to_string(), price: "$4.99".to_string(), description: "Current Plan".to_string(), is_current: true }
                        PlanOption {
                            plan_name: "Pro".to_string(),
                            price: "$9.99".to_string(),
                            description: "Everything in Basic +".to_string(),
                            is_recommended: true,
                            features: vec![
                                ("bolt", "Unlimited High Speed"),
                                ("public", "50+ Global Locations"),
                                ("devices", "5 Devices"),
                            ]
                        }
                        PlanOption { plan_name: "Premium".to_string(), price: "$14.99".to_string(), description: "Best for streaming".to_string() }
                    }
                }
            }
             // Footer
            footer {
                class: "mt-auto px-6 pb-8 pt-4",
                crate::components::ui::PrimaryButton {
                    label: "Upgrade to Pro".to_string(),
                    icon: Some("arrow_forward".to_string()),
                    onclick: move |_| {} // Todo: add upgrade handler
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PlanOptionProps {
    plan_name: String,
    price: String,
    description: String,
    #[props(default = false)]
    is_current: bool,
    #[props(default = false)]
    is_recommended: bool,
    #[props(default = Vec::new())]
    features: Vec<(&'static str, &'static str)>,
}

#[component]
fn PlanOption(props: PlanOptionProps) -> Element {
    rsx! {
        crate::components::ui::GlassCard {
            class: if props.is_recommended {
                "relative flex flex-col p-5 border border-primary/40 bg-primary/5 shadow-glow-cyan hover:shadow-glow-cyan-intense transition-all duration-300"
            } else if !props.is_current {
                "relative flex flex-col p-5 cursor-pointer hover:bg-white/10 hover:-translate-y-0.5 transition-all duration-300 border border-white/5 hover:border-white/20 hover:shadow-lg"
            } else {
                 "relative flex flex-col p-5 bg-black/40 border border-white/5 opacity-80"
            },
            div {
                if props.is_recommended {
                    {rsx! {
                        div {
                            class: "absolute -top-3 right-5 bg-gradient-to-r from-primary to-blue-500 text-white text-[10px] font-bold px-3 py-1 pt-3.5 rounded-b-lg uppercase tracking-wide shadow-[0_0_15px_rgba(0,240,255,0.4)] z-10",
                            "Recommended"
                        }
                    }}
                }
                div {
                    class: "relative z-10 flex items-center justify-between w-full",
                    div {
                        class: "flex items-center gap-4",
                        div {
                            class: "flex h-5 w-5 items-center justify-center rounded-full border border-white/20 transition-all",
                            if props.is_recommended {
                                div { class: "h-3 w-3 rounded-full bg-primary shadow-glow-cyan animate-pulse-slow" }
                            } else if !props.is_current {
                                 div { class: "h-2 w-2 rounded-full bg-transparent group-hover:bg-white/20" }
                            } else {
                                div { class: "h-2 w-2 rounded-full bg-gray-500" }
                            }
                        }
                        div {
                            p { class: "text-sm font-bold text-white tracking-wide", "{props.plan_name}" }
                            p { class: "text-[11px] text-gray-400 font-medium", "{props.description}" }
                        }
                    }
                    div {
                        class: "text-right",
                        p { class: "text-base font-bold text-white font-mono", "{props.price}" }
                        p { class: "text-[10px] text-gray-500", "/mo" }
                    }
                }
                 if !props.features.is_empty() {
                    {rsx! {
                        div {
                            class: "relative z-10 pl-9 mt-4 space-y-2.5",
                            for (icon, text) in &props.features {
                                div {
                                    class: "flex items-center gap-2.5 text-xs font-medium text-gray-300",
                                    div {
                                        class: "p-1 rounded-full bg-primary/10 border border-primary/20",
                                        Icon { name: icon.to_string(), class: "text-[12px] text-primary".to_string() }
                                    }
                                    span { class: "text-gray-200", "{text}" }
                                }
                            }
                        }
                    }}
                }
            }
        }
    }
}
