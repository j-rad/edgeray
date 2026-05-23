//! Handshake Forensics Tracer — Mobile-First
//!
//! A vertical timeline visualizing REALITY/Fragment handshake stages
//! optimized for thumb scrolling on Android/iOS.
//!
//! - **Success paths** glow Emerald
//! - **DPI interference** glows Amber
//! - **Failures** glow Red
//! - **Active stage** pulses Cyan

use crate::components::ui::{GlassCard, Icon};
use dioxus::prelude::*;
use std::collections::VecDeque;

/// Maximum forensics entries retained in memory (mobile RAM constraint).
pub const MAX_FORENSICS_ENTRIES: usize = 100;

// ─── Handshake Stage ───────────────────────────────────────────────────────────

/// The discrete stages of an EdgeRay handshake traced for forensics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HandshakeStage {
    DnsResolve,
    TcpHandshake,
    TlsReality,
    Fragment,
    Success,
    Failed,
}

impl HandshakeStage {
    /// Human-readable label for the timeline node.
    pub fn label(&self) -> &'static str {
        match self {
            HandshakeStage::DnsResolve => "DNS Resolve",
            HandshakeStage::TcpHandshake => "TCP Handshake",
            HandshakeStage::TlsReality => "TLS / REALITY",
            HandshakeStage::Fragment => "Fragment Split",
            HandshakeStage::Success => "Connected",
            HandshakeStage::Failed => "Failed",
        }
    }

    /// Material Symbols icon name.
    pub fn icon(&self) -> &'static str {
        match self {
            HandshakeStage::DnsResolve => "dns",
            HandshakeStage::TcpHandshake => "swap_horiz",
            HandshakeStage::TlsReality => "lock",
            HandshakeStage::Fragment => "call_split",
            HandshakeStage::Success => "check_circle",
            HandshakeStage::Failed => "error",
        }
    }

    /// Tailwind text color class for this stage (used when status is Ok).
    pub fn color_class(&self) -> &'static str {
        match self {
            HandshakeStage::DnsResolve => "text-cyan-400",
            HandshakeStage::TcpHandshake => "text-blue-400",
            HandshakeStage::TlsReality => "text-violet-400",
            HandshakeStage::Fragment => "text-purple-400",
            HandshakeStage::Success => "text-emerald-400",
            HandshakeStage::Failed => "text-red-400",
        }
    }

    /// Tailwind background tint class for the circle badge.
    pub fn bg_class(&self) -> &'static str {
        match self {
            HandshakeStage::DnsResolve => "bg-cyan-500/20",
            HandshakeStage::TcpHandshake => "bg-blue-500/20",
            HandshakeStage::TlsReality => "bg-violet-500/20",
            HandshakeStage::Fragment => "bg-purple-500/20",
            HandshakeStage::Success => "bg-emerald-500/20",
            HandshakeStage::Failed => "bg-red-500/20",
        }
    }

    /// 0-based ordinal (for sequential ordering).
    pub fn ordinal(&self) -> u8 {
        match self {
            HandshakeStage::DnsResolve => 0,
            HandshakeStage::TcpHandshake => 1,
            HandshakeStage::TlsReality => 2,
            HandshakeStage::Fragment => 3,
            HandshakeStage::Success => 4,
            HandshakeStage::Failed => 4,
        }
    }
}

// ─── Trace Status ──────────────────────────────────────────────────────────────

/// Outcome of a single handshake stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceStatus {
    Ok,
    DpiDetected,
    Retrying,
    Timeout,
    Error,
}

impl TraceStatus {
    /// Tailwind text color for this status.
    pub fn color_class(&self) -> &'static str {
        match self {
            TraceStatus::Ok => "text-emerald-400",
            TraceStatus::DpiDetected => "text-amber-400",
            TraceStatus::Retrying => "text-amber-400",
            TraceStatus::Timeout => "text-amber-400",
            TraceStatus::Error => "text-red-400",
        }
    }

    /// Background badge class.
    pub fn badge_class(&self) -> &'static str {
        match self {
            TraceStatus::Ok => "text-emerald-400 bg-emerald-500/10",
            TraceStatus::DpiDetected => "text-amber-400 bg-amber-500/10",
            TraceStatus::Retrying => "text-amber-300 bg-amber-500/10",
            TraceStatus::Timeout => "text-amber-400 bg-amber-500/10",
            TraceStatus::Error => "text-red-400 bg-red-500/10",
        }
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            TraceStatus::Ok => "OK",
            TraceStatus::DpiDetected => "DPI",
            TraceStatus::Retrying => "RETRY",
            TraceStatus::Timeout => "TIMEOUT",
            TraceStatus::Error => "ERROR",
        }
    }

    /// Whether this status should trigger the warning/amber glow path.
    pub fn is_warning(&self) -> bool {
        matches!(
            self,
            TraceStatus::DpiDetected | TraceStatus::Retrying | TraceStatus::Timeout
        )
    }

    /// Whether this is a hard failure.
    pub fn is_error(&self) -> bool {
        matches!(self, TraceStatus::Error)
    }
}

