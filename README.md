# ArchP

A simple instruction set architecture (ISA) that operates within the game [Turing Complete](https://store.steampowered.com/app/1444480/Turing_Complete/).

This repository contains its corresponding assembler.

## ISA

For details of the instruction set, see the file [isa.txt](./isa.txt).

### Features

- All instructions support conditional execution (predication)
- Up to a 12-bit immediate value can be used directly in arithmetic instructions
  - For larger immediates, use `lui` to load the upper 20 bits into a register, then `ori` with a lower 12-bit immediate
- `b-*` branch instructions support 12-bit absolute PC jumps
- The `call` instruction supports 12-bit absolute PC jumps
- `lw`/`sw` allow relative addressing with a positive 12-bit offset
- The `rnd` instruction generates pseudo-random numbers based on system time (sandbox mode only)
- Pixel-display–related instructions and dedicated registers

### Known Issues

- Signed integers are generally not supported
- PC-relative jumps are not directly supported, but a pseudo-register for the PC is provided
- `b-*` branch instructions cannot compare against an immediate; load the value into a register first
