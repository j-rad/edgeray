use crate::models::ServerConfig;
use dioxus::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};

#[derive(Props, Clone, PartialEq)]
pub struct QrShareProps {
    pub config: ServerConfig,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn QrShare(props: QrShareProps) -> Element {
    let uri = props.config.to_uri();

    // Generate QR Code
    // Use QrCodeEcc::Medium for balance between error correction and size
    let qr_result = QrCode::encode_text(&uri, QrCodeEcc::Medium);

    let svg_content = match qr_result {
        Ok(qr) => {
            let size = qr.size();
            let mut path_data = String::new();

            for y in 0..size {
                for x in 0..size {
                    if qr.get_module(x, y) {
                        // M x y h 1 v 1 h -1 z draws a 1x1 square at (x,y)
                        path_data.push_str(&format!("M{},{}h1v1h-1z", x, y));
                    }
                }
            }

            rsx! {
                svg {
                    view_box: "0 0 {size} {size}",
                    class: "w-full h-full",
                    shape_rendering: "crispEdges",
                    path {
                        d: "{path_data}",
                        fill: "black" // QR code itself is black
                    }
                }
            }
        },
        Err(_) => rsx! {
            div { class: "text-red-500 text-sm", "Error generating QR Code" }
        }
    };

    let copy_uri = uri.clone();
    let mut copied = use_signal(|| false);

    let on_copy = move |_| {
        let uri_js = copy_uri.clone();
        // Use eval to copy to clipboard
        let script = format!("navigator.clipboard.writeText('{}')", uri_js.replace("'", "\\'"));
        let _ = dioxus::document::eval(&script);
        copied.set(true);

        // Reset copy status after 2 seconds
        let mut copied_sig = copied;
        spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            copied_sig.set(false);
        });
    };

    rsx! {
        div {
            class: "fixed inset-0 z-[100] bg-black/80 backdrop-blur-sm flex items-center justify-center p-4 animate-in fade-in duration-200",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "bg-[#121212] border border-white/10 rounded-3xl p-6 max-w-sm w-full flex flex-col items-center gap-6 shadow-2xl scale-100 animate-in zoom-in-95 duration-200",
                onclick: move |e| e.stop_propagation(),

                // Header
                div {
                    class: "text-center w-full",
                    h3 { class: "text-lg font-bold text-white mb-1", "Share Config" }
                    p { class: "text-xs text-gray-400 truncate max-w-[200px] mx-auto", "{props.config.remarks}" }
                }

                // QR Display Container
                div {
                    class: "bg-white p-4 rounded-2xl w-64 h-64 shadow-inner flex items-center justify-center",
                    {svg_content}
                }

                // Actions
                div {
                    class: "flex flex-col gap-3 w-full",

                    button {
                        class: format!(
                            "w-full py-3 rounded-xl font-medium transition-all duration-200 flex items-center justify-center gap-2 {}",
                            if *copied.read() { "bg-emerald-500/20 text-emerald-400" } else { "bg-primary/20 hover:bg-primary/30 text-primary" }
                        ),
                        onclick: on_copy,
                        if *copied.read() {
                            span { class: "material-symbols-outlined text-[18px]", "check" }
                            "Copied!"
                        } else {
                            span { class: "material-symbols-outlined text-[18px]", "content_copy" }
                            "Copy URI"
                        }
                    }

                    button {
                        class: "w-full py-3 rounded-xl bg-white/5 hover:bg-white/10 text-gray-400 hover:text-white font-medium transition-colors",
                        onclick: move |_| props.on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
