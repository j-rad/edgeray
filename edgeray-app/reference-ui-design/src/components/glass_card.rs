use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct GlassCardProps {
    #[props(default)]
    pub class: String,
    #[props(default = "none".to_string())]
    pub glow: String,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn GlassCard(props: GlassCardProps) -> Element {
    let base_classes = "glass-panel rounded-2xl p-3 sm:p-5 transition-all duration-300 ease-out relative group overflow-hidden";

    let interactive_classes = if props.onclick.is_some() {
        "cursor-pointer active:scale-[0.98] glass-panel-hover"
    } else {
        ""
    };

    let glow_style = match props.glow.as_str() {
        "cyan" => "shadow-[0_0_30px_-5px_rgba(34,211,238,0.15)] border-l-primary/30",
        "purple" => "shadow-[0_0_30px_-5px_rgba(188,0,255,0.15)] border-l-purple/30",
        "emerald" => "shadow-[0_0_30px_-5px_rgba(0,255,163,0.15)] border-l-emerald/30",
        _ => "",
    };

    let combined_class = format!(
        "{} {} {} {}",
        base_classes, interactive_classes, glow_style, props.class
    );

    rsx! {
        div {
            class: "{combined_class}",
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },

            // Specular Highlight Gradient
            div {
                class: "absolute top-0 left-0 w-full h-full bg-gradient-to-br from-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500 pointer-events-none"
            }

            // Content
            div {
                class: "relative z-10",
                {props.children}
            }
        }
    }
}
