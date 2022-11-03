mod cpu;
mod debug;
mod error;
mod ui;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use cpu::{Command, Cpu};
use debug::CpuDebugger;
use error::{ExecutionError, ParseError};

pub type Result<T> = std::result::Result<T, ExecutionError>;

pub fn run(name: &str) -> Result<()> {
    let (mem_size, commands) = parse_program(name)?;
    let mut cpu = Cpu::new(mem_size, &commands);
    let run_result = cpu.run();
    let mut debugger = CpuDebugger::new(&mut cpu, &run_result);

    debugger.interact()?;

    Ok(())
}

pub fn test(challenge: &u8, name: &str) -> Result<()> {
    Ok(())
}

fn parse_program(name: &str) -> Result<(usize, Vec<Command>)> {
    let file = File::open(name)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>()?;

    let mem_size = parse_mem_size(&lines[0])?;

    let commands: Result<Vec<Command>> = lines[1..]
        .iter()
        .enumerate()
        .map(|(num, l)| parse_command(l, num))
        .collect();

    Ok((mem_size, commands?))
}

fn parse_mem_size(line: &str) -> Result<usize> {
    let mem_size_str = line
        .split(":")
        .nth(1)
        .ok_or(ParseError::BitCountParseError)?
        .trim();

    let mem_size = mem_size_str
        .parse()
        .map_err(|_| ParseError::BitCountParseError)?;

    Ok(mem_size)
}

fn parse_command(line: &str, line_num: usize) -> Result<Command> {
    match line.trim() {
        "INC" => Ok(Command::Inc),
        "INV" => Ok(Command::Inv),
        "LOAD" => Ok(Command::Load),
        "CDEC" => Ok(Command::Cdec),
        cmd_str => {
            let err = ParseError::CommandParseError {
                cmd: cmd_str.to_owned(),
                line: line_num,
            };
            Err(err.into())
        }
    }
}
