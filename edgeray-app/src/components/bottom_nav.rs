//! Bottom Navigation Component
//!
//! A floating pill-style bottom navigation bar matching the v2ray-glass design.

use super::Page;
use super::ui::Icon;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BottomNavProps {
    pub active_screen: Page,
    pub on_navigate: EventHandler<Page>,
}

#[component]
pub fn BottomNav(props: BottomNavProps) -> Element {
    let nav_items = [
        Page::Dashboard,
        Page::Configs,
        Page::SubscriptionGroups,
        Page::Settings,
    ];

    rsx! {
        nav {
            class: "fixed bottom-6 left-4 right-4 z-50 lg:hidden mb-safe transition-all duration-300",
            style: "padding-bottom: env(safe-area-inset-bottom);",
            // Floating Glass Pill
            div {
                class: "absolute inset-0 glass-panel rounded-2xl",
            }
            div {
                class: "relative flex justify-around items-center h-20 px-4 z-10 mx-auto max-w-md",
                for item in nav_items {
                    {nav_item(item, props.active_screen == item, props.on_navigate.clone())}
                }
            }
        }
    }
}

fn nav_item(screen: Page, is_active: bool, on_navigate: EventHandler<Page>) -> Element {
    let icon_class = if is_active {
        "text-primary text-shadow-glow"
    } else {
        "text-slate-400 dark:text-gray-500 group-hover:text-slate-600 dark:group-hover:text-gray-300"
    };

    rsx! {
        button {
            class: "relative flex flex-col items-center justify-center w-12 h-12 group transition-all duration-200 bg-transparent border-none cursor-pointer",
            onclick: move |_| on_navigate.call(screen),
            // Active indicator background
            if is_active {
                div {
                    class: "absolute inset-0 bg-primary/10 rounded-xl border border-primary/20",
                }
            }
            div {
                class: "relative z-10 transition-colors duration-200 {icon_class}",
                Icon { name: screen.icon().to_string(), class: "text-[24px]".to_string() }
            }
        }
    }
}
