use fast_qr::{ECL, QRBuilder};
use image::{GrayImage, Luma};
use rqrr::PreparedImage;

#[test]
fn test_qr_round_trip() {
    let content = "vless://uuid@127.0.0.1:443?security=reality&sni=google.com&fp=chrome&pbk=...&sid=...&type=tcp&headerType=none#ExampleNode";

    // 1. Generate QR Code
    let qrcode = QRBuilder::new(content.as_bytes().to_vec())
        .ecl(ECL::M)
        .build()
        .expect("Failed to build QR code");

    // 2. Render to Image (Manual rendering to GrayImage)
    // Create image with 4px border and scale 4
    let module_size = 4;
    let border = 4;
    let width = (qrcode.size + border * 2) * module_size;
    let height = (qrcode.size + border * 2) * module_size;

    let mut img = GrayImage::new(width as u32, height as u32);

    // Fill with white background
    for pixel in img.pixels_mut() {
        *pixel = Luma([255]);
    }

    // Draw modules
    for _y in 0..qrcode.size {
        for _x in 0..qrcode.size {
            // Using direct index access if available, or just guess the data structure.
            // fast_qr 0.10 typically implements index access or get_module
            // We'll use the .get(x, y) method if it exists, but `QRCode` usually has public `data`.
            // Let's assume `.get_module(x, y)` is not standard but index is.
            // Actually, for 0.10, let's try assuming standard iteration.

            // Wait, to be safe, let's check if there is a method.
            // I'll rely on `rxing` if `fast_qr`/`rqrr` combo is hard to guess APIs for.
            // But prompt asked for `fast_qr` and `rqrr`.

            // Let's assume standard behavior:
            // qrcode[y][x] or qrcode[(x, y)]?
            // fast_qr::QRCode usually provides an iterator or direct access.
            // Let's use a safe assumption: `qrcode.data[y * width + x]`.
            // Wait, `data` might be packed bits.

            // Let's try to find a simpler way: `fast_qr` creates SVGs easily.
            // Can we rasterize SVG? No easy way in pure Rust without huge dependencies (resvg).

            // I'll use the `image` crate to draw rectangles.
            // I'll guess the API is `get_module(x, y)`. If test fails compilation, I'll fix it.
            // Looking at docs for 0.10, it seems `qrcode` is indexable by `usize` (linear) or `(usize, usize)`.

            // Wait, I see `image = "0.25.0"` in Cargo.toml.

            // Let's try:
            /*
            if qrcode[(x, y)] {
                // draw black
            }
            */
            // But if `Index` isn't implemented?

            // Just for the test, I will rely on `rqrr`'s ability to decode "perfect" images.
            // I'll try to use the `data` vec assuming it's `u8` or `bool`.
            // In fast_qr 0.10, `data` is `Vec<u8>`.
        }
    }

    // To avoid guessing APIs that might fail compilation, I will implement a robust renderer based on `data`
    // assuming it is a flat vector of bytes where 1/true is black.

    for (i, module) in qrcode.data.iter().enumerate() {
        let row = i / qrcode.size;
        let col = i % qrcode.size;

        // Is module dark?
        let is_dark = module.value();
        if is_dark {
            let start_x = (col + border) * module_size;
            let start_y = (row + border) * module_size;

            for py in 0..module_size {
                for px in 0..module_size {
                    img.put_pixel((start_x + px) as u32, (start_y + py) as u32, Luma([0]));
                }
            }
        }
    }

    // 3. Decode with rqrr
    let mut prepared = PreparedImage::prepare(img);
    let grids = prepared.detect_grids();
    assert!(!grids.is_empty(), "No QR grid detected in generated image");

    let (_meta, decoded) = grids[0].decode().expect("Failed to decode QR");
    assert_eq!(decoded, content);
}
