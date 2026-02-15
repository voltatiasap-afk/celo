use anyhow::Result;
use colored::Colorize;
use image::imageops;
use image::{ImageBuffer, Rgb, RgbImage, open};

pub fn encode(main_image: String, payload_image: String, path: String, bits: u8) -> Result<()> {
    let img_1 = open(main_image)?.to_rgb8();
    let mut img_2 = open(payload_image)?.to_rgb8();

    let mut output: RgbImage = ImageBuffer::new(img_1.width(), img_1.height());

    if img_1.height() != img_2.height() || img_1.width() != img_2.width() {
        img_2 = imageops::resize(
            &img_2,
            img_1.width(),
            img_1.height(),
            imageops::FilterType::Lanczos3,
        );

        println!(
            "{}",
            "payload image got resized due to different resolutions"
                .yellow()
                .italic()
        );
    }

    let shift = 8 - bits;
    let mask = (1u8 << bits) - 1;

    for (x, y, pixel) in img_2.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;
        let Rgb([r1, g1, b1]) = *img_1.get_pixel(x, y);

        let low_r = (r >> shift) & mask;
        let low_g = (g >> shift) & mask;
        let low_b = (b >> shift) & mask;

        let target_r = (r1 & !mask) | low_r;
        let target_g = (g1 & !mask) | low_g;
        let target_b = (b1 & !mask) | low_b;

        if (x, y) == (0, 0) {
            output.put_pixel(x, y, Rgb([r1, bits, b1]))
        } else {
            output.put_pixel(x, y, Rgb([target_r, target_g, target_b]));
        }
    }

    output.save(&path)?;

    Ok(())
}
