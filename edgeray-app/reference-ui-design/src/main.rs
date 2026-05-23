mod components;
mod constants;
mod pages;
mod types;

use components::{BottomNav, Sidebar};
use constants::USER_AVATAR;
use dioxus::prelude::*;
use pages::*;
use types::NavigationTab;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut active_tab = use_signal(|| NavigationTab::Core);
    let is_vpn_active = use_signal(|| true);
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut is_desktop_sidebar_expanded = use_signal(|| false);

    let sidebar_margin = if is_desktop_sidebar_expanded() {
        "lg:pl-[260px]"
    } else {
        "lg:pl-[80px]"
    };
    let header_left = if is_desktop_sidebar_expanded() {
        "260px"
    } else {
        "80px"
    };
    let status_dot = if is_vpn_active() {
        "bg-emerald shadow-[0_0_8px_#00ffa3]"
    } else {
        "bg-gray-500"
    };
    let status_text = if is_vpn_active() {
        "Connected"
    } else {
        "Disconnected"
    };
    let breadcrumb = active_tab().label();

    rsx! {
        div {
            class: "flex min-h-dvh relative overflow-hidden bg-background text-white font-sans bg-grid-pattern",

            // Ambient Glows
            div {
                class: "fixed inset-0 z-0 pointer-events-none overflow-hidden",
                // Cyan Primary Blob
                div { class: "absolute top-[-10%] left-[-10%] w-[50vw] h-[50vw] bg-primary/20 blur-[120px] rounded-full animate-blob mix-blend-screen" }
                // Purple Secondary Blob
                div { class: "absolute bottom-[-10%] right-[-10%] w-[40vw] h-[40vw] bg-purple/20 blur-[120px] rounded-full animate-blob mix-blend-screen", style: "animation-delay: 2s;" }
                // Emerald Accent Blob
                div { class: "absolute top-[40%] left-[40%] w-[30vw] h-[30vw] bg-emerald/15 blur-[100px] rounded-full animate-blob mix-blend-screen", style: "animation-delay: 4s;" }
            }

            // Sidebar
            Sidebar {
                active_tab: active_tab(),
                on_tab_change: move |tab| {
                    active_tab.set(tab);
                    is_mobile_menu_open.set(false);
                },
                is_vpn_active: is_vpn_active(),
                is_open: is_mobile_menu_open(),
                on_close: move |_| is_mobile_menu_open.set(false),
                is_desktop_expanded: is_desktop_sidebar_expanded(),
                on_desktop_toggle: move |_| is_desktop_sidebar_expanded.set(!is_desktop_sidebar_expanded()),
            }

            // Header & Main Content Wrapper
            div {
                class: "flex-1 flex flex-col transition-all duration-300 relative z-10 {sidebar_margin}",

                // Header
                header {
                    class: "fixed top-0 right-0 z-40 min-h-[70px] h-auto pt-safe glass-panel border-b-0 flex items-center px-4 lg:px-6 transition-all duration-300 z-50",
                    style: "left: {header_left};",

                    div {
                        class: "w-full h-[70px] max-w-[1400px] mx-auto flex items-center justify-between",
                        div {
                            class: "flex items-center gap-3",
                            // Mobile Hamburger
                            button {
                                class: "lg:hidden w-10 h-10 flex items-center justify-center text-gray-400 hover:text-white active:scale-95 transition-transform",
                                onclick: move |_| is_mobile_menu_open.set(true),
                                svg { class: "w-6 h-6", fill: "none", stroke: "currentColor", stroke_width: "2", view_box: "0 0 24 24", path { d: "M3 12h18M3 6h18M3 18h18" } }
                            }

                            // Mobile Logo
                            div {
                                class: "lg:hidden",
                                h1 {
                                    class: "text-base font-bold tracking-tight",
                                    "EDGE"
                                    span { class: "text-primary", "RAY" }
                                }
                            }

                            // Desktop Breadcrumb
                            div {
                                class: "hidden lg:flex items-center gap-4",
                                span {
                                    class: "text-xs font-mono text-gray-400 uppercase tracking-widest text-shadow-sm",
                                    "SYSTEM / {breadcrumb}"
                                }
                                div { class: "h-4 w-px bg-white/20" }
                                div {
                                    class: "flex items-center gap-2 px-3 py-1 rounded-full bg-black/20 border border-white/5 backdrop-blur-sm",
                                    div { class: "w-1.5 h-1.5 rounded-full {status_dot}" }
                                    span { class: "text-[10px] text-gray-300 font-mono font-medium uppercase", "{status_text}" }
                                }
                            }
                        }

                        button {
                            class: "w-9 h-9 lg:w-10 lg:h-10 rounded-xl overflow-hidden border border-white/20 hover:border-primary/50 transition-all hover:shadow-[0_0_15px_rgba(34,211,238,0.2)]",
                            img { src: USER_AVATAR, alt: "Profile", class: "w-full h-full object-cover" }
                        }
                    }
                }

                // Main Scrollable Area
                main {
                    class: "flex-1 pt-[calc(70px+env(safe-area-inset-top))] pb-[calc(90px+env(safe-area-inset-bottom))] lg:pb-8 px-3 sm:px-6 lg:px-8",
                    div {
                        class: "max-w-[1400px] mx-auto h-full",
                        {render_content(active_tab())}
                    }
                }
            }

            // Mobile Navigation
            BottomNav {
                active_tab: active_tab(),
                on_tab_change: move |tab| active_tab.set(tab),
                on_open_menu: move |_| is_mobile_menu_open.set(true),
            }
        }

        // Scanlines Effect
        div { class: "scanlines" }
    }
}

fn render_content(tab: NavigationTab) -> Element {
    match tab {
        NavigationTab::Core => rsx! { CoreDashboard {} },
        NavigationTab::Nodes => rsx! { NodesPage {} },
        NavigationTab::Mesh => rsx! { ShieldPage {} },
        NavigationTab::Topology => rsx! { MeshMap {} },
        NavigationTab::Tracer => rsx! { TracerPage {} },
        NavigationTab::Setup => rsx! { SetupPage {} },
        NavigationTab::Routing => rsx! { RoutingRulesPage {} },
        NavigationTab::Settings => rsx! { SettingsPage {} },
        NavigationTab::Diagnostics => rsx! { DiagnosticsPage {} },
    }
}
