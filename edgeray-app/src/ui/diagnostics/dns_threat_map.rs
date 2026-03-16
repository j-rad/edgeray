//! DNS Threat Map - Real-time blocked domain visualization
//!
//! Visual threat intelligence dashboard showing blocked domains, query statistics,
//! and geographic threat distribution.

use crate::components::ui::{GlassCard, Icon, PageHeader, SectionHeader};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Blocked domain log entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockedDomain {
    pub id: String,
    pub domain: String,
    pub category: ThreatCategory,
    pub blocked_at: u64,
    pub request_count: u32,
    pub source_ip: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatCategory {
    Malware,
    Phishing,
    Tracking,
    Advertising,
    Adult,
    Gambling,
    SocialMedia,
    Unknown,
}

impl ThreatCategory {
    fn color(&self) -> &'static str {
        match self {
            Self::Malware => "text-red-600 dark:text-red-400",
            Self::Phishing => "text-orange-600 dark:text-orange-400",
            Self::Tracking => "text-yellow-600 dark:text-yellow-400",
            Self::Advertising => "text-blue-600 dark:text-blue-400",
            Self::Adult => "text-pink-600 dark:text-pink-400",
            Self::Gambling => "text-purple-600 dark:text-purple-400",
            Self::SocialMedia => "text-cyan-600 dark:text-cyan-400",
            Self::Unknown => "text-gray-600 dark:text-gray-400",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            Self::Malware => "bug_report",
            Self::Phishing => "phishing",
            Self::Tracking => "visibility",
            Self::Advertising => "campaign",
            Self::Adult => "block",
            Self::Gambling => "casino",
            Self::SocialMedia => "group",
            Self::Unknown => "help",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Malware => "Malware",
            Self::Phishing => "Phishing",
            Self::Tracking => "Tracking",
            Self::Advertising => "Ads",
            Self::Adult => "Adult",
            Self::Gambling => "Gambling",
            Self::SocialMedia => "Social",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Default)]
struct DnsStats {
    total_queries: u64,
    blocked_queries: u64,
    allowed_queries: u64,
    unique_domains_blocked: u32,
    block_rate: f64,
}

#[component]
pub fn DnsThreatMap() -> Element {
    let mut blocked_domains = use_signal(|| Vec::<BlockedDomain>::new());
    let mut stats = use_signal(|| DnsStats::default());
    let mut loading = use_signal(|| false);
    let mut auto_refresh = use_signal(|| true);
    let mut filter_category = use_signal(|| None::<ThreatCategory>);

    // Load blocked domains on mount
    use_effect(move || {
        spawn(async move {
            loading.set(true);

            #[cfg(not(target_arch = "wasm32"))]
            {
                match load_blocked_domains().await {
                    Ok(domains) => {
                        blocked_domains.set(domains.clone());
                        stats.set(calculate_dns_stats(&domains));
                    }
                    Err(e) => log::error!("Failed to load blocked domains: {}", e),
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let mock_domains = generate_mock_blocked_domains();
                blocked_domains.set(mock_domains.clone());
                stats.set(calculate_dns_stats(&mock_domains));
            }

            loading.set(false);
        });
    });

    // Auto-refresh every 5 seconds
    use_effect(move || {
        if auto_refresh() {
            #[cfg(not(target_arch = "wasm32"))]
            {
                spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
                    loop {
                        interval.tick().await;
                        if !auto_refresh() {
                            break;
                        }

                        if let Ok(domains) = load_blocked_domains().await {
                            blocked_domains.set(domains.clone());
                            stats.set(calculate_dns_stats(&domains));
                        }
                    }
                });
            }
        }
    });

    let filtered_domains = use_memo(move || {
        let domains = blocked_domains();
        if let Some(category) = filter_category() {
            domains
                .into_iter()
                .filter(|d| d.category == category)
                .collect()
        } else {
            domains
        }
    });

    rsx! {
        div {
            class: "relative flex h-full min-h-screen w-full flex-col overflow-x-hidden font-display text-slate-900 dark:text-white antialiased",

            // Background
            div { class: "fixed inset-0 bg-[#f8fafc] dark:bg-[#020617] -z-20" }
            div { class: "fixed top-[-20%] left-[-20%] w-[60vw] h-[60vw] bg-red-400/20 dark:bg-red-600/20 rounded-full blur-[120px] pointer-events-none -z-10 mix-blend-multiply dark:mix-blend-screen animate-pulse" }
            div { class: "fixed bottom-[-20%] right-[-20%] w-[60vw] h-[60vw] bg-orange-400/20 dark:bg-orange-600/20 rounded-full blur-[120px] pointer-events-none -z-10 mix-blend-multiply dark:mix-blend-screen" }

            PageHeader {
                title: "DNS Threat Map".to_string(),
                subtitle: Some("Real-time blocked domain monitoring".to_string()),
            }

            main {
                class: "flex-1 flex flex-col px-4 lg:px-8 pb-8 pt-4 z-10 gap-4",

                // Stats overview
                div {
                    class: "grid grid-cols-2 md:grid-cols-5 gap-3",
                    ThreatStatCard {
                        label: "Total Queries",
                        value: format!("{}", stats().total_queries),
                        icon: "dns",
                        color: "text-blue-600 dark:text-blue-400"
                    }
                    ThreatStatCard {
                        label: "Blocked",
                        value: format!("{}", stats().blocked_queries),
                        icon: "block",
                        color: "text-red-600 dark:text-red-400"
                    }
                    ThreatStatCard {
                        label: "Allowed",
                        value: format!("{}", stats().allowed_queries),
                        icon: "check_circle",
                        color: "text-green-600 dark:text-green-400"
                    }
                    ThreatStatCard {
                        label: "Unique Blocked",
                        value: format!("{}", stats().unique_domains_blocked),
                        icon: "domain",
                        color: "text-orange-600 dark:text-orange-400"
                    }
                    ThreatStatCard {
                        label: "Block Rate",
                        value: format!("{:.1}%", stats().block_rate),
                        icon: "shield",
                        color: "text-purple-600 dark:text-purple-400"
                    }
                }

                // Category filter
                GlassCard {
                    class: "p-4",
                    children: rsx! {
                        div { class: "flex items-center justify-between mb-3",
                            SectionHeader {
                                title: "Threat Categories".to_string(),
                                icon: Some("category".to_string())
                            }
                            label {
                                class: "flex items-center gap-2 text-sm text-slate-500 dark:text-gray-400 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    class: "rounded border-white/20 bg-white/5 text-primary focus:ring-primary/50",
                                    checked: auto_refresh(),
                                    onchange: move |evt| auto_refresh.set(evt.checked()),
                                }
                                "Auto-refresh"
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2",
                            CategoryChip {
                                category: None,
                                active: filter_category().is_none(),
                                onclick: move |_| filter_category.set(None),
                            }
                            for cat in [
                                ThreatCategory::Malware,
                                ThreatCategory::Phishing,
                                ThreatCategory::Tracking,
                                ThreatCategory::Advertising,
                                ThreatCategory::Adult,
                                ThreatCategory::Gambling,
                                ThreatCategory::SocialMedia,
                            ] {
                                CategoryChip {
                                    category: Some(cat),
                                    active: filter_category() == Some(cat),
                                    onclick: move |_| filter_category.set(Some(cat)),
                                }
                            }
                        }
                    }
                }

                // Blocked domains list
                div {
                    SectionHeader {
                        title: format!("Blocked Domains ({})", filtered_domains().len()),
                        icon: Some("list".to_string())
                    }
                }

                if loading() {
                    div { class: "flex-1 flex items-center justify-center",
                        div { class: "flex flex-col items-center gap-3",
                            div { class: "size-8 border-2 border-primary border-t-transparent rounded-full animate-spin" }
                            span { class: "text-sm text-slate-500 dark:text-gray-400", "Loading threat data..." }
                        }
                    }
                } else {
                    div {
                        class: "flex-1 overflow-y-auto space-y-2 custom-scrollbar",
                        for domain in filtered_domains() {
                            BlockedDomainCard {
                                key: "{domain.id}",
                                domain: domain.clone(),
                            }
                        }
                        if filtered_domains().is_empty() {
                            div { class: "flex flex-col items-center justify-center py-12 text-center",
                                Icon { name: "check_circle", class: "text-6xl text-green-500 mb-4" }
                                p { class: "text-lg font-semibold text-slate-900 dark:text-white", "No threats detected" }
                                p { class: "text-sm text-slate-500 dark:text-gray-400 mt-1", "Your DNS is clean" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ThreatStatCard(label: String, value: String, icon: String, color: String) -> Element {
    rsx! {
        GlassCard {
            class: "p-4 flex items-center gap-3",
            children: rsx! {
                div { class: format!("p-2 rounded-lg bg-white/10 dark:bg-black/20"),
                    Icon { name: icon, class: format!("{} text-xl", color) }
                }
                div { class: "flex-1 min-w-0",
                    div { class: "text-xs text-slate-500 dark:text-gray-400 uppercase tracking-wide", "{label}" }
                    div { class: format!("text-lg font-bold truncate {}", color), "{value}" }
                }
            }
        }
    }
}

#[component]
fn CategoryChip(
    category: Option<ThreatCategory>,
    active: bool,
    onclick: EventHandler<()>,
) -> Element {
    let (label, icon, color) = if let Some(cat) = category {
        (cat.label(), cat.icon(), cat.color())
    } else {
        ("All", "filter_list", "text-slate-600 dark:text-gray-400")
    };

    rsx! {
        button {
            class: format!(
                "px-3 py-2 rounded-lg text-sm font-medium transition-all flex items-center gap-2 {}",
                if active {
                    "bg-white dark:bg-white/10 shadow-sm ring-2 ring-primary/50"
                } else {
                    "bg-white/50 dark:bg-black/20 hover:bg-white dark:hover:bg-white/5"
                }
            ),
            onclick: move |_| onclick.call(()),
            Icon { name: icon.to_string(), class: format!("text-base {}", color) }
            span { class: color, "{label}" }
        }
    }
}

#[component]
fn BlockedDomainCard(domain: BlockedDomain) -> Element {
    let time_ago = format_time_ago(domain.blocked_at);

    rsx! {
        GlassCard {
            class: "p-4 hover:bg-white/10 dark:hover:bg-white/5 transition-all",
            children: rsx! {
                div { class: "flex items-start gap-4",
                    div { class: format!("p-2 rounded-lg bg-white/10 dark:bg-black/20 shrink-0"),
                        Icon { name: domain.category.icon().to_string(), class: format!("text-xl {}", domain.category.color()) }
                    }
                    div { class: "flex-1 min-w-0",
                        div { class: "flex items-center gap-2 mb-1",
                            span { class: "font-mono text-sm font-semibold text-slate-900 dark:text-white truncate", "{domain.domain}" }
                            span {
                                class: format!("text-[10px] px-2 py-0.5 rounded-full font-medium {}",
                                    match domain.category {
                                        ThreatCategory::Malware => "bg-red-500/20 text-red-600 dark:text-red-400",
                                        ThreatCategory::Phishing => "bg-orange-500/20 text-orange-600 dark:text-orange-400",
                                        ThreatCategory::Tracking => "bg-yellow-500/20 text-yellow-600 dark:text-yellow-400",
                                        ThreatCategory::Advertising => "bg-blue-500/20 text-blue-600 dark:text-blue-400",
                                        _ => "bg-gray-500/20 text-gray-600 dark:text-gray-400",
                                    }
                                ),
                                "{domain.category.label()}"
                            }
                        }
                        div { class: "flex items-center gap-3 text-xs text-slate-500 dark:text-gray-400",
                            span { "🕒 {time_ago}" }
                            span { "🔢 {domain.request_count} requests" }
                            if let Some(country) = &domain.country_code {
                                span { "🌍 {country}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn calculate_dns_stats(domains: &[BlockedDomain]) -> DnsStats {
    let blocked_queries: u64 = domains.iter().map(|d| d.request_count as u64).sum();
    let total_queries = blocked_queries * 2; // Mock: assume 50% block rate
    let allowed_queries = total_queries - blocked_queries;
    let unique_domains_blocked = domains.len() as u32;
    let block_rate = if total_queries > 0 {
        (blocked_queries as f64 / total_queries as f64) * 100.0
    } else {
        0.0
    };

    DnsStats {
        total_queries,
        blocked_queries,
        allowed_queries,
        unique_domains_blocked,
        block_rate,
    }
}

fn format_time_ago(timestamp: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let diff = now.saturating_sub(timestamp);

    if diff < 60 {
        format!("{}s ago", diff)
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_blocked_domains() -> Result<Vec<BlockedDomain>, String> {
    use crate::db;

    let db: &surrealdb::Surreal<surrealdb::engine::local::Db> =
        db::get_db().await.map_err(|e| e.to_string())?;
    let domains: Vec<BlockedDomain> = db
        .select("blocked_domains")
        .await
        .map_err(|e| format!("Failed to query blocked domains: {}", e))?;

    Ok(domains)
}

fn generate_mock_blocked_domains() -> Vec<BlockedDomain> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    vec![
        BlockedDomain {
            id: uuid::Uuid::new_v4().to_string(),
            domain: "malicious-tracker.com".to_string(),
            category: ThreatCategory::Malware,
            blocked_at: now - 120,
            request_count: 15,
            source_ip: Some("192.168.1.100".to_string()),
            country_code: Some("US".to_string()),
        },
        BlockedDomain {
            id: uuid::Uuid::new_v4().to_string(),
            domain: "phishing-site.net".to_string(),
            category: ThreatCategory::Phishing,
            blocked_at: now - 300,
            request_count: 3,
            source_ip: Some("192.168.1.101".to_string()),
            country_code: Some("RU".to_string()),
        },
        BlockedDomain {
            id: uuid::Uuid::new_v4().to_string(),
            domain: "ad-tracker.io".to_string(),
            category: ThreatCategory::Tracking,
            blocked_at: now - 60,
            request_count: 47,
            source_ip: Some("192.168.1.100".to_string()),
            country_code: Some("US".to_string()),
        },
        BlockedDomain {
            id: uuid::Uuid::new_v4().to_string(),
            domain: "doubleclick.net".to_string(),
            category: ThreatCategory::Advertising,
            blocked_at: now - 45,
            request_count: 89,
            source_ip: Some("192.168.1.100".to_string()),
            country_code: Some("US".to_string()),
        },
        BlockedDomain {
            id: uuid::Uuid::new_v4().to_string(),
            domain: "facebook-analytics.com".to_string(),
            category: ThreatCategory::SocialMedia,
            blocked_at: now - 180,
            request_count: 23,
            source_ip: Some("192.168.1.102".to_string()),
            country_code: Some("IE".to_string()),
        },
    ]
}
