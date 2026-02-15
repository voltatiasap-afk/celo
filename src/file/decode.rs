use anyhow::Result;
use image::Rgb;
use image::open;

pub fn decode(image: String, path: String) -> Result<()> {
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

            curr_byte |= (top << 6) | (low << 4);

            output.push(curr_byte);

            curr_byte = 0;
            curr_half = 0;
        }
    }

    std::fs::write(&path, output)?;

    Ok(())
}
