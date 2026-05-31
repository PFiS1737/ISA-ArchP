#![feature(decl_macro)]

mod assembler;
mod instructions;
mod macro_instructions;
mod operand;
mod parser;
mod pass1;
mod pass2;
mod pseudo_instructions;
mod utils;

#[cfg(test)]
mod diff_testing;
#[cfg(test)]
mod testkit;

pub use assembler::{Assembler, AssemblerSettings};
