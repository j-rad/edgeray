//! EdgeRay UI Components
//!
//! Glass-style UI components for the EdgeRay client.
//! Ported from the v2ray-glass React/Tailwind design.

pub mod asset_integrity;
pub mod asset_manager_view;

pub mod bottom_nav;
pub mod connect_button;
pub mod dashboard;
pub mod forms;
pub mod gestures;
pub mod icons;
pub mod import_view;
pub mod log_view;
pub mod mesh_dashboard;
pub mod metric_value;
pub mod per_app_view;
pub mod power_core;
pub mod protocol_forms;
pub mod routing_manager;
pub mod routing_view;
pub mod server_add_modal;
pub mod server_card;
pub mod server_editor;
pub mod server_list;
pub mod settings_screen;

pub mod glass_card;
pub mod qr;
pub mod qr_scanner;
pub mod qr_share;
pub mod reconnect_overlay;
pub mod scanline_layer;
pub mod shimmer;
pub mod sidebar;
pub mod sparkline;
pub mod subscription_view;
pub mod telemetry_panel;
pub mod theme;
pub mod ui;
pub mod ui_engine;

// Re-export commonly used types
pub use asset_manager_view::AssetManagerView;
pub use bottom_nav::BottomNav;
pub use dashboard::Dashboard;
pub use import_view::ImportView;
pub use log_view::LogView;
pub use mesh_dashboard::MeshDashboard;
pub use server_list::ServerList;
pub use settings_screen::SettingsScreen;

pub use shimmer::GlassShimmer;

pub use sidebar::Sidebar;
pub use subscription_view::SubscriptionView;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Page {
    Dashboard,
    Configs,
    SubscriptionGroups,
    RoutingRules,
    Assets,
    Settings,
    Logs,
    Mesh,
    Firewall,
    DnsTuning,
    FlowJTuning,
    StackMonitor,
    PerAppProxy,
    AdvancedTuning,
    About,
    Setup,
    Shield,
    Forensics,
}

impl Page {
    pub fn icon(&self) -> &'static str {
        match self {
            Page::Dashboard => "grid_view",
            Page::Configs => "dns",
            Page::SubscriptionGroups => "folder_open",
            Page::RoutingRules => "alt_route",
            Page::Assets => "folder_zip",
            Page::Settings => "settings",
            Page::Logs => "terminal",
            Page::Mesh => "hub",
            Page::Firewall => "security",
            Page::DnsTuning => "dns",
            Page::FlowJTuning => "healing",
            Page::StackMonitor => "show_chart",
            Page::PerAppProxy => "apps",
            Page::AdvancedTuning => "tune",
            Page::About => "info",
            Page::Setup => "rocket_launch",
            Page::Shield => "shield",
            Page::Forensics => "fingerprint",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Page::Dashboard => "Home",
            Page::Configs => "Configs",
            Page::SubscriptionGroups => "Groups",
            Page::RoutingRules => "Routing",
            Page::Assets => "Assets",
            Page::Settings => "Settings",
            Page::Logs => "Logs",
            Page::Mesh => "Mesh",
            Page::Firewall => "Firewall",
            Page::DnsTuning => "DNS Tuning",
            Page::FlowJTuning => "Flow-J",
            Page::StackMonitor => "Stack Monitor",
            Page::PerAppProxy => "Per-App Proxy",
            Page::AdvancedTuning => "Advanced Tuning",
            Page::About => "About",
            Page::Setup => "Setup",
            Page::Shield => "Shield",
            Page::Forensics => "Forensics",
        }
    }
}
