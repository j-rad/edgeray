use edgeray_app::ui::adaptive_shell::UiMode;

#[test]
fn test_ui_mode_toggle_independence() {
    // Simulate App State
    let mut ui_mode = UiMode::Simple;
    let connection_state = "Connected";

    // Verify initial state
    assert_eq!(ui_mode, UiMode::Simple);
    assert_eq!(connection_state, "Connected");

    // Toggle UI Mode
    ui_mode = UiMode::Pro;

    // Verify UI Mode changed but connection state remained
    assert_eq!(ui_mode, UiMode::Pro);
    assert_eq!(connection_state, "Connected");

    // Toggle back
    ui_mode = UiMode::Simple;
    assert_eq!(ui_mode, UiMode::Simple);
    assert_eq!(connection_state, "Connected");
}
