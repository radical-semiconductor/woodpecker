use bitvec::prelude::*;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

const INC_STR: &'static str = "INC";
const INV_STR: &'static str = "INV";
const LOAD_STR: &'static str = "LOAD";
const CDEC_STR: &'static str = "CDEC";

const MEM_SIZE: usize = 1 << 16;


#[derive(Debug, Error)]
#[error("invalid CPU command '{command_text}'")]
pub struct ParseCommandError {
    command_text: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Inc,
    Inv,
    Load,
    Cdec,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Inc => write!(f, "{}", INC_STR),
            Command::Inv => write!(f, "{}", INV_STR),
            Command::Load => write!(f, "{}", LOAD_STR),
            Command::Cdec => write!(f, "{}", CDEC_STR),
        }
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            INC_STR => Ok(Self::Inc),
            INV_STR => Ok(Self::Inv),
            LOAD_STR => Ok(Self::Load),
            CDEC_STR => Ok(Self::Cdec),
            s => Err(ParseCommandError {
                command_text: s.to_owned(),
            }),
        }
    }
}

pub struct Cpu<'a> {
    pub memory: BitVec,
    pub addr: u16,
    pub store: bool,
    pub step: usize,
    pub done: bool,
    pub commands: &'a [Command],
}

impl<'a> Cpu<'a> {
    pub fn new(commands: &'a[Command]) -> Self {
        Self {
            memory: bitvec![0; MEM_SIZE],
            addr: 0,
            store: false,
            step: 0,
            done: false,
            commands,
        }
    }

    pub fn reset(&mut self) {
        self.memory.fill(false);
        self.addr = 0;
        self.store = false;
        self.step = 0;
        self.done = false;
    }

    pub fn step(&mut self) {
        if !self.done {
            let mem_idx = self.addr as usize;
            let current_bit = self.memory[mem_idx];
            match self.commands[self.step] {
                Command::Inc => self.addr = self.addr.wrapping_add(1),
                Command::Inv => self.memory.set(mem_idx, !current_bit),
                Command::Load => self.store = current_bit,
                Command::Cdec => {
                    if self.store {
                        self.addr = self.addr.wrapping_sub(1);
                    }
                }
            }

            self.step += 1;
            if self.step == self.commands.len() {
                self.done = true;
            }
        }
    }
}
