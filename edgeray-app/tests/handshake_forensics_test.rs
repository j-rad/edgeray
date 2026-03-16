//! Handshake forensics test suite
//!
//! Tests for Phase 3 forensics components:
//! - ForensicsTracer: stage color mapping, history capacity, failure→Amber/Red states
//! - RoutingCanvas: layout engine, outbound position, dot duration, Flow-J jitter

#[cfg(test)]
mod tests {
    use edgeray_app::ui::diagnostics::forensics_tracer::{
        ForensicsEntry, ForensicsHistory, HandshakeStage, MAX_FORENSICS_ENTRIES, TraceStatus,
    };
    use edgeray_app::ui::diagnostics::routing_canvas::{
        CanvasNode, NodeType, compute_layout, dot_duration_s, flowj_jitter_offset,
        outbound_position,
    };
    use edgeray_app::ui::diagnostics::switchboard::{ConnectionState, TlsConnectionInfo};

    // ─── TLS Connection Info (existing tests) ──────────────────────────────

    #[test]
    fn test_tls_connection_info() {
        let conn = TlsConnectionInfo {
            id: "test_conn".to_string(),
            local_port: 54321,
            remote_host: "example.com".to_string(),
            remote_port: 443,
            sni: "example.com".to_string(),
            tls_version: "TLS 1.3".to_string(),
            cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
            utls_fingerprint: "771,4865-4866-4867,0-23-65281".to_string(),
            state: ConnectionState::Established,
            handshake_duration_ms: 45,
            bytes_sent: 1024,
            bytes_received: 2048,
            established_at: 1234567890,
        };

        assert_eq!(conn.local_port, 54321);
        assert_eq!(conn.remote_port, 443);
        assert_eq!(conn.state, ConnectionState::Established);
        assert!(conn.handshake_duration_ms < 100);
    }

    #[test]
    fn test_connection_state_transitions() {
        let states = vec![
            ConnectionState::Handshaking,
            ConnectionState::Established,
            ConnectionState::Closing,
            ConnectionState::Closed,
        ];

        assert_eq!(states.len(), 4);
        assert_ne!(states[0], states[1]);
        assert_ne!(states[1], states[2]);
        assert_ne!(states[2], states[3]);
    }

    #[test]
    fn test_utls_fingerprint_format() {
        let fingerprints = vec![
            "771,4865-4866-4867,0-23-65281",
            "769,49195-49199-52393,0-23-65281-10",
            "772,4865-4867-4866,0-23-65281-10-11",
        ];

        for fp in fingerprints {
            let parts: Vec<&str> = fp.split(',').collect();
            assert!(parts.len() >= 2, "Invalid fingerprint format: {}", fp);
            assert!(
                parts[0].parse::<u16>().is_ok(),
                "Invalid TLS version: {}",
                parts[0]
            );
        }
    }

    #[test]
    fn test_connection_metrics() {
        let connections = vec![
            create_test_connection("1", ConnectionState::Established),
            create_test_connection("2", ConnectionState::Handshaking),
        ];

        let total_bytes_sent: u64 = connections.iter().map(|c| c.bytes_sent).sum();
        let total_bytes_received: u64 = connections.iter().map(|c| c.bytes_received).sum();
        let active_count = connections
            .iter()
            .filter(|c| {
                matches!(
                    c.state,
                    ConnectionState::Established | ConnectionState::Handshaking
                )
            })
            .count();

        assert_eq!(total_bytes_sent, 2048);
        assert_eq!(total_bytes_received, 4096);
        assert_eq!(active_count, 2);
    }

    #[test]
    fn test_cipher_suite_parsing() {
        let cipher_suites = vec![
            "TLS_AES_256_GCM_SHA384",
            "TLS_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
        ];

        for suite in cipher_suites {
            assert!(suite.starts_with("TLS_"), "Invalid cipher suite: {}", suite);
            assert!(suite.len() > 10, "Cipher suite too short: {}", suite);
        }
    }

    #[test]
    fn test_connection_filtering() {
        let connections = vec![
            create_test_connection("1", ConnectionState::Established),
            create_test_connection("2", ConnectionState::Handshaking),
            create_test_connection("3", ConnectionState::Closed),
            create_test_connection("4", ConnectionState::Established),
        ];

        let established: Vec<_> = connections
            .iter()
            .filter(|c| c.state == ConnectionState::Established)
            .collect();

        assert_eq!(established.len(), 2);
    }

    // ─── Forensics Tracer Tests ────────────────────────────────────────────

