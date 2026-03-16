// MetricValue Component - Phase 1: Data Density
// Uses font-mono with drop-shadow for readability against complex backgrounds

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MetricValueProps {
    pub value: String,
    #[props(default = None)]
    pub unit: Option<String>,
    #[props(default = "text-white".to_string())]
    pub value_class: String,
    #[props(default = "text-slate-400".to_string())]
    pub unit_class: String,
}

#[component]
pub fn MetricValue(props: MetricValueProps) -> Element {
    rsx! {
        div {
            class: "flex items-baseline gap-1",
            span {
                class: "font-mono {props.value_class} drop-shadow-[0_0_8px_rgba(255,255,255,0.3)]",
                "{props.value}"
            }
            if let Some(unit) = &props.unit {
                span {
                    class: "font-mono text-[0.5em] {props.unit_class}",
                    "{unit}"
                }
            }
        }
    }
}
