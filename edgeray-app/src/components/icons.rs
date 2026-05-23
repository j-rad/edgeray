use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(default = 24)]
    pub size: u32,
    #[props(default = "currentColor".to_string())]
    pub color: String,
    #[props(default = "none".to_string())]
    pub fill: String,
    #[props(default = 2.0)]
    pub stroke_width: f32,
    pub class: Option<String>,
}

#[component]
pub fn ShieldCheck(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            path { d: "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z" }
            path { d: "m9 12 2 2 4-4" }
        }
    }
}

#[component]
pub fn Globe(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" }
            path { d: "M2 12h20" }
        }
    }
}

#[component]
pub fn Settings(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
            circle { cx: "12", cy: "12", r: "3" }
        }
    }
}

#[component]
pub fn Power(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            path { d: "M18.36 6.64a9 9 0 1 1-12.73 0" }
            line { x1: "12", y1: "2", x2: "12", y2: "12" }
        }
    }
}

#[component]
pub fn Activity(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            path { d: "M22 12h-4l-3 9L9 3l-3 9H2" }
        }
    }
}

#[component]
pub fn Cpu(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            rect { width: "16", height: "16", x: "4", y: "4", rx: "2" }
            path { d: "M9 9h6v6H9z" }
            path { d: "M15 2v2" }
            path { d: "M15 20v2" }
            path { d: "M9 2v2" }
            path { d: "M9 20v2" }
            path { d: "M20 15h2" }
            path { d: "M2 15h2" }
            path { d: "M20 9h2" }
            path { d: "M2 9h2" }
        }
    }
}

#[component]
pub fn HardDrive(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            line { x1: "22", y1: "12", x2: "2", y2: "12" }
            path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" }
            line { x1: "6", y1: "16", x2: "6.01", y2: "16" }
            line { x1: "10", y1: "16", x2: "10.01", y2: "16" }
        }
    }
}

#[component]
pub fn Wifi(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "{props.fill}",
            stroke: "{props.color}",
            stroke_width: "{props.stroke_width}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: props.class,
            path { d: "M5 13a10 10 0 0 1 14 0" }
            path { d: "M8.5 16.5a5 5 0 0 1 7 0" }
            path { d: "M2 8.82a15 15 0 0 1 20 0" }
            line { x1: "12", y1: "20", x2: "12.01", y2: "20" }
        }
    }
}
