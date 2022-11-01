use bitvec::prelude::*; 

const DEFAULT_MEM_SIZE: usize = 1024;

#[derive(Debug, Copy, Clone)]
enum Command {
    INC,
    INV,
    LOAD,
    CDEC
}

struct CPU {
    memory: BitVec,
    addr: usize,
    store: bool,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            memory: bitvec![0; DEFAULT_MEM_SIZE],
            addr: 0,
            store: false,
        }
    }

    fn reset(&mut self) {
        self.memory = bitvec![0; DEFAULT_MEM_SIZE];
        self.addr = 0;
        self.store = false;
    }

    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::INC => {
                // increment the address and expand memory if needed
                self.addr += 1;
                let mem_len = self.memory.len();
                if self.addr > mem_len {
                    self.memory.resize(mem_len * 2, false);
                }
            },
            Command::INV => {
                // invert a single bit
                let bit = self.memory[self.addr];
                self.memory.set(self.addr, !bit);
            },
            Command::LOAD => {
                // load a bit into the store
                self.store = self.memory[self.addr]
            },
            Command::CDEC => {
                // conditionally decrement if store is set
                if self.store {
                    if let Some(addr) = self.addr.checked_sub(1) {
                        self.addr = addr;
                    } else {
                        panic!("attempted to CDEC past 0")
                    }
                }
            },
        }
    }

    fn run(&mut self, program: &[Command]) {
        self.reset();

        for cmd in program {
            self.exec(cmd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_works() {
        let mut cpu = CPU::new();
        cpu.memory.set(0, true);
        cpu.addr = 5;
        cpu.store = true;

        cpu.reset();
        assert_eq!(cpu.memory[0], false);
        assert_eq!(cpu.addr, 0);
        assert_eq!(cpu.store, false);
    }

    #[test]
    fn test_inc_works() {
        let mut cpu = CPU::new();

        cpu.exec(&Command::INC);
        assert_eq!(cpu.addr, 1);
    }

    #[test]
    fn test_inc_expands_memory() {
        let mut cpu = CPU::new();

        for _ in 0..(DEFAULT_MEM_SIZE * 2) {
            cpu.exec(&Command::INC);
        }
        assert_eq!(cpu.addr, DEFAULT_MEM_SIZE * 2);
        assert!(cpu.memory.capacity() > DEFAULT_MEM_SIZE);
    }

    #[test]
    fn test_inv_works() {
        let mut cpu = CPU::new();

        cpu.exec(&Command::INV);
        assert_eq!(cpu.memory[0], true);
    }

    #[test]
    fn test_load_works() {
        let mut cpu = CPU::new();
        cpu.memory.set(0, true);

        cpu.exec(&Command::LOAD);
        assert_eq!(cpu.store, true);
    }

    #[test]
    fn test_cdec_works() { 
        let mut cpu = CPU::new();
        cpu.store = true;
        cpu.addr = 1;

        cpu.exec(&Command::CDEC);
        assert_eq!(cpu.addr, 0);
    }

    #[test]
    fn test_cdec_skips_when_unset() {
        let mut cpu = CPU::new();
        cpu.addr = 1;

        cpu.exec(&Command::CDEC);
        assert_eq!(cpu.addr, 1);
    }

    #[test]
    fn test_full_program_runs() {
        let mut cpu = CPU::new();
        let program = [
            Command::INC,
            Command::INV,
            Command::LOAD,
            Command::CDEC,
            Command::INV
        ];

        cpu.run(&program);
        assert_eq!(cpu.memory[0], true);
        assert_eq!(cpu.memory[1], true);
    }
}
