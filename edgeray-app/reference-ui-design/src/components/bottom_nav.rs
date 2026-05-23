use crate::types::NavigationTab;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct BottomNavProps {
    pub active_tab: NavigationTab,
    pub on_tab_change: EventHandler<NavigationTab>,
    pub on_open_menu: EventHandler,
}

#[component]
pub fn BottomNav(props: BottomNavProps) -> Element {
    let nav_items = vec![
        (NavigationTab::Core, "Dashboard"),
        (NavigationTab::Nodes, "Nodes"),
        (NavigationTab::Mesh, "Shield"),
    ];

    rsx! {
        nav {
            class: "fixed bottom-0 left-0 right-0 z-50 lg:hidden animate-slide-up",

            div {
                class: "glass-panel border-t border-white/10 flex items-start justify-around px-2 pt-2 pb-safe min-h-[80px]",

                for (tab, label) in nav_items {
                    {
                        let is_active = props.active_tab == tab;
                        let tab_clone = tab;
                        let btn_class = format!("flex flex-col items-center gap-1 w-16 h-14 justify-center transition-all duration-300 active:scale-95 {}", if is_active { "text-primary" } else { "text-gray-400" });
                        let icon_wrap_class = format!("relative p-2 rounded-xl transition-all {}", if is_active { "bg-primary/10 shadow-[0_0_15px_rgba(34,211,238,0.2)]" } else { "" });

                        rsx! {
                            button {
                                key: "{tab:?}",
                                class: "{btn_class}",
                                onclick: move |_| props.on_tab_change.call(tab_clone),

                                div {
                                    class: "{icon_wrap_class}",
                                    {render_icon(tab, is_active)}
                                }
                                span {
                                    class: "text-[9px] font-bold tracking-wide",
                                    "{label}"
                                }
                            }
                        }
                    }
                }

                button {
                    class: "flex flex-col items-center gap-1 w-16 h-14 justify-center text-gray-400 active:scale-95",
                    onclick: move |_| props.on_open_menu.call(()),

                    div {
                        class: "relative p-2",
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path { d: "M3 12h18M3 6h18M3 18h18" }
                        }
                    }
                    span {
                        class: "text-[9px] font-bold tracking-wide",
                        "More"
                    }
                }
            }
        }
    }
}

fn render_icon(tab: NavigationTab, is_active: bool) -> Element {
    let stroke_width = if is_active { "2.5" } else { "2" };

    match tab {
        NavigationTab::Core => rsx! {
            svg {
                class: "w-6 h-6",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "{stroke_width}",
                view_box: "0 0 24 24",
                path { d: "M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z" }
            }
        },
        NavigationTab::Nodes => rsx! {
            svg {
                class: "w-6 h-6",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "{stroke_width}",
                view_box: "0 0 24 24",
                path { d: "M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v7z" }
            }
        },
        NavigationTab::Mesh => rsx! {
            svg {
                class: "w-6 h-6",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "{stroke_width}",
                view_box: "0 0 24 24",
                path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
            }
        },
        _ => rsx! { div {} },
    }
}
