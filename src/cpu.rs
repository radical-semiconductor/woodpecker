use bitvec::prelude::*; 
use crate::error::{CpuError, CpuErrorKind};

const DEFAULT_MEM_SIZE: usize = 1024;


#[derive(Debug, Copy, Clone)]
pub enum Command {
    Inc,
    Inv,
    Load,
    Cdec
}

pub struct Processor {
    memory: BitVec,
    mem_usage: usize,
    addr: usize,
    store: bool,
    step: usize,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            memory: bitvec![0; DEFAULT_MEM_SIZE],
            mem_usage: 1,
            addr: 0,
            store: false,
            step: 0,
        }
    }

    fn increment_addr(&mut self) -> std::result::Result<(), CpuError> {
        // try to increment the address
        if let Some(addr)= self.addr.checked_add(1) {
            self.addr = addr
        } else {
            let err = CpuError {
                kind: CpuErrorKind::OutOfMemory,
                step: self.step,
            };

            return Err(err);
        }

        // expand memory if necessary by 2x
        let mem_len = self.memory.len();
        if self.addr > mem_len {
            self.memory.resize(mem_len * 2, false);
        }

        // if we've never been this high, update memory size
        let mem_usage = self.addr + 1;
        if mem_usage > self.mem_usage {
            self.mem_usage = mem_usage;
        }

        Ok(())
    }

    fn invert_bit(&mut self) {
        // invert a single bit
        let bit = self.memory[self.addr];
        self.memory.set(self.addr, !bit);
    }

    fn load_bit_to_store(&mut self) {
        // load a bit into the store
        self.store = self.memory[self.addr]
    }

    fn decrement_addr_if_store_set(&mut self) -> std::result::Result<(), CpuError> {
        // conditionally decrement if store is set
        if self.store {
            if let Some(addr) = self.addr.checked_sub(1) {
                self.addr = addr;
            } else {
                let err = CpuError {
                    kind: CpuErrorKind::NegativeAddr,
                    step: self.step,
                };

                return Err(err);
            }
        }

        Ok(())
    }

    pub fn exec(&mut self, cmd: &Command) -> std::result::Result<(), CpuError> {
        self.step += 1;

        match cmd {
            Command::Inc => self.increment_addr()?,
            Command::Inv => self.invert_bit(),
            Command::Load => self.load_bit_to_store(),
            Command::Cdec => self.decrement_addr_if_store_set()?,
        }

        Ok(())
    }

    pub fn dump(&mut self) -> &BitSlice {
        &self.memory[..self.mem_usage]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_works() {
        let mut cpu = Processor::new();

        cpu.exec(&Command::Inc);
        assert_eq!(cpu.addr, 1);
    }

    #[test]
    fn test_inc_expands_memory() {
        let mut cpu = Processor::new();

        for _ in 0..(DEFAULT_MEM_SIZE * 2) {
            cpu.exec(&Command::Inc);
        }
        assert_eq!(cpu.addr, DEFAULT_MEM_SIZE * 2);
        assert!(cpu.memory.capacity() > DEFAULT_MEM_SIZE);
    }

    #[test]
    fn test_inv_works() {
        let mut cpu = Processor::new();

        cpu.exec(&Command::Inv);
        assert_eq!(cpu.memory[0], true);
    }

    #[test]
    fn test_load_works() {
        let mut cpu = Processor::new();
        cpu.memory.set(0, true);

        cpu.exec(&Command::Load);
        assert_eq!(cpu.store, true);
    }

    #[test]
    fn test_cdec_works() { 
        let mut cpu = Processor::new();
        cpu.store = true;
        cpu.addr = 1;

        cpu.exec(&Command::Cdec);
        assert_eq!(cpu.addr, 0);
    }

    #[test]
    fn test_cdec_skips_when_unset() {
        let mut cpu = Processor::new();
        cpu.addr = 1;

        cpu.exec(&Command::Cdec);
        assert_eq!(cpu.addr, 1);
    }
}