use crate::AppState;
use std::sync::atomic::Ordering;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    // Create menu items
    let connect_i = MenuItem::with_id(app, "connect", "Connect", true, None::<&str>)?;
    let disconnect_i = MenuItem::with_id(app, "disconnect", "Disconnect", true, None::<&str>)?;
    let dashboard_i = MenuItem::with_id(app, "dashboard", "Open Dashboard", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit EdgeRay", true, None::<&str>)?;

    // Create menu
    let menu = Menu::with_items(
        app,
        &[
            &connect_i,
            &disconnect_i,
            &dashboard_i,
            &tauri::menu::PredefinedMenuItem::separator(app)?,
            &quit_i,
        ],
    )?;

    // Build tray
    let _tray = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .on_menu_event(move |app: &AppHandle<R>, event: MenuEvent| {
            let id = event.id.as_ref();
            match id {
                "connect" => {
                    // Emit event to frontend to trigger connection logic or trigger backend logic directly
                    // Ideally we invoke the connect_tunnel command, but that requires arguments.
                    // For now, let's open the window to let user connect.
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "disconnect" => {
                    // Direct disconnect if possible
                    let state = app.state::<AppState>();
                    if state.is_connected.load(Ordering::SeqCst) {
                        // Trigger disconnect logic
                        // This is tricky without async context here.
                        // We can emit an event that the frontend listens to?
                        let _ = app.emit("tray-disconnect", ());
                    }
                }
                "dashboard" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray: &TrayIcon<R>, event: TrayIconEvent| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state,
                ..
            } = event
            {
                if button_state == tauri::tray::MouseButtonState::Down {
                    // Toggle window visibility
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            }
        })
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;

    Ok(())
}