    #[test]
    fn test_handshake_stage_color_classes() {
        assert_eq!(HandshakeStage::DnsResolve.color_class(), "text-cyan-400");
        assert_eq!(HandshakeStage::TcpHandshake.color_class(), "text-blue-400");
        assert_eq!(HandshakeStage::TlsReality.color_class(), "text-violet-400");
        assert_eq!(HandshakeStage::Fragment.color_class(), "text-purple-400");
        assert_eq!(HandshakeStage::Success.color_class(), "text-emerald-400");
        assert_eq!(HandshakeStage::Failed.color_class(), "text-red-400");
    }

    #[test]
    fn test_failure_events_map_to_amber() {
        let dpi = ForensicsEntry {
            stage: HandshakeStage::TcpHandshake,
            timestamp_ms: 50,
            latency_ms: 120.0,
            status: TraceStatus::DpiDetected,
            geo_label: Some("SIN".to_string()),
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(dpi.resolved_color_class(), "text-amber-400");
        assert_eq!(dpi.resolved_bg_class(), "bg-amber-500/20");
    }

    #[test]
    fn test_failure_events_map_to_red() {
        let error = ForensicsEntry {
            stage: HandshakeStage::Failed,
            timestamp_ms: 300,
            latency_ms: 0.0,
            status: TraceStatus::Error,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(error.resolved_color_class(), "text-red-400");
        assert_eq!(error.resolved_bg_class(), "bg-red-500/20");
    }

    #[test]
    fn test_timeout_events_map_to_amber() {
        let timeout = ForensicsEntry {
            stage: HandshakeStage::TlsReality,
            timestamp_ms: 200,
            latency_ms: 5000.0,
            status: TraceStatus::Timeout,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(timeout.resolved_color_class(), "text-amber-400");
    }

    #[test]
    fn test_active_stage_overrides_to_cyan() {
        let active = ForensicsEntry {
            stage: HandshakeStage::DnsResolve,
            timestamp_ms: 0,
            latency_ms: 10.0,
            status: TraceStatus::Ok,
            geo_label: None,
            outbound_tag: None,
            is_active: true,
        };
        assert_eq!(active.resolved_color_class(), "text-cyan-400");
        assert_eq!(active.resolved_bg_class(), "bg-cyan-500/20");
    }

    #[test]
    fn test_forensics_history_capacity_capped_at_100() {
        let mut hist = ForensicsHistory::new(500);
        assert_eq!(hist.capacity, MAX_FORENSICS_ENTRIES);

        for i in 0..200u64 {
            hist.push(ForensicsEntry {
                stage: HandshakeStage::DnsResolve,
                timestamp_ms: i * 10,
                latency_ms: i as f32,
                status: TraceStatus::Ok,
                geo_label: None,
                outbound_tag: None,
                is_active: false,
            });
        }
        assert_eq!(hist.len(), MAX_FORENSICS_ENTRIES);
        assert!(hist.len() <= 100);
    }

    #[test]
    fn test_forensics_history_eviction_order() {
        let mut hist = ForensicsHistory::new(3);
        for i in 0..5 {
            hist.push(ForensicsEntry {
                stage: HandshakeStage::DnsResolve,
                timestamp_ms: i * 100,
                latency_ms: i as f32,
                status: TraceStatus::Ok,
                geo_label: None,
                outbound_tag: None,
                is_active: false,
            });
        }
        assert_eq!(hist.len(), 3);
        // Oldest entries evicted first
        assert_eq!(hist.entries.front().unwrap().timestamp_ms, 200);
        assert_eq!(hist.latest().unwrap().timestamp_ms, 400);
    }

    #[test]
    fn test_success_glow_style() {
        let success = ForensicsEntry {
            stage: HandshakeStage::Success,
            timestamp_ms: 100,
            latency_ms: 0.0,
            status: TraceStatus::Ok,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert!(success.glow_style().contains("16,185,129")); // Emerald RGB
    }

    // ─── Routing Canvas Tests ──────────────────────────────────────────────

    #[test]
    fn test_compute_layout_positions_3_columns() {
        let mut nodes = vec![
            CanvasNode {
                id: "app".into(),
                label: "A".into(),
                node_type: NodeType::AppSource,
                traffic_volume: 0,
                x: 0.0,
                y: 0.0,
            },
            CanvasNode {
                id: "flt".into(),
                label: "F".into(),
                node_type: NodeType::Filter,
                traffic_volume: 0,
                x: 0.0,
                y: 0.0,
            },
            CanvasNode {
                id: "out".into(),
                label: "O".into(),
                node_type: NodeType::Outbound,
                traffic_volume: 0,
                x: 0.0,
                y: 0.0,
            },
        ];
        compute_layout(&mut nodes, 360.0, 280.0);

        // Column 0 ≈ 54, Column 1 ≈ 180, Column 2 ≈ 306
        assert!((nodes[0].x - 54.0).abs() < 1.0);
        assert!((nodes[1].x - 180.0).abs() < 1.0);
        assert!((nodes[2].x - 306.0).abs() < 1.0);
    }

    #[test]
    fn test_outbound_position_lookup() {
        let mut nodes = vec![
            CanvasNode {
                id: "app".into(),
                label: "A".into(),
                node_type: NodeType::AppSource,
                traffic_volume: 0,
                x: 0.0,
                y: 0.0,
            },
            CanvasNode {
                id: "out-1".into(),
                label: "O".into(),
                node_type: NodeType::Outbound,
                traffic_volume: 0,
                x: 0.0,
                y: 0.0,
            },
        ];
        compute_layout(&mut nodes, 360.0, 280.0);

        let pos = outbound_position("out-1", &nodes);
        assert!(pos.is_some());
        assert!(outbound_position("nonexistent", &nodes).is_none());
        // AppSource should not be found via outbound_position
        assert!(outbound_position("app", &nodes).is_none());
    }

    #[test]
    fn test_dot_duration_bounds() {
        // Zero traffic → max duration (4s)
        assert!((dot_duration_s(0) - 4.0).abs() < 0.01);
        // Max traffic → min duration (0.5s)
        assert!((dot_duration_s(10_000_000) - 0.5).abs() < 0.01);
        // Mid-range is between bounds
        let mid = dot_duration_s(5_000_000);
        assert!(mid > 0.5);
        assert!(mid < 4.0);
    }

    #[test]
    fn test_flowj_jitter_bounded() {
        for seed in 0..50 {
            for frame in 0..50 {
                let (dx, dy) = flowj_jitter_offset(seed, frame);
                assert!(
                    dx.abs() <= 3.0,
                    "dx={} out of range for seed={}, frame={}",
                    dx,
                    seed,
                    frame
                );
                assert!(
                    dy.abs() <= 3.0,
                    "dy={} out of range for seed={}, frame={}",
                    dy,
                    seed,
                    frame
                );
            }
        }
    }

    #[test]
    fn test_layout_with_100_streams() {
        let mut nodes: Vec<CanvasNode> = Vec::with_capacity(110);
        for i in 0..5 {
            nodes.push(CanvasNode {
                id: format!("app-{i}"),
                label: format!("App {i}"),
                node_type: NodeType::AppSource,
                traffic_volume: (i as u64 + 1) * 100_000,
                x: 0.0,
                y: 0.0,
            });
        }
        for i in 0..15 {
            nodes.push(CanvasNode {
                id: format!("filter-{i}"),
                label: format!("Rule {i}"),
                node_type: NodeType::Filter,
                traffic_volume: (i as u64 + 1) * 50_000,
                x: 0.0,
                y: 0.0,
            });
        }
        for i in 0..100 {
            nodes.push(CanvasNode {
                id: format!("out-{i}"),
                label: format!("Dest {i}"),
                node_type: NodeType::Outbound,
                traffic_volume: (i as u64 + 1) * 30_000,
                x: 0.0,
                y: 0.0,
            });
        }
        assert_eq!(nodes.len(), 120);
        compute_layout(&mut nodes, 800.0, 600.0);
        for node in &nodes {
            assert!(node.x > 0.0, "Node {} has x=0", node.id);
            assert!(node.y > 0.0, "Node {} has y=0", node.id);
        }
    }

    // ─── Helpers ───────────────────────────────────────────────────────────

    fn create_test_connection(id: &str, state: ConnectionState) -> TlsConnectionInfo {
        TlsConnectionInfo {
            id: id.to_string(),
            local_port: 54321,
            remote_host: "example.com".to_string(),
            remote_port: 443,
            sni: "example.com".to_string(),
            tls_version: "TLS 1.3".to_string(),
            cipher_suite: "TLS_AES_256_GCM_SHA384".to_string(),
            utls_fingerprint: "771,4865-4866-4867,0-23-65281".to_string(),
            state,
            handshake_duration_ms: 45,
            bytes_sent: 1024,
            bytes_received: 2048,
            established_at: 1234567890,
        }
    }
}
