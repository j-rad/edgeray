//! Subscription Manager Service
//!
//! Handles background tasks for subscription management, including:
//! - Auto-updating subscriptions based on intervals
//! - Latency benchmarking to find the best server
//! - Saving updated server lists to the database

#[cfg(not(target_arch = "wasm32"))]
use crate::db;
use crate::models::ServerConfig;
#[cfg(not(target_arch = "wasm32"))]
use crate::subscription::update_subscription;
#[cfg(not(target_arch = "wasm32"))]
use futures::stream::{self, StreamExt};
#[cfg(not(target_arch = "wasm32"))]
use tokio::net::TcpStream;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::{Duration, sleep};

/// Manages subscription updates and server benchmarking
pub struct SubscriptionManager;

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
impl SubscriptionManager {
    /// Start the background auto-update loop for subscriptions
    ///
    /// This spawns a Tokio task that runs indefinitely. It checks for subscriptions
    /// that are due for update every 60 seconds.
    pub fn start_background_loop() {
        tokio::spawn(async move {
            log::info!("Starting SubscriptionManager background loop");
            loop {
                // Check every minute
                sleep(Duration::from_secs(60)).await;

                match db::list_subscriptions().await {
                    Ok(subs) => {
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();

                        for mut sub in subs {
                            if !sub.enabled {
                                continue;
                            }

                            let last = sub.last_update.unwrap_or(0);
                            // Default to 1 hour if 0
                            let interval = if sub.update_interval == 0 {
                                3600
                            } else {
                                sub.update_interval
                            };

                            if now.saturating_sub(last) >= interval {
                                log::info!("Auto-updating subscription: {}", sub.name);
                                let mut updated = false;

                                // Try URLs in order
                                for url in &sub.urls {
                                    log::info!("Fetching subscription from: {}", url);
                                    match update_subscription(url).await {
                                        Ok(servers) => {
                                            log::info!(
                                                "Fetched {} servers. Saving...",
                                                servers.len()
                                            );
                                            // Save servers to DB using the subscription name as group
                                            if let Err(e) =
                                                db::save_subscription_group(&sub.name, servers)
                                                    .await
                                            {
                                                log::error!(
                                                    "Failed to save subscription group '{}': {}",
                                                    sub.name,
                                                    e
                                                );
                                            } else {
                                                // Success
                                                updated = true;
                                            }
                                            break;
                                        }
                                        Err(e) => {
                                            log::warn!(
                                                "Failed to fetch subscription URL '{}': {}",
                                                url,
                                                e
                                            );
                                        }
                                    }
                                }

                                if updated {
                                    sub.last_update = Some(now);
                                    if let Err(e) = db::save_subscription(sub.clone()).await {
                                        log::error!(
                                            "Failed to update subscription metadata: {}",
                                            e
                                        );
                                    } else {
                                        log::info!(
                                            "Subscription '{}' updated successfully",
                                            sub.name
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to list subscriptions for auto-update: {}", e);
                    }
                }
            }
        });
    }

    /// Find the best performing server from the database
    ///
    /// Performs a TCP connect latency test on all servers (excluding those in `exclude_ids`).
    /// Returns the server with the lowest latency.
    ///
    /// # Arguments
    /// * `exclude_ids` - List of server IDs to skip (e.g. currently failing ones)
    pub async fn get_best_performing_server(exclude_ids: &[String]) -> Option<ServerConfig> {
        let mut servers: Vec<ServerConfig> = match db::list_servers().await {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to list servers: {}", e);
                return None;
            }
        };

        // Filter out excluded servers
        if !exclude_ids.is_empty() {
            servers.retain(|s| s.id.as_ref().map_or(true, |id| !exclude_ids.contains(id)));
        }

        if servers.is_empty() {
            return None;
        }

        log::info!(
            "Benchmarking {} servers to find the best node...",
            servers.len()
        );

        // Limit concurrency to avoid file descriptor exhaustion
        // We ping 10 at a time
        let results: Vec<(ServerConfig, Option<u128>)> = stream::iter(servers)
            .map(|server| async move {
                let rtt = Self::ping_server(&server.address, server.port).await;
                (server, rtt)
            })
            .buffer_unordered(10)
            .collect()
            .await;

        let best = results
            .into_iter()
            .filter_map(|(s, rtt)| rtt.map(|t| (s, t)))
            .min_by_key(|(_, rtt)| *rtt);

        if let Some((ref s, rtt)) = best {
            log::info!("Best server found: {} ({}ms)", s.remarks, rtt);
        } else {
            log::warn!("No reachable servers found.");
        }

        best.map(|(s, _)| s)
    }

    /// Sync a specific subscription by ID
    pub async fn sync_subscription(id: &str) -> anyhow::Result<usize> {
        let mut sub = db::list_subscriptions()
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?
            .into_iter()
            .find(|s| s.id == id)
            .ok_or_else(|| anyhow::anyhow!("Subscription not found"))?;

        let mut all_fetched_servers: Vec<ServerConfig> = Vec::new();

        for url in &sub.urls {
            match update_subscription(url).await {
                Ok(servers) => {
                    all_fetched_servers.extend(servers);
                }
                Err(e) => {
                    log::warn!("Failed to fetch {} in sync: {}", url, e);
                }
            }
        }

        if all_fetched_servers.is_empty() {
            return Err(anyhow::anyhow!("No servers found in any URL"));
        }

        let total_servers = all_fetched_servers.len();

        // Save to DB
        db::save_subscription_group(&sub.name, all_fetched_servers.clone())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        // Update metadata
        sub.last_update = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        );
        sub.node_count = total_servers;
        db::save_subscription(sub)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(total_servers)
    }

    /// Measure TCP connect latency to a target
    async fn ping_server(host: &str, port: u16) -> Option<u128> {
        let start = std::time::Instant::now();
        // Timeout after 2 seconds
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(2),
            TcpStream::connect((host, port)),
        )
        .await
        {
            Ok(Ok(_)) => Some(start.elapsed().as_millis()),
            _ => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl SubscriptionManager {
    pub fn start_background_loop() {}
    pub async fn get_best_performing_server(_exclude_ids: &[String]) -> Option<ServerConfig> {
        None
    }
    pub async fn sync_subscription(_id: &str) -> anyhow::Result<usize> {
        Err(anyhow::anyhow!("Not supported in WASM"))
    }
}
