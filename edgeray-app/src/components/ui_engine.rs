//! UI Engine for EdgeRay
//!
//! Provides native Rust-based UI utilities to replace legacy JavaScript logic.

use crate::components::ui::Icon;
use dioxus::prelude::*;
use qrcode::QrCode;
use qrcode::EcLevel;

/// Renders a QR code natively as an SVG.
#[component]
pub fn RenderQrNative(content: String, size: Option<u32>) -> Element {
    let size = size.unwrap_or(256);
    let qr = match QrCode::new(&content) {
        Ok(qr) => qr,
        Err(_) => return rsx! { div { "QR Encoding Error" } },
    };

    let border = 4;
    let width = qr.width();
    let len = width + border * 2;
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">",
        len
    );
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>");
    svg.push_str("<path d=\"");
    for y in 0..width {
        for x in 0..width {
            if qr[(x, y)] == qrcode::Color::Dark {
                if x != 0 || y != 0 {
                    svg.push(' ');
                }
                svg.push_str(&format!("M{},{}h1v1h-1z", x + border, y + border));
            }
        }
    }
    svg.push_str("\" fill=\"#000000\"/>");
    svg.push_str("</svg>");

    rsx! {
        div {
            class: "qr-container bg-white p-2 rounded-lg shadow-inner",
            style: "width: {size}px; height: {size}px;",
            dangerous_inner_html: "{svg}"
        }
    }
}

/// Icon Factory using local Icon component.
#[component]
pub fn IconFactory(
    name: String,
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor".to_string())] color: String,
) -> Element {
    let size_str = if size <= 16 {
        "sm"
    } else if size <= 20 {
        "md"
    } else if size <= 24 {
        "lg"
    } else {
        "xl"
    };

    rsx! {
        Icon {
            name: name,
            size: size_str.to_string(),
            class: color,
        }
    }
}

/// Telemetry Stream Component
///
/// Subscribes to the gRPC control bus signals and maps them to Dioxus state.
#[component]
pub fn TelemetryStream() -> Element {
    let mut signal = use_signal(|| "Initializing Telemetry...".to_string());

    use_future(move || async move {
        // In a real implementation, this would connect to the gRPC client
        // let mut client = rustray::api::control::ControlServiceClient::connect("http://[::1]:50051").await?;
        // let mut stream = client.push_delta_update(DeltaRequest { ... }).await?.into_inner();

        // Mocking the stream for now to demonstrate the pattern
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        signal.set("Live Telemetry: Active".to_string());
    });

    rsx! {
        div {
            class: "telemetry-status text-xs font-mono text-emerald-400 opacity-80",
            "{signal}"
        }
    }
}
