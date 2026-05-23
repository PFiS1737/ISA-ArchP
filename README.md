# ArchP

A simple instruction set architecture (ISA) that operates within the game [Turing Complete](https://store.steampowered.com/app/1444480/Turing_Complete/).

This repository contains its corresponding assembler.

## Schematics

> [!NOTE]
> The schematic may not be the latest version; please check the commit list to locate the most up-to-date implementation.  
> Alternatively, you may just get a fixed version from the git tags.

See [schematics](./schematics).

## Project Structure

- Rust Crates:
  - `archp_assembler`: the assembler for ArchP.
- _WIP_

## TODO

- drop Turing Complete dependency.
- CPU: Verilog implementation of the ArchP CPU.
  - Remove the workaround for working in the game.
  - Corresponding ISA also needs to be modified, removing some unnecessary instructions and optimizing instruction encoding.
  - Add privileged architecture.
  - Use Verilator for top-level encapsulation and implement peripheral device simulation.
- ELF: a simple ELF file format for ArchP.
- linker: a custom linker for the ELF.
- OS: a simple operating system kernel.
