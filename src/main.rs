mod cli;
mod file;
mod image;
mod text;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Image(args) => image::handle(args),
        Commands::Text(args) => text::handle(args),
        Commands::File(args) => file::handle(args),
    }
}
