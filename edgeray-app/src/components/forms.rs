use crate::components::ui::{Icon, glass};
use dioxus::prelude::*;

// ============================================================================
// FORM COMPONENTS
// ============================================================================

#[derive(Props, Clone, PartialEq)]
pub struct TextInputProps {
    pub label: String,
    pub value: Signal<String>,
    #[props(default = "text".to_string())]
    pub input_type: String,
    #[props(default = String::new())]
    pub placeholder: String,
    #[props(default = false)]
    pub required: bool,
    #[props(default = None)]
    pub help: Option<String>,
    #[props(default = String::new())]
    pub class: String,
}

#[component]
pub fn TextInput(mut props: TextInputProps) -> Element {
    rsx! {
        div {
            class: format!("flex flex-col gap-1.5 {}", props.class),
            label {
                class: "text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 ml-1",
                "{props.label}"
                if props.required {
                    span { class: "text-red-400 ml-0.5", "*" }
                }
            }
            input {
                class: format!("w-full px-4 py-2.5 rounded-xl text-sm transition-all outline-none text-slate-800 dark:text-slate-200 placeholder:text-slate-400 dark:placeholder:text-slate-600 {}", glass::INPUT),
                r#type: "{props.input_type}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                oninput: move |evt| props.value.set(evt.value()),
            }
            if let Some(help) = &props.help {
                p { class: "text-xs text-slate-400 dark:text-slate-500 ml-1", "{help}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct NumberInputProps {
    pub label: String,
    pub value: Signal<i64>,
    #[props(default = None)]
    pub min: Option<i64>,
    #[props(default = None)]
    pub max: Option<i64>,
    #[props(default = false)]
    pub required: bool,
    #[props(default = None)]
    pub help: Option<String>,
    #[props(default = String::new())]
    pub class: String,
}

#[component]
pub fn NumberInput(mut props: NumberInputProps) -> Element {
    rsx! {
        div {
            class: format!("flex flex-col gap-1.5 {}", props.class),
            label {
                class: "text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 ml-1",
                "{props.label}"
                if props.required {
                    span { class: "text-red-400 ml-0.5", "*" }
                }
            }
            input {
                class: format!("w-full px-4 py-2.5 rounded-xl text-sm transition-all outline-none text-slate-800 dark:text-slate-200 placeholder:text-slate-400 dark:placeholder:text-slate-600 {}", glass::INPUT),
                r#type: "number",
                min: props.min.map(|v| v.to_string()),
                max: props.max.map(|v| v.to_string()),
                value: "{props.value}",
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<i64>() {
                        props.value.set(val);
                    }
                },
            }
            if let Some(help) = &props.help {
                p { class: "text-xs text-slate-400 dark:text-slate-500 ml-1", "{help}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps {
    pub label: String,
    pub value: Signal<String>,
    pub options: Vec<(String, String)>, // (value, label)
    #[props(default = None)]
    pub help: Option<String>,
    #[props(default = String::new())]
    pub class: String,
}

#[component]
pub fn Select(mut props: SelectProps) -> Element {
    rsx! {
        div {
            class: format!("flex flex-col gap-1.5 {}", props.class),
            label {
                class: "text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400 ml-1",
                "{props.label}"
            }
            div {
                class: "relative",
                select {
                    class: format!("w-full pl-4 pr-10 py-2.5 rounded-xl text-sm appearance-none transition-all outline-none text-slate-800 dark:text-slate-200 bg-transparent {}", glass::INPUT),
                    value: "{props.value}",
                    onchange: move |evt| props.value.set(evt.value()),
                    for (val, label) in props.options {
                        option { value: "{val}", "{label}" }
                    }
                }
                div {
                    class: "absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400",
                    Icon { name: "arrow_drop_down".to_string(), class: "text-xl".to_string() }
                }
            }
            if let Some(help) = &props.help {
                p { class: "text-xs text-slate-400 dark:text-slate-500 ml-1", "{help}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    pub label: String,
    pub value: Signal<bool>,
    #[props(default = None)]
    pub description: Option<String>,
}

#[component]
pub fn Toggle(mut props: ToggleProps) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between py-2",
            div {
                class: "flex flex-col",
                span { class: "text-sm font-medium text-slate-700 dark:text-slate-200", "{props.label}" }
                if let Some(desc) = &props.description {
                    span { class: "text-xs text-slate-400 dark:text-slate-500", "{desc}" }
                }
            }
            button {
                class: format!(
                    "relative w-11 h-6 rounded-full transition-colors duration-200 ease-in-out focus:outline-none {}",
                    if *props.value.read() { "bg-primary" } else { "bg-slate-200 dark:bg-slate-700" }
                ),
                onclick: move |_| {
                    let new_val = !*props.value.read();
                    props.value.set(new_val);
                },
                span {
                    class: format!(
                        "absolute left-0.5 top-0.5 w-5 h-5 rounded-full bg-white shadow transform transition-transform duration-200 ease-in-out {}",
                        if *props.value.read() { "translate-x-5" } else { "translate-x-0" }
                    )
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FormCardProps {
    #[props(default = None)]
    pub title: Option<String>,
    #[props(default = None)]
    pub description: Option<String>,
    pub children: Element,
}

#[component]
pub fn FormCard(props: FormCardProps) -> Element {
    rsx! {
        div {
            class: format!("p-5 rounded-2xl {}", glass::PANEL),
            if let Some(title) = &props.title {
                h3 { class: "text-lg font-bold text-slate-800 dark:text-slate-100 mb-1", "{title}" }
            }
            if let Some(desc) = &props.description {
                p { class: "text-sm text-slate-500 dark:text-slate-400 mb-6", "{desc}" }
            } else if props.title.is_some() {
                div { class: "mb-6" }
            }
            div {
                class: "space-y-4",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlertProps {
    #[props(default = AlertVariant::Info)]
    pub variant: AlertVariant,
    #[props(default = None)]
    pub title: Option<String>,
    pub message: String,
}

#[derive(Clone, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    let (bg_color, text_color, border_color, icon) = match props.variant {
        AlertVariant::Info => (
            "bg-blue-500/10",
            "text-blue-600 dark:text-blue-400",
            "border-blue-500/20",
            "info",
        ),
        AlertVariant::Success => (
            "bg-emerald-500/10",
            "text-emerald-600 dark:text-emerald-400",
            "border-emerald-500/20",
            "check_circle",
        ),
        AlertVariant::Warning => (
            "bg-amber-500/10",
            "text-amber-600 dark:text-amber-400",
            "border-amber-500/20",
            "warning",
        ),
        AlertVariant::Error => (
            "bg-red-500/10",
            "text-red-600 dark:text-red-400",
            "border-red-500/20",
            "error",
        ),
    };

    rsx! {
        div {
            class: format!("flex gap-3 p-4 rounded-xl border {} {}", bg_color, border_color),
            div {
                Icon { name: icon.to_string(), class: format!("text-xl {}", text_color) }
            }
            div {
                if let Some(title) = &props.title {
                    h4 { class: format!("text-sm font-bold mb-0.5 {}", text_color), "{title}" }
                }
                p { class: format!("text-sm opacity-90 {}", text_color), "{props.message}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[props(default = None)]
    pub icon: Option<String>,
    pub children: Element,
    pub on_click: EventHandler<()>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default = false)]
    pub loading: bool,
    #[props(default = "button".to_string())]
    pub type_: String,
}

#[derive(Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Destructive,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_class = "px-4 py-2 rounded-xl font-medium transition-all flex items-center justify-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed active:scale-[0.98]";

    let variant_class = match props.variant {
        ButtonVariant::Primary => {
            "bg-primary text-white shadow-lg shadow-primary/30 hover:bg-primary/90"
        }
        ButtonVariant::Secondary => {
            "bg-white/10 text-slate-200 border border-white/10 hover:bg-white/20"
        }
        ButtonVariant::Ghost => "text-slate-400 hover:text-slate-200 hover:bg-white/5",
        ButtonVariant::Destructive => {
            "bg-red-500/10 text-red-400 border border-red-500/20 hover:bg-red-500/20"
        }
    };

    rsx! {
        button {
            class: format!("{} {}", base_class, variant_class),
            disabled: props.disabled || props.loading,
            r#type: "{props.type_}",
            onclick: move |_| props.on_click.call(()),
            if props.loading {
                div { class: "w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" }
            } else if let Some(icon) = &props.icon {
                Icon { name: icon.clone(), class: "text-lg".to_string() }
            }
            {props.children}
        }
    }
}