// ─── Forensics Entry ───────────────────────────────────────────────────────────

/// A single entry in the forensics handshake timeline.
#[derive(Clone, Debug, PartialEq)]
pub struct ForensicsEntry {
    pub stage: HandshakeStage,
    pub timestamp_ms: u64,
    pub latency_ms: f32,
    pub status: TraceStatus,
    pub geo_label: Option<String>,
    pub outbound_tag: Option<String>,
    /// Whether this is the currently active (in-flight) stage.
    pub is_active: bool,
}

impl ForensicsEntry {
    /// Returns the resolved Tailwind color class, accounting for status overrides.
    pub fn resolved_color_class(&self) -> &'static str {
        if self.is_active {
            "text-cyan-400"
        } else if self.status.is_error() {
            "text-red-400"
        } else if self.status.is_warning() {
            "text-amber-400"
        } else {
            self.stage.color_class()
        }
    }

    /// Returns the resolved bg class for the circle badge.
    pub fn resolved_bg_class(&self) -> &'static str {
        if self.is_active {
            "bg-cyan-500/20"
        } else if self.status.is_error() {
            "bg-red-500/20"
        } else if self.status.is_warning() {
            "bg-amber-500/20"
        } else {
            self.stage.bg_class()
        }
    }

    /// Box-shadow style for glow effects.
    pub fn glow_style(&self) -> &'static str {
        if self.is_active {
            "box-shadow: 0 0 16px rgba(6,182,212,0.5);"
        } else if self.stage == HandshakeStage::Success && self.status == TraceStatus::Ok {
            "box-shadow: 0 0 20px rgba(16,185,129,0.6);"
        } else if self.status.is_warning() {
            "box-shadow: 0 0 12px rgba(245,158,11,0.4);"
        } else if self.status.is_error() {
            "box-shadow: 0 0 12px rgba(239,68,68,0.4);"
        } else {
            ""
        }
    }
}

// ─── Forensics History ─────────────────────────────────────────────────────────

/// Ring buffer of forensics entries, capped for mobile RAM.
#[derive(Clone, Debug, PartialEq)]
pub struct ForensicsHistory {
    pub entries: VecDeque<ForensicsEntry>,
    pub capacity: usize,
}

impl ForensicsHistory {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity.min(MAX_FORENSICS_ENTRIES)),
            capacity: capacity.min(MAX_FORENSICS_ENTRIES),
        }
    }

    pub fn with_default_capacity() -> Self {
        Self::new(MAX_FORENSICS_ENTRIES)
    }

    pub fn push(&mut self, entry: ForensicsEntry) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn latest(&self) -> Option<&ForensicsEntry> {
        self.entries.back()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ForensicsEntry> {
        self.entries.iter()
    }

    /// Clears all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for ForensicsHistory {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

// ─── Component Props ───────────────────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct ForensicsTracerProps {
    pub history: ForensicsHistory,
    /// Currently active connection tag shown in the header.
    #[props(default = String::new())]
    pub connection_label: String,
}

// ─── Component ─────────────────────────────────────────────────────────────────

