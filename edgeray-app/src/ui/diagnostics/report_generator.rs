use dioxus::prelude::*;

#[component]
pub fn DiagnosticReportCard() -> Element {
    rsx! {
        div {
            class: "bg-surface-800/50 backdrop-blur-md rounded-xl p-6 border border-white/10 flex flex-col sm:flex-row items-center justify-between gap-4",
            div {
                class: "flex flex-col gap-2",
                h3 { class: "text-lg font-medium text-white", "System Diagnostics" }
                p { class: "text-sm text-gray-400 max-w-md",
                    "Generate and download a comprehensive diagnostic bundle containing system logs, traffic statistics, and configuration snapshots for troubleshooting."
                }
            }
            a {
                href: "/api/diagnostics/report",
                // 'download' attribute forces download in browser
                download: "diagnostic_report.zip",
                class: "bg-primary-600 hover:bg-primary-500 text-white px-6 py-3 rounded-lg font-medium transition-all duration-200 flex items-center gap-2 shadow-lg shadow-primary-900/20",
                span { class: "material-symbols-rounded text-xl", "monitor_heart" }
                span { "Generate Report" }
            }
        }
    }
}
