//! EdgeRay UI Design System
//!
//! A comprehensive set of reusable UI components with consistent
//! glassmorphism styling and responsive design support.

use dioxus::prelude::*;

// ============================================================================
// DESIGN TOKENS (CSS-in-Rust approach for consistency)
// ============================================================================

/// Common border radius classes
pub mod radius {
    pub const SM: &str = "rounded-lg";
    pub const MD: &str = "rounded-xl";
    pub const LG: &str = "rounded-2xl";
    pub const XL: &str = "rounded-3xl";
    pub const FULL: &str = "rounded-full";
}

/// Glass effect classes
pub mod glass {
    pub const CARD: &str = "glass rounded-xl";
    pub const PANEL: &str = "glass-panel";
    pub const BUTTON: &str = "glass-button rounded-xl";
    pub const INPUT: &str =
        "glass-inset rounded-lg focus:ring-2 focus:ring-primary/20 transition-all";
}

/// Color palette
pub mod colors {
    pub const PRIMARY: &str = "text-primary"; // #3b82f6
    pub const SUCCESS: &str = "text-emerald-500";
    pub const WARNING: &str = "text-amber-500";
    pub const ERROR: &str = "text-red-500";
    pub const MUTED: &str = "text-slate-500 dark:text-slate-400";
}

// ============================================================================
// CORE COMPONENTS
// ============================================================================

