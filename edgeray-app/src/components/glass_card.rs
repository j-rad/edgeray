//! GlassCard Component
//!
//! A repeatable glassmorphism card component with:
//! - Configurable backdrop blur (default 30px)
//! - Neon border variants (purple/cyan/white)
//! - Platform-specific padding
//! - GPU-accelerated transforms for 60fps scrolling

use dioxus::prelude::*;

/// Neon border color variants for glass cards
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum NeonBorder {
    /// Default subtle white border
    #[default]
    Subtle,
    /// Cyber purple neon glow
    Purple,
    /// Cyan neon glow
    Cyan,
    /// Emerald neon glow
    Emerald,
    /// Ignite — reactive border that glows on hover (desktop) / touch (mobile)
    Ignite,
    /// No border
    None,
}

impl NeonBorder {
    /// Returns the CSS classes for this border style
    pub fn classes(&self) -> &'static str {
        match self {
            NeonBorder::Subtle => "border border-white/10 dark:border-white/5",
            NeonBorder::Purple => {
                "border border-purple-500/30 shadow-[0_0_15px_rgba(139,92,246,0.15)]"
            }
            NeonBorder::Cyan => "border border-cyan-400/30 shadow-[0_0_15px_rgba(34,211,238,0.15)]",
            NeonBorder::Emerald => {
                "border border-emerald-400/30 shadow-[0_0_15px_rgba(52,211,153,0.15)]"
            }
            NeonBorder::Ignite => {
                "border border-white/10 transition-all duration-300 hover:border-cyan-400/50 hover:shadow-[0_0_20px_rgba(0,242,255,0.25)] active:border-cyan-400/70 active:shadow-[0_0_30px_rgba(0,242,255,0.4)]"
            }
            NeonBorder::None => "",
        }
    }
}

/// Card size/padding variants
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardPadding {
    /// No padding (for custom content)
    None,
    /// Compact padding (p-3 / p-2 mobile)
    Compact,
    /// Standard padding (p-4 / p-3 mobile)
    #[default]
    Standard,
    /// Large padding (p-6 / p-4 mobile)
    Large,
}

impl CardPadding {
    /// Returns responsive Tailwind padding classes
    pub fn classes(&self) -> &'static str {
        match self {
            CardPadding::None => "",
            CardPadding::Compact => "p-2 sm:p-3",
            CardPadding::Standard => "p-3 sm:p-4",
            CardPadding::Large => "p-4 sm:p-6",
        }
    }
}

/// Border radius variants
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CardRadius {
    /// Small radius (rounded-lg)
    Small,
    /// Medium radius (rounded-xl)
    Medium,
    /// Large radius (rounded-2xl)
    #[default]
    Large,
    /// Extra large radius (rounded-3xl)
    ExtraLarge,
}

impl CardRadius {
    /// Returns Tailwind border-radius class
    pub fn classes(&self) -> &'static str {
        match self {
            CardRadius::Small => "rounded-lg",
            CardRadius::Medium => "rounded-xl",
            CardRadius::Large => "rounded-2xl",
            CardRadius::ExtraLarge => "rounded-3xl",
        }
    }
}

/// Props for the GlassCard component
#[derive(Props, Clone, PartialEq)]
pub struct GlassCardProps {
    /// Child elements to render inside the card
    pub children: Element,
    /// Additional CSS classes to apply
    #[props(default = String::new())]
    pub class: String,
    /// Neon border style
    #[props(default)]
    pub border: NeonBorder,
    /// Padding size
    #[props(default)]
    pub padding: CardPadding,
    /// Border radius
    #[props(default)]
    pub radius: CardRadius,
    /// Enable hover effect
    #[props(default = true)]
    pub hover: bool,
    /// Enable backdrop blur (disable for performance on low-end devices)
    #[props(default = true)]
    pub blur: bool,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<()>>,
}

