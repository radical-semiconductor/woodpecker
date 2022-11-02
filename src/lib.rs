mod cpu;
mod error;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use bitvec::prelude::*;
use cpu::{Command, Processor};
use error::{ExecutionError, ParseError};

pub type Result<T> = std::result::Result<T, ExecutionError>;

const BYTES_PER_LINE: usize = 8;


pub fn run(name: &str) -> Result<()> {
    let mut cpu = Processor::new();
    let file = File::open(name)?;
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let cmd = match line.trim() {
            "INC" => Command::Inc,
            "INV" => Command::Inv,
            "LOAD" => Command::Load,
            "CDEC" => Command::Cdec,
            cmd_str => {
                let err = ParseError {
                    cmd: cmd_str.to_owned(),
                    line: line_num
                };
                return Err(err.into())
            }
        };

        cpu.exec(&cmd)?;
    }

    let mem = cpu.dump();
    dump_memory(mem);

    Ok(())
}

pub fn test(challenge: &u8, name: &str) -> Result<()> {
    Ok(())
}


fn dump_memory(mem: &BitSlice) {
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
}