/// Mobile-optimized vertical handshake forensics timeline.
///
/// Touch-scrollable, tap-to-expand detail, color-coded by outcome.
#[component]
pub fn ForensicsTracer(props: ForensicsTracerProps) -> Element {
    let expanded_idx = use_signal::<Option<usize>>(|| None);
    let entry_count = props.history.len();

    rsx! {
        GlassCard {
            class: "p-4 md:p-6",
            children: rsx! {
                // Header
                div { class: "flex items-center justify-between mb-4 pb-3 border-b border-white/10",
                    div { class: "flex items-center gap-3",
                        Icon { name: "fingerprint".to_string(), class: "text-cyan-400 text-xl".to_string() }
                        div {
                            h3 { class: "text-base font-semibold text-white", "Handshake Forensics" }
                            if !props.connection_label.is_empty() {
                                span { class: "text-[10px] font-mono text-gray-400",
                                    "{props.connection_label}"
                                }
                            }
                        }
                    }
                    div { class: "flex items-center gap-2",
                        div {
                            class: format!(
                                "w-2 h-2 rounded-full {}",
                                if entry_count > 0 && props.history.latest().map_or(false, |e| e.is_active) {
                                    "bg-cyan-400 animate-pulse shadow-[0_0_8px_rgba(6,182,212,0.6)]"
                                } else if entry_count > 0 {
                                    "bg-emerald-500"
                                } else {
                                    "bg-gray-500"
                                }
                            )
                        }
                        span { class: "text-xs font-mono text-gray-500",
                            "{entry_count} events"
                        }
                    }
                }

                // Scrollable timeline
                div { class: "overflow-y-auto max-h-[60vh] -mx-1 px-1 space-y-0",
                    for (idx , entry) in props.history.iter().enumerate() {
                        {render_timeline_node(
                            entry.clone(),
                            idx,
                            idx == entry_count - 1,
                            expanded_idx,
                        )}
                    }

                    if entry_count == 0 {
                        div { class: "py-12 text-center",
                            Icon { name: "hourglass_empty".to_string(), class: "text-gray-600 text-3xl mx-auto mb-2".to_string() }
                            p { class: "text-sm text-gray-500", "Waiting for handshake events…" }
                        }
                    }
                }

                // Summary footer
                if entry_count > 0 {
                    {render_summary_footer(&props.history)}
                }
            }
        }
    }
}

// ─── Timeline Node ─────────────────────────────────────────────────────────────

