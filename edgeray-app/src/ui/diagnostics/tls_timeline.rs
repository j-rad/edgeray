//! TLS handshake timeline visualization component
//!
//! Displays detailed timeline of TLS handshake events

use crate::components::ui::{GlassCard, Icon, SectionHeader};
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct HandshakeEvent {
    timestamp_ms: u32,
    event_type: EventType,
    description: String,
    duration_ms: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventType {
    ClientHello,
    ServerHello,
    Certificate,
    KeyExchange,
    Finished,
    ApplicationData,
}

impl EventType {
    fn icon(&self) -> &'static str {
        match self {
            Self::ClientHello => "arrow_forward",
            Self::ServerHello => "arrow_back",
            Self::Certificate => "verified",
            Self::KeyExchange => "key",
            Self::Finished => "check_circle",
            Self::ApplicationData => "data_usage",
        }
    }

    fn color(&self) -> &'static str {
        match self {
            Self::ClientHello => "text-blue-600 dark:text-blue-400",
            Self::ServerHello => "text-green-600 dark:text-green-400",
            Self::Certificate => "text-purple-600 dark:text-purple-400",
            Self::KeyExchange => "text-orange-600 dark:text-orange-400",
            Self::Finished => "text-cyan-600 dark:text-cyan-400",
            Self::ApplicationData => "text-gray-600 dark:text-gray-400",
        }
    }
}

#[component]
pub fn TlsHandshakeTimeline(connection_id: String) -> Element {
    let events = use_signal(|| generate_mock_timeline());

    rsx! {
        GlassCard {
            class: "p-6",
            children: rsx! {
                SectionHeader {
                    title: "TLS Handshake Timeline".to_string(),
                    icon: Some("timeline".to_string())
                }

                div { class: "mt-4 space-y-4",
                    for (idx, event) in events().iter().enumerate() {
                        TimelineEvent {
                            key: "{idx}",
                            event: event.clone(),
                            is_last: idx == events().len() - 1,
                        }
                    }
                }

                // Summary
                div { class: "mt-6 pt-4 border-t border-white/10",
                    div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                        TimelineStat {
                            label: "Total Duration",
                            value: format!("{}ms", calculate_total_duration(&events())),
                            icon: "schedule"
                        }
                        TimelineStat {
                            label: "Events",
                            value: format!("{}", events().len()),
                            icon: "event"
                        }
                        TimelineStat {
                            label: "TLS Version",
                            value: "1.3".to_string(),
                            icon: "security"
                        }
                        TimelineStat {
                            label: "Cipher",
                            value: "AES-256-GCM".to_string(),
                            icon: "lock"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineEvent(event: HandshakeEvent, is_last: bool) -> Element {
    rsx! {
        div { class: "flex gap-4",
            // Timeline indicator
            div { class: "flex flex-col items-center",
                div {
                    class: format!("size-10 rounded-full flex items-center justify-center {}",
                        match event.event_type {
                            EventType::ClientHello => "bg-blue-500/20",
                            EventType::ServerHello => "bg-green-500/20",
                            EventType::Certificate => "bg-purple-500/20",
                            EventType::KeyExchange => "bg-orange-500/20",
                            EventType::Finished => "bg-cyan-500/20",
                            EventType::ApplicationData => "bg-gray-500/20",
                        }
                    ),
                    Icon {
                        name: event.event_type.icon().to_string(),
                        class: format!("text-lg {}", event.event_type.color())
                    }
                }
                if !is_last {
                    div { class: "w-0.5 flex-1 bg-white/10 my-2 min-h-[40px]" }
                }
            }

            // Event details
            div { class: "flex-1 pb-6",
                div { class: "flex items-center justify-between mb-1",
                    span { class: "font-semibold text-slate-900 dark:text-white",
                        "{event.description}"
                    }
                    span { class: "text-xs text-slate-500 dark:text-gray-400",
                        "{event.timestamp_ms}ms"
                    }
                }
                if let Some(duration) = event.duration_ms {
                    div { class: "text-xs text-slate-500 dark:text-gray-400",
                        "Duration: {duration}ms"
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineStat(label: String, value: String, icon: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-3",
            Icon { name: icon, class: "text-xl text-primary".to_string() }
            div {
                div { class: "text-xs text-slate-500 dark:text-gray-400 uppercase", "{label}" }
                div { class: "text-sm font-bold text-slate-900 dark:text-white", "{value}" }
            }
        }
    }
}

fn calculate_total_duration(events: &[HandshakeEvent]) -> u32 {
    events.last().map(|e| e.timestamp_ms).unwrap_or(0)
}

fn generate_mock_timeline() -> Vec<HandshakeEvent> {
    vec![
        HandshakeEvent {
            timestamp_ms: 0,
            event_type: EventType::ClientHello,
            description: "Client Hello".to_string(),
            duration_ms: Some(2),
        },
        HandshakeEvent {
            timestamp_ms: 5,
            event_type: EventType::ServerHello,
            description: "Server Hello".to_string(),
            duration_ms: Some(3),
        },
        HandshakeEvent {
            timestamp_ms: 12,
            event_type: EventType::Certificate,
            description: "Server Certificate".to_string(),
            duration_ms: Some(8),
        },
        HandshakeEvent {
            timestamp_ms: 25,
            event_type: EventType::KeyExchange,
            description: "Key Exchange".to_string(),
            duration_ms: Some(15),
        },
        HandshakeEvent {
            timestamp_ms: 42,
            event_type: EventType::Finished,
            description: "Handshake Finished".to_string(),
            duration_ms: Some(3),
        },
        HandshakeEvent {
            timestamp_ms: 45,
            event_type: EventType::ApplicationData,
            description: "Application Data".to_string(),
            duration_ms: None,
        },
    ]
}
