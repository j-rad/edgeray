use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub packet_loss_percent: f32,
    pub jitter_ms: u32,
    pub rtt_ms: u32,
    pub last_update: Instant,
}

pub struct ConnectionMonitor {
    stats: Arc<Mutex<ConnectionStats>>,
    failover_threshold_loss: f32,
    failover_threshold_jitter: u32,
}

impl ConnectionMonitor {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(ConnectionStats {
                packet_loss_percent: 0.0,
                jitter_ms: 0,
                rtt_ms: 0,
                last_update: Instant::now(),
            })),
            failover_threshold_loss: 20.0,  // 20% loss trigger
            failover_threshold_jitter: 200, // 200ms jitter trigger
        }
    }

    pub fn update_stats(&self, loss: f32, jitter: u32, rtt: u32) {
        let mut stats = self.stats.lock().unwrap();
        stats.packet_loss_percent = loss;
        stats.jitter_ms = jitter;
        stats.rtt_ms = rtt;
        stats.last_update = Instant::now();
    }

    pub fn should_trigger_failover(&self) -> bool {
        let stats = self.stats.lock().unwrap();
        // Check if stats are stale (> 10 seconds old)
        if stats.last_update.elapsed() > Duration::from_secs(10) {
            return false;
        }

        if stats.packet_loss_percent > self.failover_threshold_loss {
            return true;
        }

        if stats.jitter_ms > self.failover_threshold_jitter {
            return true;
        }

        false
    }

    pub fn get_stats(&self) -> ConnectionStats {
        self.stats.lock().unwrap().clone()
    }
}
