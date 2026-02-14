mod cli;

use anyhow::{Ok, Result, anyhow};
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use image::imageops;
use image::{ImageBuffer, Rgb, RgbImage, open};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Image(args) => match args.action {
            cli::ImageAction::Encode {
                main,
                payload,
                depth,
            } => {
                img_encode(main, payload, args.output, depth)?;
                Ok(())
            }

            cli::ImageAction::Decode { input } => {
                img_decode(input, args.output)?;
                Ok(())
            }
        },
        Commands::Text(args) => match args.action {
            cli::TextAction::Encode {
                text,
                image,
                output,
            } => {
                text_encode(image, text, output)?;
                Ok(())
            }
            cli::TextAction::Decode { image } => {
                text_decode(image)?;
                Ok(())
            }
        },
        Commands::File(args) => match args.action {
            cli::FileAction::Encode { mask, file } => {
                file_encode(mask, file, args.output)?;
                Ok(())
            }
            cli::FileAction::Decode { input } => {
                file_decode(input, args.output)?;
                Ok(())
            }
        },
    }
}

fn img_encode(main_image: String, payload_image: String, path: String, bits: u8) -> Result<()> {
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
    println!("Saved to {}", path.blue());

    return Ok(());
}

fn img_decode(image: String, path: String) -> Result<()> {
    let img = open(image)?.to_rgb8();

    let mut output: RgbImage = ImageBuffer::new(img.width(), img.height());

    let Rgb([_, bits, _]) = img.get_pixel(0, 0);

    let mask = (1u8 << bits) - 1;

    for (x, y, pixel) in img.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;

        let Rgb([r_low, g_low, b_low]) = Rgb([
            (r & mask) << (8 - bits),
            (g & mask) << (8 - bits),
            (b & mask) << (8 - bits),
        ]);

        output.put_pixel(x, y, Rgb([r_low, g_low, b_low]));
    }

    output.save(&path)?;
    println!("Saved to {}", path.blue());

    Ok(())
}

fn text_encode(image: String, text: String, path: String) -> Result<()> {
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
    println!("Saved to {}", path.blue());
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

fn file_encode(image: String, file: String, path: String) -> Result<()> {
    let file = std::fs::read(file)?;
    let mut img = open(image)?.to_rgb8();

    let mut curr_byte = 0;
    let mut curr_half: u8 = 0;

    if (file.len() * 2) as u32 >= (img.width() * img.height()) {
        println!("{}", "Your file is too big for this image".red().italic());
        return Err(anyhow!("too_big"));
    }

    for (_, _, pixel) in img.enumerate_pixels_mut() {
        let Rgb([r, g, b]) = *pixel;
        if curr_byte >= file.len() {
            *pixel = Rgb([33, 33, 33]);
            break;
        }

        let target_r: u8;
        let target_b: u8;

        match curr_half {
            0 => {
                target_r = (r & 0xFC) | (file[curr_byte] & 0x03);
                target_b = (b & 0xFC) | ((file[curr_byte] & 0x0C) >> 2);
                curr_half += 1;
            }
            1 => {
                target_r = (r & 0xFC) | ((file[curr_byte] & 0x30) >> 4);
                target_b = (b & 0xFC) | ((file[curr_byte] & 0xC0) >> 6);
                curr_half = 0;
                curr_byte += 1;
            }

            _ => {
                unreachable!();
            }
        }

        *pixel = Rgb([target_r, g, target_b])
    }

    img.save(path)?;

    Ok(())
}

fn file_decode(image: String, path: String) -> Result<()> {
    let img = open(image)?.to_rgb8();
    let mut output: Vec<u8> = Vec::new();

    let mut curr_byte: u8 = 0;
    let mut curr_half = 0;
    for (_, _, pixel) in img.enumerate_pixels() {
        let Rgb([r, g, b]) = *pixel;

        if (r, g, b) == (33, 33, 33) {
            break;
        }

        if curr_half == 0 {
            let low = r & 0x03;
            let top = b & 0x03;
            let low_half = (top << 2) | low;
            curr_byte = low_half;

            curr_half += 1;
        } else if curr_half == 1 {
            let low = r & 0x03;
            let top = b & 0x03;

            curr_byte = ((top << 6) | (low << 4)) | curr_byte;

            output.push(curr_byte);

            curr_byte = 0;
            curr_half = 0;
        }
    }

    std::fs::write(path, output)?;

    Ok(())
}
