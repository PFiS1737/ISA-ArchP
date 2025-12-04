#![allow(clippy::unusual_byte_groupings)]
#![feature(if_let_guard)]

mod assembler;
mod cli;
mod instructions;
mod pass1;
mod pass2;
mod pseudo_instructions;

use std::{fs::read_to_string, io::stdout};

use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::{assembler::Assembler, cli::Cli};

fn main() {
    let cli = Cli::parse();

    if let Some(shell) = cli.complete {
        return generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_BIN_NAME"),
            &mut stdout(),
        );
    }

    let source_lines = read_to_string(&cli.src_file)
        .expect("Failed to read source file")
        .lines()
        .map(|s| s.to_string())
        .collect();

    for n in Assembler::new(source_lines)
        .assemble()
        .expect("Assembly failed")
    {
        println!("0x{:08X}", n);
        println!("0b{:032b}", n);
    }
}
