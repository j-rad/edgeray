use qrcode::{QrCode, Color};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct QrProps {
    /// Data to encode in the QR code
    pub data: String,
    /// CSS classes for the SVG element
    #[props(default)]
    pub class: String,
    /// Fill color for the QR code modules
    #[props(default = "black".to_string())]
    pub fill: String,
}

/// A pure-Rust QR code component that renders to a single SVG path.
/// Zero Javascript, zero layout shifts.
#[component]
pub fn Qr(props: QrProps) -> Element {
    let qr_result = QrCode::new(&props.data);

    match qr_result {
        Ok(qr) => {
            let width = qr.width();
            let mut path_data = String::new();
            
            // The qrcode crate uses a 1D array of colors.
            // Converting to path data. Dark pixels are drawn as 1x1 rectangles.
            for (i, color) in qr.to_colors().iter().enumerate() {
                if *color == Color::Dark {
                    let x = i % width;
                    let y = i / width;
                    // Optimization: Use relative horizontal moves (h1) to reduce path length
                    path_data.push_str(&format!("M{},{}h1v1h-1z", x, y));
                }
            }

            rsx! {
                svg {
                    view_box: "0 0 {width} {width}",
                    class: "{props.class}",
                    shape_rendering: "crispEdges",
                    path {
                        d: "{path_data}",
                        fill: "{props.fill}"
                    }
                }
            }
        }
        Err(e) => rsx! {
            div { class: "text-red-500 text-xs font-mono p-4 border border-red-500/20 rounded-lg bg-red-500/5",
                "QR Error: {e}"
            }
        },
    }
}

/// Helper function to generate SVG path string directly as requested by mandate.
pub fn generate_qr_path(data: &str) -> Result<(String, usize), String> {
    let qr = QrCode::new(data).map_err(|e| e.to_string())?;
    let width = qr.width();
    let mut path_data = String::new();
    
    for (i, color) in qr.to_colors().iter().enumerate() {
        if *color == Color::Dark {
            let x = i % width;
            let y = i / width;
            path_data.push_str(&format!("M{},{}h1v1h-1z", x, y));
        }
    }
    
    Ok((path_data, width))
}
