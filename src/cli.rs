use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Image(ImageArgs),
    Text(TextArgs),
    File(FileArgs),
}

#[derive(clap::Args)]
pub struct ImageArgs {
    #[command(subcommand)]
    pub action: ImageAction,
    #[arg(short, long, default_value = "output.png")]
    pub output: String,
}
#[derive(clap::Args)]
pub struct FileArgs {
    #[command(subcommand)]
    pub action: FileAction,
    #[arg(short, long)]
    pub output: String,
}

#[derive(Subcommand)]
pub enum FileAction {
    Encode {
        #[arg(short, long)]
        mask: String,
        #[arg(short, long)]
        file: String,
    },

    Decode {
        #[arg(short, long)]
        input: String,
    },
}
#[derive(Subcommand)]
pub enum ImageAction {
    Encode {
        #[arg(short, long)]
        main: String,
        #[arg(short, long)]
        payload: String,
        #[arg(short, long, default_value = "2")]
        depth: u8,
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
        #[arg(short, long, default_value = "output.png")]
        output: String,
    },

    Decode {
        #[arg(short, long)]
        image: String,
    },
}
