use anyhow::Result;
use clap::{Parser, Subcommand};
use woodpecker::{debug, test};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Debug { name: String },
    Test { 
        challenge: u8,
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();            

    match &cli.command {
        Commands::Debug { name } => debug(name),
        Commands::Test { challenge, name } => test(challenge, name),
    }
}