/// SVG Icon component using local assets
#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub name: String,
    #[props(default = String::new())]
    pub class: String,
    #[props(default = false)]
    pub filled: bool,
    #[props(default = "md".to_string())]
    pub size: String,
    #[props(default = "none".to_string())]
    pub glow: String,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let size_class = match props.size.as_str() {
        "sm" => "w-4 h-4",
        "md" => "w-5 h-5",
        "lg" => "w-6 h-6",
        "xl" => "w-8 h-8",
        _ => "w-5 h-5",
    };

    let glow_class = match props.glow.as_str() {
        "cyan" => "text-primary drop-shadow-[0_0_8px_rgba(0,240,255,0.8)]",
        "purple" => "text-purple-500 drop-shadow-[0_0_8px_rgba(191,0,255,0.8)]",
        _ => "",
    };

    let combined_class = format!("{} {} {}", size_class, glow_class, props.class);

    // Try to load local SVG asset first
    let svg_content = match props.name.as_str() {
        // ── Asset-based icons (include_str!) ──
        "add" => Some(include_str!("../../assets/icons/add.svg")),
        "add_circle" => Some(include_str!("../../assets/icons/add_circle.svg")),
        "alt_route" => Some(include_str!("../../assets/icons/alt_route.svg")),
        "arrow_back" => Some(include_str!("../../assets/icons/arrow_back.svg")),
        "article" => Some(include_str!("../../assets/icons/article.svg")),
        "block" => Some(include_str!("../../assets/icons/block.svg")),
        "bolt" => Some(include_str!("../../assets/icons/bolt.svg")),
        "calendar_today" => Some(include_str!("../../assets/icons/calendar_today.svg")),
        "check" => Some(include_str!("../../assets/icons/check.svg")),
        "chevron_left" => Some(include_str!("../../assets/icons/chevron_left.svg")),
        "chevron_right" => Some(include_str!("../../assets/icons/chevron_right.svg")),
        "close" => Some(include_str!("../../assets/icons/close.svg")),
        "content_paste" => Some(include_str!("../../assets/icons/content_paste.svg")),
        "content_paste_go" => Some(include_str!("../../assets/icons/content_paste_go.svg")),
        "control_point" => Some(include_str!("../../assets/icons/control_point.svg")),
        "dashboard" => Some(include_str!("../../assets/icons/dashboard.svg")),
        "delete" => Some(include_str!("../../assets/icons/delete.svg")),
        "devices" => Some(include_str!("../../assets/icons/devices.svg")),
        "dns" => Some(include_str!("../../assets/icons/dns.svg")),
        "download" => Some(include_str!("../../assets/icons/download.svg")),
        "edit" => Some(include_str!("../../assets/icons/edit.svg")),
        "error" => Some(include_str!("../../assets/icons/error.svg")),
        "folder_open" => Some(include_str!("../../assets/icons/folder_open.svg")),
        "graphic_eq" => Some(include_str!("../../assets/icons/graphic_eq.svg")),
        "grid_view" => Some(include_str!("../../assets/icons/grid_view.svg")),
        "history" => Some(include_str!("../../assets/icons/history.svg")),
        "home" => Some(include_str!("../../assets/icons/home.svg")),
        "hub" => Some(include_str!("../../assets/icons/hub.svg")),
        "info" => Some(include_str!("../../assets/icons/info.svg")),
        "keyboard_double_arrow_right" => Some(include_str!(
            "../../assets/icons/keyboard_double_arrow_right.svg"
        )),
        "language" => Some(include_str!("../../assets/icons/language.svg")),
        "link" => Some(include_str!("../../assets/icons/link.svg")),
        "menu" => Some(include_str!("../../assets/icons/menu.svg")),
        "navigate_next" => Some(include_str!("../../assets/icons/navigate_next.svg")),
        "network_check" => Some(include_str!("../../assets/icons/network_check.svg")),
        "notifications" => Some(include_str!("../../assets/icons/notifications.svg")),
        "play_circle" => Some(include_str!("../../assets/icons/play_circle.svg")),
        "power_settings_new" => Some(include_str!("../../assets/icons/power_settings_new.svg")),
        "public" => Some(include_str!("../../assets/icons/public.svg")),
        "qr_code_scanner" => Some(include_str!("../../assets/icons/qr_code_scanner.svg")),
        "settings" => Some(include_str!("../../assets/icons/settings.svg")),
        "share" => Some(include_str!("../../assets/icons/share.svg")),
        "snippet_folder" => Some(include_str!("../../assets/icons/snippet_folder.svg")),
        "star" => Some(include_str!("../../assets/icons/star.svg")),
        "terminal" => Some(include_str!("../../assets/icons/terminal.svg")),
        "tune" => Some(include_str!("../../assets/icons/tune.svg")),
        "upload" => Some(include_str!("../../assets/icons/upload.svg")),
        "upload_file" => Some(include_str!("../../assets/icons/upload_file.svg")),
        "visibility" => Some(include_str!("../../assets/icons/visibility.svg")),
        "warning" => Some(include_str!("../../assets/icons/warning.svg")),

        // ── Inline SVG icons (used by Page::icon() and sidebar) ──
        "server" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect><rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect><line x1="6" y1="6" x2="6.01" y2="6"></line><line x1="6" y1="18" x2="6.01" y2="18"></line></svg>"#,
        ),
        "shield" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path></svg>"#,
        ),
        "network" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="5" r="3"></circle><circle cx="5" cy="19" r="3"></circle><circle cx="19" cy="19" r="3"></circle><line x1="12" y1="8" x2="12" y2="12"></line><path d="M12 12l-7 7M12 12l7 7"></path></svg>"#,
        ),
        "chart" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="12" y1="20" x2="12" y2="10"></line><line x1="18" y1="20" x2="18" y2="4"></line><line x1="6" y1="20" x2="6" y2="16"></line></svg>"#,
        ),
        "activity" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="22,12 18,12 15,21 9,3 6,12 2,12"></polyline></svg>"#,
        ),
        "merge" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="18" cy="18" r="3"></circle><circle cx="6" cy="6" r="3"></circle><path d="M6 21V9a9 9 0 0 0 9 9"></path></svg>"#,
        ),
        "sliders" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><line x1="4" y1="21" x2="4" y2="14"></line><line x1="4" y1="10" x2="4" y2="3"></line><line x1="12" y1="21" x2="12" y2="12"></line><line x1="12" y1="8" x2="12" y2="3"></line><line x1="20" y1="21" x2="20" y2="16"></line><line x1="20" y1="12" x2="20" y2="3"></line><line x1="1" y1="14" x2="7" y2="14"></line><line x1="9" y1="8" x2="15" y2="8"></line><line x1="17" y1="16" x2="23" y2="16"></line></svg>"#,
        ),
        "cog" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>"#,
        ),
        // Icons used by Page::icon() that have no SVG file
        "folder_zip" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M4 20V4a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z"></path><path d="M12 10v4M10 12h4"></path></svg>"#,
        ),
        "security" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path><path d="M9 12l2 2 4-4"></path></svg>"#,
        ),
        "healing" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M17.73 12L21.71 8.04a1 1 0 0 0 0-1.41l-4.34-4.34a1 1 0 0 0-1.41 0L12 6.27 8.04 2.29a1 1 0 0 0-1.41 0L2.29 6.63a1 1 0 0 0 0 1.41L6.27 12l-3.98 3.96a1 1 0 0 0 0 1.41l4.34 4.34a1 1 0 0 0 1.41 0L12 17.73l3.96 3.98a1 1 0 0 0 1.41 0l4.34-4.34a1 1 0 0 0 0-1.41L17.73 12z"></path><line x1="12" y1="10" x2="12" y2="14"></line><line x1="10" y1="12" x2="14" y2="12"></line></svg>"#,
        ),
        "show_chart" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><polyline points="3,17 9,11 13,15 21,7"></polyline></svg>"#,
        ),
        "apps" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><rect x="3" y="3" width="6" height="6" rx="1"></rect><rect x="15" y="3" width="6" height="6" rx="1"></rect><rect x="3" y="15" width="6" height="6" rx="1"></rect><rect x="15" y="15" width="6" height="6" rx="1"></rect></svg>"#,
        ),
        "rocket_launch" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"></path><path d="M12 15l-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"></path><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"></path><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"></path></svg>"#,
        ),
        "fingerprint" => Some(
            r#"<svg fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M2 12C2 6.5 6.5 2 12 2a10 10 0 0 1 8 4"></path><path d="M5 19.5C5.5 18 6 15 6 12c0-3.3 2.7-6 6-6 1.8 0 3.4.8 4.5 2"></path><path d="M17.8 14c-.3 2.8-1.1 4.5-1.8 5.5"></path><path d="M12 12a2 2 0 1 0-2 2"></path><path d="M10 18c.3 2 1.2 3.5 2 4"></path></svg>"#,
        ),
        _ => None,
    };

    if let Some(content) = svg_content {
        rsx! {
            span {
                class: format!("inline-flex shrink-0 leading-none items-center justify-center [&>svg]:w-full [&>svg]:h-full {}", combined_class),
                dangerous_inner_html: "{content}"
            }
        }
    } else {
        // Fallback to Material Symbols Font
        // We use material-symbols-rounded for a more modern look if available, or outlined
        rsx! {
            span {
                class: format!("material-symbols-outlined select-none text-2xl {}", combined_class),
                "{props.name}"
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AppLayoutProps {
    children: Element,
}

#[component]
pub fn AppLayout(props: AppLayoutProps) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 font-sans",
            {props.children}
        }
    }
}

