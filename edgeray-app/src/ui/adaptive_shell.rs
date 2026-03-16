//! Adaptive Shell - Responsive Root Navigation System
//!
//! Implements a responsive navigation layout that switches between:
//! - Mobile (<1024px): BottomNav + Hamburger Drawer
//! - Desktop (≥1024px): Fixed Sidebar
//!
//! Uses CSS media queries and Tailwind's responsive prefixes for layout switching.

use crate::components::scanline_layer::ScanlineLayer;
use crate::components::sidebar::{MeshSafety, Sidebar};
use crate::components::theme;
use crate::components::ui::Icon;
use crate::components::{BottomNav, Page};
use dioxus::prelude::*;

/// Props for the AdaptiveShell component
#[derive(Props, Clone, PartialEq)]
pub struct AdaptiveShellProps {
    /// Current active page
    pub current_page: Signal<Page>,
    /// Navigation event handler
    pub on_navigate: EventHandler<Page>,
    /// Mesh network safety status
    pub mesh_safety: Signal<MeshSafety>,
    /// Child content to render in the main area
    pub children: Element,
    /// UI Mode (Simple/Pro)
    #[props(default)]
    pub ui_mode: Signal<UiMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiMode {
    #[default]
    Simple,
    Pro,
}

#[derive(Clone, Copy, PartialEq)]
enum SimplePage {
    Home,
}

/// RootShell - Adaptive navigation and layout container
///
/// Provides responsive navigation that automatically switches between:
/// - Mobile: Floating bottom nav + slide-in hamburger drawer
/// - Desktop: Fixed sidebar navigation
/// - Dual-UX: Simple (Magic Button) vs Pro (Full Dashboard)
#[component]
pub fn AdaptiveShell(mut props: AdaptiveShellProps) -> Element {
    let mut drawer_open = use_signal(|| false);
    // In Simple Mode, we only have one effective page, but we keep props.current_page for API compatibility

    // Handler that closes drawer and navigates
    let on_nav = move |page: Page| {
        drawer_open.set(false);
        props.on_navigate.call(page);
    };

    if *props.ui_mode.read() == UiMode::Simple {
        // --- SIMPLE MODE LAYOUT ---
        return rsx! {
             div {
                class: "relative min-h-screen w-full text-white font-sans select-none overflow-hidden flex flex-col",
                style: "{theme::obsidian_bg_style()} {theme::safe_area_style()}",

                 // Obsidian Background
                 div { class: "fixed inset-0 z-0 bg-gradient-to-b from-transparent to-[#1a0b2e]/50" }
                 ScanlineLayer {}

                 // Main Content (Centered)
                 main { class: "flex-1 flex flex-col items-center justify-center relative z-10 p-6",
                    // Magic Connect Button Area is rendered by the child (Dashboard in Simple Mode)
                    {props.children}
                 }

                 // Footer / Toggle
                 div { class: "absolute bottom-8 w-full flex justify-center z-20",
                     button {
                         class: "flex items-center gap-2 px-6 py-3 rounded-full bg-white/5 hover:bg-white/10 border border-white/10 backdrop-blur-md transition-all text-sm font-medium text-gray-300",
                         onclick: move |_| props.ui_mode.set(UiMode::Pro),
                         Icon { name: "tune", class: "text-lg" }
                         "Advanced Mode"
                     }
                 }
            }
        };
    }

    // --- PRO MODE LAYOUT ---

    let purple_blob_style = format!(
        "top: -10%; left: -10%; width: 70vw; height: 70vw; max-width: 1000px; filter: blur(120px); opacity: 0.15; background: radial-gradient(circle, {} 0%, transparent 70%);",
        theme::CYBER_PURPLE
    );
    let cyan_blob_style = format!(
        "bottom: -15%; right: -10%; width: 80vw; height: 80vw; max-width: 1200px; filter: blur(150px); opacity: 0.12; background: radial-gradient(circle, {} 0%, transparent 70%);",
        theme::ELECTRIC_CYAN
    );
    let obsidian_style = format!(
        "{} {}",
        theme::obsidian_bg_style(),
        theme::safe_area_style()
    );

    rsx! {
        // Root container — Obsidian background with safe-area insets
        div {
            class: "relative min-h-screen w-full text-white font-sans select-none overflow-hidden",
            style: "{obsidian_style}",

            // ============================================================
            // OBSIDIAN ATMOSPHERIC BACKGROUND
            // Deep Night Void with Cyber Purple/Cyan ambient blobs
            // ============================================================
            div {
                class: "fixed inset-0 z-0 overflow-hidden pointer-events-none",
                // Primary blob - Cyber Purple
                div {
                    class: "absolute rounded-full will-change-transform animate-blob",
                    style: "{purple_blob_style}",
                }
                // Secondary blob - Electric Cyan
                div {
                    class: "absolute rounded-full will-change-transform animate-blob-2",
                    style: "{cyan_blob_style}",
                }
            }

            // Technical grid + CRT scanlines
            ScanlineLayer {}

            // ============================================================
            // DESKTOP SIDEBAR (≥1024px)
            // ============================================================
            nav {
                class: "hidden lg:flex fixed left-0 top-0 h-full flex-col z-50 glass-panel border-r border-white/5 transition-all duration-500 ease-out w-20 hover:w-72 group",

                Sidebar {
                    current_page: props.current_page,
                    on_navigate: on_nav,
                    mesh_safety: props.mesh_safety,
                    on_toggle_mode: move |_| props.ui_mode.set(UiMode::Simple),
                }
            }

            // ============================================================
            // MOBILE DRAWER OVERLAY (<1024px)
            // ============================================================
            div {
                class: format!(
                    "fixed inset-0 z-[100] bg-black/60 backdrop-blur-md transition-opacity duration-300 lg:hidden {}",
                    if *drawer_open.read() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" }
                ),
                onclick: move |_| drawer_open.set(false),

                div {
                    class: format!(
                        "absolute top-0 left-0 bottom-0 w-[300px] max-w-[85vw] glass-ultra border-r border-white/10 transform transition-transform duration-500 cubic-bezier(0.16, 1, 0.3, 1) {}",
                        if *drawer_open.read() { "translate-x-0" } else { "-translate-x-full" }
                    ),
                    onclick: move |e| e.stop_propagation(),

                    div {
                        class: "h-full pt-safe",
                        Sidebar {
                            current_page: props.current_page,
                            on_navigate: on_nav,
                            mesh_safety: props.mesh_safety,
                            on_toggle_mode: move |_| props.ui_mode.set(UiMode::Simple),
                            is_drawer: true,
                        }
                    }
                }
            }

            // ============================================================
            // MAIN CONTENT AREA
            // ============================================================
            main {
                class: "flex-1 min-h-screen w-full relative z-10 transition-all duration-300 lg:pl-20 sm:pb-safe",

                // Mobile Header
                header {
                    class: "lg:hidden flex items-center justify-between px-6 py-4 bg-void/60 backdrop-blur-3xl border-b border-white/5 sticky top-0 z-40 pt-safe",

                    button {
                        class: "flex items-center justify-center size-12 rounded-2xl glass-button text-gray-400 hover:text-primary transition-all active:scale-95",
                        onclick: move |_| drawer_open.set(true),
                        Icon { name: "menu", class: "text-2xl" }
                    }

                    h1 {
                        class: "font-bold text-xl tracking-tight text-gradient-neon filter drop-shadow-[0_0_8px_rgba(0,240,255,0.3)]",
                        "{props.current_page.read().label()}"
                    }

                    div { class: "w-12 h-12 rounded-2xl glass border border-white/10 flex items-center justify-center",
                        Icon { name: "account_circle", class: "text-2xl text-white/20" }
                    }
                }

                // Page Content
                div {
                    class: "w-full mx-auto px-4 sm:px-6 lg:px-8 py-6 lg:py-10 pb-32 lg:pb-10",
                    {props.children}
                }
            }

            // ============================================================
            // MOBILE BOTTOM NAV (<1024px)
            // ============================================================
            div {
                class: "lg:hidden fixed bottom-0 left-0 right-0 z-50 px-4 pb-safe",
                BottomNav {
                    active_screen: *props.current_page.read(),
                    on_navigate: on_nav,
                }
            }
        }
    }
}
