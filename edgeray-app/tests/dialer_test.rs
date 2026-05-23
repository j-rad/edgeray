use edgeray_app::models::{Protocol, ServerConfig};
use edgeray_app::networking::dialer::{IspAwareDialer, IspCode, IspInfo};
use edgeray_app::networking::monitor::ConnectionMonitor;

#[test]
fn test_isp_switching_and_handoff() {
    let mut dialer = IspAwareDialer::new();
    let monitor = ConnectionMonitor::new();

    // 1. Create Mock Servers
    let s1_mci = ServerConfig {
        id: Some("mci-optimized".to_string()),
        address: "1.1.1.1".to_string(),
        remarks: "MCI Optimized Node".to_string(),
        protocol: Protocol::Vless,
        ..Default::default()
    };

    let s2_irancell = ServerConfig {
        id: Some("irancell-optimized".to_string()),
        address: "2.2.2.2".to_string(),
        remarks: "Irancell Optimized Node".to_string(),
        protocol: Protocol::Vless,
        ..Default::default()
    };

    let s3_generic = ServerConfig {
        id: Some("generic".to_string()),
        address: "3.3.3.3".to_string(),
        remarks: "Generic Node".to_string(),
        protocol: Protocol::Vless,
        ..Default::default()
    };

    let servers = vec![s1_mci.clone(), s2_irancell.clone(), s3_generic.clone()];

    // 2. Simulate MCI ISP
    dialer.set_manual_isp(IspInfo {
        name: "MCI".to_string(),
        country_code: "IR".to_string(),
        asn: "AS197207".to_string(),
        isp_code: IspCode::Mci,
    });

    // 3. Rank Nodes - Expect MCI first
    let ranked_mci = dialer.rank_nodes(servers.clone(), &monitor);
    assert_eq!(
        ranked_mci[0].id, s1_mci.id,
        "MCI ISP should prefer MCI node"
    );

    // 4. Switch ISP to Irancell
    dialer.set_manual_isp(IspInfo {
        name: "Irancell".to_string(),
        country_code: "IR".to_string(),
        asn: "AS44244".to_string(),
        isp_code: IspCode::Irancell,
    });

    // 5. Rank Nodes - Expect Irancell first
    let ranked_irancell = dialer.rank_nodes(servers.clone(), &monitor);
    assert_eq!(
        ranked_irancell[0].id, s2_irancell.id,
        "Irancell ISP should prefer Irancell node"
    );

    // 6. Test Failover logic
    let current_server = s2_irancell.clone();

    // Simulate high jitter (350ms)
    // Note: monitor implementation requires us to update stats.
    monitor.update_stats(0.0, 350, 50);

    // Check failover trigger
    let recommendation = dialer.recommend_switch(&monitor, &current_server, servers.clone());

    assert!(
        recommendation.is_some(),
        "Should recommend switch due to high jitter"
    );
    let rec = recommendation.unwrap();
    assert_ne!(
        rec.id, current_server.id,
        "Should not recommend current failing server"
    );

    // Since we are on Irancell ISP, and current server (Irancell) is failing,
    // it should pick next best (MCI or Generic depending on score).
    // MCI has score 0 (ISP mismatch) but generic also 0.
    // Wait, MCI might have score 0. Generic score 0.
    // If scores equal, order is stable or undefined.
    // Let's ensure fallback has lower score or just check logic.

    log::info!("Recommended fallback: {}", rec.remarks);
}
