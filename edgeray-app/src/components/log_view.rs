//! Log View Component
//!
//! Real-time log streaming from Rust backend to UI.
//! Supports filtering by log level and log export.

use super::ui::Icon;
use crate::i18n::t;
use dioxus::prelude::*;
use log::{Level, Metadata, Record};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

static LOGGER: AppLogger = AppLogger;

struct AppLogger;

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Forward to console/stdout
            #[cfg(not(target_arch = "wasm32"))]
            println!("{} - {}", record.level(), record.args());

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&format!("{} - {}", record.level(), record.args()).into());

            // Add to internal buffer
            let level = match record.level() {
                Level::Error => LogLevel::Error,
                Level::Warn => LogLevel::Warn,
                Level::Info => LogLevel::Info,
                Level::Debug => LogLevel::Debug,
                Level::Trace => LogLevel::Debug,
            };

            add_log_entry(
                level,
                record.target().to_string(),
                record.args().to_string(),
            );
        }
    }

    fn flush(&self) {}
}

pub fn init_logging() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    fn color(&self) -> &'static str {
        match self {
            LogLevel::Debug => "text-gray-400",
            LogLevel::Info => "text-primary",
            LogLevel::Warn => "text-amber-400",
            LogLevel::Error => "text-red-400",
        }
    }

    fn bg_color(&self) -> &'static str {
        match self {
            LogLevel::Debug => "bg-gray-500/20",
            LogLevel::Info => "bg-blue-500/20",
            LogLevel::Warn => "bg-yellow-500/20",
            LogLevel::Error => "bg-red-500/20",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            LogLevel::Debug => "bug_report",
            LogLevel::Info => "info",
            LogLevel::Warn => "warning",
            LogLevel::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
}

/// Global log buffer for real-time streaming
static LOG_BUFFER: once_cell::sync::Lazy<Arc<Mutex<VecDeque<LogEntry>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(VecDeque::with_capacity(1000))));

/// Add a log entry to the global buffer
pub fn add_log_entry(level: LogLevel, target: String, message: String) {
    let entry = LogEntry {
        timestamp: chrono::Local::now().format("%H:%M:%S%.3f").to_string(),
        level,
        target,
        message,
    };

    if let Ok(mut buffer) = LOG_BUFFER.lock() {
        buffer.push_back(entry);
        if buffer.len() > 1000 {
            buffer.pop_front();
        }
    }
}

/// Get all log entries
pub fn get_log_entries() -> Vec<LogEntry> {
    LOG_BUFFER
        .lock()
        .map(|buffer| buffer.iter().cloned().collect())
        .unwrap_or_default()
}

/// Clear all log entries
pub fn clear_logs() {
    if let Ok(mut buffer) = LOG_BUFFER.lock() {
        buffer.clear();
    }
}

