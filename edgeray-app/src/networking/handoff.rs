use crate::networking::dialer::IspAwareDialer;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct SeamlessHandoff {
    dialer: Arc<IspAwareDialer>,
    // Notification channel to signal UI or main loop
    event_tx: mpsc::Sender<HandoffEvent>,
}

#[derive(Debug, Clone)]
pub enum HandoffEvent {
    PreDialing(String),
    Switching(String, String), // Old -> New
    Completed(String),
    Failed(String),
}

impl SeamlessHandoff {
    pub fn new(dialer: Arc<IspAwareDialer>, event_tx: mpsc::Sender<HandoffEvent>) -> Self {
        Self { dialer, event_tx }
    }

    /// Execute a seamless handoff to a new server
    pub async fn execute_handoff(&self, current_server_id: &str) {
        log::info!("Initiating seamless handoff from {}", current_server_id);

        // 1. Get best candidates from dialer
        let candidates = self.dialer.get_best_paths();

        // Find a candidate that isn't the current one
        let target = candidates.iter().find(|id| *id != current_server_id);

        if let Some(new_server_id) = target {
            let _ = self
                .event_tx
                .send(HandoffEvent::PreDialing(new_server_id.clone()))
                .await;

            // 2. Pre-Dial: Establish connection BEFORE tearing down old one
            // In a real implementation this would invoke the VpnController to spin up a secondary interface/socket
            log::info!("Pre-dialing target: {}", new_server_id);

            // Simulate connection establishment time
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            // 3. Switch Routing
            let _ = self
                .event_tx
                .send(HandoffEvent::Switching(
                    current_server_id.to_string(),
                    new_server_id.clone(),
                ))
                .await;
            log::info!("Switching routing table to {}", new_server_id);

            // 4. Teardown old connection
            log::info!("Tearing down {}", current_server_id);

            let _ = self
                .event_tx
                .send(HandoffEvent::Completed(new_server_id.clone()))
                .await;
        } else {
            log::warn!("No suitable failover candidates found");
            let _ = self
                .event_tx
                .send(HandoffEvent::Failed("No candidates".to_string()))
                .await;
        }
    }
}
