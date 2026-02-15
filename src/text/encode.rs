use anyhow::{Result, anyhow};
use colored::Colorize;
use image::{ImageBuffer, Rgb, RgbImage, open};

pub fn encode(image: String, text: String, path: String) -> Result<()> {
    let img = open(image)?.to_rgb8();
    let text_bytes = text.as_bytes();

    let mut output: RgbImage = ImageBuffer::new(img.width(), img.height());

    if text_bytes.len() as u32 >= (img.width() * img.height()) {
        println!("{}", "Your text is too big for this image".red().italic());
        return Err(anyhow!("too_big"));
    }
    let mut curr_pixel = 0;
    for (x, y, pixel) in img.enumerate_pixels() {
        if curr_pixel >= text_bytes.len() {
            let Rgb([r, g, b]) = *pixel;

            let clear_r = r & 0xF0;
            let clear_b = b & 0xF0;
            output.put_pixel(x, y, Rgb([clear_r, g, clear_b]));
            continue;
        }

        let high_byte = (0xF0 & text_bytes[curr_pixel]) >> 4;
        let low_byte = 0x0F & text_bytes[curr_pixel];

        let Rgb([r, g, b]) = *pixel;

        let target_r = (r & 0xF0) | high_byte;
        let target_b = (b & 0xF0) | low_byte;

        output.put_pixel(x, y, Rgb([target_r, g, target_b]));
        curr_pixel += 1;
    }

    output.save(&path)?;
    Ok(())
}
