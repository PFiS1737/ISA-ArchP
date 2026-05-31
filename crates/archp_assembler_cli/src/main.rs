mod command;
mod utils;

use std::{
    fs::read_to_string,
    io::{BufWriter, Write, stdout},
};

use anyhow::{Result, bail};
use archp_assembler::{Assembler, AssemblerSettings};
use clap::{CommandFactory, Parser};
use clap_complete::generate;

use crate::{
    command::{Cli, Output},
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

    let settings = AssemblerSettings {
        disable_macro: cli.disable_macro,
    };

    let asmblr = Assembler::new(settings, source_lines);
    let (codes, displays) = asmblr.assemble()?;

    let mut out = BufWriter::new(cli.output.get()?);

    for (code, display) in codes.iter().zip(align_tabbed_lines(&displays)) {
        if cli.bin {
            out.write_all(&code.to_le_bytes())?;
        } else {
            writeln!(out, "0x{:08X} # {}", code, display)?;
        }
    }

    Ok(())
}
