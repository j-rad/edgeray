use crate::components::ui::{GlassCard, Icon, PrimaryButton, SectionHeader, ToggleItem};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Props, Clone, PartialEq)]
pub struct DnsManagerProps {
    pub on_back: EventHandler<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DnsConfig {
    fake_dns_enabled: bool,
    fake_ip_range: String,
    sniffer_enabled: bool,
    sniffer_protocols: Vec<String>,
    force_doh: bool,
    doh_server: String,
    dnscrypt_enabled: bool,
    dnscrypt_server: String,
    block_ads: bool,
    block_trackers: bool,
    block_malware: bool,
}

impl Default for DnsConfig {
    fn default() -> Self {
        Self {
            fake_dns_enabled: true,
            fake_ip_range: "198.18.0.0/16".to_string(),
            sniffer_enabled: true,
            sniffer_protocols: vec!["http".to_string(), "tls".to_string()],
            force_doh: true,
            doh_server: "https://1.1.1.1/dns-query".to_string(),
            dnscrypt_enabled: false,
            dnscrypt_server: "sdns://AQcAAAAAAAAADDkuOS45Ljk6ODQ0MyBnyEe4yHWM0SAkVUO-dWdG3zTfHYTAC4xHA2jfgh2GPhkyLmRuc2NyeXQuY2VydC5xdWFkOS5uZXQ".to_string(),
            block_ads: true,
            block_trackers: true,
            block_malware: true,
        }
    }
}

#[component]
pub fn DnsManager(props: DnsManagerProps) -> Element {
    let mut config = use_signal(|| DnsConfig::default());
    let mut show_threat_map = use_signal(|| false);

    // Load config on mount
    use_effect(move || {
        spawn(async move {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(loaded_config) = load_dns_config().await {
                    config.set(loaded_config);
                }
            }
        });
    });

