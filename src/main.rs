#![allow(clippy::unusual_byte_groupings)]
#![feature(if_let_guard)]
#![feature(decl_macro)]
#![feature(new_range_api)]

mod assembler;
mod cli;
mod instructions;
mod operand_types;
mod pass1;
mod pass2;
mod pseudo_instructions;
#[cfg(test)]
mod testkit;
mod utils;

use std::{
    fs::read_to_string,
    io::{BufWriter, Write, stdout},
};

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::{
    assembler::Assembler,
    cli::{Cli, Output},
    utils::align_tabbed_lines,
};

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    if let Some(shell) = cli.complete {
        generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_BIN_NAME"),
            &mut stdout(),
        );
        return Ok(());
    }

    let Some(src_file) = cli.src_file else {
        unreachable!()
    };

    let source_lines = read_to_string(src_file)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    if matches!(cli.output, Output::Stdout) && cli.bin {
        bail!("Cannot write binary output to stdout.");
    }

    let mut out = BufWriter::new(cli.output.get()?);

    let asmblr = Assembler::new(source_lines);
    let (codes, displays) = asmblr.assemble()?;

    for (code, display) in codes.iter().zip(align_tabbed_lines(&displays)) {
        if cli.bin {
            out.write_all(&code.to_be_bytes())?;
        } else {
            writeln!(out, "0x{:08X} # {}", code, display)?;
        }
    }

    Ok(())
}
