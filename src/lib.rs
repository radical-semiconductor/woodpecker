mod cpu;
mod error;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use cpu::{Command, Cpu};
use error::{ExecutionError, ParseError};

pub type Result<T> = std::result::Result<T, ExecutionError>;

const BYTES_PER_LINE: usize = 8;


pub fn run(name: &str) -> Result<()> {
    let (cpu, commands) = load_cpu_and_commands(name)?;

    Ok(())
}

pub fn test(challenge: &u8, name: &str) -> Result<()> {
    Ok(())
}

fn load_cpu_and_commands(name: &str) -> Result<(Cpu, Vec<Command>)>{
    let file = File::open(name)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<std::result::Result<_, _>>()?;

    let mem_size = parse_mem_size(&lines[0])?;
    let mut cpu = Cpu::new(mem_size);

    let commands: Result<Vec<Command>> = lines[1..].iter()
        .enumerate()
        .map(|(num, l)| parse_command(l, num))
        .collect();

    Ok((cpu, commands?))
}

fn parse_mem_size(line: &str) -> Result<usize> {
    let mem_size_str = line.split(":").next().ok_or(ParseError::BitCountParseError)?;
    let mem_size = mem_size_str.parse().map_err(|_| ParseError::BitCountParseError)?;

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

/* fn dump_memory(mem: &BitSlice) {
    // turn the bytes into Strings
    let mut byte_strs: Vec<String> = mem.chunks(8)
        .map(|c| c.iter().map(|b| if *b { '1' } else { '0' }).collect())
        .collect();

    // complete the last byte with underscores
    let last_byte = byte_strs.last_mut().unwrap();
    let num_missing = 8 - last_byte.len();
    last_byte.extend((0..num_missing).map(|_| '_'));

    // get the chunks of bytes
    let byte_chunks = byte_strs.chunks(BYTES_PER_LINE);
    let num_chunks = byte_chunks.len();

    // print each byte chunk as a line
    for (chunk_num, chunk) in byte_chunks.enumerate() {
        let addr = chunk_num * BYTES_PER_LINE * 8;
        let mut chunk_elts: Vec<String>;

        // extend the chunk if necessary by "bytes" of underscores
        let full_chunk = if chunk_num == num_chunks - 1 {
            let num_missing = BYTES_PER_LINE - chunk.len();
            chunk_elts = Vec::from(chunk);
            chunk_elts.extend((0..num_missing).map(|_| "_".repeat(8)));
            &chunk_elts
        } else {
            chunk
        };

        let line = format!("[ADDR 0x{:012x}] ", addr) + &full_chunk.join(" "); 
        println!("{}", line);
    }
} */