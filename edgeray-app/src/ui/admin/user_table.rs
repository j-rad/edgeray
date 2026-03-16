// edgeray-app/src/ui/admin/user_table.rs
//! User Management Table Component
//!
//! High-performance user table with:
//! - Virtual scrolling for 10k+ users
//! - CRUD operations
//! - Traffic quota editing
//! - Bulk operations

use dioxus::prelude::*;

/// User data for display
#[derive(Clone, PartialEq, Debug)]
pub struct UserRowData {
    pub id: String,
    pub email: String,
    pub inbound_tag: String,
    pub enabled: bool,
    pub upload_bytes: i64,
    pub download_bytes: i64,
    pub total_limit_gb: Option<i64>,
    pub expiry_time: Option<i64>,
}

/// Format bytes to human-readable string
fn format_bytes(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Format timestamp to relative days
fn format_expiry(ts: i64) -> String {
    if ts == 0 {
        return "∞".to_string();
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let diff = ts - now;
    if diff < 0 {
        "Expired".to_string()
    } else {
        let days = diff / 86400;
        format!("{}d", days)
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct UserTableProps {
    /// List of users to display
    pub users: Vec<UserRowData>,
    /// Currently selected user IDs
    #[props(default)]
    pub selected: Vec<String>,
    /// Callback when user is selected
    #[props(default)]
    pub on_select: Option<EventHandler<String>>,
    /// Callback when edit is clicked
    #[props(default)]
    pub on_edit: Option<EventHandler<String>>,
    /// Callback when delete is clicked
    #[props(default)]
    pub on_delete: Option<EventHandler<String>>,
    /// Callback when QR code is requested
    #[props(default)]
    pub on_qr: Option<EventHandler<String>>,
    /// Callback when toggle enabled is clicked
    #[props(default)]
    pub on_toggle: Option<EventHandler<(String, bool)>>,
    /// Virtual scroll: start index
    #[props(default)]
    pub scroll_start: usize,
    /// Virtual scroll: visible count
    #[props(default = 50)]
    pub visible_count: usize,
}

/// High-performance user management table
#[component]
pub fn UserTable(props: UserTableProps) -> Element {
    let scroll_end = (props.scroll_start + props.visible_count).min(props.users.len());
    let visible_users: Vec<_> = props.users[props.scroll_start..scroll_end].to_vec();

    rsx! {
        div { class: "bg-surface rounded-lg border border-border overflow-hidden",
            // Table Header
            div { class: "grid grid-cols-12 gap-2 px-4 py-3 bg-surface-dark text-xs font-semibold text-muted uppercase tracking-wider border-b border-border",
                div { class: "col-span-1 flex items-center",
                    input {
                        r#type: "checkbox",
                        class: "rounded border-border",
                    }
                }
                div { class: "col-span-3", "User" }
                div { class: "col-span-2 text-center", "Status" }
                div { class: "col-span-2 text-center", "Traffic" }
                div { class: "col-span-2 text-center", "Expiry" }
                div { class: "col-span-2 text-center", "Actions" }
            }

            // Virtual scroll container
            div {
                class: "divide-y divide-border max-h-[600px] overflow-y-auto",
                style: "will-change: transform;",

                // Spacer for items above viewport
                if props.scroll_start > 0 {
                    div {
                        style: "height: {props.scroll_start * 48}px;",
                    }
                }

                // Visible rows
                for user in visible_users.iter() {
                    {
                        let user_id = user.id.clone();
                        let email = user.email.clone();
                        let enabled = user.enabled;
                        let is_selected = props.selected.contains(&user_id);

                        rsx! {
                            div {
                                key: "{user_id}",
                                class: if is_selected {
                                    "grid grid-cols-12 gap-2 px-4 py-3 items-center bg-primary/10 hover:bg-primary/20 transition-colors"
                                } else {
                                    "grid grid-cols-12 gap-2 px-4 py-3 items-center hover:bg-surface-hover transition-colors"
                                },

                                // Checkbox
                                div { class: "col-span-1 flex items-center",
                                    input {
                                        r#type: "checkbox",
                                        class: "rounded border-border",
                                        checked: is_selected,
                                        onclick: {
                                            let id = user_id.clone();
                                            let handler = props.on_select.clone();
                                            move |_| {
                                                if let Some(ref h) = handler {
                                                    h.call(id.clone());
                                                }
                                            }
                                        },
                                    }
                                }

                                // User email and tag
                                div { class: "col-span-3",
                                    div { class: "flex flex-col",
                                        span { class: "text-sm font-medium text-foreground truncate", "{email}" }
                                        span { class: "text-xs text-muted", "{user.inbound_tag}" }
                                    }
                                }

                                // Status toggle
                                div { class: "col-span-2 flex justify-center",
                                    button {
                                        class: if enabled {
                                            "px-2 py-1 rounded-full text-xs font-medium bg-green-500/20 text-green-400"
                                        } else {
                                            "px-2 py-1 rounded-full text-xs font-medium bg-gray-500/20 text-gray-400"
                                        },
                                        onclick: {
                                            let id = user_id.clone();
                                            let handler = props.on_toggle.clone();
                                            move |_| {
                                                if let Some(ref h) = handler {
                                                    h.call((id.clone(), !enabled));
                                                }
                                            }
                                        },
                                        if enabled { "Active" } else { "Disabled" }
                                    }
                                }

                                // Traffic
                                div { class: "col-span-2 flex justify-center",
                                    div { class: "flex flex-col items-center text-xs",
                                        span { class: "text-foreground",
                                            "{format_bytes(user.upload_bytes + user.download_bytes)}"
                                        }
                                        if let Some(limit) = user.total_limit_gb {
                                            span { class: "text-muted", "/ {limit}GB" }
                                        } else {
                                            span { class: "text-muted", "/ ∞" }
                                        }
                                    }
                                }

                                // Expiry
                                div { class: "col-span-2 flex justify-center",
                                    span {
                                        class: if user.expiry_time.map(|t| {
                                            let now = std::time::SystemTime::now()
                                                .duration_since(std::time::UNIX_EPOCH)
                                                .unwrap_or_default()
                                                .as_secs() as i64;
                                            t > 0 && now > t
                                        }).unwrap_or(false) {
                                            "text-xs text-red-400"
                                        } else {
                                            "text-xs text-muted"
                                        },
                                        "{format_expiry(user.expiry_time.unwrap_or(0))}"
                                    }
                                }

                                // Actions
                                div { class: "col-span-2 flex justify-center gap-1",
                                    // QR Code
                                    button {
                                        class: "p-1.5 rounded hover:bg-surface-hover text-muted hover:text-primary transition-colors",
                                        title: "QR Code",
                                        onclick: {
                                            let id = user_id.clone();
                                            let handler = props.on_qr.clone();
                                            move |_| {
                                                if let Some(ref h) = handler {
                                                    h.call(id.clone());
                                                }
                                            }
                                        },
                                        "📱"
                                    }
                                    // Edit
                                    button {
                                        class: "p-1.5 rounded hover:bg-surface-hover text-muted hover:text-blue-400 transition-colors",
                                        title: "Edit",
                                        onclick: {
                                            let id = user_id.clone();
                                            let handler = props.on_edit.clone();
                                            move |_| {
                                                if let Some(ref h) = handler {
                                                    h.call(id.clone());
                                                }
                                            }
                                        },
                                        "✏️"
                                    }
                                    // Delete
                                    button {
                                        class: "p-1.5 rounded hover:bg-surface-hover text-muted hover:text-red-400 transition-colors",
                                        title: "Delete",
                                        onclick: {
                                            let id = user_id.clone();
                                            let handler = props.on_delete.clone();
                                            move |_| {
                                                if let Some(ref h) = handler {
                                                    h.call(id.clone());
                                                }
                                            }
                                        },
                                        "🗑️"
                                    }
                                }
                            }
                        }
                    }
                }

                // Spacer for items below viewport
                {
                    let remaining = props.users.len().saturating_sub(scroll_end);
                    if remaining > 0 {
                        rsx! {
                            div {
                                style: "height: {remaining * 48}px;",
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }

            // Footer with pagination info
            div { class: "px-4 py-3 bg-surface-dark border-t border-border flex justify-between items-center text-xs text-muted",
                span { "Showing {props.scroll_start + 1}-{scroll_end} of {props.users.len()} users" }
                div { class: "flex gap-2",
                    span { class: "text-primary", "{props.selected.len()} selected" }
                }
            }
        }
    }
}

/// Bulk actions toolbar
#[derive(Props, Clone, PartialEq)]
pub struct BulkActionsProps {
    pub selected_count: usize,
    #[props(default)]
    pub on_enable_all: Option<EventHandler<()>>,
    #[props(default)]
    pub on_disable_all: Option<EventHandler<()>>,
    #[props(default)]
    pub on_delete_all: Option<EventHandler<()>>,
    #[props(default)]
    pub on_reset_traffic: Option<EventHandler<()>>,
}

#[component]
pub fn BulkActions(props: BulkActionsProps) -> Element {
    if props.selected_count == 0 {
        return rsx! {};
    }

    rsx! {
        div { class: "flex items-center gap-3 p-3 bg-primary/10 rounded-lg border border-primary/20 mb-4",
            span { class: "text-sm font-medium text-primary",
                "{props.selected_count} users selected"
            }
            div { class: "flex gap-2 ml-auto",
                button {
                    class: "px-3 py-1.5 text-xs rounded bg-green-500/20 text-green-400 hover:bg-green-500/30 transition-colors",
                    onclick: move |_| {
                        if let Some(ref h) = props.on_enable_all {
                            h.call(());
                        }
                    },
                    "Enable All"
                }
                button {
                    class: "px-3 py-1.5 text-xs rounded bg-yellow-500/20 text-yellow-400 hover:bg-yellow-500/30 transition-colors",
                    onclick: move |_| {
                        if let Some(ref h) = props.on_disable_all {
                            h.call(());
                        }
                    },
                    "Disable All"
                }
                button {
                    class: "px-3 py-1.5 text-xs rounded bg-blue-500/20 text-blue-400 hover:bg-blue-500/30 transition-colors",
                    onclick: move |_| {
                        if let Some(ref h) = props.on_reset_traffic {
                            h.call(());
                        }
                    },
                    "Reset Traffic"
                }
                button {
                    class: "px-3 py-1.5 text-xs rounded bg-red-500/20 text-red-400 hover:bg-red-500/30 transition-colors",
                    onclick: move |_| {
                        if let Some(ref h) = props.on_delete_all {
                            h.call(());
                        }
                    },
                    "Delete All"
                }
            }
        }
    }
}
