use std::fmt;

use crate::error::CpuError;
use bitvec::prelude::*;

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

#[derive(Debug)]
pub struct Cpu {
    pub memory: BitVec,
    pub addr: usize,
    pub store: bool,
    pub step: usize,
    pub commands: Vec<Command>,
}

impl Cpu {
    pub fn new(size: usize, commands: &[Command]) -> Cpu {
        Cpu {
            memory: bitvec![0; size],
            addr: 0,
            store: false,
            step: 0,
            commands: Vec::from(commands),
        }
    }

    pub fn run(&mut self) -> Result<(), CpuError> {
        for _ in 0..self.commands.len() {
            self.forward()?;
        }

        Ok(())
    }

    pub fn forward(&mut self) -> Result<(), CpuError> {
        if self.step < self.commands.len() {
            match self.get_command() {
                Some(Command::Inc) => self.increment_addr()?,
                Some(Command::Inv) => self.invert_bit(),
                Some(Command::Load) => self.load_bit_to_store(),
                Some(Command::Cdec) => self.decrement_addr_if_store_set()?,
                None => (),
            }

            self.step += 1;
        }

        Ok(())
    }

    pub fn backward(&mut self) -> Result<(), CpuError> {
        if self.step > 0 {
            self.step -= 1;

            match self.get_command() {
                Some(Command::Inc) => self.decrement_addr()?,
                Some(Command::Inv) => self.invert_bit(),
                Some(Command::Load) => self.unload_bit_from_store(),
                Some(Command::Cdec) => self.increment_addr_if_store_set()?,
                None => (),
            }
        }

        Ok(())
    }

    pub fn get_command(&self) -> Option<Command> {
        match self.step {
            0 => None,
            step => Some(self.commands[step - 1]),
        }
    }

    fn increment_addr(&mut self) -> Result<(), CpuError> {
        // try to increment the address
        if self.addr < self.memory.len() {
            self.addr += 1;
            Ok(())
        } else {
            Err(CpuError::OutOfMemory)
        }
    }

    fn decrement_addr(&mut self) -> Result<(), CpuError> {
        // try to decrement the address
        if let Some(addr) = self.addr.checked_sub(1) {
            self.addr = addr;
            Ok(())
        } else {
            Err(CpuError::NegativeAddr)
        }
    }

    fn increment_addr_if_store_set(&mut self) -> Result<(), CpuError> {
        // conditionally incrementif store is set
        if self.store {
            self.increment_addr()
        } else {
            Ok(())
        }
    }

    fn decrement_addr_if_store_set(&mut self) -> Result<(), CpuError> {
        // conditionally decrement if store is set
        if self.store {
            self.decrement_addr()
        } else {
            Ok(())
        }
    }

    fn invert_bit(&mut self) {
        // invert a single bit
        let bit = self.memory[self.addr];
        self.memory.set(self.addr, !bit);
    }

    fn load_bit_to_store(&mut self) {
        // TODO: FIX
        // load a bit into the store
        self.store = self.memory[self.addr]
    }

    fn unload_bit_from_store(&mut self) {
        // TODO: FIX
        // load a bit into the store
        self.store = self.memory[self.addr]
    }
}
