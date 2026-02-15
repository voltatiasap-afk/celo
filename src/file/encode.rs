use image::{Rgb, open};

use anyhow::{Ok, Result, anyhow};
use colored::Colorize;
pub fn encode(image: String, file: String, path: String) -> Result<()> {
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

    img.save(&path)?;

    Ok(())
}
