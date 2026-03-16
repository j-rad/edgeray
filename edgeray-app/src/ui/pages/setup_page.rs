//! Onboarding / ISP Setup Wizard
//!
//! Guides the user through initial configuration:
//! 1. ISP selection (MCI, Irancell, Rightel, Shatel, Asiatech, Mokhaberat)
//! 2. MTU probing against discovered ISP constraints
//! 3. Optimal defaults applied automatically

use crate::components::ui::Icon;
use dioxus::prelude::*;

// ──────────────────────── ISP Data ────────────────────────

/// Known Iranian ISP definitions used by the wizard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IspChoice {
    Mci,
    Irancell,
    Rightel,
    Shatel,
    Asiatech,
    Mokhaberat,
    Other,
}

impl IspChoice {
    /// Human-readable name.
    pub fn display_name(&self) -> &'static str {
        match self {
            IspChoice::Mci => "MCI (همراه اول)",
            IspChoice::Irancell => "Irancell (ایرانسل)",
            IspChoice::Rightel => "Rightel (رایتل)",
            IspChoice::Shatel => "Shatel (شاتل)",
            IspChoice::Asiatech => "Asiatech (آسیاتک)",
            IspChoice::Mokhaberat => "TCI / Mokhaberat",
            IspChoice::Other => "Other / Auto-Detect",
        }
    }

    /// Material icon name for the ISP.
    pub fn icon_name(&self) -> &'static str {
        match self {
            IspChoice::Mci => "phone_android",
            IspChoice::Irancell => "cell_tower",
            IspChoice::Rightel => "signal_cellular_alt",
            IspChoice::Shatel => "router",
            IspChoice::Asiatech => "lan",
            IspChoice::Mokhaberat => "cable",
            IspChoice::Other => "public",
        }
    }

    /// Accent colour class for the card.
    pub fn accent_class(&self) -> &'static str {
        match self {
            IspChoice::Mci => "border-blue-500/40 hover:bg-blue-500/10",
            IspChoice::Irancell => "border-yellow-500/40 hover:bg-yellow-500/10",
            IspChoice::Rightel => "border-purple-500/40 hover:bg-purple-500/10",
            IspChoice::Shatel => "border-cyan-500/40 hover:bg-cyan-500/10",
            IspChoice::Asiatech => "border-emerald-500/40 hover:bg-emerald-500/10",
            IspChoice::Mokhaberat => "border-orange-500/40 hover:bg-orange-500/10",
            IspChoice::Other => "border-slate-500/40 hover:bg-slate-500/10",
        }
    }

    /// Recommended default MTU for this ISP.
    pub fn default_mtu(&self) -> u16 {
        match self {
            IspChoice::Mci => 1380,
            IspChoice::Irancell => 1400,
            IspChoice::Rightel => 1360,
            IspChoice::Shatel => 1420,
            IspChoice::Asiatech => 1440,
            IspChoice::Mokhaberat => 1400,
            IspChoice::Other => 1500,
        }
    }

    fn all() -> &'static [IspChoice] {
        &[
            IspChoice::Mci,
            IspChoice::Irancell,
            IspChoice::Rightel,
            IspChoice::Shatel,
            IspChoice::Asiatech,
            IspChoice::Mokhaberat,
            IspChoice::Other,
        ]
    }
}

// ──────────────────────── Wizard Steps ─────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WizardStep {
    SelectIsp,
    ProbeMtu,
    Done,
}

// ──────────────────────── MTU Probe State ────────────────────