    let save_config_handler = move |_| {
        spawn(async move {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Err(e) = save_dns_config(config()).await {
                    log::error!("Failed to save DNS config: {}", e);
                } else {
                    log::info!("✓ DNS configuration saved");
                }
            }
        });
    };

    rsx! {
        div {
            class: "h-full w-full flex flex-col p-4 md:p-6 animate-fade-in-up",

            crate::components::ui::PageHeader {
                title: "DNS & Security",
                subtitle: Some("Advanced DNS resolution and threat protection".to_string()),
                left_action: Some(rsx! {
                    button {
                        class: "p-2 rounded-xl hover:bg-white/10 transition-colors",
                        onclick: move |_| props.on_back.call(()),
                        Icon { name: "arrow_back".to_string(), class: "text-xl text-slate-400 hover:text-white".to_string() }
                    }
                })
            }

            crate::components::ui::GlassPanel {
                class: "flex-1 mt-4 p-4 md:p-6 overflow-y-auto custom-scrollbar flex flex-col gap-8",

                // DNS-over-HTTPS Section
                div {
                    SectionHeader {
                        title: "DNS-over-HTTPS (DoH)".to_string(),
                        icon: Some("https".to_string())
                    }
                    GlassCard {
                        class: "flex flex-col",
                        ToggleItem {
                            label: "Force DNS-over-HTTPS".to_string(),
                            sublabel: Some("Encrypt all DNS queries via HTTPS".to_string()),
                            checked: Signal::new(config().force_doh),
                            last_item: !config().force_doh,
                            onchange: move |checked| {
                                let mut c = config();
                                c.force_doh = checked;
                                config.set(c);
                            }
                        }

                        if config().force_doh {
                            div {
                                class: "p-4 border-t border-white/5 bg-white/5",
                                label { class: "text-xs font-bold text-slate-500 uppercase tracking-wide mb-2 block", "DoH Server" }
                                select {
                                    class: format!("w-full px-4 py-3 rounded-xl text-sm outline-none {}", crate::components::ui::glass::INPUT),
                                    value: "{config().doh_server}",
                                    onchange: move |e| {
                                        let mut c = config();
                                        c.doh_server = e.value();
                                        config.set(c);
                                    },
                                    option { value: "https://1.1.1.1/dns-query", "Cloudflare (1.1.1.1)" }
                                    option { value: "https://dns.google/dns-query", "Google (8.8.8.8)" }
                                    option { value: "https://dns.quad9.net/dns-query", "Quad9 (9.9.9.9)" }
                                    option { value: "https://doh.opendns.com/dns-query", "OpenDNS" }
                                }
                            }
                        }
                    }
                }

                // DNSCrypt Section
                div {
                    SectionHeader {
                        title: "DNSCrypt".to_string(),
                        icon: Some("vpn_lock".to_string())
                    }
                    GlassCard {
                        class: "flex flex-col",
                        ToggleItem {
                            label: "Enable DNSCrypt".to_string(),
                            sublabel: Some("Encrypted DNS protocol with authentication".to_string()),
                            checked: Signal::new(config().dnscrypt_enabled),
                            last_item: !config().dnscrypt_enabled,
                            onchange: move |checked| {
                                let mut c = config();
                                c.dnscrypt_enabled = checked;
                                config.set(c);
                            }
                        }

                        if config().dnscrypt_enabled {
                            div {
                                class: "p-4 border-t border-white/5 bg-white/5",
                                label { class: "text-xs font-bold text-slate-500 uppercase tracking-wide mb-2 block", "DNSCrypt Stamp" }
                                input {
                                    class: format!("w-full px-4 py-3 rounded-xl font-mono text-xs text-slate-200 outline-none {}", crate::components::ui::glass::INPUT),
                                    value: "{config().dnscrypt_server}",
                                    oninput: move |e| {
                                        let mut c = config();
                                        c.dnscrypt_server = e.value();
                                        config.set(c);
                                    }
                                }
                                p { class: "text-[10px] text-slate-500 mt-2", "DNSCrypt server stamp (sdns://...)" }
                            }
                        }
                    }
                }

                // Threat Protection
                div {
                    SectionHeader {
                        title: "Threat Protection".to_string(),
                        icon: Some("shield".to_string())
                    }
                    GlassCard {
                        class: "flex flex-col",
                        ToggleItem {
                            label: "Block Malware Domains".to_string(),
                            sublabel: Some("Prevent connections to known malicious sites".to_string()),
                            checked: Signal::new(config().block_malware),
                            last_item: false,
                            onchange: move |checked| {
                                let mut c = config();
                                c.block_malware = checked;
                                config.set(c);
                            }
                        }
                        ToggleItem {
                            label: "Block Tracking Domains".to_string(),
                            sublabel: Some("Prevent analytics and tracking scripts".to_string()),
                            checked: Signal::new(config().block_trackers),
                            last_item: false,
                            onchange: move |checked| {
                                let mut c = config();
                                c.block_trackers = checked;
                                config.set(c);
                            }
                        }
                        ToggleItem {
                            label: "Block Advertising Domains".to_string(),
                            sublabel: Some("Block ad servers and networks".to_string()),
                            checked: Signal::new(config().block_ads),
                            last_item: true,
                            onchange: move |checked| {
                                let mut c = config();
                                c.block_ads = checked;
                                config.set(c);
                            }
                        }
                    }

                    // Link to Threat Map
                    button {
                        class: format!("{} mt-3 p-4 flex items-center justify-between hover:bg-white/10 dark:hover:bg-white/5 transition-all", crate::components::ui::glass::CARD),
                        onclick: move |_| show_threat_map.set(true),
                        div { class: "flex items-center gap-3",
                            Icon { name: "map".to_string(), class: "text-xl text-primary".to_string() }
                            div {
                                div { class: "font-semibold text-slate-900 dark:text-white", "View Threat Map" }
                                div { class: "text-xs text-slate-500 dark:text-gray-400", "Real-time blocked domain visualization" }
                            }
                        }
                        Icon { name: "arrow_forward".to_string(), class: "text-slate-400".to_string() }
                    }
                }

                // FakeDNS Section
                div {
                    SectionHeader {
                        title: "FakeDNS Engine".to_string(),
                        icon: Some("dns".to_string())
                    }
                    GlassCard {
                        class: "flex flex-col",
                        ToggleItem {
                            label: "Enable FakeDNS".to_string(),
                            sublabel: Some("Improve resolution speed for rules by caching".to_string()),
                            checked: Signal::new(config().fake_dns_enabled),
                            last_item: !config().fake_dns_enabled,
                            onchange: move |checked| {
                                let mut c = config();
                                c.fake_dns_enabled = checked;
                                config.set(c);
                            }
                        }

                        if config().fake_dns_enabled {
                            div {
                                class: "p-4 border-t border-white/5 bg-white/5",
                                label { class: "text-xs font-bold text-slate-500 uppercase tracking-wide mb-2 block", "IP Range" }
                                input {
                                    class: format!("w-full px-4 py-3 rounded-xl font-mono text-sm text-slate-200 outline-none {}", crate::components::ui::glass::INPUT),
                                    value: "{config().fake_ip_range}",
                                    oninput: move |e| {
                                        let mut c = config();
                                        c.fake_ip_range = e.value();
                                        config.set(c);
                                    }
                                }
                                p { class: "text-[10px] text-slate-500 mt-2", "CIDR range used for returning fake IP addresses" }
                            }
                        }
                    }
                }

                // Sniffer Section
                div {
                    SectionHeader {
                        title: "Traffic Sniffing".to_string(),
                        icon: Some("graphic_eq".to_string())
                    }
                    GlassCard {
                        class: "flex flex-col",
                        ToggleItem {
                            label: "Enable Sniffing".to_string(),
                            sublabel: Some("Recover domain names from IP traffic".to_string()),
                            checked: Signal::new(config().sniffer_enabled),
                            last_item: !config().sniffer_enabled,
                            onchange: move |checked| {
                                let mut c = config();
                                c.sniffer_enabled = checked;
                                config.set(c);
                            }
                        }

                        if config().sniffer_enabled {
                            div {
                                class: "p-4 border-t border-white/5 bg-white/5",
                                label { class: "text-xs font-bold text-slate-500 uppercase tracking-wide mb-3 block", "Target Protocols" }
                                div {
                                    class: "grid grid-cols-2 gap-3",
                                    for proto in ["HTTP", "TLS", "QUIC", "BitTorrent"] {
                                        label {
                                            class: "flex items-center gap-3 p-3 rounded-xl bg-black/20 border border-white/5 hover:border-white/20 cursor-pointer transition-all hover:bg-white/5",
                                            input {
                                                "type": "checkbox",
                                                class: "w-4 h-4 rounded border-white/20 bg-black/40 text-primary focus:ring-primary focus:ring-offset-0",
                                                checked: config().sniffer_protocols.contains(&proto.to_lowercase()),
                                                onchange: move |_| {
                                                    let p = proto.to_lowercase();
                                                    let mut c = config();
                                                    if c.sniffer_protocols.contains(&p) {
                                                        c.sniffer_protocols.retain(|x| x != &p);
                                                    } else {
                                                        c.sniffer_protocols.push(p);
                                                    }
                                                    config.set(c);
                                                }
                                            }
                                            span { class: "text-sm font-medium text-slate-300", "{proto}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Action Footer
                div {
                    class: "mt-auto pt-4 flex justify-end",
                    div {
                        class: "w-full md:w-auto",
                        PrimaryButton {
                            label: "Apply Settings".to_string(),
                            icon: Some("save".to_string()),
                            onclick: save_config_handler,
                        }
                    }
                }
            }
        }

        // Threat Map Modal (if needed)
        if show_threat_map() {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm",
                onclick: move |_| show_threat_map.set(false),
                div {
                    class: "w-full h-full max-w-6xl max-h-[90vh] m-4",
                    onclick: move |e| e.stop_propagation(),
                    crate::ui::diagnostics::dns_threat_map::DnsThreatMap {}
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_dns_config() -> Result<DnsConfig, String> {
    use crate::db;

    let db: &surrealdb::Surreal<surrealdb::engine::local::Db> =
        db::get_db().await.map_err(|e| e.to_string())?;
    let config: Option<DnsConfig> = db
        .select(("dns_config", "default"))
        .await
        .map_err(|e| format!("Failed to load DNS config: {}", e))?;

    Ok(config.unwrap_or_default())
}

#[cfg(not(target_arch = "wasm32"))]
async fn save_dns_config(config: DnsConfig) -> Result<(), String> {
    use crate::db;

    let db: &surrealdb::Surreal<surrealdb::engine::local::Db> =
        db::get_db().await.map_err(|e| e.to_string())?;
    let _: Option<DnsConfig> = db
        .update(("dns_config", "default"))
        .content(config)
        .await
        .map_err(|e| format!("Failed to save DNS config: {}", e))?;

    Ok(())
}
