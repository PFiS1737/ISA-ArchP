#![allow(clippy::unusual_byte_groupings)]
#![feature(if_let_guard)]

mod assembler;
mod cli;
mod instructions;
mod pass1;
mod pass2;
mod pseudo_instructions;

use std::{fs::read_to_string, io::stdout, process::ExitCode};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::{assembler::Assembler, cli::Cli};

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    if let Some(shell) = cli.complete {
        generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_BIN_NAME"),
            &mut stdout(),
        );
        return Ok(ExitCode::SUCCESS);
    }

    let Some(src_file) = cli.src_file else {
        unreachable!()
    };

    let source_lines = read_to_string(src_file)?
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut out = cli.output.get()?;

    for (code, line) in Assembler::new(source_lines).assemble()? {
        writeln!(out, "0x{:08X} # {}", code, line)?;
    }

    Ok(ExitCode::SUCCESS)
}
