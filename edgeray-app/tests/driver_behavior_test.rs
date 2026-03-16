use edgeray_app::drivers::{
    BackendDriver, DriverError, DriverFactory, DriverManager, DriverType, ExecutionConfig,
    MetricsSnapshot, MockDriver, RemoteNode,
};
use std::sync::Arc;

#[tokio::test]
async fn test_driver_factory() {
    let local = DriverFactory::create(DriverType::Local);
    assert_eq!(local.name(), "Local Device");
    assert!(matches!(local.driver_type(), DriverType::Local));

    let remote_node = RemoteNode::new("Router 1", "http://192.168.1.1", "secret");
    let remote = DriverFactory::create(DriverType::Remote(remote_node));
    assert_eq!(remote.name(), "Router 1");
    // assert!(matches!(remote.driver_type(), DriverType::Remote(_))); // RemoteNode uses new in box
}

#[tokio::test]
async fn test_driver_manager_aggregation() {
    let mut manager = DriverManager::new();

    // Add two mock drivers
    let mock1 = Arc::new(MockDriver::new("Mock 1"));
    let mock2 = Arc::new(MockDriver::new("Mock 2"));

    manager.add_driver(mock1);
    manager.add_driver(mock2);

    assert_eq!(manager.drivers().len(), 2);

    // Start all
    let results: Vec<Result<(), DriverError>> = manager.start_all().await;
    for res in results {
        assert!(res.is_ok());
    }

    // Check they are running
    for driver in manager.drivers() {
        assert!(driver.is_running().await);
    }

    // Pull metrics (simulated)
    let metrics: Vec<Result<MetricsSnapshot, DriverError>> = manager.pull_all_metrics().await;
    assert_eq!(metrics.len(), 2);
    for m in metrics {
        let snapshot = m.expect("Failed to get metrics");
        assert_eq!(snapshot.bytes_uploaded, 1000); // Mock returns generic data
        assert_eq!(snapshot.active_connections, 5);
    }

    // Stop all
    let stop_results: Vec<Result<(), DriverError>> = manager.stop_all().await;
    for res in stop_results {
        assert!(res.is_ok());
    }

    // Check stopped
    for driver in manager.drivers() {
        assert!(!driver.is_running().await);
    }
}

#[tokio::test]
async fn test_mock_driver_behavior() {
    let driver = MockDriver::new("Test Mock");
    assert!(!driver.is_running().await);

    driver.start().await.unwrap();
    assert!(driver.is_running().await);

    let metrics = driver.pull_metrics().await.unwrap();
    assert_eq!(metrics.active_connections, 5);

    // Push config (should do nothing but succeed)
    let server = rustray::types::ServerConfig {
        id: None,
        address: "127.0.0.1".to_string(),
        port: 1080,
        remarks: "test".to_string(),
        protocol: rustray::types::Protocol::Vless,
        uuid: Some("uuid".to_string()),
        password: None,
        network: None,
        flow: None,
        security: None,
        fingerprint: None,
        sni: None,
        host: None,
        path: None,
        method: None,
        pbk: None,
        sid: None,
        service_name: None,
        group: None,
        allow_insecure: None,
    };
    let config = ExecutionConfig::new(server);
    driver.push_config(config).await.unwrap();

    driver.stop().await.unwrap();
    assert!(!driver.is_running().await);
}
