A language VM built in Rust.

Based on [this](https://blog.subnetzero.io/post/building-language-vm-part-00/) tutorial by Fletcher Haynes, and what I'm learning in the [TOL304G Programming Language](https://ugla.hi.is/kennsluskra/index.php?tab=nam&chapter=namskeid&id=08713320196&namskra=0) and [TOL309G Computer Architecture](https://ugla.hi.is/kennsluskra/index.php?tab=nam&chapter=namskeid&id=70973020196&namskra=0) courses at the University of Iceland.


## Assembler

A simple assembler is provided to generate machine code.

### Instructions

| Instruction      | Description                                      |
|-                 |-                                                 |
| `hlt`            | Halts execution                                  |
| `load r i`       | Loads value `i` into `r`                         |
| `add r1 r2 r3`   | Adds `r1` and `r2` and outputs to `r3`           |
| `sub r1 r2 r3`   | Subtracts `r2` from `r1` and outputs to `r3`     |
| `mul r1 r2 r3`   | Multiplies `r1` by `r2` and outputs to `r3`      |
| `div r1 r2 r3`   | Divides `r1` by `r2` and outputs to `r3`         |
| `jmp i`          | Jumps to byte `i` in program                     |
| `jmpb i`         | Jumps back `i` bytes in program                  |
| `jmpf i`         | Jumps forward `i` bytes in program               |
| `jmpc i`         | Jumps to byte `i` in program if condition is met |
| `eq r1 r2`       | Checks if `r1` is equal to `r2`                  |
| `neq r1 r2`      | Checks if `r1` is not equal to `r2`              |
| `gt r1 r2`       | Checks if `r1` > `r2`                            |
| `lt r1 r2`       | Checks if `r1` < `r2`                            |
| `gtq r1 r2`      | Checks if `r1` >= `r2`                           |
| `ltq r1 r2`      | Checks if `r1` <= `r2`                           |
| `nop`            | A no-op                                          |

An integer value is denoted by a hash symbol and one or more digits (`#123`).

A register address is denoted by a dollar symbol and a digit from 0 to 31 (`$10`).

