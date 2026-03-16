//! Mobile Gesture Components
//!
//! Cross-platform gesture support for iOS and Android including:
//! - Pull-to-refresh
//! - Swipe-to-delete
//! - Long-press context menus
//! - Material ripple effects
//! - Back button/gesture handling

use dioxus::prelude::*;

/// Pull-to-refresh wrapper component
#[derive(Props, Clone, PartialEq)]
pub struct PullToRefreshProps {
    /// Called when refresh is triggered
    pub on_refresh: EventHandler<()>,
    /// Whether refresh is currently in progress
    #[props(default = false)]
    pub refreshing: bool,
    /// Content to wrap
    pub children: Element,
    /// Threshold in pixels to trigger refresh
    #[props(default = 80.0)]
    pub threshold: f64,
}

#[component]
pub fn PullToRefresh(props: PullToRefreshProps) -> Element {
    let mut pull_distance = use_signal(|| 0.0_f64);
    let mut is_pulling = use_signal(|| false);
    let mut start_y = use_signal(|| 0.0_f64);

    let on_touch_start = move |e: Event<TouchData>| {
        if let Some(touch) = e.touches().first() {
            start_y.set(touch.client_coordinates().y);
            is_pulling.set(true);
        }
    };

    let threshold = props.threshold;
    let on_touch_move = move |e: Event<TouchData>| {
        if !*is_pulling.read() {
            return;
        }
        if let Some(touch) = e.touches().first() {
            let current_y = touch.client_coordinates().y;
            let delta = current_y - *start_y.read();
            if delta > 0.0 {
                // Apply resistance factor for natural feel
                let resistance = 0.5;
                let distance = (delta * resistance).min(threshold * 1.5);
                pull_distance.set(distance);
            }
        }
    };

    let on_refresh_handler = props.on_refresh.clone();
    let on_touch_end = move |_: Event<TouchData>| {
        is_pulling.set(false);
        let distance = *pull_distance.read();
        if distance >= threshold {
            on_refresh_handler.call(());
        }
        pull_distance.set(0.0);
    };

    let transform = if props.refreshing {
        format!("translateY({}px)", props.threshold)
    } else {
        format!("translateY({}px)", *pull_distance.read())
    };

    let indicator_opacity = (*pull_distance.read() / props.threshold).min(1.0);
    let indicator_scale = (0.5 + (*pull_distance.read() / props.threshold) * 0.5).min(1.0);

    rsx! {
        div {
            class: "relative overflow-hidden",
            ontouchstart: on_touch_start,
            ontouchmove: on_touch_move,
            ontouchend: on_touch_end,

            // Refresh indicator
            div {
                class: "absolute top-0 left-0 right-0 flex justify-center items-center h-16 -mt-16 transition-transform",
                style: "transform: translateY({pull_distance}px); opacity: {indicator_opacity};",
                div {
                    class: "w-8 h-8 rounded-full border-2 border-primary border-t-transparent",
                    style: "transform: scale({indicator_scale});",
                    class: if props.refreshing { "animate-spin" } else { "" }
                }
            }

            // Content
            div {
                class: "transition-transform duration-200",
                style: "transform: {transform};",
                {props.children}
            }
        }
    }
}

/// Swipeable card wrapper with delete action
#[derive(Props, Clone, PartialEq)]
pub struct SwipeableCardProps {
    /// Called when swipe-to-delete is triggered
    pub on_delete: EventHandler<()>,
    /// Called when swipe-to-edit is triggered (left swipe)
    #[props(default)]
    pub on_edit: Option<EventHandler<()>>,
    /// Content to wrap
    pub children: Element,
    /// Swipe threshold to trigger action
    #[props(default = 80.0)]
    pub threshold: f64,
    /// Enable delete action (right swipe)
    #[props(default = true)]
    pub enable_delete: bool,
    /// Enable edit action (left swipe)
    #[props(default = false)]
    pub enable_edit: bool,
}

