mod cli;
mod commands;
mod utils;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            file,
            asm,
            trace,
            console,
            sim_args: args,
        } => {
            commands::run::run(file, asm, trace, console, args)?;
        },
        Command::Trace { file } => {
            commands::trace::trace(file, ".".into())?;
        },
    }

    Ok(())
}
