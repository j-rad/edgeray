use edgeray_app::drivers::execution_config::{GeoDataManager, RoutingRule};
use rustray::types::RoutingMode;

#[test]
fn test_geo_data_manager_rule_generation() {
    let geo_manager = GeoDataManager::new();

    // 1. Bypass Mainland Mode
    let rules = geo_manager.generate_rules(RoutingMode::BypassMainland);
    assert!(!rules.is_empty(), "BypassMainland should generate rules");

    // Verify expected structure
    let has_geoip_cn = rules
        .iter()
        .any(|r| r.ips.contains(&"geoip:cn".to_string()));
    assert!(has_geoip_cn, "Should contain geoip:cn rule");

    let has_geosite_cn = rules
        .iter()
        .any(|r| r.domains.contains(&"geosite:cn".to_string()));
    assert!(has_geosite_cn, "Should contain geosite:cn rule");

    // 2. Bypass LAN Mode
    let lan_rules = geo_manager.generate_rules(RoutingMode::BypassLan);
    let has_private_ip = lan_rules
        .iter()
        .any(|r| r.ips.contains(&"geoip:private".to_string()));
    assert!(has_private_ip, "Should contain geoip:private rule");

    // 3. Global Mode
    let global_rules = geo_manager.generate_rules(RoutingMode::Global);
    assert!(
        global_rules.is_empty(),
        "Global mode should have no bypass rules"
    );
}

#[test]
fn test_routing_rule_serialization() {
    let rule = RoutingRule::bypass_domains(vec!["geosite:google".to_string()]);
    let json = serde_json::to_string(&rule).expect("Failed to serialize rule");

    assert!(json.contains("geosite:google"));
    assert!(json.contains("field"));
    assert!(json.contains("direct"));
}