/// Glass card container
#[derive(Props, Clone, PartialEq)]
pub struct GlassCardProps {
    #[props(default = String::new())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn GlassCard(props: GlassCardProps) -> Element {
    rsx! {
        div {
            class: format!(
                "{} {}",
                glass::CARD,
                props.class
            ),
            {props.children}
        }
    }
}

/// Glass panel (larger container)
#[derive(Props, Clone, PartialEq)]
pub struct GlassPanelProps {
    #[props(default = String::new())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn GlassPanel(props: GlassPanelProps) -> Element {
    rsx! {
        div {
            class: format!(
                "bg-white/60 dark:bg-slate-900/60 backdrop-blur-2xl border border-white/40 dark:border-white/10 rounded-3xl shadow-xl {}",
                props.class
            ),
            {props.children}
        }
    }
}

/// Page header component with back button support
#[derive(Props, Clone, PartialEq)]
pub struct PageHeaderProps {
    pub title: String,
    #[props(default = None)]
    pub subtitle: Option<String>,
    #[props(default = None)]
    pub left_action: Option<Element>,
    #[props(default = None)]
    pub right_action: Option<Element>,
}

#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-30 flex items-center justify-between px-4 lg:px-6 py-3 bg-white/60 dark:bg-slate-900/60 backdrop-blur-2xl border-b border-white/40 dark:border-white/10 shadow-sm",
            // Left: Action or spacer
            if let Some(action) = props.left_action {
                {action}
            } else {
                div { class: "w-10" }
            }

            // Center: Title
            div {
                class: "flex flex-col items-center",
                h1 {
                    class: "text-lg lg:text-xl font-bold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-slate-900 to-slate-700 dark:from-white dark:to-slate-300",
                    "{props.title}"
                }
                if let Some(subtitle) = &props.subtitle {
                    span { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 mt-0.5", "{subtitle}" }
                }
            }

            // Right: Action or spacer
            if let Some(action) = props.right_action {
                {action}
            } else {
                div { class: "w-10" }
            }
        }
    }
}