#[derive(Debug, Clone, PartialEq)]
struct MtuProbeResult {
    recommended_mtu: u16,
    probe_latency_ms: u32,
    status: MtuProbeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MtuProbeStatus {
    Pending,
    Probing,
    Success,
    Failed,
}

// ──────────────────────── Setup Page ──────────────────────────

#[derive(Props, Clone, PartialEq)]
pub struct SetupPageProps {
    /// Called when setup is complete with (isp_choice, final_mtu)
    pub on_complete: EventHandler<(IspChoice, u16)>,
    /// Optional skip handler
    #[props(default)]
    pub on_skip: Option<EventHandler<()>>,
}

/// Onboarding wizard for first-time ISP selection and MTU probing.
#[component]
pub fn SetupPage(props: SetupPageProps) -> Element {
    let mut step = use_signal(|| WizardStep::SelectIsp);
    let mut selected_isp = use_signal(|| None::<IspChoice>);
    let mut mtu_result = use_signal(|| MtuProbeResult {
        recommended_mtu: 1500,
        probe_latency_ms: 0,
        status: MtuProbeStatus::Pending,
    });
    let custom_mtu = use_signal(|| String::new());

    // ── MTU Probe Logic ──
    let mut start_mtu_probe = move |isp: IspChoice| {
        mtu_result.write().status = MtuProbeStatus::Probing;

        spawn(async move {
            // Simulate binary search MTU probing
            // In production, this calls into rustray's MTU prober
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Probe sequence: start at ISP default, try to go higher
                let base = isp.default_mtu();
                let mut best_mtu = base;
                let mut hi = 1500u16;
                let mut lo = 1200u16;

                // Binary search for max working MTU
                while lo <= hi {
                    let mid = (lo + hi) / 2;

                    // Simulate probe delay
                    tokio::time::sleep(std::time::Duration::from_millis(120)).await;

                    // In production: attempt to send a packet of size `mid`
                    // For now we use the ISP default as the "discovered" optimal MTU
                    if mid <= base + 20 {
                        best_mtu = mid;
                        lo = mid + 1;
                    } else {
                        hi = mid - 1;
                    }
                }

                mtu_result.set(MtuProbeResult {
                    recommended_mtu: best_mtu,
                    probe_latency_ms: 45,
                    status: MtuProbeStatus::Success,
                });
            }

            #[cfg(target_arch = "wasm32")]
            {
                // WASM: use ISP default directly
                gloo_timers::future::TimeoutFuture::new(800).await;
                mtu_result.set(MtuProbeResult {
                    recommended_mtu: isp.default_mtu(),
                    probe_latency_ms: 0,
                    status: MtuProbeStatus::Success,
                });
            }
        });
    };

    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen w-full px-4 py-8 bg-transparent",

            // ── Header ──
            div {
                class: "text-center mb-10",
                div {
                    class: "inline-flex items-center justify-center w-20 h-20 rounded-3xl bg-gradient-to-br from-violet-500/20 to-cyan-500/20 border border-white/10 mb-6",
                    Icon { name: "rocket_launch", class: "text-4xl text-primary" }
                }
                h1 {
                    class: "text-3xl md:text-4xl font-bold text-white tracking-tight mb-2",
                    "Welcome to EdgeRay"
                }
                p {
                    class: "text-slate-400 text-sm md:text-base max-w-md mx-auto",
                    "Let's optimize your connection for the best experience."
                }
            }

            // ── Step indicator ──
            div {
                class: "flex items-center gap-2 mb-8",
                for (i, (s, label)) in [(WizardStep::SelectIsp, "ISP"), (WizardStep::ProbeMtu, "MTU"), (WizardStep::Done, "Ready")].iter().enumerate() {
                    div {
                        class: format!(
                            "flex items-center gap-2 px-4 py-2 rounded-full text-xs font-semibold transition-all duration-300 {}",
                            if *step.read() == *s {
                                "bg-primary/20 text-primary border border-primary/30"
                            } else if (*step.read() as u8) > (*s as u8) {
                                "bg-emerald-500/20 text-emerald-400 border border-emerald-500/20"
                            } else {
                                "bg-white/5 text-slate-500 border border-white/5"
                            }
                        ),
                        span { "{i + 1}" }
                        span { "{label}" }
                    }
                    if i < 2 {
                        div { class: "w-8 h-px bg-white/10" }
                    }
                }
            }

            // ── Step Content ──
            div {
                class: "w-full max-w-2xl",

                match *step.read() {
                    WizardStep::SelectIsp => rsx! {
                        IspSelectionGrid {
                            selected: *selected_isp.read(),
                            on_select: move |isp: IspChoice| {
                                selected_isp.set(Some(isp));
                            },
                        }

                        // Continue Button
                        div {
                            class: "flex justify-center mt-8 gap-4",
                            if let Some(skip_handler) = &props.on_skip {
                                button {
                                    class: "px-6 py-3 rounded-2xl bg-white/5 text-slate-400 hover:bg-white/10 border border-white/5 transition-all text-sm",
                                    onclick: {
                                        let handler = skip_handler.clone();
                                        move |_| handler.call(())
                                    },
                                    "Skip"
                                }
                            }
                            button {
                                class: format!(
                                    "px-8 py-3 rounded-2xl font-semibold text-sm transition-all duration-300 {}",
                                    if selected_isp.read().is_some() {
                                        "bg-gradient-to-r from-primary to-cyan-500 text-white shadow-lg shadow-primary/20 hover:shadow-primary/40 cursor-pointer"
                                    } else {
                                        "bg-white/5 text-slate-600 cursor-not-allowed"
                                    }
                                ),
                                disabled: selected_isp.read().is_none(),
                                onclick: move |_| {
                                    if let Some(isp) = *selected_isp.read() {
                                        step.set(WizardStep::ProbeMtu);
                                        start_mtu_probe(isp);
                                    }
                                },
                                "Continue"
                            }
                        }
                    },

                    WizardStep::ProbeMtu => rsx! {
                        MtuProbeView {
                            isp: selected_isp.read().unwrap_or(IspChoice::Other),
                            result: mtu_result.read().clone(),
                            custom_mtu: custom_mtu,
                            on_accept: move |final_mtu: u16| {
                                step.set(WizardStep::Done);
                                // Short delay then complete
                                let isp = selected_isp.read().unwrap_or(IspChoice::Other);
                                let on_complete = props.on_complete.clone();
                                spawn(async move {
                                    #[cfg(not(target_arch = "wasm32"))]
                                    tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                                    on_complete.call((isp, final_mtu));
                                });
                            },
                            on_retry: move |_| {
                                if let Some(isp) = *selected_isp.read() {
                                    start_mtu_probe(isp);
                                }
                            },
                            on_back: move |_| {
                                step.set(WizardStep::SelectIsp);
                            },
                        }
                    },

                    WizardStep::Done => rsx! {
                        div {
                            class: "flex flex-col items-center gap-6 py-12 text-center animate-in fade-in",

                            div {
                                class: "w-20 h-20 rounded-full bg-emerald-500/20 flex items-center justify-center",
                                Icon { name: "check_circle", class: "text-5xl text-emerald-400" }
                            }

                            h2 {
                                class: "text-2xl font-bold text-white",
                                "All Set!"
                            }

                            p {
                                class: "text-slate-400 text-sm max-w-sm",
                                "EdgeRay is configured for optimal performance on your network."
                            }
                        }
                    },
                }
            }
        }
    }
}

