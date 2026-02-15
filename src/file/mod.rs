pub mod decode;
pub mod encode;

use crate::cli::{FileAction, FileArgs};
use anyhow::Result;
use colored::Colorize;
use decode::*;
use encode::*;

pub fn handle(args: FileArgs) -> Result<()> {
    match args.action {
        FileAction::Encode { mask, file } => {
            println!("Encoding file {} into {}", file.blue(), mask.blue());
            encode(mask, file, args.output.clone())?;
            println!("Saved to {}", args.output.blue());
            Ok(())
        }
        FileAction::Decode { input } => {
            println!("Decoding file from {}", input.blue());
            decode(input, args.output.clone())?;
            println!("Saved to {}", args.output.blue());
            Ok(())
        }
    }
}
