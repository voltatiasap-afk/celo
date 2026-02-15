use anyhow::Result;
use colored::Colorize;
use image::{Rgb, open};

pub fn decode(image: String) -> Result<()> {
    let img = open(image)?.to_rgb8();

    let mut text_bytes = Vec::new();

    for (_, _, pixel) in img.enumerate_pixels() {
        let Rgb([r, _, b]) = *pixel;

        let high = (r & 0x0F) << 4;
        let low = b & 0x0F;

        let byte = high | low;

        text_bytes.push(byte);

        if byte == 0 {
            break;
        }
    }

    let decoded = String::from_utf8_lossy(&text_bytes);

    println!("Decoded: {}", decoded.blue());

    Ok(())
}