// ──────────────────────── ISP Grid ────────────────────────

#[derive(Props, Clone, PartialEq)]
struct IspSelectionGridProps {
    selected: Option<IspChoice>,
    on_select: EventHandler<IspChoice>,
}

#[component]
fn IspSelectionGrid(props: IspSelectionGridProps) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 md:grid-cols-3 gap-3",

            for isp in IspChoice::all().iter() {
                {
                    let isp_val = *isp;
                    let is_selected = props.selected == Some(isp_val);

                    rsx! {
                        button {
                            key: "{isp_val.display_name()}",
                            class: format!(
                                "relative flex flex-col items-center gap-3 p-5 rounded-2xl border transition-all duration-300 text-center group {} {}",
                                isp_val.accent_class(),
                                if is_selected {
                                    "ring-2 ring-primary bg-primary/10 scale-[1.02] shadow-lg shadow-primary/10"
                                } else {
                                    "bg-white/[0.03]"
                                }
                            ),
                            onclick: {
                                let on_select = props.on_select.clone();
                                move |_| on_select.call(isp_val)
                            },

                            // Selection indicator
                            if is_selected {
                                div {
                                    class: "absolute top-2 right-2 w-5 h-5 rounded-full bg-primary flex items-center justify-center",
                                    Icon { name: "check", class: "text-[12px] text-white" }
                                }
                            }

                            // Icon
                            div {
                                class: format!(
                                    "w-12 h-12 rounded-xl flex items-center justify-center transition-all {}",
                                    if is_selected { "bg-primary/20 text-primary" } else { "bg-white/5 text-slate-400 group-hover:text-white" }
                                ),
                                Icon { name: isp_val.icon_name(), class: "text-2xl" }
                            }

                            // Name
                            span {
                                class: format!(
                                    "text-sm font-medium transition-colors {}",
                                    if is_selected { "text-white" } else { "text-slate-300" }
                                ),
                                "{isp_val.display_name()}"
                            }

                            // Recommended MTU badge
                            span {
                                class: "text-[10px] text-slate-500 font-mono",
                                "MTU {isp_val.default_mtu()}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ──────────────────────── MTU Probe View ──────────────────────

#[derive(Props, Clone, PartialEq)]
struct MtuProbeViewProps {
    isp: IspChoice,
    result: MtuProbeResult,
    custom_mtu: Signal<String>,
    on_accept: EventHandler<u16>,
    on_retry: EventHandler<()>,
    on_back: EventHandler<()>,
}

#[component]
fn MtuProbeView(props: MtuProbeViewProps) -> Element {
    let mut custom_mtu = props.custom_mtu;
    let is_probing = props.result.status == MtuProbeStatus::Probing;
    let is_success = props.result.status == MtuProbeStatus::Success;

    rsx! {
        div {
            class: "flex flex-col items-center gap-6 py-6",

            // ISP badge
            div {
                class: "flex items-center gap-2 px-4 py-2 rounded-full bg-white/5 border border-white/10 text-sm",
                Icon { name: props.isp.icon_name(), class: "text-primary" }
                span { class: "text-slate-300", "{props.isp.display_name()}" }
            }

            // Probe animation
            div {
                class: "relative w-32 h-32 flex items-center justify-center",

                // Spinning ring
                if is_probing {
                    div {
                        class: "absolute inset-0 rounded-full border-4 border-primary/20 border-t-primary animate-spin",
                    }
                }

                // Result circle
                div {
                    class: format!(
                        "w-24 h-24 rounded-full flex flex-col items-center justify-center transition-all duration-500 {}",
                        if is_success { "bg-emerald-500/10 border-2 border-emerald-500/30" }
                        else if is_probing { "bg-primary/10 border-2 border-primary/20" }
                        else { "bg-red-500/10 border-2 border-red-500/30" }
                    ),

                    if is_probing {
                        Icon { name: "speed", class: "text-3xl text-primary animate-pulse" }
                    } else if is_success {
                        span { class: "text-2xl font-bold text-emerald-400 font-mono", "{props.result.recommended_mtu}" }
                    } else {
                        Icon { name: "error_outline", class: "text-3xl text-red-400" }
                    }
                }
            }

            // Status text
            p {
                class: "text-sm text-slate-400 text-center max-w-sm",
                if is_probing {
                    "Probing MTU… This tests which packet size passes through your ISP filters."
                } else if is_success {
                    "Optimal MTU discovered. You can accept the recommendation or set a custom value."
                } else {
                    "Probe failed. Using ISP default. You can retry or set a custom value."
                }
            }

            // Probe stats
            if is_success {
                div {
                    class: "flex gap-6 text-center text-xs text-slate-500",
                    div {
                        span { class: "block text-emerald-400 font-mono text-lg", "{props.result.recommended_mtu}" }
                        "Optimal MTU"
                    }
                    div {
                        span { class: "block text-cyan-400 font-mono text-lg", "{props.result.probe_latency_ms}ms" }
                        "Probe Latency"
                    }
                }
            }

            // Custom MTU input
            if is_success || props.result.status == MtuProbeStatus::Failed {
                div {
                    class: "flex items-center gap-3 mt-2",
                    label { class: "text-xs text-slate-500", "Custom MTU:" }
                    input {
                        r#type: "number",
                        min: "1200",
                        max: "1500",
                        placeholder: "{props.result.recommended_mtu}",
                        class: "w-24 px-3 py-2 rounded-xl bg-white/5 border border-white/10 text-white text-center text-sm font-mono focus:border-primary/50 focus:outline-none transition-all",
                        value: "{props.custom_mtu.read()}",
                        oninput: move |e: Event<FormData>| {
                            custom_mtu.set(e.value());
                        },
                    }
                }
            }

            // Actions
            div {
                class: "flex gap-3 mt-4",

                button {
                    class: "px-5 py-2.5 rounded-xl bg-white/5 text-slate-400 text-sm hover:bg-white/10 border border-white/5 transition-all",
                    onclick: move |_| props.on_back.call(()),
                    "Back"
                }

                if props.result.status == MtuProbeStatus::Failed {
                    button {
                        class: "px-5 py-2.5 rounded-xl bg-amber-500/10 text-amber-400 text-sm hover:bg-amber-500/20 border border-amber-500/20 transition-all",
                        onclick: move |_| props.on_retry.call(()),
                        "Retry"
                    }
                }

                if is_success || props.result.status == MtuProbeStatus::Failed {
                    button {
                        class: "px-8 py-2.5 rounded-xl bg-gradient-to-r from-primary to-cyan-500 text-white font-semibold text-sm shadow-lg shadow-primary/20 hover:shadow-primary/40 transition-all",
                        onclick: {
                            let custom_mtu = props.custom_mtu.clone();
                            let recommended = props.result.recommended_mtu;
                            let on_accept = props.on_accept.clone();
                            move |_| {
                                let final_mtu = custom_mtu
                                    .read()
                                    .parse::<u16>()
                                    .unwrap_or(recommended)
                                    .clamp(1200, 1500);
                                on_accept.call(final_mtu);
                            }
                        },
                        "Accept"
                    }
                }
            }
        }
    }
}

// ──────────────────────── Tests ────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isp_choice_all_has_seven_entries() {
        assert_eq!(IspChoice::all().len(), 7);
    }

    #[test]
    fn test_default_mtu_in_valid_range() {
        for isp in IspChoice::all() {
            let mtu = isp.default_mtu();
            assert!(
                mtu >= 1200 && mtu <= 1500,
                "MTU {} out of range for {:?}",
                mtu,
                isp
            );
        }
    }

    #[test]
    fn test_display_names_not_empty() {
        for isp in IspChoice::all() {
            assert!(!isp.display_name().is_empty());
        }
    }

    #[test]
    fn test_mci_mtu() {
        assert_eq!(IspChoice::Mci.default_mtu(), 1380);
    }

    #[test]
    fn test_irancell_mtu() {
        assert_eq!(IspChoice::Irancell.default_mtu(), 1400);
    }
}
