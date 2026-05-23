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

    let size_px = match props.size.as_str() {
        "sm" => 16,
        "md" => 20,
        "lg" => 24,
        "xl" => 32,
        _ => 20,
    };

    let glow_class = match props.glow.as_str() {
        "cyan" => "text-primary drop-shadow-[0_0_8px_rgba(0,240,255,0.8)]",
        "purple" => "text-purple-500 drop-shadow-[0_0_8px_rgba(191,0,255,0.8)]",
        _ => "",
    };

    let combined_class = format!("{} {} {}", size_class, glow_class, props.class);

    // Use our new inline Rust icons for critical UI elements
    match props.name.as_str() {
        "shield" | "security" => rsx! {
            crate::components::icons::ShieldCheck {
                size: size_px,
                class: combined_class,
            }
        },
        "public" | "language" => rsx! {
            crate::components::icons::Globe {
                size: size_px,
                class: combined_class,
            }
        },
        "settings" | "cog" => rsx! {
            crate::components::icons::Settings {
                size: size_px,
                class: combined_class,
            }
        },
        "power_settings_new" => rsx! {
            crate::components::icons::Power {
                size: size_px,
                class: combined_class,
            }
        },
        "activity" => rsx! {
            crate::components::icons::Activity {
                size: size_px,
                class: combined_class,
            }
        },
        "dns" | "hub" => rsx! {
            crate::components::icons::Cpu {
                size: size_px,
                class: combined_class,
            }
        },
        "folder_open" | "folder_zip" => rsx! {
            crate::components::icons::HardDrive {
                size: size_px,
                class: combined_class,
            }
        },
        _ => {
            // Fallback to Material Symbols Font for non-critical icons
            rsx! {
                span {
                    class: format!("material-symbols-outlined select-none text-2xl {}", combined_class),
                    "{props.name}"
                }
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
