use dioxus::prelude::*;
use std::time::{Duration, Instant};
use tokio::time::sleep;

fn app() -> Element {
    rsx! { div {} }
}

#[tokio::test]
async fn test_dioxus_signal_propagation_latency() {
    // This test simulates the telemetry update loop and verifies that signals
    // are propagated within the 16ms window (60fps).

    let mut vdom = VirtualDom::new(app);
    let _ = vdom.rebuild(&mut dioxus::core::NoOpMutations);

    let mut signal = vdom.in_runtime(|| Signal::new(0u64));
    let mut update_count = 0;
    let mut total_latency = Duration::ZERO;

    let iterations = 100;

    for _ in 0..iterations {
        let start = Instant::now();

        // Simulate a telemetry gRPC update
        vdom.in_runtime(|| {
            signal.set(update_count as u64);
        });

        // Trigger a virtual render cycle
        vdom.render_immediate(&mut dioxus::core::NoOpMutations);

        let latency = start.elapsed();
        total_latency += latency;
        update_count += 1;

        // Assert each individual update is well within the 16ms limit
        assert!(
            latency < Duration::from_millis(16),
            "Signal propagation exceeded 16ms: {:?}",
            latency
        );

        sleep(Duration::from_millis(1)).await;
    }

    let avg_latency = total_latency / iterations as u32;
    println!("Average Signal Propagation Latency: {:?}", avg_latency);

    assert!(
        avg_latency < Duration::from_millis(5),
        "Average latency too high for 60fps UI: {:?}",
        avg_latency
    );
}

#[tokio::test]
async fn test_telemetry_stream_responsiveness() {
    // Verifies that the telemetry panel can handle a burst of updates
    // without stalling the main UI signal.

    let mut vdom = VirtualDom::new(app);
    let _ = vdom.rebuild(&mut dioxus::core::NoOpMutations);

    let (mut bytes_up, mut bytes_down) = vdom.in_runtime(|| (Signal::new(0f64), Signal::new(0f64)));

    let start = Instant::now();
    for i in 0..1000 {
        vdom.in_runtime(|| {
            bytes_up.set(i as f64 * 1024.0);
            bytes_down.set(i as f64 * 2048.0);
        });
        vdom.render_immediate(&mut dioxus::core::NoOpMutations);
    }

    let elapsed = start.elapsed();
    println!("1000 telemetry updates processed in: {:?}", elapsed);

    // 1000 updates should be processed very fast, much faster than 1s
    assert!(elapsed < Duration::from_secs(1));
}
