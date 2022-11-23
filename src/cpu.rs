use bitvec::prelude::*;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

const INC_STR: &str = "INC";
const INV_STR: &str = "INV";
const LOAD_STR: &str = "LOAD";
const CDEC_STR: &str = "CDEC";

const MEM_SIZE: usize = 1 << 32;

#[derive(Debug, Error)]
#[error("invalid CPU command '{command_text}'")]
pub struct ParseCommandError {
    command_text: String,
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Inc(u32),
    Inv,
    Load,
    Cdec(u32),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Inc(n) => write!(f, "{} {}", INC_STR, n),
            Command::Inv => write!(f, "{}", INV_STR),
            Command::Load => write!(f, "{}", LOAD_STR),
            Command::Cdec(n) => write!(f, "{} {}", CDEC_STR, n),
        }
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [INC_STR] => Ok(Self::Inc(1)),
            [INC_STR, nstr] => nstr.parse().map(Self::Inc).map_err(|_| ParseCommandError {
                command_text: s.to_owned() + ": invalid repetition count",
            }),
            [INV_STR] => Ok(Self::Inv),
            [LOAD_STR] => Ok(Self::Load),
            [CDEC_STR] => Ok(Self::Cdec(1)),
            [CDEC_STR, nstr] => nstr.parse().map(Self::Cdec).map_err(|_| ParseCommandError {
                command_text: s.to_owned() + ": invalid repetition count",
            }),
            _ => Err(ParseCommandError {
                command_text: s.to_owned(),
            }),
        }
    }
}

pub struct Cpu<'a> {
    pub memory: BitVec<u8>,
    pub addr: u32,
    pub store: bool,
    pub step: usize,
    pub done: bool,
    pub commands: &'a [Command],
}

impl<'a> Cpu<'a> {
    pub fn new(commands: &'a [Command]) -> Self {
        Self {
            memory: bitvec![u8, Lsb0; 0; MEM_SIZE],
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
                Command::Inc(n) => self.addr = self.addr.wrapping_add(n),
                Command::Inv => self.memory.set(mem_idx, !current_bit),
                Command::Load => self.store = current_bit,
                Command::Cdec(n) => {
                    if self.store {
                        self.addr = self.addr.wrapping_sub(n);
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
