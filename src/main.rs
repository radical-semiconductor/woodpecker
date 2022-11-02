mod cpu;

use clap::{Parser, Subcommand};
use cpu::{Command, Processor};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process;


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

fn main() {
    let cli = Cli::parse();            

    match &cli.command {
        Commands::Run { name } => run(name),
        Commands::Test { challenge, name } => test(challenge, name),
    }
}
 
fn run(name: &str) {
    let mut cpu = Processor::new();
    let file = File::open(name).unwrap();
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let maybe_cmd = match line.trim() {
            "INC" => Some(Command::Inc),
            "INV" => Some(Command::Inv),
            "LOAD" => Some(Command::Load),
            "CDEC" => Some(Command::Cdec),
            _ => None,
        };

        if let Some(cmd) = maybe_cmd {
            cpu.exec(&cmd);
        } else {
            println!("ERROR: invalid command '{}' at line {}", line_num, line);
            process::exit(1);
        }
    }

    let mem = cpu.dump();
    let mem_str: String = mem
        .chunks(8)
        .map(|c| c.iter().map(|b| if *b { '1' } else { '0' }).collect())
        .collect::<Vec<String>>()
        .join(" ");
    
    println!("Memory usage consisted of {} bits.", mem.len());
    println!("Memory at program finish:");
    println!("[ADDR 0] {} [ADDR {}]", mem_str, mem.len() - 1);
}

fn test(challenge: &u8, name: &str) {

}