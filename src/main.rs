mod cli;

use anyhow::{Ok, Result};
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use image::imageops;
use image::{ImageBuffer, Rgb, RgbImage, open};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Image(args) => match args.action {
            cli::ImageAction::Encode { main, payload } => {
                img_encode(main, payload)?;
                Ok(())
            }

            cli::ImageAction::Decode { input } => {
                img_decode(input)?;
                Ok(())
            }
        },
        Commands::Text(args) => match args.action {
            cli::TextAction::Encode { text, image } => {
                text_encode(image, text)?;
                Ok(())
            }
            cli::TextAction::Decode { image } => {
                text_decode(image)?;
                Ok(())
            }
        },
    }
}

fn img_encode(main_image: String, payload_image: String) -> Result<()> {
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

    for (x, y, pixel) in img_2.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;
        let Rgb([r1, g1, b1]) = *img_1.get_pixel(x, y);

        let low_r = (0xF0 & r) >> 4;
        let low_g = (0xF0 & g) >> 4;
        let low_b = (0xF0 & b) >> 4;

        let target_r = (r1 & 0xF0) | low_r;
        let target_g = (g1 & 0xF0) | low_g;
        let target_b = (b1 & 0xF0) | low_b;

        output.put_pixel(x, y, Rgb([target_r, target_g, target_b]));
    }

    output.save("merged_images.bmp")?;
    let message = "Saved to merged_images.bmp";
    println!("{}", message.blue().italic());

    return Ok(());
}

fn img_decode(image: String) -> Result<()> {
    let img = open(image)?.to_rgb8();

    let mut output: RgbImage = ImageBuffer::new(img.width(), img.height());

    for (x, y, pixel) in img.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;

        let Rgb([r_low, g_low, b_low]) = Rgb([(r & 0x0F) << 4, (g & 0x0F) << 4, (b & 0x0F) << 4]);

        output.put_pixel(x, y, Rgb([r_low, g_low, b_low]));
    }

    output.save("decoded_image.bmp")?;
    println!("{}", "Saved to decoded_image.bmp".blue());

    Ok(())
}

fn text_encode(image: String, text: String) -> Result<()> {
    let img = open(image)?.to_rgb8();
    let text_bytes = text.as_bytes();

    let mut output: RgbImage = ImageBuffer::new(img.width(), img.height());

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

    output.save("text_encoded.bmp")?;
    println!("{}", "Saved to text_encoded.bmp".blue().italic());
    Ok(())
}

fn text_decode(image: String) -> Result<()> {
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
