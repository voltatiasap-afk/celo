pub mod decode;
pub mod encode;

use crate::cli::{TextAction, TextArgs};
use anyhow::Result;
use colored::Colorize;
use decode::*;
use encode::*;

pub fn handle(args: TextArgs) -> Result<()> {
    match args.action {
        TextAction::Encode {
            text,
            image,
            output,
        } => {
            println!("Encoding {} into {}", text.blue(), image.blue());
            encode(image, text, output.clone())?;
            println!("Saved to {}", output.blue());
            Ok(())
        }
        TextAction::Decode { image } => {
            decode(image)?;
            Ok(())
        }
    }
}
