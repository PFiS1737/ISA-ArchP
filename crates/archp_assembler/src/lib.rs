#![feature(decl_macro)]

mod assembler;
mod context;
mod directives;
mod encoder;
mod expression;
mod instructions;
mod operand;
mod parser;
mod pass1;
mod pass2;
mod pseudo_instructions;
mod relocation;
mod utils;

#[cfg(feature = "macros")]
mod macro_instructions;

pub use assembler::{Assembler, AssemblerSettings};
pub use utils::fmt::fmt_line;
