use std::cmp;

use anyhow::Result;
use thiserror::Error;
use parse_int::parse;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::cpu::Cpu;

const LICENSE_MESSAGE: &'static str = r#"
                      |              |            
\ \  \ / _ \  _ \  _` | _ \  -_)  _| | /  -_)  _| 
 \_/\_/\___/\___/\__,_|.__/\___|\__|_\_\\___|_|   
                      _|                          

        Woodpecker(R) Embedded Debugger
                             
          Version v0.3.2 ("Wild Oak")  
         Copyright 2089 Arbor Systems.
         Licensed to Kaizen Security. 
                 Do not copy!                 
"#;

const HELP_MESSAGE: &'static str = r#"help           (h) => show this message
step `n`       (s) => step the program forward `n` steps (default: 1)
data           (d) => show the current register values
mem `addr` `n` (m) => read `n` bits (default: 1) at address `addr` 
list           (l) => show the current position in the program
reset          (r) => reset the CPU
quit           (q) => quit the debugger"#;

#[derive(Error, Debug)]
pub enum DebugError{
    #[error("command provided is invalid")]
    InvalidCommand,
    #[error("argument missing from command")]
    MissingArg,
}

pub struct Debugger<'a> {
    cpu: &'a mut Cpu<'a>,
}

impl<'a> Debugger<'a> {
    pub fn new(cpu: &'a mut Cpu<'a>) -> Self {
        Self { cpu }
    }

    pub fn interact(&mut self) -> Result<()> {
        let mut rl = Editor::<()>::new()?;

        println!("{}", LICENSE_MESSAGE);

        loop {
            let readline = rl.readline("dbg > ");
            let line = match readline {
                Ok(line) => {
                    rl.add_history_entry(&line);
                    line
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => return Err(err.into()),
            };

            let mut cmd_args = line.trim().split_whitespace();

            let cmd_op = match cmd_args.next() {
                Some(op) => op,
                None => continue,
            };

            let cmd_result = match cmd_op {
                "help"  | "h" => { println!("{}", HELP_MESSAGE); Ok(()) },
                "step"  | "s" => self.do_step(&mut cmd_args),
                "data"  | "d" => self.do_data(),
                "mem"   | "m" => self.do_mem(&mut cmd_args),
                "list"  | "l" => self.do_list(),
                "reset" | "r" => { self.cpu.reset(); Ok(()) },
                "quit"  | "q" => break,
                &_ => Err(DebugError::InvalidCommand.into()),
            };

            cmd_result.unwrap_or_else(|err| println!("{}", err));
        }

        Ok(())
    }

    fn do_step<'b>(&mut self, args: &mut impl Iterator<Item = &'b str>) -> Result<()> {
        if !self.cpu.done {
            let steps: u32 = if let Some(step_str) = args.next() {
                parse(step_str)?
            } else {
                1
            };

            for _ in 0..steps {
                self.cpu.step();
            }
        }

        if self.cpu.done {
            println!("Execution complete. Use 'r' to reset.");
        }

        Ok(())
    }

    fn do_data(&self) -> Result<()> {
        println!("ADDR:  0x{:08x}", self.cpu.addr);
        println!("STORE: 0x{}", if self.cpu.store { "1" } else { "0" });

        Ok(())
    }

    fn do_mem<'b>(&mut self, args: &mut impl Iterator<Item = &'b str>) -> Result<()> {
        let addr: u32 = if let Some(addr_str) = args.next() {
            parse(addr_str)?
        } else {
            return Err(DebugError::MissingArg.into());
        };

        let count: u32 = if let Some(count_str) = args.next() {
            parse(count_str)?
        } else {
            1
        };

        let end = cmp::min(addr as usize + count as usize, self.cpu.memory.len());
        let data = &self.cpu.memory[addr as usize..end as usize];

        println!("MEM[0x{:08x}:0x{:08x}]: {}", addr, end, data);

        Ok(())
    }

    fn do_list(&self) -> Result<()> {
        let lower: usize = self.cpu.step.saturating_sub(5);
        let upper: usize = cmp::min(self.cpu.step.saturating_add(5), self.cpu.commands.len());

        for cmd_idx in lower..upper {
            let marker: char = if cmd_idx == self.cpu.step { '>' } else { ' ' };
            println!("{} [0x{:08x}] {}", marker, cmd_idx, &self.cpu.commands[cmd_idx]);
        }


        Ok(())
    }
}
