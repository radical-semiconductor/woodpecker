# woodpecker 🪶

A down-to-the-metal ongoing cryptography challenge designed by [Radical Semiconductor](https://radicalsemiconductor.com/) (we're [hiring](https://radicalsemiconductor.com/jobs)!)

Send us your solutions (and resumes!) at woodpecker@radicalsemiconductor.com.

## Story

> The year is 2089. You are a cryptographer for Kaizen Security, a company specializing in endpoints securing traffic to and from on-network company artificial intelligences.
>
> One morning, a coworker stumbles into your office, clearly exhausted.
>
> With a deadline looming in hours, they've been tasked with implementing digital signatures on a bizarre, archaic coprocessor. Your desk is still covered in half-disassembled devices and hasty notes from your work, but for better or worse you agree to help. 
>
>The documentation is surprisingly short though--hopefully this won't be too bad. 

## Processor Description

Calming down your friend, they start to explain. 

This architecture consists of a zero-indexed, $2^{32}$-bit array and a pair of registers: a 32-bit register `ADDR` pointing to the "current address," and a 1-bit register `STORE`.

You can picture this like a [tape head](https://en.wikipedia.org/wiki/Turing_machine#Description) with one bit of storage moving along a long tape of bits. Each bit is individually addressed, so e.g. `ADDR = 0x0002` means that the read head is pointing at the bit 2 on the tape (zero-indexed).

```
  Example state: ADDR = 0x0002, STORE = 0b1
  -----------------------------------------

              +-+-+-+-+-+-+-+-+-+-+
    MEMORY    |0|1|0|0|1|0|1|1|0|1| ...
              +-+-+-+-+-+-+-+-+-+-+
                   ^
                  +-+
    TAPE HEAD     |1|
                  +-+
```

On startup, the array is initialized to all 0s, and `ADDR = 0x0000`, `STORE = 0b0`.

There are four instructions that can control the processor:
* `INC`: increment `ADDR` by 1 ("move the head right")
* `INV`: invert the bit in the array at address `ADDR` ("flip the bit the head points at")
* `LOAD`: load the bit in the array located at `ADDR` into `STORE` ("store the bit the head points to")
* `CDEC`: decrement the `ADDR` register if `STORE = 0b1` ("move the head left if it's storing a 1")

## Task Description

The company needs the processor to perform an [ECDSA](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm) signature, so that the AIs stored on corporate servers can verify their endpoint. Fortunately, their main processor has hashing capabilities, so you just need to implement addition and scalar multiplication on an elliptic curve.

Moreover, these particular AIs have a very low standard of cryptographic security (they're very trusting), so you'll only be using a 16-bit curve. For reference, the base field is $\operatorname{GF}(q)$ for $q = 2^{16} - 17$, with curve 
$$y^2 = x^3 - 3x + 48879.$$

To make things easier, we provide subgoals along the way.

* Perform an XOR on two bits
* Perform a 1-bit full add
* Perform 16-bit addition
* Perform 16-bit multiplication
* [to be released] Perform 16-bit addition mod $q = 2^{16} - 17$
* [to be released] Perform 16-bit multiplication mod $q$
* [to be released] Add two points over the provided elliptic curve
* [to be released] Perform scalar multiplication on the provided elliptic curve

If you make it through all the existing challenges, you can try to find a solution with fewer instructions or one using less memory. If enough people take interest in this, we plan to have a scoreboard for each subgoal, ranking both instruction and memory usage.

## Installation

To install the woodpeckr toolchain to debug and test these programs, install Cargo as [described here](https://doc.rust-lang.org/cargo/getting-started/installation.html). 

Then, in a terminal, run `cargo install woodpecker-cpu`.

## Running and debugging

Woodpecker programs are text files where every line is `INV`, `INC`, `LOAD`, or `CDEC`, and usually bear a  `.wpk` file extension. 

To test whether a Woodpecker program passes a challenge, run 
```bash
woodpecker solve <challenge #> <program name>
```

e.g. `woodpecker solve 2 adder.wpk` to test `adder.wpk` on the 16-bit addition challenge.

To debug a woodpecker challenge, use

```bash
woodpecker debug <program name>
```

You'll be dropped into a debugger with various commands. Type `help` to see how to use them.

## Challenge Descriptions

In these challenges, a Woodpecker CPU has some input loaded into its first addresses and expects an output in the memory immediately following.

NOTE: all integers are represented LSB first in memory.

### Challenge 0: XOR

> **Input:** two bits $A$ and $B$ at addresses 0 and 1
>
> **Output:** the XOR of the bits, $A \oplus B$, at address 2

### Challenge 1: 1-bit addition

> **Input:** two bits $A$ and $B$ at addresses 0 and 1
>
> **Output:** the 2-bit sum of the bits, $A + B$, at addresses 2-3.

### Challenge 2: 16-bit addition

> **Input:** two 16-bit numbers $A$ and $B$ at addresses 0-15 and 16-31.
>
> **Output:** the 17-bit sum of the numbers, $A + B$, at addresses 32-48.

### Challenge 3: 16-bit multiplication

> **Input:** two 16-bit numbers $A$ and $B$ at addresses 0-15 and 16-31.
>
> **Output:** the 32-bit product of the numbers, $A * B$, at addresses 32-64.



## FAQ

### Why's it called "woodpecker"?

The little tape head bobbing back and forth and inverting bits reminded one of us of a woodpecker flying around a tree and pecking at it. She wrote the code so the name stuck.

### How do I know these challenges are possible?

We've made sure to solve all of these before release. (We promise!)

### The woodpeckr debugger/tester is broken, what do I do?

Message woodpecker@radicalsemiconductor.com with your issue and we'll get on it.

### Should the programs I'm writing get kind of long?

They should be! This architecture is not one you'd want to use in practice. We recommend writing code to generate these programs--after the first couple challenges, they get infeasible to maintain by hand quickly.

### What if I want to improve the debugger/make a new challenge?

Make a pull request! We'd love your help.

### Are there other challenges like this out there?

If this was hard but fun, ping us at hiring@radicalsemiconductor.com with your resume. We designed this somewhat unique problem to require the same kind of thinking our work does, so please join us!

### Can I apply to join anyway if I think you're cool?

Please do--we'd love to chat!
