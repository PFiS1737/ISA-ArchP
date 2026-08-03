# ArchP

A simple instruction set architecture (ISA) and its infrastructure.

## Project Structure

- Rust Crates:
  - `archp_assembler`: The assembler.
  - `archp_assembler_cli`: The command-line interface for the assembler.
  - `archp_simulator`: The simulator.

## TODO

- CPU: Verilog implementation of the ArchP CPU.
  - Add privileged architecture.
- ELF: a simple ELF file format for ArchP.
- linker: a custom linker for the ELF.
- OS: a simple operating system kernel.
