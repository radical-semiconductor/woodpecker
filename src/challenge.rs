use anyhow::Result;
use bitvec::prelude::*;
use rand::Rng;
use thiserror::Error;

use crate::cpu::{Command, Cpu};

const NUM_TRIALS: u8 = 100;

#[derive(Debug, Error)]
pub enum ChallengeError {
    #[error("no such challenge exists")]
    InvalidChallenge,
}

pub fn get_xor_problem() -> (BitVec<u8>, BitVec<u8>) {
    let mut rng = rand::thread_rng();

    let input_mem: BitVec<u8> = (0..2).map(|_| rng.gen::<bool>()).collect();
    let mut output_mem = BitVec::new();
    output_mem.push(input_mem[0] != input_mem[1]);

    (input_mem, output_mem)
}

pub fn get_one_bit_add_problem() -> (BitVec<u8>, BitVec<u8>) {
    let mut rng = rand::thread_rng();

    let mut input_mem = bitvec![u8, Lsb0; 0; 2];
    let mut output_mem = bitvec![u8, Lsb0; 0; 2];

    let a: u8 = rng.gen_range(0..=1);
    let b: u8 = rng.gen_range(0..=1);
    let sum: u8 = a + b;

    input_mem[0..1].store::<u8>(a);
    input_mem[1..2].store::<u8>(b);
    output_mem.store::<u8>(sum);

    (input_mem, output_mem)
}

pub fn get_full_add_problem() -> (BitVec<u8>, BitVec<u8>) {
    let mut rng = rand::thread_rng();

    let mut input_mem = bitvec![u8, Lsb0; 0; 32];
    let mut output_mem = bitvec![u8, Lsb0; 0; 17];

    let a: u16 = rng.gen();
    let b: u16 = rng.gen();

    input_mem[0..16].store::<u16>(a);
    input_mem[16..32].store::<u16>(b);
    output_mem.store::<u32>(a as u32 + b as u32);

    (input_mem, output_mem)
}

pub fn get_multiply_problem() -> (BitVec<u8>, BitVec<u8>) {
    let mut rng = rand::thread_rng();

    let mut input_mem = bitvec![u8, Lsb0; 0; 32];
    let mut output_mem = bitvec![u8, Lsb0; 0; 32];

    let a: u16 = rng.gen();
    let b: u16 = rng.gen();

    input_mem[0..16].store::<u16>(a);
    input_mem[16..32].store::<u16>(b);
    output_mem.store::<u32>(a as u32 * b as u32);

    (input_mem, output_mem)
}

pub fn evaluate_solution(challenge: u8, commands: &[Command]) -> Result<()> {
    let mut cpu = Cpu::new(commands);

    let get_problem: fn() -> (BitVec<u8>, BitVec<u8>) = match challenge {
        0 => get_xor_problem,
        1 => get_one_bit_add_problem,
        2 => get_full_add_problem,
        3 => get_multiply_problem,
        _ => return Err(ChallengeError::InvalidChallenge.into()),
    };

    println!("\nRunning {} trials...", NUM_TRIALS);

    for trial in 0..NUM_TRIALS {
        let (input_mem, output_mem) = get_problem();

        cpu.reset();
        cpu.memory[..input_mem.len()].copy_from_bitslice(&input_mem);

        for _ in 0..commands.len() {
            cpu.step();
        }

        let cpu_output_mem = &cpu.memory[input_mem.len()..(input_mem.len() + output_mem.len())];
        if cpu_output_mem != output_mem {
            println!("\nProblem failed on trial #{}!\n", trial + 1);
            println!("> Input was:           {}", input_mem);
            println!("> Actual output was:   {}", cpu_output_mem);
            println!("> Expected output was: {}\n", output_mem);
            return Ok(());
        }
    }

    println!("\nChallenge complete! 🎉\n");

    Ok(())
}
