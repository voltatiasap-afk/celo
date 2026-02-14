use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "secret-image")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Image(ImageArgs),
    Text(TextArgs),
}

#[derive(clap::Args)]
pub struct ImageArgs {
    #[command(subcommand)]
    pub action: ImageAction,

    #[arg(short, long)]
    pub input: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Subcommand)]
pub enum ImageAction {
    Encode {
        #[arg(short, long)]
        main: String,
        #[arg(short, long)]
        payload: String,
    },

    Decode {
        #[arg(short, long)]
        input: String,
    },
}

#[derive(clap::Args)]
pub struct TextArgs {
    #[command(subcommand)]
    pub action: TextAction,
}

#[derive(Subcommand)]
pub enum TextAction {
    Encode {
        #[arg(short, long)]
        text: String,
        #[arg(short, long)]
        image: String,
    },

    Decode {
        #[arg(short, long)]
        image: String,
    },
}
