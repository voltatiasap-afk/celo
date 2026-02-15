use anyhow::Result;
use image::{ImageBuffer, Rgb, RgbImage, open};

pub fn decode(image: String, path: String) -> Result<()> {
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

    Ok(())
}
