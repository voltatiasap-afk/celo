pub mod decode;
pub mod encode;

use crate::cli::{ImageAction, ImageArgs};
use anyhow::Result;
use colored::Colorize;
use decode::*;
use encode::*;

pub fn handle(args: ImageArgs) -> Result<()> {
    match args.action {
        ImageAction::Encode {
            main,
            payload,
            depth,
        } => {
            println!(
                "Encoding {} into {} with depth {} ",
                payload.blue(),
                main.blue(),
                depth.to_string().yellow()
            );
            encode(main, payload, args.output.clone(), depth)?;
            println!("Saved to {}", args.output.blue());
            Ok(())
        }
        ImageAction::Decode { input } => {
            println!("Decoding image from {}", input.blue());
            decode(input, args.output.clone())?;
            println!("Saved to {}", args.output.blue());
            Ok(())
        }
    }
}
