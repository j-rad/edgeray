use dioxus::prelude::*;

#[component]
pub fn Card(
    title: Option<String>,
    children: Element,
    extra: Option<Element>,
    actions: Option<Element>,
    class: Option<String>,
    hoverable: Option<bool>,
) -> Element {
    let hover_class = if hoverable.unwrap_or(false) {
        "cursor-pointer transition-shadow hover:shadow-lg"
    } else {
        ""
    };
    let extra_class = class.unwrap_or_default();

    rsx! {
        div {
            class: "bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden {hover_class} {extra_class}",
            if let Some(t) = title {
                div {
                    class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center",
                    span { class: "font-semibold text-lg", "{t}" }
                    if let Some(e) = extra {
                        div { class: "flex items-center gap-2", {e} }
                    }
                }
            } else if let Some(e) = extra {
                  div {
                    class: "px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-end items-center",
                    div { class: "flex items-center gap-2", {e} }
                }
            }

            div {
                class: "p-6",
                {children}
            }

            if let Some(a) = actions {
                div {
                     class: "px-6 py-4 bg-gray-50 dark:bg-gray-700/50 border-t border-gray-200 dark:border-gray-700",
                     {a}
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertType {
    fn classes(&self) -> &'static str {
        match self {
            AlertType::Info => {
                "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/20 dark:text-blue-300 dark:border-blue-800"
            }
            AlertType::Success => {
                "bg-green-50 text-green-700 border-green-200 dark:bg-green-900/20 dark:text-green-300 dark:border-green-800"
            }
            AlertType::Warning => {
                "bg-yellow-50 text-yellow-700 border-yellow-200 dark:bg-yellow-900/20 dark:text-yellow-300 dark:border-yellow-800"
            }
            AlertType::Error => {
                "bg-red-50 text-red-700 border-red-200 dark:bg-red-900/20 dark:text-red-300 dark:border-red-800"
            }
        }
    }
}

#[component]
pub fn Alert(
    r#type: Option<AlertType>,
    message: String,
    description: Option<String>,
    closable: Option<bool>,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let alert_type = r#type.unwrap_or(AlertType::Info);
    let mut visible = use_signal(|| true);

    if !(*visible.read()) {
        return rsx! {};
    }

    rsx! {
        div {
            class: "rounded-md p-4 mb-4 border flex items-start {alert_type.classes()}",
            div {
                class: "flex-1",
                div { class: "font-medium", "{message}" }
                if let Some(desc) = description {
                    div { class: "mt-1 text-sm opacity-90", "{desc}" }
                }
                {children}
            }
            if closable.unwrap_or(false) {
                button {
                    class: "ml-4 inline-flex flex-shrink-0 cursor-pointer opacity-60 hover:opacity-100",
                    onclick: move |_| {
                        visible.set(false);
                        if let Some(h) = onclose {
                            h.call(());
                        }
                    },
                    "×"
                }
            }
        }
    }
}

#[component]
pub fn Input(
    value: Signal<String>,
    label: Option<String>,
    placeholder: Option<String>,
    r#type: Option<String>,
    class: Option<String>,
) -> Element {
    let input_type = r#type.unwrap_or("text".to_string());
    let extra_class = class.unwrap_or_default();
    let ph = placeholder.unwrap_or_default();

    rsx! {
        div {
            class: "form-group mb-4",
            if let Some(l) = label {
                label { class: "block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300", "{l}" }
            }
            input {
                r#type: "{input_type}",
                class: "w-full px-3 py-3 min-h-[44px] bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-primary-500 text-sm transition-all {extra_class}",
                placeholder: "{ph}",
                value: "{value}",
                oninput: move |evt| value.set(evt.value())
            }
        }
    }
}

#[component]
pub fn Button(
    onclick: EventHandler<MouseEvent>,
    children: Element,
    class: Option<String>,
    disabled: Option<bool>,
    variant: Option<String>, // primary, default, danger
) -> Element {
    let variant_class = match variant.as_deref().unwrap_or("default") {
        "primary" => "bg-blue-600 text-white hover:bg-blue-700 disabled:bg-blue-400",
        "danger" => "bg-red-600 text-white hover:bg-red-700 disabled:bg-red-400",
        _ => {
            "bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-600"
        }
    };
    let extra_class = class.unwrap_or_default();
    let is_disabled = disabled.unwrap_or(false);

    rsx! {
        button {
            class: "inline-flex items-center justify-center px-4 py-3 min-h-[44px] text-sm font-medium rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer gap-2 transition-all {variant_class} {extra_class}",
            onclick: move |evt| if !is_disabled { onclick.call(evt) },
            disabled: is_disabled,
            {children}
        }
    }
}

#[component]
pub fn Switch(checked: Signal<bool>, label: Option<String>) -> Element {
    let bg_class = if *checked.read() {
        "bg-blue-600 dark:bg-blue-600"
    } else {
        "bg-gray-200 dark:bg-gray-700"
    };
    let dot_class = if *checked.read() {
        "translate-x-4"
    } else {
        "translate-x-0"
    };

    rsx! {
        label {
            class: "flex items-center cursor-pointer py-3 min-h-[44px]",
            div {
                class: "relative",
                input {
                    r#type: "checkbox",
                    class: "sr-only",
                    checked: "{checked}",
                    onchange: move |evt| checked.set(evt.value() == "true"),
                    onclick: move |_| {
                        let current = *checked.read();
                        checked.set(!current);
                    }
                }
                div {
                    class: "w-10 h-6 rounded-full shadow-inner transition-colors {bg_class}"
                }
                div {
                    class: "dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full shadow transition-transform duration-200 {dot_class}"
                }
            }
            if let Some(l) = label {
                div { class: "ml-3 text-sm font-medium text-gray-700 dark:text-gray-300 select-none", "{l}" }
            }
        }
    }
}
