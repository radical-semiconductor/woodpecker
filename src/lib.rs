mod cpu;
mod debug;
mod challenge;

use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use cpu::{Command, Cpu};
use debug::Debugger;
use challenge::evaluate_solution;

pub fn debug(name: &str) -> Result<()> {
    let commands = parse_program(name)?;
    let mut cpu = Cpu::new(&commands);
    let mut debugger = Debugger::new(&mut cpu);
    debugger.interact()?;

    Ok(())
}

pub fn solve(challenge: u8, name: &str) -> Result<()> {
    let commands = parse_program(name)?;  
    evaluate_solution(challenge, &commands)?;

    Ok(())
}

fn parse_program(name: &str) -> Result<Vec<Command>> {
    let file = File::open(name)?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let commands: Vec<Command> = lines
        .map(|l| Ok(l?.parse::<Command>()?))
        .collect::<Result<Vec<Command>>>()?;

    Ok(commands)
}