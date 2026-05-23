use crate::models::ServerConfig;
use crate::networking::monitor::ConnectionMonitor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IspInfo {
    pub name: String,
    pub country_code: String,
    pub asn: String, // Changed to String to handle "AS12345" format
    pub isp_code: IspCode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IspCode {
    Mci,
    Irancell,
    Rightel,
    Shatel,
    Asiatech,
    Mokhaberat,
    Unknown,
}

#[derive(Clone)]
pub struct IspAwareDialer {
    current_isp: Option<IspInfo>,
    // Mapping of ISP Code to list of preferred server IDs (or tags)
    best_paths: HashMap<IspCode, Vec<String>>,
    // GeoIP Database Reader (shared)
    // We store the path to the DB, or load it into memory?
    // maxminddb::Reader is not Clone, so we wrap in Arc<Mutex> or load on demand.
    // For performance, we should keep it open.
    // However, for this implementation step without modifying strict struct bounds,
    // we'll open it on demand or use a static/lazy initialization if possible.
    // Or we just store the path.
    db_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IpApiResponse {
    status: String,
    country: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    isp: String,
    org: String,
    #[serde(rename = "as")]
    asn: String,
    query: String,
}

impl IspAwareDialer {
    pub fn new() -> Self {
        let mut best_paths = HashMap::new();

        // Default Optimizations for Iranian ISPs
        best_paths.insert(
            IspCode::Mci,
            vec!["mci-optimized".to_string(), "cloudfront".to_string()],
        );
        best_paths.insert(
            IspCode::Irancell,
            vec!["irancell-optimized".to_string(), "cloudflare".to_string()],
        );
        best_paths.insert(
            IspCode::Mokhaberat,
            vec!["tci-optimized".to_string(), "arvan".to_string()],
        );

        // Try to find the DB
        let db_path = if std::path::Path::new("assets/GeoLite2-ASN.mmdb").exists() {
            Some("assets/GeoLite2-ASN.mmdb".to_string())
        } else {
            None
        };

        Self {
            current_isp: None,
            best_paths,
            db_path,
        }
    }

    /// Detect current ISP using MaxMind DB (local) or IP-API (fallback)
    pub async fn detect_isp(&mut self) -> Result<IspInfo, String> {
        // 1. Get Public IP
        let mut builder = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        {
            builder = builder.timeout(std::time::Duration::from_secs(5));
        }

        let client = builder.build().map_err(|e| e.to_string())?;

        let public_ip_str = client
            .get("https://api.ipify.org")
            .send()
            .await
            .map_err(|_| "Failed to fetch public IP".to_string())?
            .text()
            .await
            .map_err(|_| "Failed to read public IP".to_string())?;

        let public_ip: IpAddr = public_ip_str
            .parse()
            .map_err(|_| "Invalid IP format".to_string())?;

        // 2. Try MaxMind DB Lookup
        if let Some(path) = &self.db_path {
            if let Ok(reader) = maxminddb::Reader::open_readfile(path) {
                // Look up ASN
                // MaxMind ASN database usually returns a struct with autonomous_system_organization and autonomous_system_number
                #[derive(Deserialize, Debug)]
                struct AsnEntry<'a> {
                    autonomous_system_organization: Option<&'a str>,
                    autonomous_system_number: Option<u32>,
                }

                if let Ok(asn_entry) = reader.lookup::<AsnEntry>(public_ip) {
                    let org = asn_entry
                        .autonomous_system_organization
                        .unwrap_or("Unknown")
                        .to_string();
                    let asn_num = asn_entry.autonomous_system_number.unwrap_or(0);
                    let asn_str = format!("AS{}", asn_num);

                    let isp_code = self.resolve_isp_code(&asn_str, &org);

                    let info = IspInfo {
                        name: org,
                        country_code: "XX".to_string(), // GeoLite2-ASN doesn't have country, would need Country DB.
                        asn: asn_str,
                        isp_code,
                    };

                    self.current_isp = Some(info.clone());
                    return Ok(info);
                }
            }
        }

        // 3. Fallback to IP-API
        let url =
            "http://ip-api.com/json/?fields=status,message,country,countryCode,isp,org,as,query";

        let resp: IpApiResponse = client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?
            .json()
            .await
            .map_err(|e| format!("JSON parse error: {}", e))?;

        if resp.status != "success" {
            return Err("IP-API returned fail status".to_string());
        }

        let isp_code = self.resolve_isp_code(&resp.asn, &resp.isp);

        let info = IspInfo {
            name: resp.isp,
            country_code: resp.country_code,
            asn: resp.asn,
            isp_code,
        };

        self.current_isp = Some(info.clone());
        Ok(info)
    }

    fn resolve_isp_code(&self, asn_str: &str, isp_name: &str) -> IspCode {
        let asn = asn_str.to_uppercase();
        let name = isp_name.to_lowercase();

        // Rough heuristic matching for major Iranian ISPs
        if asn.contains("AS197207")
            || name.contains("mobile communication company of iran")
            || name.contains("mci")
        {
            return IspCode::Mci;
        }
        if asn.contains("AS44244") || name.contains("irancell") {
            return IspCode::Irancell;
        }
        if asn.contains("AS57218") || name.contains("rightel") {
            return IspCode::Rightel;
        }
        if asn.contains("AS31549") || name.contains("shatel") {
            return IspCode::Shatel;
        }
        if asn.contains("AS43754") || name.contains("asiatech") {
            return IspCode::Asiatech;
        }
        if asn.contains("AS58224")
            || name.contains("telecommunication company of iran")
            || name.contains("tci")
        {
            return IspCode::Mokhaberat;
        }

        IspCode::Unknown
    }

    /// Get best server paths for the detected ISP
    pub fn get_best_paths(&self) -> Vec<String> {
        if let Some(isp) = &self.current_isp {
            if let Some(paths) = self.best_paths.get(&isp.isp_code) {
                return paths.clone();
            }
        }
        // Fallback or generic content
        vec!["general-pool".to_string()]
    }

    /// Check monitor stats and recommend failover if necessary
    pub fn check_failover(&self, monitor: &ConnectionMonitor) -> bool {
        let stats = monitor.get_stats();

        // Fast-Failover: Jitter > 300ms
        if stats.jitter_ms > 300 {
            log::warn!(
                "High jitter detected ({:.2}ms). Triggering failover.",
                stats.jitter_ms
            );
            return true;
        }

        // Standard Loss Failover
        if stats.packet_loss_percent > 15.0 {
            log::warn!(
                "High packet loss detected ({:.2}%). Triggering failover.",
                stats.packet_loss_percent
            );
            return true;
        }

        false
    }

    /// Update routing knowledge base dynamically
    pub fn update_knowledge_base(&mut self, isp: IspCode, server_ids: Vec<String>) {
        self.best_paths.insert(isp, server_ids);
    }

    /// Manually set the current ISP (useful for testing or override)
    pub fn set_manual_isp(&mut self, isp: IspInfo) {
        self.current_isp = Some(isp);
    }

    /// Rank nodes based on ISP match and Path Quality
    pub fn rank_nodes(
        &self,
        servers: Vec<ServerConfig>,
        _monitor: &ConnectionMonitor,
    ) -> Vec<ServerConfig> {
        let mut scored_servers: Vec<(ServerConfig, f64)> = servers
            .into_iter()
            .map(|s| {
                let mut score = 0.0;

                // 1. ISP Affinity
                if let Some(current_isp) = &self.current_isp {
                    if let Some(preferred) = self.best_paths.get(&current_isp.isp_code) {
                        // Check if server ID or remarks match preference
                        if let Some(id) = &s.id {
                            if preferred.contains(id) {
                                score += 50.0;
                            }
                        }
                        if preferred
                            .iter()
                            .any(|tag| s.remarks.to_lowercase().contains(tag))
                        {
                            score += 30.0;
                        }
                    }
                }

                // 2. Path Quality (if available)
                // We assume monitor has stats accessible via some ID or address
                // For now, we simulate stats lookup.
                // In real impl, monitor.get_stats(s.id)

                // let stats = monitor.get_stats_for(&s.id);
                // score -= stats.latency * 0.1;
                // score -= stats.jitter * 0.5;
                // score -= stats.loss * 10.0;

                (s, score)
            })
            .collect();

        // Sort descending by score
        scored_servers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scored_servers.into_iter().map(|(s, _)| s).collect()
    }

    /// Recommend a switch if current connection is degrading
    pub fn recommend_switch(
        &self,
        monitor: &ConnectionMonitor,
        current_server: &ServerConfig,
        available_servers: Vec<ServerConfig>,
    ) -> Option<ServerConfig> {
        if self.check_failover(monitor) {
            // Find best alternative
            let ranked = self.rank_nodes(available_servers, monitor);

            // Return top ranked that isn't current
            for s in ranked {
                if s.id != current_server.id {
                    log::info!("Zero-Drop Handoff: Recommending switch to {}", s.remarks);
                    return Some(s);
                }
            }
        }
        None
    }
}
