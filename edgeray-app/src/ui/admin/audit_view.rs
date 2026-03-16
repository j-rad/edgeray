// edgeray-app/src/ui/admin/audit_view.rs
//! Audit Log Viewer Component
//!
//! Displays administrative action logs with filtering and search.

use dioxus::prelude::*;

/// Audit log entry for display
#[derive(Clone, PartialEq, Debug)]
pub struct AuditEntryData {
    pub id: String,
    pub timestamp: i64,
    pub user_id: String,
    pub action: String,
    pub path: String,
    pub method: String,
    pub status: u16,
    pub ip_address: String,
    pub duration_ms: u64,
}

/// Format timestamp to human-readable
fn format_timestamp(ts: i64) -> String {
    // Simple formatting - in production use chrono
    let secs = ts % 60;
    let mins = (ts / 60) % 60;
    let hours = (ts / 3600) % 24;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

#[derive(Props, Clone, PartialEq)]
pub struct AuditViewProps {
    /// List of audit entries
    pub entries: Vec<AuditEntryData>,
    /// Current filter by user
    #[props(default)]
    pub filter_user: Option<String>,
    /// Current filter by action
    #[props(default)]
    pub filter_action: Option<String>,
}

/// Audit log viewer component
#[component]
pub fn AuditView(props: AuditViewProps) -> Element {
    let mut filter_user = use_signal(|| props.filter_user.clone().unwrap_or_default());
    let mut filter_action = use_signal(|| props.filter_action.clone().unwrap_or_default());

    // Filter entries
    let filtered_entries: Vec<_> = props
        .entries
        .iter()
        .filter(|e| {
            let user_match = filter_user().is_empty() || e.user_id.contains(&filter_user());
            let action_match = filter_action().is_empty() || e.action == filter_action();
            user_match && action_match
        })
        .cloned()
        .collect();

    rsx! {
        div { class: "bg-surface rounded-lg border border-border overflow-hidden",
            // Header with filters
            div { class: "p-4 border-b border-border",
                div { class: "flex items-center gap-4",
                    h2 { class: "text-lg font-semibold text-foreground", "Audit Log" }

                    div { class: "flex gap-3 ml-auto",
                        // User filter
                        input {
                            r#type: "text",
                            class: "px-3 py-1.5 text-sm rounded bg-surface-dark border border-border text-foreground placeholder:text-muted",
                            placeholder: "Filter by user...",
                            value: "{filter_user}",
                            oninput: move |e| filter_user.set(e.value()),
                        }

                        // Action filter
                        select {
                            class: "px-3 py-1.5 text-sm rounded bg-surface-dark border border-border text-foreground",
                            value: "{filter_action}",
                            onchange: move |e| filter_action.set(e.value()),
                            option { value: "", "All Actions" }
                            option { value: "create", "Create" }
                            option { value: "update", "Update" }
                            option { value: "delete", "Delete" }
                        }
                    }
                }
            }

            // Table
            div { class: "overflow-x-auto",
                table { class: "w-full",
                    thead { class: "bg-surface-dark text-xs font-semibold text-muted uppercase",
                        tr {
                            th { class: "px-4 py-3 text-left", "Time" }
                            th { class: "px-4 py-3 text-left", "User" }
                            th { class: "px-4 py-3 text-left", "Action" }
                            th { class: "px-4 py-3 text-left", "Path" }
                            th { class: "px-4 py-3 text-center", "Status" }
                            th { class: "px-4 py-3 text-left", "IP" }
                            th { class: "px-4 py-3 text-right", "Duration" }
                        }
                    }
                    tbody { class: "divide-y divide-border",
                        for entry in filtered_entries.iter() {
                            tr {
                                key: "{entry.id}",
                                class: "hover:bg-surface-hover transition-colors",

                                td { class: "px-4 py-3 text-sm text-muted font-mono",
                                    "{format_timestamp(entry.timestamp)}"
                                }
                                td { class: "px-4 py-3 text-sm text-foreground",
                                    "{entry.user_id}"
                                }
                                td { class: "px-4 py-3",
                                    span {
                                        class: match entry.action.as_str() {
                                            "create" => "px-2 py-0.5 text-xs rounded bg-green-500/20 text-green-400",
                                            "update" => "px-2 py-0.5 text-xs rounded bg-blue-500/20 text-blue-400",
                                            "delete" => "px-2 py-0.5 text-xs rounded bg-red-500/20 text-red-400",
                                            _ => "px-2 py-0.5 text-xs rounded bg-gray-500/20 text-gray-400",
                                        },
                                        "{entry.action}"
                                    }
                                }
                                td { class: "px-4 py-3 text-sm text-muted font-mono",
                                    "{entry.path}"
                                }
                                td { class: "px-4 py-3 text-center",
                                    span {
                                        class: if entry.status < 300 {
                                            "text-xs text-green-400"
                                        } else if entry.status < 400 {
                                            "text-xs text-yellow-400"
                                        } else {
                                            "text-xs text-red-400"
                                        },
                                        "{entry.status}"
                                    }
                                }
                                td { class: "px-4 py-3 text-sm text-muted",
                                    "{entry.ip_address}"
                                }
                                td { class: "px-4 py-3 text-sm text-muted text-right",
                                    "{entry.duration_ms}ms"
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            div { class: "px-4 py-3 bg-surface-dark border-t border-border text-xs text-muted",
                "Showing {filtered_entries.len()} of {props.entries.len()} entries"
            }
        }
    }
}
