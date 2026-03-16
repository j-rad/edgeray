//! Interactive Sparkline — 60-Point Circular Buffer + SVG Path
//!
//! A real-time sparkline widget for telemetry data.
//!
//! - `SparklineBuffer` wraps a `VecDeque<f64>` with a fixed 60-point capacity.
//! - SVG rendering generates a smooth polyline path with gradient fill.
//! - Handles empty data gracefully (flat zero line).
//! - `will-change: transform` for GPU-accelerated rendering.

use std::collections::VecDeque;

use dioxus::prelude::*;

/// Maximum data points in the sparkline buffer.
pub const SPARKLINE_CAPACITY: usize = 60;

/// Circular buffer for sparkline data points.
#[derive(Clone, Debug, PartialEq)]
pub struct SparklineBuffer {
    pub data: VecDeque<f64>,
    pub capacity: usize,
}

impl SparklineBuffer {
    pub fn new() -> Self {
        Self {
            data: VecDeque::with_capacity(SPARKLINE_CAPACITY),
            capacity: SPARKLINE_CAPACITY,
        }
    }

    /// Push a new value, evicting the oldest if at capacity.
    pub fn push(&mut self, value: f64) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the maximum value in the buffer, or 1.0 if empty.
    pub fn max_value(&self) -> f64 {
        self.data
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(1.0)
    }

    /// Returns the most recent value.
    pub fn latest(&self) -> f64 {
        self.data.back().copied().unwrap_or(0.0)
    }
}

impl Default for SparklineBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate an SVG polyline path string from the buffer.
///
/// Maps data points to the `[0, width] × [0, height]` coordinate space.
/// Returns a flat zero-line if the buffer is empty.
pub fn generate_sparkline_path(data: &VecDeque<f64>, width: f64, height: f64) -> String {
    if data.is_empty() {
        return format!("M 0 {} L {} {}", height, width, height);
    }

    let max_val = data
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0);
    let count = data.len();
    let step = if count > 1 {
        width / (count as f64 - 1.0)
    } else {
        width
    };

    let mut path = String::with_capacity(count * 12);
    for (i, val) in data.iter().enumerate() {
        let x = i as f64 * step;
        let y = height - (val / max_val * height);
        if i == 0 {
            path.push_str(&format!("M {:.1} {:.1}", x, y));
        } else {
            path.push_str(&format!(" L {:.1} {:.1}", x, y));
        }
    }
    path
}

/// Generate the closed area path (for gradient fill under the line).
pub fn generate_sparkline_area(data: &VecDeque<f64>, width: f64, height: f64) -> String {
    let line = generate_sparkline_path(data, width, height);
    format!("{} L {:.1} {:.1} L 0 {:.1} Z", line, width, height, height)
}

/// Props for the InteractiveSparkline component.
#[derive(Props, Clone, PartialEq)]
pub struct SparklineProps {
    /// Data buffer.
    pub buffer: SparklineBuffer,
    /// SVG width in px.
    #[props(default = 200.0)]
    pub width: f64,
    /// SVG height in px.
    #[props(default = 60.0)]
    pub height: f64,
    /// Stroke color (CSS color string).
    #[props(default = "#00F2FF".to_string())]
    pub color: String,
    /// Unique ID suffix for the gradient (must be unique per instance).
    #[props(default = "default".to_string())]
    pub id: String,
    /// Extra CSS classes on the wrapper.
    #[props(default = String::new())]
    pub class: String,
}

/// InteractiveSparkline — 60-point real-time chart.
#[component]
pub fn InteractiveSparkline(props: SparklineProps) -> Element {
    let line_path = generate_sparkline_path(&props.buffer.data, props.width, props.height);
    let area_path = generate_sparkline_area(&props.buffer.data, props.width, props.height);
    let grad_id = format!("sparkline-grad-{}", props.id);
    let latest = props.buffer.latest();

    rsx! {
        div {
            class: "relative will-change-transform {props.class}",

            svg {
                class: "w-full h-full",
                view_box: "0 0 {props.width} {props.height}",
                xmlns: "http://www.w3.org/2000/svg",
                preserve_aspect_ratio: "none",

                // Gradient fill definition
                defs {
                    linearGradient {
                        id: "{grad_id}",
                        x1: "0", y1: "0", x2: "0", y2: "1",
                        stop { offset: "0%", stop_color: "{props.color}", stop_opacity: "0.3" }
                        stop { offset: "100%", stop_color: "{props.color}", stop_opacity: "0.0" }
                    }
                }

                // Area fill
                path {
                    d: "{area_path}",
                    fill: "url(#{grad_id})",
                }

                // Line stroke
                path {
                    d: "{line_path}",
                    fill: "none",
                    stroke: "{props.color}",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }

                // Latest value dot
                if !props.buffer.is_empty() {
                    circle {
                        cx: "{props.width}",
                        cy: "{props.height - (latest / props.buffer.max_value() * props.height)}",
                        r: "3",
                        fill: "{props.color}",
                    }
                }
            }
        }
    }
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_buffer_capacity() {
        let mut buf = SparklineBuffer::new();
        assert_eq!(buf.capacity, SPARKLINE_CAPACITY);
        assert!(buf.is_empty());

        for i in 0..70 {
            buf.push(i as f64);
        }
        assert_eq!(buf.len(), SPARKLINE_CAPACITY);
        assert_eq!(buf.latest(), 69.0);
    }

    #[test]
    fn test_sparkline_buffer_eviction() {
        let mut buf = SparklineBuffer::new();
        for i in 0..65 {
            buf.push(i as f64);
        }
        assert_eq!(buf.len(), 60);
        // Oldest should be 5.0 (0..4 evicted)
        assert_eq!(*buf.data.front().unwrap(), 5.0);
    }

    #[test]
    fn test_sparkline_empty_path() {
        let data = VecDeque::new();
        let path = generate_sparkline_path(&data, 200.0, 60.0);
        assert!(path.starts_with("M 0 60"));
        assert!(path.contains("L 200 60"));
    }

    #[test]
    fn test_sparkline_single_point() {
        let mut data = VecDeque::new();
        data.push_back(50.0);
        let path = generate_sparkline_path(&data, 200.0, 60.0);
        assert!(path.starts_with("M 0.0"));
    }

    #[test]
    fn test_sparkline_area_closes_path() {
        let mut data = VecDeque::new();
        data.push_back(10.0);
        data.push_back(20.0);
        let area = generate_sparkline_area(&data, 200.0, 60.0);
        assert!(area.ends_with('Z'));
    }

    #[test]
    fn test_max_value_minimum() {
        let buf = SparklineBuffer::new();
        assert_eq!(buf.max_value(), 1.0);
    }

    #[test]
    fn test_latest_empty() {
        let buf = SparklineBuffer::new();
        assert_eq!(buf.latest(), 0.0);
    }

    #[test]
    fn test_sparkline_with_all_zeros() {
        let mut buf = SparklineBuffer::new();
        for _ in 0..10 {
            buf.push(0.0);
        }
        let path = generate_sparkline_path(&buf.data, 200.0, 60.0);
        // All zeros should render at the bottom (y = height)
        assert!(path.contains("60.0"));
    }
}
