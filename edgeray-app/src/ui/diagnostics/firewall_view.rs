use crate::components::ui::Icon;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FirewallViewProps {
    pub on_back: EventHandler<()>,
}

#[component]
pub fn FirewallView(props: FirewallViewProps) -> Element {
    let mut nft_rules = use_resource(move || async move {
        // Mock fetching nftables rules
        vec![
            NftRule {
                table: "inet edgeray",
                chain: "prerouting",
                rule: "ip daddr 10.0.0.0/24 return",
                comment: "Bypass LAN",
            },
            NftRule {
                table: "inet edgeray",
                chain: "prerouting",
                rule: "meta l4proto tcp tproxy to :12345 meta mark set 1",
                comment: "TProxy TCP",
            },
            NftRule {
                table: "inet edgeray",
                chain: "output",
                rule: "meta mark 1 return",
                comment: "Avoid loop",
            },
            NftRule {
                table: "nat edgeray",
                chain: "postrouting",
                rule: "oifname \"eth0\" masquerade",
                comment: "NAT for clients",
            },
        ]
    });

    rsx! {
        div {
            class: "h-full w-full flex flex-col p-4 md:p-6 animate-fade-in-up",

            // Standard Page Header
            crate::components::ui::PageHeader {
                title: "Firewall Diagnostics",
                subtitle: Some("Real-time nftables rules managed by EdgeRay".to_string()),
                left_action: Some(rsx! {
                    button {
                        class: "p-2 rounded-xl hover:bg-white/10 transition-colors",
                        onclick: move |_| props.on_back.call(()),
                        Icon { name: "arrow_back".to_string(), class: "text-xl text-gray-400 hover:text-white".to_string() }
                    }
                }),
                right_action: Some(rsx! {
                    button {
                        class: "flex items-center gap-2 px-3 py-1.5 rounded-lg bg-primary/20 text-primary font-bold text-xs ring-1 ring-primary/30 hover:bg-primary/30 transition-all",
                        onclick: move |_| nft_rules.restart(),
                        Icon { name: "refresh".to_string(), class: "text-sm".to_string() }
                        "Refresh"
                    }
                })
            }

            // Main Content
            crate::components::ui::GlassPanel {
                class: "flex-1 mt-4 overflow-hidden flex flex-col relative",

                // Rules Table Container
                div {
                    class: "flex-1 overflow-y-auto custom-scrollbar",
                    table {
                        class: "w-full text-left border-collapse",
                        thead {
                            class: "sticky top-0 bg-black/60 backdrop-blur-md z-10 border-b border-white/10",
                            tr {
                                th { class: "px-6 py-4 text-[10px] font-bold uppercase tracking-widest text-gray-500", "Table/Chain" }
                                th { class: "px-6 py-4 text-[10px] font-bold uppercase tracking-widest text-gray-500", "Rule Definition" }
                                th { class: "px-6 py-4 text-[10px] font-bold uppercase tracking-widest text-gray-500", "Comment" }
                            }
                        }
                        tbody {
                            class: "divide-y divide-white/5",
                            if let Some(rules) = nft_rules.read().as_ref() {
                                for r in rules {
                                    tr {
                                        class: "hover:bg-white/5 transition-colors group",
                                        td {
                                            class: "px-6 py-4",
                                            div { class: "flex items-center gap-2",
                                                Icon { name: "shield".to_string(), class: "text-primary text-xs".to_string() }
                                                span { class: "text-xs font-bold text-white", "{r.table}" }
                                            }
                                            p { class: "text-[10px] text-gray-500 font-mono pl-5", "{r.chain}" }
                                        }
                                        td {
                                            class: "px-6 py-4",
                                            code { class: "text-xs text-gray-300 font-mono break-all bg-black/40 px-2 py-1 rounded border border-white/5 shadow-inner", "{r.rule}" }
                                        }
                                        td {
                                            class: "px-6 py-4",
                                            span { class: "text-[10px] px-2 py-1 rounded bg-secondary/10 text-secondary border border-secondary/20 italic", "{r.comment}" }
                                        }
                                    }
                                }
                            } else {
                                tr { td { colspan: 3, class: "p-10 text-center text-slate-500 animate-pulse", "Fetching active rules..." } }
                            }
                        }
                    }
                }

                // Footer Status
                div {
                    class: "p-3 border-t border-white/5 bg-white/5 backdrop-blur-md flex items-center justify-between",
                    div {
                        class: "flex items-center gap-2",
                        div { class: "w-2 h-2 rounded-full bg-green-400 animate-ping shadow-[0_0_8px_rgba(74,222,128,0.5)]" }
                        span { class: "text-[10px] text-gray-400 uppercase tracking-widest font-bold", "Monitoring Active" }
                    }
                    span { class: "text-[10px] text-gray-500 font-mono", "v0.12.4" }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct NftRule {
    table: &'static str,
    chain: &'static str,
    rule: &'static str,
    comment: &'static str,
}