fn render_timeline_node(
    entry: ForensicsEntry,
    idx: usize,
    is_last: bool,
    mut expanded: Signal<Option<usize>>,
) -> Element {
    let is_expanded = expanded.read().map_or(false, |i| i == idx);
    let color_class = entry.resolved_color_class();
    let bg_class = entry.resolved_bg_class();
    let glow = entry.glow_style();
    let icon_name = entry.stage.icon().to_string();
    let stage_label = entry.stage.label();
    let latency_str = format!("{:.0}ms", entry.latency_ms);
    let status_label = entry.status.label();
    let status_badge = entry.status.badge_class();
    let geo = entry.geo_label.clone().unwrap_or_default();
    let outbound = entry.outbound_tag.clone().unwrap_or_default();
    let is_active = entry.is_active;
    let is_success_terminal =
        entry.stage == HandshakeStage::Success && entry.status == TraceStatus::Ok;

    rsx! {
        div {
            class: "flex gap-3 cursor-pointer active:bg-white/5 rounded-xl transition-colors",
            onclick: move |_| {
                let current = *expanded.read();
                if current == Some(idx) {
                    expanded.set(None);
                } else {
                    expanded.set(Some(idx));
                }
            },

            // ── Timeline spine ──
            div { class: "flex flex-col items-center pt-1",
                // Node circle
                div {
                    class: format!(
                        "size-9 rounded-full flex items-center justify-center transition-all duration-300 {}{}",
                        bg_class,
                        if is_active { " animate-pulse" } else if is_success_terminal { " scale-110" } else { "" }
                    ),
                    style: "{glow}",
                    Icon {
                        name: icon_name,
                        class: format!("text-base {}", color_class)
                    }
                }
                // Connector line
                if !is_last {
                    div { class: "w-0.5 flex-1 bg-white/10 my-1.5 min-h-[28px]" }
                }
            }

            // ── Content ──
            div { class: "flex-1 pb-4 pt-0.5",
                div { class: "flex items-center justify-between mb-0.5",
                    span { class: format!("text-sm font-semibold {}", color_class),
                        "{stage_label}"
                    }
                    div { class: "flex items-center gap-2",
                        span { class: "text-xs font-mono text-gray-500",
                            "{latency_str}"
                        }
                        span {
                            class: format!("px-1.5 py-0.5 rounded text-[9px] font-bold {}", status_badge),
                            "{status_label}"
                        }
                    }
                }

                // Geo tag line
                if !geo.is_empty() {
                    span { class: "text-[10px] font-mono text-gray-500",
                        "📍 {geo}"
                    }
                }

                // Expanded detail
                if is_expanded {
                    div { class: "mt-2 p-3 rounded-lg bg-white/5 border border-white/5 space-y-2",
                        if !outbound.is_empty() {
                            div { class: "flex justify-between",
                                span { class: "text-[10px] font-bold text-gray-500 uppercase tracking-wider", "Outbound" }
                                span { class: "text-xs font-mono text-cyan-400", "{outbound}" }
                            }
                        }
                        div { class: "flex justify-between",
                            span { class: "text-[10px] font-bold text-gray-500 uppercase tracking-wider", "Latency" }
                            span { class: "text-xs font-mono text-gray-300", "{latency_str}" }
                        }
                        div { class: "flex justify-between",
                            span { class: "text-[10px] font-bold text-gray-500 uppercase tracking-wider", "Status" }
                            span { class: format!("text-xs font-mono {}", color_class), "{status_label}" }
                        }
                        if !geo.is_empty() {
                            div { class: "flex justify-between",
                                span { class: "text-[10px] font-bold text-gray-500 uppercase tracking-wider", "Region" }
                                span { class: "text-xs font-mono text-gray-300", "{geo}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Summary Footer ────────────────────────────────────────────────────────────

fn render_summary_footer(history: &ForensicsHistory) -> Element {
    let total_latency: f32 = history.iter().map(|e| e.latency_ms).sum();
    let ok_count = history
        .iter()
        .filter(|e| e.status == TraceStatus::Ok)
        .count();
    let warn_count = history.iter().filter(|e| e.status.is_warning()).count();
    let err_count = history.iter().filter(|e| e.status.is_error()).count();

    rsx! {
        div { class: "mt-4 pt-3 border-t border-white/10 grid grid-cols-4 gap-2",
            div { class: "text-center",
                div { class: "text-[10px] text-gray-500 uppercase", "Total" }
                div { class: "text-sm font-bold text-white font-mono", "{total_latency:.0}ms" }
            }
            div { class: "text-center",
                div { class: "text-[10px] text-gray-500 uppercase", "OK" }
                div { class: "text-sm font-bold text-emerald-400 font-mono", "{ok_count}" }
            }
            div { class: "text-center",
                div { class: "text-[10px] text-gray-500 uppercase", "Warn" }
                div { class: "text-sm font-bold text-amber-400 font-mono", "{warn_count}" }
            }
            div { class: "text-center",
                div { class: "text-[10px] text-gray-500 uppercase", "Errors" }
                div { class: "text-sm font-bold text-red-400 font-mono", "{err_count}" }
            }
        }
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_labels_and_icons() {
        assert_eq!(HandshakeStage::DnsResolve.label(), "DNS Resolve");
        assert_eq!(HandshakeStage::TcpHandshake.label(), "TCP Handshake");
        assert_eq!(HandshakeStage::TlsReality.label(), "TLS / REALITY");
        assert_eq!(HandshakeStage::Fragment.label(), "Fragment Split");
        assert_eq!(HandshakeStage::Success.label(), "Connected");
        assert_eq!(HandshakeStage::Failed.label(), "Failed");

        assert_eq!(HandshakeStage::DnsResolve.icon(), "dns");
        assert_eq!(HandshakeStage::Success.icon(), "check_circle");
        assert_eq!(HandshakeStage::Failed.icon(), "error");
    }

    #[test]
    fn test_stage_color_classes() {
        assert_eq!(HandshakeStage::Success.color_class(), "text-emerald-400");
        assert_eq!(HandshakeStage::Failed.color_class(), "text-red-400");
        assert_eq!(HandshakeStage::DnsResolve.color_class(), "text-cyan-400");
        assert_eq!(HandshakeStage::TlsReality.color_class(), "text-violet-400");
        assert_eq!(HandshakeStage::Fragment.color_class(), "text-purple-400");
    }

    #[test]
    fn test_status_is_warning_and_error() {
        assert!(!TraceStatus::Ok.is_warning());
        assert!(!TraceStatus::Ok.is_error());
        assert!(TraceStatus::DpiDetected.is_warning());
        assert!(TraceStatus::Retrying.is_warning());
        assert!(TraceStatus::Timeout.is_warning());
        assert!(!TraceStatus::Error.is_warning());
        assert!(TraceStatus::Error.is_error());
    }

    #[test]
    fn test_entry_resolved_color_active_overrides() {
        let active = ForensicsEntry {
            stage: HandshakeStage::DnsResolve,
            timestamp_ms: 0,
            latency_ms: 10.0,
            status: TraceStatus::Ok,
            geo_label: None,
            outbound_tag: None,
            is_active: true,
        };
        assert_eq!(active.resolved_color_class(), "text-cyan-400");

        let dpi = ForensicsEntry {
            stage: HandshakeStage::TcpHandshake,
            timestamp_ms: 10,
            latency_ms: 80.0,
            status: TraceStatus::DpiDetected,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(dpi.resolved_color_class(), "text-amber-400");

        let error = ForensicsEntry {
            stage: HandshakeStage::Failed,
            timestamp_ms: 100,
            latency_ms: 0.0,
            status: TraceStatus::Error,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(error.resolved_color_class(), "text-red-400");
    }

    #[test]
    fn test_entry_glow_styles() {
        let active = ForensicsEntry {
            stage: HandshakeStage::DnsResolve,
            timestamp_ms: 0,
            latency_ms: 5.0,
            status: TraceStatus::Ok,
            geo_label: None,
            outbound_tag: None,
            is_active: true,
        };
        assert!(active.glow_style().contains("6,182,212"));

        let success = ForensicsEntry {
            stage: HandshakeStage::Success,
            timestamp_ms: 100,
            latency_ms: 0.0,
            status: TraceStatus::Ok,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert!(success.glow_style().contains("16,185,129"));
    }

    #[test]
    fn test_history_capacity_enforced_at_100() {
        let mut hist = ForensicsHistory::new(200);
        assert_eq!(hist.capacity, MAX_FORENSICS_ENTRIES);

        for i in 0..200u64 {
            hist.push(ForensicsEntry {
                stage: HandshakeStage::DnsResolve,
                timestamp_ms: i * 10,
                latency_ms: i as f32,
                status: TraceStatus::Ok,
                geo_label: None,
                outbound_tag: None,
                is_active: false,
            });
        }
        assert_eq!(hist.len(), MAX_FORENSICS_ENTRIES);
        assert_eq!(hist.latest().unwrap().timestamp_ms, 1990);
        assert_eq!(hist.entries.front().unwrap().timestamp_ms, 1000);
    }

    #[test]
    fn test_history_push_eviction() {
        let mut hist = ForensicsHistory::new(3);
        for i in 0..5 {
            hist.push(ForensicsEntry {
                stage: HandshakeStage::DnsResolve,
                timestamp_ms: i as u64 * 100,
                latency_ms: i as f32,
                status: TraceStatus::Ok,
                geo_label: None,
                outbound_tag: None,
                is_active: false,
            });
        }
        assert_eq!(hist.len(), 3);
        assert_eq!(hist.entries.front().unwrap().timestamp_ms, 200);
        assert_eq!(hist.latest().unwrap().timestamp_ms, 400);
    }

    #[test]
    fn test_history_empty() {
        let hist = ForensicsHistory::with_default_capacity();
        assert!(hist.is_empty());
        assert!(hist.latest().is_none());
    }

    #[test]
    fn test_failure_events_map_to_amber_red() {
        let dpi = ForensicsEntry {
            stage: HandshakeStage::TcpHandshake,
            timestamp_ms: 50,
            latency_ms: 120.0,
            status: TraceStatus::DpiDetected,
            geo_label: Some("SIN".to_string()),
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(dpi.resolved_color_class(), "text-amber-400");
        assert_eq!(dpi.resolved_bg_class(), "bg-amber-500/20");

        let timeout = ForensicsEntry {
            stage: HandshakeStage::TlsReality,
            timestamp_ms: 200,
            latency_ms: 5000.0,
            status: TraceStatus::Timeout,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(timeout.resolved_color_class(), "text-amber-400");

        let error = ForensicsEntry {
            stage: HandshakeStage::Failed,
            timestamp_ms: 300,
            latency_ms: 0.0,
            status: TraceStatus::Error,
            geo_label: None,
            outbound_tag: None,
            is_active: false,
        };
        assert_eq!(error.resolved_color_class(), "text-red-400");
        assert_eq!(error.resolved_bg_class(), "bg-red-500/20");
    }
}
