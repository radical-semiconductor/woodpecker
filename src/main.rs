use clap::{Parser, Subcommand};
use woodpecker::{run, test, Result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Run { name: String },
    Test { 
        challenge: u8,
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();            

    match &cli.command {
        Commands::Run { name } => run(name),
        Commands::Test { challenge, name } => test(challenge, name),
    }
}