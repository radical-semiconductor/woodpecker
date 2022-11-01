use bitvec::prelude::*; 

const DEFAULT_MEM_SIZE: usize = 1024;

#[derive(Debug, Copy, Clone)]
enum Command {
    Inc,
    Inv,
    Load,
    Cdec
}

struct Woodpecker {
    memory: BitVec,
    addr: usize,
    store: bool,
}

impl Woodpecker {
    fn new() -> Woodpecker {
        Woodpecker {
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

    fn increment_addr(&mut self) {
        // increment the address and expand memory if needed
        self.addr += 1;

        let mem_len = self.memory.len();
        if self.addr > mem_len {
            self.memory.resize(mem_len * 2, false);
        }
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

    fn decrement_addr_if_store_set(&mut self) {
        // conditionally decrement if store is set
        if self.store {
            if let Some(addr) = self.addr.checked_sub(1) {
                self.addr = addr;
            } else {
                panic!("attempted to Cdec past 0")
            }
        }
    }

    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::Inc => self.increment_addr(),
            Command::Inv => self.invert_bit(),
            Command::Load => self.load_bit_to_store(),
            Command::Cdec => self.decrement_addr_if_store_set(),
        }
    }

    pub fn run(&mut self, program: &[Command]) {
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
        let mut cpu = Woodpecker::new();
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
        let mut cpu = Woodpecker::new();

        cpu.exec(&Command::Inc);
        assert_eq!(cpu.addr, 1);
    }

    #[test]
    fn test_inc_expands_memory() {
        let mut cpu = Woodpecker::new();

        for _ in 0..(DEFAULT_MEM_SIZE * 2) {
            cpu.exec(&Command::Inc);
        }
        assert_eq!(cpu.addr, DEFAULT_MEM_SIZE * 2);
        assert!(cpu.memory.capacity() > DEFAULT_MEM_SIZE);
    }

    #[test]
    fn test_inv_works() {
        let mut cpu = Woodpecker::new();

        cpu.exec(&Command::Inv);
        assert_eq!(cpu.memory[0], true);
    }

    #[test]
    fn test_load_works() {
        let mut cpu = Woodpecker::new();
        cpu.memory.set(0, true);

        cpu.exec(&Command::Load);
        assert_eq!(cpu.store, true);
    }

    #[test]
    fn test_cdec_works() { 
        let mut cpu = Woodpecker::new();
        cpu.store = true;
        cpu.addr = 1;

        cpu.exec(&Command::Cdec);
        assert_eq!(cpu.addr, 0);
    }

    #[test]
    fn test_cdec_skips_when_unset() {
        let mut cpu = Woodpecker::new();
        cpu.addr = 1;

        cpu.exec(&Command::Cdec);
        assert_eq!(cpu.addr, 1);
    }

    #[test]
    fn test_full_program_runs() {
        let mut cpu = Woodpecker::new();
        let program = [
            Command::Inc,
            Command::Inv,
            Command::Load,
            Command::Cdec,
            Command::Inv
        ];

        cpu.run(&program);
        assert_eq!(cpu.memory[0], true);
        assert_eq!(cpu.memory[1], true);
    }
}