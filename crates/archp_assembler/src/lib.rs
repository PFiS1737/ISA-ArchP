#![feature(decl_macro)]
#![allow(clippy::unusual_byte_groupings)]

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
mod testkit;

pub use assembler::{Assembler, AssemblerSettings};
pub use utils::fmt::fmt_line;