/// GlassCard - Repeatable glassmorphism container
///
/// A flexible glass-effect card with:
/// - 30px backdrop blur for premium frosted glass effect
/// - Neon border options for cyber aesthetic
/// - Responsive padding that adapts to screen size
/// - GPU-accelerated transforms for smooth 60fps scrolling
/// - Hover states for interactivity
#[component]
pub fn GlassCard(props: GlassCardProps) -> Element {
    let base_classes =
        "relative bg-white/5 dark:bg-white/[0.03] will-change-transform transform-gpu";

    let blur_classes = if props.blur {
        "backdrop-blur-[30px] [-webkit-backdrop-filter:blur(30px)]"
    } else {
        ""
    };

    let hover_classes = if props.hover {
        "transition-all duration-200 hover:bg-white/10 dark:hover:bg-white/[0.06] hover:border-white/20 dark:hover:border-white/10 hover:shadow-lg"
    } else {
        ""
    };

    let interactive_classes = if props.onclick.is_some() {
        "cursor-pointer active:scale-[0.98]"
    } else {
        ""
    };

    let full_class = format!(
        "{} {} {} {} {} {} {} {}",
        base_classes,
        blur_classes,
        props.border.classes(),
        props.padding.classes(),
        props.radius.classes(),
        hover_classes,
        interactive_classes,
        props.class
    );

    if let Some(handler) = &props.onclick {
        let handler = handler.clone();
        rsx! {
            div {
                class: full_class,
                onclick: move |_| handler.call(()),
                {props.children}
            }
        }
    } else {
        rsx! {
            div {
                class: full_class,
                {props.children}
            }
        }
    }
}

/// GlassPanel - Full-height glass container for sidebars and overlays
#[derive(Props, Clone, PartialEq)]
pub struct GlassPanelProps {
    /// Child elements
    pub children: Element,
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
}

#[component]
pub fn GlassPanel(props: GlassPanelProps) -> Element {
    rsx! {
        div {
            class: format!(
                "bg-black/20 backdrop-blur-2xl [-webkit-backdrop-filter:blur(40px)] border-r border-white/5 will-change-transform transform-gpu {}",
                props.class
            ),
            {props.children}
        }
    }
}

/// GlassButton - Glassmorphism button with gradient
#[derive(Props, Clone, PartialEq)]
pub struct GlassButtonProps {
    /// Child elements (button content)
    pub children: Element,
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    /// Click handler
    pub onclick: EventHandler<()>,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
    /// Loading state
    #[props(default = false)]
    pub loading: bool,
}

#[component]
pub fn GlassButton(props: GlassButtonProps) -> Element {
    let base_classes = "relative flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl font-medium text-white bg-gradient-to-br from-purple-600/80 to-violet-700/90 backdrop-blur-xl border border-purple-400/20 shadow-lg shadow-purple-500/20 transition-all duration-200 will-change-transform";

    let state_classes = if props.disabled || props.loading {
        "opacity-50 cursor-not-allowed"
    } else {
        "hover:from-purple-500/90 hover:to-violet-600/95 hover:shadow-xl hover:shadow-purple-500/30 active:scale-95 cursor-pointer"
    };

    rsx! {
        button {
            class: format!("{} {} {}", base_classes, state_classes, props.class),
            disabled: props.disabled || props.loading,
            onclick: move |_| {
                if !props.disabled && !props.loading {
                    props.onclick.call(());
                }
            },
            if props.loading {
                div {
                    class: "w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"
                }
            }
            {props.children}
        }
    }
}

/// GlassInput - Glassmorphism styled input field
#[derive(Props, Clone, PartialEq)]
pub struct GlassInputProps {
    /// Input value
    pub value: String,
    /// Change handler
    pub onchange: EventHandler<String>,
    /// Placeholder text
    #[props(default = String::new())]
    pub placeholder: String,
    /// Input type (text, email, password, etc.)
    #[props(default = "text".to_string())]
    pub input_type: String,
    /// Additional CSS classes
    #[props(default = String::new())]
    pub class: String,
    /// Disabled state
    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn GlassInput(props: GlassInputProps) -> Element {
    rsx! {
        input {
            class: format!(
                "w-full px-4 py-3 rounded-xl bg-white/5 backdrop-blur-xl border border-white/10 text-white placeholder-white/40 focus:outline-none focus:border-purple-500/50 focus:ring-2 focus:ring-purple-500/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed {}",
                props.class
            ),
            r#type: "{props.input_type}",
            value: "{props.value}",
            placeholder: "{props.placeholder}",
            disabled: props.disabled,
            oninput: move |e| props.onchange.call(e.value()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neon_border_classes() {
        assert!(NeonBorder::Purple.classes().contains("purple"));
        assert!(NeonBorder::Cyan.classes().contains("cyan"));
        assert!(NeonBorder::Subtle.classes().contains("white"));
        assert!(
            NeonBorder::Ignite
                .classes()
                .contains("hover:border-cyan-400")
        );
        assert!(
            NeonBorder::Ignite
                .classes()
                .contains("active:border-cyan-400")
        );
        assert!(NeonBorder::Ignite.classes().contains("active:shadow"));
    }

    #[test]
    fn test_card_padding_responsive() {
        let classes = CardPadding::Standard.classes();
        assert!(classes.contains("p-3"));
        assert!(classes.contains("sm:p-4"));
    }
}
