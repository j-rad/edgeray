// edgeray-app/src/services/mtu_prober.rs
#[cfg(not(target_arch = "wasm32"))]
use dioxus::prelude::spawn;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicU16, Ordering};
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::broadcast;

/// Result of an MTU probe
#[derive(Debug, Clone)]
pub struct MtuProbeResult {
    pub mtu: u16,
    pub latency: u32,
    pub packet_loss: f32,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct MtuProber {
    current_mtu: Arc<AtomicU16>,
    tx: broadcast::Sender<MtuProbeResult>,
}

#[cfg(not(target_arch = "wasm32"))]
impl MtuProber {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            current_mtu: Arc::new(AtomicU16::new(1500)),
            tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<MtuProbeResult> {
        self.tx.subscribe()
    }

    pub fn get_mtu(&self) -> u16 {
        self.current_mtu.load(Ordering::SeqCst)
    }

    /// Start automatic MTU discovery
    pub async fn start_discovery(&self, target: String) {
        log::info!("Starting MTU discovery for target: {}", target);

        // Simulate binary search: 1500 -> 1492 -> 1480 -> 1300
        // In real implementation, this would use `ping -M do -s size`

        let tx = self.tx.clone();
        let mtu = self.current_mtu.clone();

        spawn(async move {
            let sizes = [1500, 1492, 1480, 1460, 1450, 1400, 1360, 1280];

            for &size in &sizes {
                // Mock network delay and check
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                let success = size <= 1460; // Mock constraint

                if success {
                    mtu.store(size, Ordering::SeqCst);
                    let _ = tx.send(MtuProbeResult {
                        mtu: size,
                        latency: 45, // Mock latency
                        packet_loss: 0.0,
                    });
                    log::info!("MTU discovery settled on: {}", size);
                    break;
                }
            }
        });
    }
}

/// WASM Stub for MtuProber
#[cfg(target_arch = "wasm32")]
pub struct MtuProber;

#[cfg(target_arch = "wasm32")]
impl MtuProber {
    pub fn new() -> Self {
        Self
    }
    pub fn get_mtu(&self) -> u16 {
        1500
    }
    pub async fn start_discovery(&self, _target: String) {
        log::warn!("MTU discovery not supported on WASM");
    }
}
