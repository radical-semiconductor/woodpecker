use bitvec::prelude::*;
use std::fmt;

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
            Command::Inc => write!(f, "INC"),
            Command::Inv => write!(f, "INV"),
            Command::Load => write!(f, "LOAD"),
            Command::Cdec => write!(f, "CDEC"),
        }
    }
}

pub struct Cpu<'a> {
    pub memory: BitVec,
    pub addr: usize,
    pub store: bool,
    pub step: usize,
    pub done: bool,
    commands: &'a [Command],
}

impl<'a> Cpu<'a> {
    pub fn new(size: usize, commands: &[Command]) -> Cpu {
        Cpu {
            memory: bitvec![0; size],
            addr: 0,
            store: false,
            step: 0,
            done: false,
            commands,
        }
    }

    pub fn step(&mut self) {
        if !self.done {
            let current_bit = self.memory[self.addr];
            match self.commands[self.step] {
                Command::Inc => self.addr = self.addr.wrapping_add(1),
                Command::Inv => self.memory.set(self.addr, !current_bit),
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