/// Section header for grouping content
#[derive(Props, Clone, PartialEq)]
pub struct SectionHeaderProps {
    pub title: String,
    #[props(default = None)]
    pub icon: Option<String>,
    #[props(default = None)]
    pub action: Option<Element>,
}

#[component]
pub fn SectionHeader(props: SectionHeaderProps) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between mb-3 px-1",
            div {
                class: "flex items-center gap-2",
                if let Some(icon) = &props.icon {
                    Icon { name: icon.clone(), class: "text-primary text-xl drop-shadow-sm".to_string() }
                }
                h2 {
                    class: "text-xs font-bold uppercase tracking-widest text-slate-500 dark:text-slate-400",
                    "{props.title}"
                }
            }
            if let Some(action) = props.action {
                {action}
            }
        }
    }
}

/// Settings toggle item
#[derive(Props, Clone, PartialEq)]
pub struct ToggleItemProps {
    pub label: String,
    #[props(default = None)]
    pub sublabel: Option<String>,
    pub checked: Signal<bool>,
    #[props(default = false)]
    pub last_item: bool,
    #[props(default = None)]
    pub onchange: Option<EventHandler<bool>>,
}

#[component]
pub fn ToggleItem(mut props: ToggleItemProps) -> Element {
    let border_class = if props.last_item {
        ""
    } else {
        "border-b border-white/20 dark:border-white/5"
    };

    rsx! {
        div {
            class: format!("flex items-center justify-between px-4 py-3.5 hover:bg-white/30 dark:hover:bg-white/5 transition-colors {}", border_class),
            div {
                class: "flex flex-col",
                span { class: "text-slate-800 dark:text-slate-100 text-base font-medium", "{props.label}" }
                if let Some(sublabel) = &props.sublabel {
                    span { class: "text-xs text-slate-500 dark:text-slate-400 mt-0.5", "{sublabel}" }
                }
            }
            // Toggle switch
            label {
                class: "relative flex h-7 w-12 cursor-pointer items-center rounded-full bg-slate-300/50 dark:bg-white/10 p-0.5 has-[:checked]:bg-primary transition-colors duration-300 shadow-inner",
                input {
                    class: "peer sr-only",
                    r#type: "checkbox",
                    checked: *props.checked.read(),
                    onchange: move |_| {
                        let new_val = !*props.checked.read();
                        props.checked.set(new_val);
                        if let Some(handler) = &props.onchange {
                            handler.call(new_val);
                        }
                    },
                }
                div { class: "h-6 w-6 rounded-full bg-white shadow-md transition-all duration-300 translate-x-0 peer-checked:translate-x-5" }
            }
        }
    }
}

/// Navigation item for settings/lists
#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    pub label: String,
    #[props(default = None)]
    pub value: Option<String>,
    pub onclick: EventHandler<()>,
    #[props(default = false)]
    pub last_item: bool,
}

#[component]
pub fn NavItem(props: NavItemProps) -> Element {
    let border_class = if props.last_item {
        ""
    } else {
        "border-b border-white/20 dark:border-white/5"
    };

    rsx! {
        button {
            class: format!("w-full flex items-center justify-between px-4 py-3.5 hover:bg-white/30 dark:hover:bg-white/5 transition-colors text-left group {}", border_class),
            onclick: move |_| props.onclick.call(()),
            span { class: "text-slate-800 dark:text-slate-100 text-base font-medium", "{props.label}" }
            div {
                class: "flex items-center gap-2",
                if let Some(value) = &props.value {
                    span { class: "text-sm text-slate-500 dark:text-slate-400", "{value}" }
                }
                Icon { name: "chevron_right".to_string(), class: "text-slate-400 group-hover:text-primary transition-colors text-xl".to_string() }
            }
        }
    }
}

/// Primary action button
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryButtonProps {
    pub label: String,
    #[props(default = None)]
    pub icon: Option<String>,
    pub onclick: EventHandler<()>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
}