#[component]
pub fn SwipeableCard(props: SwipeableCardProps) -> Element {
    let mut swipe_x = use_signal(|| 0.0_f64);
    let mut start_x = use_signal(|| 0.0_f64);
    let mut is_swiping = use_signal(|| false);

    let on_touch_start = move |e: Event<TouchData>| {
        if let Some(touch) = e.touches().first() {
            start_x.set(touch.client_coordinates().x);
            is_swiping.set(true);
        }
    };

    let threshold = props.threshold;
    let enable_delete = props.enable_delete;
    let enable_edit = props.enable_edit;

    let on_touch_move = move |e: Event<TouchData>| {
        if !*is_swiping.read() {
            return;
        }
        if let Some(touch) = e.touches().first() {
            let current_x = touch.client_coordinates().x;
            let mut delta = current_x - *start_x.read();

            // Limit swipe based on enabled actions
            if delta < 0.0 && !enable_edit {
                delta = 0.0;
            }
            if delta > 0.0 && !enable_delete {
                delta = 0.0;
            }

            // Apply resistance
            let max_swipe = threshold * 1.5;
            delta = delta.clamp(-max_swipe, max_swipe);
            swipe_x.set(delta);
        }
    };

    let on_delete_handler = props.on_delete.clone();
    let on_edit_handler = props.on_edit.clone();

    let on_touch_end = move |_: Event<TouchData>| {
        is_swiping.set(false);
        let x = *swipe_x.read();

        if x >= threshold {
            // Right swipe - delete
            on_delete_handler.call(());
        } else if x <= -threshold {
            // Left swipe - edit
            if let Some(handler) = &on_edit_handler {
                handler.call(());
            }
        }
        swipe_x.set(0.0);
    };

    let swipe_value = *swipe_x.read();
    let delete_opacity = (swipe_value / props.threshold).clamp(0.0, 1.0);
    let edit_opacity = (-swipe_value / props.threshold).clamp(0.0, 1.0);

    rsx! {
        div {
            class: "relative overflow-hidden rounded-2xl",

            // Delete background (right swipe)
            if props.enable_delete {
                div {
                    class: "absolute inset-y-0 left-0 flex items-center justify-start pl-6 bg-gradient-to-r from-red-500 to-red-600",
                    style: "width: {swipe_value.max(0.0)}px; opacity: {delete_opacity};",
                    span { class: "material-symbols-outlined text-white text-2xl", "delete" }
                }
            }

            // Edit background (left swipe)
            if props.enable_edit {
                div {
                    class: "absolute inset-y-0 right-0 flex items-center justify-end pr-6 bg-gradient-to-l from-blue-500 to-blue-600",
                    style: "width: {(-swipe_value).max(0.0)}px; opacity: {edit_opacity};",
                    span { class: "material-symbols-outlined text-white text-2xl", "edit" }
                }
            }

            // Card content
            div {
                class: "relative bg-white/5 backdrop-blur-xl transition-transform duration-200",
                style: "transform: translateX({swipe_value}px);",
                ontouchstart: on_touch_start,
                ontouchmove: on_touch_move,
                ontouchend: on_touch_end,
                {props.children}
            }
        }
    }
}

/// Long-press context menu
#[derive(Props, Clone, PartialEq)]
pub struct LongPressMenuProps {
    /// Menu items to display
    pub items: Vec<ContextMenuItem>,
    /// Content that triggers the menu
    pub children: Element,
    /// Long press duration in ms
    #[props(default = 500)]
    pub press_duration_ms: u64,
}

#[derive(Clone, PartialEq)]
pub struct ContextMenuItem {
    pub label: String,
    pub icon: String,
    pub on_click: EventHandler<()>,
    pub destructive: bool,
}

#[component]
pub fn LongPressMenu(props: LongPressMenuProps) -> Element {
    let mut show_menu = use_signal(|| false);
    let mut menu_position = use_signal(|| (0.0_f64, 0.0_f64));
    let mut press_timer = use_signal(|| None::<i32>);

    let press_duration = props.press_duration_ms;

    let on_touch_start = move |e: Event<TouchData>| {
        if let Some(touch) = e.touches().first() {
            let x = touch.client_coordinates().x;
            let y = touch.client_coordinates().y;
            menu_position.set((x, y));

            // Start timer for long press
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;
                use wasm_bindgen::prelude::*;

                let window = web_sys::window().unwrap();
                let closure = Closure::once(Box::new(move || {
                    show_menu.set(true);
                }) as Box<dyn FnOnce()>);

                let id = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        press_duration as i32,
                    )
                    .unwrap();
                closure.forget();
                press_timer.set(Some(id));
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                // For native, use spawn with delay
                let _ = press_duration; // Suppress unused warning
                spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    show_menu.set(true);
                });
            }
        }
    };

    let on_touch_end = move |_: Event<TouchData>| {
        // Cancel timer
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(id) = *press_timer.read() {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(id);
                }
            }
        }
        press_timer.set(None);
    };

    let on_touch_move = move |_: Event<TouchData>| {
        // Cancel on move
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(id) = *press_timer.read() {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(id);
                }
            }
        }
        press_timer.set(None);
    };

    let close_menu = move |_| {
        show_menu.set(false);
    };

    let (menu_x, menu_y) = *menu_position.read();

    rsx! {
        div {
            class: "relative",
            ontouchstart: on_touch_start,
            ontouchend: on_touch_end,
            ontouchmove: on_touch_move,
            {props.children}
        }

        // Context menu overlay
        if *show_menu.read() {
            div {
                class: "fixed inset-0 z-50 bg-black/50 backdrop-blur-sm",
                onclick: close_menu,

                // Menu popup
                div {
                    class: "absolute bg-gray-900/95 backdrop-blur-xl rounded-2xl border border-white/10 shadow-2xl overflow-hidden min-w-48",
                    style: "left: {menu_x}px; top: {menu_y}px; transform: translate(-50%, 10px);",
                    onclick: move |e| e.stop_propagation(),

                    for item in props.items.iter() {
                        {menu_item(item.clone(), show_menu)}
                    }
                }
            }
        }
    }
}

