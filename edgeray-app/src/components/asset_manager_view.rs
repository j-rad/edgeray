//! Asset Manager View Component
//!
//! Manages GeoIP and GeoSite database files for routing.
//! Provides download, update, and deletion functionality.

use super::ui::{GlassCard, Icon, PageHeader};
use crate::i18n::t;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    GeoIp,
    GeoSite,
}

impl AssetType {
    fn name(&self) -> &'static str {
        match self {
            AssetType::GeoIp => "GeoIP",
            AssetType::GeoSite => "GeoSite",
        }
    }

    #[allow(dead_code)]
    fn filename(&self) -> &'static str {
        match self {
            AssetType::GeoIp => "geoip.dat",
            AssetType::GeoSite => "geosite.dat",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            AssetType::GeoIp => "public",
            AssetType::GeoSite => "language",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            AssetType::GeoIp => "IP address geolocation database for routing by country/region",
            AssetType::GeoSite => "Domain categorization database for routing by site category",
        }
    }

    fn default_url(&self) -> &'static str {
        match self {
            AssetType::GeoIp => {
                "https://github.com/Chocolate4U/Iran-v2ray-rules/releases/latest/download/geoip.dat"
            }
            AssetType::GeoSite => {
                "https://github.com/Chocolate4U/Iran-v2ray-rules/releases/latest/download/geosite.dat"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    pub asset_type: AssetType,
    pub size_bytes: Option<u64>,
    pub last_updated: Option<String>,
    pub is_downloading: bool,
    pub download_progress: f32,
}

impl Asset {
    fn format_size(&self) -> String {
        match self.size_bytes {
            Some(bytes) => {
                if bytes < 1024 {
                    format!("{} B", bytes)
                } else if bytes < 1024 * 1024 {
                    format!("{:.1} KB", bytes as f64 / 1024.0)
                } else {
                    format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
                }
            }
            None => "—".to_string(),
        }
    }
}

#[component]
pub fn AssetManagerView(on_back: EventHandler<()>) -> Element {
    let trans = t();

    // Asset state (in production, load from filesystem)
    let mut assets = use_signal(|| {
        vec![
            Asset {
                asset_type: AssetType::GeoIp,
                size_bytes: Some(4_500_000),
                last_updated: Some("2025-12-15".to_string()),
                is_downloading: false,
                download_progress: 0.0,
            },
            Asset {
                asset_type: AssetType::GeoSite,
                size_bytes: Some(2_100_000),
                last_updated: Some("2025-12-14".to_string()),
                is_downloading: false,
                download_progress: 0.0,
            },
        ]
    });

    let download_asset = EventHandler::new(move |asset_type: AssetType| {
        spawn(async move {
            tracing::info!(
                "Downloading {} from {}",
                asset_type.name(),
                asset_type.default_url()
            );

            // Update state to show downloading
            assets
                .write()
                .iter_mut()
                .find(|a| a.asset_type == asset_type)
                .map(|a| {
                    a.is_downloading = true;
                    a.download_progress = 0.0;
                });

            // Simulate download (in production, use reqwest with progress tracking)
            for i in 0..=10 {
                #[cfg(not(target_arch = "wasm32"))]
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                #[cfg(target_arch = "wasm32")]
                // On WASM, we skip the delay for simulation
                {}
                assets
                    .write()
                    .iter_mut()
                    .find(|a| a.asset_type == asset_type)
                    .map(|a| a.download_progress = i as f32 / 10.0);
            }

            // Update state after download
            assets
                .write()
                .iter_mut()
                .find(|a| a.asset_type == asset_type)
                .map(|a| {
                    a.is_downloading = false;
                    a.download_progress = 0.0;
                    a.last_updated = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
                });

            tracing::info!("Downloaded {} successfully", asset_type.name());
        });
    });

    let delete_asset = EventHandler::new(move |asset_type: AssetType| {
        tracing::info!("Deleting {}", asset_type.name());
        assets
            .write()
            .iter_mut()
            .find(|a| a.asset_type == asset_type)
            .map(|a| {
                a.size_bytes = None;
                a.last_updated = None;
            });
    });

    rsx! {
        div {
            class: "flex flex-col h-full pb-32 overflow-y-auto no-scrollbar",

            // Header
            PageHeader {
                title: trans.assets.title.clone(),
                left_action: Some(rsx! {
                    button {
                        class: "text-primary font-bold text-sm cursor-pointer hover:text-white transition-colors",
                         onclick: move |_| on_back.call(()),
                        Icon { name: "arrow_back".to_string(), class: "text-lg".to_string() }
                    }
                }),
                right_action: None,
            }

            div {
                class: "p-4 space-y-6",

                // Info Section
                section {
                    div {
                        class: "bg-primary/10 border border-primary/20 rounded-xl p-4 mb-4 shadow-glow-cyan/5",
                        div {
                            class: "flex items-start gap-3",
                            Icon { name: "info".to_string(), class: "text-primary text-xl mt-0.5".to_string() }
                            div {
                                class: "flex-1 text-xs text-white/80",
                                p { class: "font-medium mb-1", "About Routing Assets" }
                                p { class: "text-white/60",
                                    "GeoIP and GeoSite databases enable advanced routing based on geographic location and domain categories. "
                                    "These files are required for 'Bypass Mainland' mode and custom routing rules."
                                }
                            }
                        }
                    }
                }

                // Assets List
                section {
                    h4 {
                        class: "text-xs font-bold text-white/40 uppercase tracking-wider ml-2 mb-2",
                        "Available Assets"
                    }

                    div {
                        class: "space-y-3",
                        for asset in assets.read().iter() {
                            AssetCard {
                                asset: asset.clone(),
                                on_download: move |asset_type| download_asset(asset_type),
                                on_delete: move |asset_type| delete_asset(asset_type),
                            }
                        }
                    }
                }

                // Sources Section
                section {
                    h4 {
                        class: "text-xs font-bold text-white/40 uppercase tracking-wider ml-2 mb-2",
                        "Data Sources"
                    }
                    GlassCard {
                        div {
                            class: "p-3 space-y-3 text-xs",
                            div {
                                class: "flex items-start gap-2",
                                Icon { name: "link".to_string(), class: "text-white/40 text-sm mt-0.5".to_string() }
                                div {
                                    p { class: "text-white/80 font-medium", "v2fly/geoip" }
                                    p { class: "text-white/40 text-[10px] font-mono break-all",
                                        "github.com/v2fly/geoip"
                                    }
                                }
                            }
                            div {
                                class: "flex items-start gap-2",
                                Icon { name: "link".to_string(), class: "text-white/40 text-sm mt-0.5".to_string() }
                                div {
                                    p { class: "text-white/80 font-medium", "Chocolate4U/Iran-v2ray-rules" }
                                    p { class: "text-white/40 text-[10px] font-mono break-all",
                                        "github.com/Chocolate4U/Iran-v2ray-rules"
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

#[component]
fn AssetCard(
    asset: Asset,
    on_download: EventHandler<AssetType>,
    on_delete: EventHandler<AssetType>,
) -> Element {
    let trans = t();
    let is_installed = asset.size_bytes.is_some();

    rsx! {
        GlassCard {
            div {
                class: "p-4",
                div {
                    class: "flex items-start gap-3",
                    // Icon
                    div {
                        class: "w-12 h-12 bg-gradient-to-br from-primary to-purple-500 rounded-xl flex items-center justify-center flex-shrink-0 shadow-glow-cyan/20",
                        Icon { name: asset.asset_type.icon().to_string(), class: "text-2xl text-white".to_string() }
                    }

                    // Info
                    div {
                        class: "flex-1 min-w-0",
                        h3 {
                            class: "text-sm font-bold text-white mb-1",
                            "{asset.asset_type.name()}"
                        }
                        p {
                            class: "text-[10px] text-white/50 mb-2",
                            "{asset.asset_type.description()}"
                        }

                        if is_installed {
                            div {
                                class: "flex items-center gap-3 text-[10px] text-white/60",
                                div {
                                    class: "flex items-center gap-1",
                                    Icon { name: "storage".to_string(), class: "text-xs".to_string() }
                                    span { "{asset.format_size()}" }
                                }
                                if let Some(date) = &asset.last_updated {
                                    div {
                                        class: "flex items-center gap-1",
                                        Icon { name: "schedule".to_string(), class: "text-xs".to_string() }
                                        span { "{date}" }
                                    }
                                }
                            }
                        } else {
                            p {
                                class: "text-[10px] text-yellow-400/80",
                                "Not installed"
                            }
                        }
                    }
                }

                // Download Progress
                if asset.is_downloading {
                    div {
                        class: "mt-3",
                        div {
                            class: "h-1.5 bg-black/30 rounded-full overflow-hidden",
                            div {
                                class: "h-full bg-gradient-to-r from-primary to-purple-500 transition-all duration-300",
                                style: format!("width: {}%", asset.download_progress * 100.0),
                            }
                        }
                        p {
                            class: "text-[10px] text-white/60 mt-1",
                            "{trans.assets.downloading} {(asset.download_progress * 100.0) as u32}%"
                        }
                    }
                }

                // Actions
                if !asset.is_downloading {
                    div {
                        class: "flex gap-2 mt-3",
                        button {
                            class: "flex-1 flex items-center justify-center gap-1 py-2 px-3 bg-primary/20 text-primary text-xs font-bold rounded-lg border border-primary/50 shadow-glow-cyan hover:bg-primary/30 transition-all",
                            onclick: move |_| on_download.call(asset.asset_type.clone()),
                            Icon {
                                name: format!("{}", if is_installed { "refresh" } else { "download" }),
                                class: "text-sm".to_string()
                            }
                            span { if is_installed { "{trans.assets.update}" } else { "{trans.assets.download}" } }
                        }
                        if is_installed {
                            button {
                                class: "py-2 px-3 bg-red-500/20 text-red-400 text-xs font-medium rounded-lg hover:bg-red-500/30 transition-colors",
                                onclick: move |_| on_delete.call(asset.asset_type.clone()),
                                Icon { name: "delete".to_string(), class: "text-sm".to_string() }
                            }
                        }
                    }
                }
            }
        }
    }
}