#[component]
pub fn PrimaryButton(props: PrimaryButtonProps) -> Element {
    let disabled_class = if props.disabled || props.loading {
        "opacity-50 cursor-not-allowed"
    } else {
        "hover:bg-primary active:scale-[0.98]"
    };

    rsx! {
        button {
            class: format!("w-full bg-primary/90 text-white font-bold py-3.5 px-4 rounded-xl shadow-lg shadow-primary/30 transition-all flex items-center justify-center gap-2 {}", disabled_class),
            disabled: props.disabled || props.loading,
            onclick: move |_| props.onclick.call(()),
            if props.loading {
                div { class: "w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin" }
            } else if let Some(icon) = &props.icon {
                Icon { name: icon.clone(), class: "".to_string() }
            }
            span { "{props.label}" }
        }
    }
}

/// Secondary/ghost button
#[derive(Props, Clone, PartialEq)]
pub struct SecondaryButtonProps {
    pub label: String,
    #[props(default = None)]
    pub icon: Option<String>,
    pub onclick: EventHandler<()>,
}

#[component]
pub fn SecondaryButton(props: SecondaryButtonProps) -> Element {
    rsx! {
        button {
            class: "px-4 py-2 rounded-xl bg-white/40 dark:bg-white/10 hover:bg-white/60 dark:hover:bg-white/20 border border-white/40 dark:border-white/10 text-slate-700 dark:text-slate-200 font-semibold text-sm transition-all flex items-center gap-2",
            onclick: move |_| props.onclick.call(()),
            if let Some(icon) = &props.icon {
                Icon { name: icon.clone(), class: "text-lg".to_string() }
            }
            span { "{props.label}" }
        }
    }
}

/// Metric tile for dashboard
#[derive(Props, Clone, PartialEq)]
pub struct MetricTileProps {
    pub label: String,
    pub value: String,
    pub unit: String,
    pub icon: String,
}

#[component]
pub fn MetricTile(props: MetricTileProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-white/40 dark:bg-white/5 backdrop-blur-lg border border-white/50 dark:border-white/10 transition-all hover:bg-white/60 dark:hover:bg-white/10 group shadow-sm",
            span {
                class: "text-[10px] font-bold text-slate-500 dark:text-slate-400 uppercase tracking-wider group-hover:text-primary transition-colors",
                "{props.label}"
            }
            div {
                class: "flex items-baseline gap-1",
                span { class: "text-2xl font-bold font-mono text-slate-800 dark:text-white tracking-tight", "{props.value}" }
                span { class: "text-xs font-medium text-slate-400 dark:text-slate-500", "{props.unit}" }
            }
        }
    }
}

/// Empty state placeholder
#[derive(Props, Clone, PartialEq)]
pub struct EmptyStateProps {
    pub icon: String,
    pub title: String,
    pub message: String,
    #[props(default = None)]
    pub action: Option<Element>,
}

#[component]
pub fn EmptyState(props: EmptyStateProps) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center py-16 px-8 text-center",
            div {
                class: "size-20 rounded-full bg-slate-100 dark:bg-slate-800 flex items-center justify-center mb-4",
                Icon { name: props.icon, class: "text-4xl text-slate-400 dark:text-slate-500".to_string() }
            }
            h3 { class: "text-lg font-semibold text-slate-700 dark:text-slate-200 mb-2", "{props.title}" }
            p { class: "text-sm text-slate-500 dark:text-slate-400 max-w-xs mb-6", "{props.message}" }
            if let Some(action) = props.action {
                {action}
            }
        }
    }
}

/// Loading spinner
#[component]
pub fn LoadingSpinner() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center py-8",
            div { class: "w-8 h-8 border-3 border-primary/30 border-t-primary rounded-full animate-spin" }
        }
    }
}

/// Status badge
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    #[props(default = "default".to_string())]
    pub variant: String, // "success", "warning", "error", "default"
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let variant_class = match props.variant.as_str() {
        "success" => {
            "bg-success/10 text-emerald-600 dark:text-success border-success/20 shadow-[0_0_10px_rgba(0,255,163,0.2)]"
        }
        "warning" => "bg-warning/10 text-amber-600 dark:text-warning border-warning/20",
        "error" => "bg-error/10 text-red-600 dark:text-error border-error/20",
        _ => "bg-primary/10 text-primary border-primary/20",
    };

    rsx! {
        span {
            class: format!("text-[10px] font-bold uppercase tracking-wide px-2 py-0.5 rounded-full border {}", variant_class),
            "{props.label}"
        }
    }
}
