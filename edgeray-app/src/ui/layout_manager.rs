use crate::components::sidebar::{MeshSafety, Sidebar};
use crate::components::{BottomNav, Page};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LayoutManagerProps {
    pub current_page: Signal<Page>,
    pub on_navigate: EventHandler<Page>,
    pub mesh_safety: Signal<MeshSafety>,
    pub children: Element,
}

#[component]
pub fn LayoutManager(props: LayoutManagerProps) -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    let on_nav = move |page: Page| {
        mobile_menu_open.set(false);
        props.on_navigate.call(page);
    };

    rsx! {
        // Root container - relative for positioning context
        div {
            class: "relative min-h-screen bg-gray-50 dark:bg-void text-gray-900 dark:text-white font-sans select-none",

            // Animated Ambient Background (matching reference design)
            div {
                class: "fixed inset-0 z-0 overflow-hidden pointer-events-none",
                div {
                    class: "absolute top-[-10%] left-[-10%] w-[60vw] h-[60vw] rounded-full bg-purple-600/20 blur-[120px] animate-pulse"
                }
                div {
                    class: "absolute bottom-[-10%] right-[-10%] w-[70vw] h-[70vw] rounded-full bg-blue-600/20 blur-[150px] animate-pulse"
                }
                div {
                    class: "absolute top-[40%] left-[30%] w-[40vw] h-[40vw] rounded-full bg-cyan-500/10 blur-[100px] animate-pulse"
                }
            }

            // Desktop Sidebar - Fixed position (hidden on mobile)
            nav {
                class: "hidden md:flex fixed left-0 top-0 h-full w-20 lg:w-72 flex-col z-50 border-r border-gray-200 dark:border-white/5 bg-white/80 dark:bg-black/10 backdrop-blur-3xl transition-all duration-300",
                Sidebar {
                    current_page: props.current_page,
                    on_navigate: on_nav,
                    mesh_safety: props.mesh_safety,
                }
            }

            // Mobile Drawer Overlay
            div {
                class: format!(
                    "fixed inset-0 z-50 bg-black/80 backdrop-blur-sm transition-opacity duration-300 md:hidden {}",
                    if *mobile_menu_open.read() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" }
                ),
                onclick: move |_| mobile_menu_open.set(false),

                // Drawer Panel
                div {
                    class: format!(
                        "absolute top-0 left-0 bottom-0 w-72 bg-[#0f1014] shadow-2xl transform transition-transform duration-300 ease-in-out {}",
                        if *mobile_menu_open.read() { "translate-x-0" } else { "-translate-x-full" }
                    ),
                    onclick: move |e| e.stop_propagation(),
                    Sidebar {
                        current_page: props.current_page,
                        on_navigate: on_nav,
                        mesh_safety: props.mesh_safety,
                        is_drawer: true,
                    }
                }
            }

            // Main Content Area - with padding for fixed sidebar
            main {
                class: "flex-1 min-h-screen overflow-y-auto overflow-x-hidden relative z-10 pb-24 md:pb-0 md:pl-20 lg:pl-72",

                // Mobile Header with hamburger
                div {
                    class: "md:hidden flex items-center justify-between pt-safe px-4 pb-4 bg-white/50 dark:bg-black/20 backdrop-blur-xl border-b border-gray-200 dark:border-white/5 sticky top-0 z-30",
                    button {
                        class: "p-2 rounded-xl text-gray-600 dark:text-gray-400 hover:bg-black/5 dark:hover:bg-white/10 hover:text-black dark:hover:text-white transition-colors",
                        onclick: move |_| mobile_menu_open.set(true),
                        crate::components::ui::Icon { name: "menu", class: "text-2xl" }
                    }
                    span { class: "font-bold text-lg tracking-tight", "{props.current_page.read().label()}" }
                    div { class: "w-10" } // Spacer for balance
                }

                // Page Content
                div {
                    class: "w-full animate-fade-in-up",
                    {props.children}
                }
            }

            // Mobile Bottom Nav - Fixed at bottom (hidden on desktop)
            BottomNav {
                active_screen: *props.current_page.read(),
                on_navigate: on_nav,
            }
        }
    }
}
