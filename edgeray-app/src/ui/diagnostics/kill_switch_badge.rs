// edgeray-app/src/ui/diagnostics/kill_switch_badge.rs
//! Kill-Switch Badge Component
//!
//! A reactive UI badge that indicates when the VPN kill-switch is active,
//! preventing traffic leaks when the VPN connection drops.

use dioxus::prelude::*;

/// Kill-Switch Badge Props
#[derive(Props, Clone, PartialEq)]
pub struct KillSwitchBadgeProps {
    /// Whether the kill-switch is currently active
    #[props(default = false)]
    pub active: bool,
    /// Whether the VPN is connected (determines badge visibility behavior)
    #[props(default = false)]
    pub vpn_connected: bool,
}

/// Kill-Switch Badge Component
///
/// Displays a visual indicator when the VPN kill-switch is engaged,
/// blocking all non-VPN traffic to prevent data leaks.
#[component]
pub fn KillSwitchBadge(props: KillSwitchBadgeProps) -> Element {
    let badge_style = if props.active {
        "background: linear-gradient(135deg, #dc2626 0%, #991b1b 100%); 
         border: 2px solid #ef4444;
         box-shadow: 0 0 20px rgba(239, 68, 68, 0.6), 0 0 40px rgba(239, 68, 68, 0.3);
         animation: pulse 1.5s ease-in-out infinite;"
    } else {
        "background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
         border: 2px solid #4ade80;
         box-shadow: 0 0 10px rgba(74, 222, 128, 0.3);"
    };

    let icon = if props.active { "🛡️" } else { "🔓" };
    let label = if props.active {
        "KILL-SWITCH ACTIVE"
    } else {
        "Protected"
    };

    rsx! {
        style { {KILL_SWITCH_STYLES} }

        div {
            class: "kill-switch-badge",
            style: "{badge_style}",

            span { class: "badge-icon", "{icon}" }
            span { class: "badge-label", "{label}" }

            if props.active {
                div { class: "badge-pulse-ring" }
            }
        }
    }
}

const KILL_SWITCH_STYLES: &str = r#"
    .kill-switch-badge {
        display: inline-flex;
        align-items: center;
        gap: 8px;
        padding: 8px 16px;
        border-radius: 24px;
        font-family: 'Inter', system-ui, sans-serif;
        font-weight: 600;
        font-size: 12px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
        color: white;
        position: relative;
        overflow: hidden;
        transition: all 0.3s ease;
    }
    
    .badge-icon {
        font-size: 16px;
    }
    
    .badge-label {
        white-space: nowrap;
    }
    
    .badge-pulse-ring {
        position: absolute;
        inset: -2px;
        border: 2px solid rgba(239, 68, 68, 0.5);
        border-radius: 26px;
        animation: ring-pulse 2s ease-out infinite;
    }
    
    @keyframes pulse {
        0%, 100% { transform: scale(1); }
        50% { transform: scale(1.02); }
    }
    
    @keyframes ring-pulse {
        0% { transform: scale(1); opacity: 1; }
        100% { transform: scale(1.3); opacity: 0; }
    }
"#;
