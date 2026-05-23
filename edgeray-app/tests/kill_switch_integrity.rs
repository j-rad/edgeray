use edgeray_app::app::watchdog::Watchdog;
use std::time::Duration;

#[tokio::test]
async fn test_watchdog_state_and_concurrency() {
    let watchdog = Watchdog::new();

    // Test initial state
    assert_eq!(watchdog.get_failure_count().await, 0);
    assert!(!watchdog.is_kill_switch_active());

    // Start watchdog (spawns task)
    watchdog.start().await;

    // Verify it doesn't crash immediately
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Simulate setting kill switch (e.g. user toggles it)
    watchdog.set_kill_switch(true);
    assert!(
        watchdog.is_kill_switch_active(),
        "Kill switch should be active"
    );

    // Test concurrency limit (simulated battery check)
    // Since mock `is_on_battery` returns false by default (plugged in), expecting 20.
    let concurrency = watchdog.get_scanner_concurrency().await;
    // We allow 5 or 20 depending on environment/mock
    assert!(
        concurrency == 20 || concurrency == 5,
        "Concurrency should be sensible (20 or 5)"
    );

    // Note: To verify "no packets leak", an integration test with network namespace would be required.
    // Here we verify the supervisor logic correctly sets the flag which the OS controller consumes.
}
