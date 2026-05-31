mod command;
mod utils;

use std::{
    fs::read_to_string,
    io::{BufWriter, Write},
};

use anyhow::{Result, bail};
use archp_assembler::{Assembler, AssemblerSettings};
use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

use crate::{
    command::{Cli, Output},
    utils::align_tabbed_lines,
};

fn main() -> Result<()> {
    CompleteEnv::with_factory(Cli::command)
        .var("ARCHP_COMPLETE")
        .complete();

    env_logger::init();

    let cli = Cli::parse();

    if matches!(cli.output, Output::Stdout) && !cli.hex {
        bail!("You must specify an output file, or use --hex to output formatted hex to <stdout>.")
    }

    let settings = AssemblerSettings {
        disable_macro: cli.disable_macro,
    };

    let asmblr = Assembler::new(settings);
    let (codes, displays) = asmblr.assemble(read_to_string(cli.src_file)?.lines())?;

    let mut out = BufWriter::new(cli.output.get()?);

    for (code, display) in codes.iter().zip(align_tabbed_lines(&displays)) {
        if cli.hex {
            writeln!(out, "0x{:08X} # {}", code, display)?;
        } else {
            out.write_all(&code.to_le_bytes())?;
        }
    }

    Ok(())
}
