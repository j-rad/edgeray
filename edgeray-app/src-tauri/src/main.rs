// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Set up a custom panic hook for crash reporting and cleanup
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap_or(std::panic::Location::caller());
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        eprintln!("CRASH: thread 'main' panicked at '{}': {}", location, msg);

        // Log to file if possible (simple implementation)
        // In a real production app, we would send this to Sentry here.
        // For now, we ensure we try to clean up the TUN interface if possible.
        // Since we can't easily access the runtime from here, we rely on OS cleanup
        // but ideally we would call rustray::ffi::force_stop_sync() if exposed.

        // Use the native logger if available or just stderr
        log::error!("Application Panic: {} at {}", msg, location);
    }));

    edgeray_app_lib::run()
}
