# woodpecker 🪶

A down-to-the-metal cryptography challenge designed by Radical Semiconductor.

## Story

> The year is 2089. You are a cryptographer for Kaizen Security, a company specializing in endpoints securing traffic to and from on-network company artificial intelligences.
>
> One morning, a coworker stumbles into your office, clearly exhausted.
>
> With a deadline looming in hours, they've been tasked with writing signature routines for a bizarre, archaic crypto coprocessor. Your desk is still covered in half-disassembled devices and hasty notes from your work, but for better or worse you agree to help. 
>
>The documentation is surprisingly short though--hopefully this won't be too bad. 

## Details

This architecture consists of a zero-indexed, `2^64` bit array and a pair of registers: a 64-bit register `ADDR`, and a 1-bit register `STORE`.

On startup, the array is initialized to all 0s, and `ADDR` and `STORE` are both 0. There are four instructions that can control the system:
* `INC`: increment `ADDR` by 1
* `INV`: invert the bit in the array at address `ADDR`
* `LOAD`: load the bit in the array located at `ADDR` into `STORE`
* `CDEC`: decrement the `ADDR` register if `STORE` is 1

The company needs to perform an EdDSA signature over the `Ed25519` curve. Fortunately, their main processor has a SHA-512 core in it, so you just need to implement point multiplication on Ed25519.

To make things easier, we provide subgoals along the way:

* Perform 255-bit addition
* Perform 255-bit addition modulo q = 2^255 - 19
* Perform 255-bit multiplication modulo q = 2^255 - 19
* Perform point addition on Ed25519
* Perform point doubling on Ed25519
* Perform point multiplication on Ed25519

If you make it to the end, you can score additional points by writing your code with fewer instructions. 

We plan to eventually have a scoreboard with instruction counts on our website.

## Running and debugging