#[component]
pub fn LogView(on_back: EventHandler<()>) -> Element {
    let trans = t();
    let mut filter_level = use_signal(|| None::<LogLevel>);
    let mut auto_scroll = use_signal(|| true);
    let mut current_logs = use_signal(Vec::<LogEntry>::new);

    use_effect(move || {
        spawn(async move {
            loop {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    current_logs.set(get_log_entries());
                }
                #[cfg(target_arch = "wasm32")]
                {
                    current_logs.set(get_log_entries());
                    break;
                }
            }
        });
    });

    // Add some sample logs on mount
    use_effect(move || {
        spawn(async move {
            add_log_entry(
                LogLevel::Info,
                "edgeray-core".to_string(),
                "TUN device initialized: edgeray0".to_string(),
            );
            add_log_entry(
                LogLevel::Debug,
                "edgeray-core::router".to_string(),
                "Loaded 1234 GeoIP entries".to_string(),
            );
            add_log_entry(
                LogLevel::Warn,
                "edgeray-core::dns".to_string(),
                "DNS query timeout for example.com".to_string(),
            );
            add_log_entry(
                LogLevel::Error,
                "edgeray-core::proxy".to_string(),
                "Failed to connect to upstream: connection refused".to_string(),
            );
        });
    });

    let export_logs = move |_| {
        let entries = get_log_entries();
        let _log_text = entries
            .iter()
            .map(|e| {
                format!(
                    "[{}] {} {}: {}",
                    e.timestamp,
                    e.level.as_str(),
                    e.target,
                    e.message
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        log::info!("Exporting {} log entries", entries.len());

        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "android")))]
        {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_text(_log_text);
            }
        }
    };

    let filtered_logs = {
        let entries = current_logs.read();
        if let Some(level) = *filter_level.read() {
            entries
                .iter()
                .filter(|e| e.level == level)
                .cloned()
                .collect::<Vec<_>>()
        } else {
            entries.clone()
        }
    };

    let clear_logs_action = move |_| clear_logs();

    rsx! {
        // Use standard Layout with animation
        div {
            class: "h-full w-full flex flex-col p-4 md:p-6 animate-fade-in-up",

            // Standard Page Header
            crate::components::ui::PageHeader {
                title: trans.logs.title,
                left_action: Some(rsx! {
                    button {
                        class: "p-2 rounded-xl hover:bg-white/10 transition-colors",
                        onclick: move |_| on_back.call(()),
                        Icon { name: "arrow_back".to_string(), class: "text-lg text-gray-400 hover:text-white".to_string() }
                    }
                }),
                right_action: Some(rsx! {
                    div {
                        class: "flex items-center gap-1",
                        // Export Button
                        button {
                            class: "p-2 rounded-xl hover:bg-white/10 transition-colors text-primary",
                            onclick: export_logs,
                            title: "Export Logs",
                            Icon { name: "download".to_string() }
                        }
                        // Clear Button
                        button {
                            class: "p-2 rounded-xl hover:bg-white/10 transition-colors text-red-400",
                            onclick: clear_logs_action,
                            title: "Clear Logs",
                            Icon { name: "delete".to_string() }
                        }
                    }
                })
            }

            // Main Content wrapped in GlassPanel
            crate::components::ui::GlassPanel {
                class: "flex-1 mt-4 overflow-hidden flex flex-col relative",

                // Toolbar (Filters)
                div {
                    class: "flex items-center justify-between p-3 border-b border-white/5 bg-black/40 backdrop-blur-md",

                    // Level Filters
                    div {
                        class: "flex gap-2 text-xs",
                        button {
                            class: format!(
                                "px-3 py-1.5 rounded-lg transition-all {}",
                                if filter_level.read().is_none() {
                                    "bg-white/20 text-white font-bold shadow-sm ring-1 ring-white/10"
                                } else {
                                    "text-gray-400 hover:bg-white/5 hover:text-white"
                                }
                            ),
                            onclick: move |_| filter_level.set(None),
                            "ALL"
                        }
                        for level in [LogLevel::Debug, LogLevel::Info, LogLevel::Warn, LogLevel::Error] {
                            button {
                                key: "{level:?}",
                                class: format!(
                                    "px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5 {}",
                                    if *filter_level.read() == Some(level) {
                                        format!("{} text-white font-bold ring-1 ring-white/10 shadow-glow", level.bg_color())
                                } else {
                                    "text-gray-500 hover:bg-white/5 hover:text-gray-300".to_string()
                                }
                                ),
                                onclick: move |_| filter_level.set(Some(level)),
                                div { class: format!("w-1.5 h-1.5 rounded-full {}", match level {
                                    LogLevel::Debug => "bg-gray-400",
                                    LogLevel::Info => "bg-blue-400",
                                    LogLevel::Warn => "bg-yellow-400",
                                    LogLevel::Error => "bg-red-400",
                                }) }
                                "{level.as_str()}"
                            }
                        }
                    }

                    // Auto-scroll Toggle
                    button {
                        class: format!(
                            "flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all {}",
                            if *auto_scroll.read() {
                                "bg-primary/20 text-primary ring-1 ring-primary/30"
                            } else {
                                "text-slate-500 hover:text-slate-300"
                            }
                        ),
                        onclick: move |_| {
                            let current = *auto_scroll.read();
                            auto_scroll.set(!current);
                        },
                        Icon { name: "vertical_align_bottom".to_string(), class: "text-sm".to_string() }
                        "Auto-scroll"
                    }
                }

                // Log Stream Container
                div {
                    class: "flex-1 overflow-y-auto p-4 space-y-1 font-mono text-xs custom-scrollbar bg-void/50",
                    id: "log-container",

                    if !filtered_logs.is_empty() {
                        for entry in filtered_logs.iter() {
                            LogEntryCard { entry: entry.clone() }
                        }
                    } else {
                        // Empty State
                        div {
                            class: "h-full flex flex-col items-center justify-center text-gray-600",
                            Icon { name: "terminal".to_string(), class: "text-4xl mb-3 opacity-20".to_string() }
                            p { "No logs to display" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LogEntryCard(entry: LogEntry) -> Element {
    let bg_color = match entry.level {
        LogLevel::Debug => "hover:bg-gray-500/10",
        LogLevel::Info => "hover:bg-blue-500/10",
        LogLevel::Warn => "bg-yellow-500/5 hover:bg-yellow-500/10",
        LogLevel::Error => "bg-red-500/5 hover:bg-red-500/10",
    };

    let text_color = match entry.level {
        LogLevel::Debug => "text-gray-500",
        LogLevel::Info => "text-primary",
        LogLevel::Warn => "text-yellow-400",
        LogLevel::Error => "text-red-400",
    };

    rsx! {
        div {
            class: format!("flex gap-3 px-2 py-1.5 rounded transition-colors select-text {}", bg_color),
            // Timestamp
            span { class: "text-slate-500 whitespace-nowrap shrink-0 opacity-60", "{entry.timestamp}" }

            // Level
            span { class: format!("w-10 font-bold shrink-0 {}", text_color), "{entry.level.as_str()}" }

            // Target
            span { class: "text-slate-400 shrink-0 opacity-50 hidden sm:block w-32 truncate", "{entry.target}" }

            // Message
            span { class: "text-gray-300 break-all", "{entry.message}" }
        }
    }
}