fn menu_item(item: ContextMenuItem, mut show_menu: Signal<bool>) -> Element {
    let text_class = if item.destructive {
        "text-red-400"
    } else {
        "text-white"
    };

    let handler = item.on_click.clone();

    rsx! {
        button {
            class: "w-full flex items-center gap-3 px-4 py-3 hover:bg-white/10 active:bg-white/20 transition-colors border-none bg-transparent cursor-pointer {text_class}",
            onclick: move |_| {
                handler.call(());
                show_menu.set(false);
            },
            span { class: "material-symbols-outlined text-lg", "{item.icon}" }
            span { class: "text-sm font-medium", "{item.label}" }
        }
    }
}

/// Material ripple effect wrapper
#[derive(Props, Clone, PartialEq)]
pub struct RippleProps {
    /// Content to wrap
    pub children: Element,
    /// Ripple color
    #[props(default = "rgba(255, 255, 255, 0.3)".to_string())]
    pub color: String,
    /// Additional classes
    #[props(default = String::new())]
    pub class: String,
    /// Click handler
    #[props(default)]
    pub onclick: Option<EventHandler<()>>,
}

#[component]
pub fn Ripple(props: RippleProps) -> Element {
    let mut ripples = use_signal(|| Vec::<(f64, f64, u64)>::new());
    let mut ripple_id = use_signal(|| 0_u64);

    let color = props.color.clone();
    let onclick_handler = props.onclick.clone();

    let on_click = move |e: Event<MouseData>| {
        let coords = e.element_coordinates();
        let x = coords.x;
        let y = coords.y;
        let id = *ripple_id.read();
        ripple_id.set(id + 1);

        ripples.write().push((x, y, id));

        // Remove ripple after animation
        let id_to_remove = id;
        spawn(async move {
            #[cfg(not(target_arch = "wasm32"))]
            tokio::time::sleep(std::time::Duration::from_millis(600)).await;

            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::TimeoutFuture::new(600).await;

            ripples.write().retain(|(_, _, rid)| *rid != id_to_remove);
        });

        if let Some(handler) = &onclick_handler {
            handler.call(());
        }
    };

    rsx! {
        div {
            class: "relative overflow-hidden {props.class}",
            onclick: on_click,

            {props.children}

            // Ripple effects
            for (x, y, id) in ripples.read().iter() {
                div {
                    key: "{id}",
                    class: "absolute rounded-full animate-ripple pointer-events-none",
                    style: "left: {x}px; top: {y}px; background: {color}; transform: translate(-50%, -50%);",
                }
            }
        }
    }
}

/// Back button/gesture handler hook
/// Returns a callback that should be called when back is pressed
#[allow(dead_code)]
// Guard struct to handle cleanup of event listener on drop
#[cfg(target_arch = "wasm32")]
struct BackListenerGuard {
    // Closure must be kept alive
    #[allow(dead_code)]
    closure: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::PopStateEvent)>,
}

#[cfg(target_arch = "wasm32")]
impl Drop for BackListenerGuard {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        if let Some(window) = web_sys::window() {
            let _ = window.remove_event_listener_with_callback(
                "popstate",
                self.closure.as_ref().unchecked_ref(),
            );
        }
    }
}

pub fn use_back_handler(on_back: impl Fn() + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::prelude::*;

        // Store the guard in a signal so it stays alive for the component lifecycle
        // and drops when the component unmounts. Signal contents do not need to be Clone.
        use_signal(move || {
            let window = web_sys::window().unwrap();

            // Create the callback closure
            let closure = Closure::wrap(Box::new(move |_: web_sys::PopStateEvent| {
                on_back();
            }) as Box<dyn FnMut(_)>);

            // Register the listener
            window
                .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
                .unwrap();

            // Push initial state to enable popstate events
            let history = window.history().unwrap();
            let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", None);

            // Return the guard which owns the closure
            BackListenerGuard { closure }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native platforms, the back handling is done via Tauri/platform APIs
        let _ = on_back; // Suppress unused warning
    }
}

/// Haptic feedback types
#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum HapticType {
    /// Light tap feedback
    Light,
    /// Medium impact feedback
    Medium,
    /// Heavy impact feedback  
    Heavy,
    /// Selection changed
    Selection,
    /// Success notification
    Success,
    /// Warning notification
    Warning,
    /// Error notification
    Error,
}

/// Trigger haptic feedback
#[allow(dead_code)]
pub fn trigger_haptic(haptic_type: HapticType) {
    #[cfg(target_arch = "wasm32")]
    {
        // Use Vibration API for web
        if let Some(window) = web_sys::window() {
            // Use Vibration API for web
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let _ = navigator.vibrate_with_duration(match haptic_type {
                    HapticType::Light => 10,
                    HapticType::Medium => 20,
                    HapticType::Heavy => 40,
                    HapticType::Selection => 5,
                    HapticType::Success => 30,
                    HapticType::Warning => 50,
                    HapticType::Error => 100,
                });
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native, this would call platform-specific haptic APIs via Tauri
        log::debug!("Haptic feedback: {:?}", haptic_type as u8);
    }
}
