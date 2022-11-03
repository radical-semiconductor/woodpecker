use bitvec::prelude::*; 
use crate::error::{CpuError, CpuErrorKind};

#[derive(Debug, Copy, Clone)]
pub enum Command {
    Inc,
    Inv,
    Load,
    Cdec
}

#[derive(Debug)]
pub struct Cpu {
    pub memory: BitVec,
    pub addr: usize,
    store: bool,
    step: usize,
}

impl Cpu {
    pub fn new(size: usize) -> Cpu {
        Cpu {
            memory: bitvec![0; size],
            addr: 0,
            store: false,
            step: 0,
        }
    }

    fn increment_addr(&mut self) -> Result<(), CpuError> {
        // try to increment the address
        if self.addr < self.memory.len() {
            self.addr += 1;
            Ok(())
        } else {
            let err = CpuError {
                kind: CpuErrorKind::OutOfMemory,
                step: self.step,
            };

            Err(err)
        }
    }

    fn decrement_addr(&mut self) -> Result<(), CpuError> {
        // try to decrement the address
        if let Some(addr) = self.addr.checked_sub(1) {
            self.addr = addr;
            Ok(())
        } else {
            let err = CpuError {
                kind: CpuErrorKind::NegativeAddr,
                step: self.step,
            };

            Err(err)
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

    pub fn forward(&mut self, cmd: &Command) -> Result<(), CpuError> {
        self.step += 1;

        match cmd {
            Command::Inc => self.increment_addr()?,
            Command::Inv => self.invert_bit(),
            Command::Load => self.load_bit_to_store(),
            Command::Cdec => self.decrement_addr_if_store_set()?,
        }

        Ok(())
    }

    pub fn backward(&mut self, cmd: &Command) -> Result<(), CpuError> {
        self.step -= 1;
        
        match cmd {
            Command::Inc => self.decrement_addr()?,
            Command::Inv => self.invert_bit(),
            Command::Load => self.unload_bit_from_store(),
            Command::Cdec => self.increment_addr_if_store_set()?,
        }

        Ok(())
    }
}