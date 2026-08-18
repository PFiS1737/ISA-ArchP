mod command;
mod utils;

use std::{
    collections::HashMap,
    fs::{File, read_to_string},
    io::{BufWriter, Write, stdout},
};

use anyhow::{Result, anyhow};
use archp_assembler::{Assembler, AssemblerSettings, fmt_line};
use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

use crate::{command::Cli, utils::align_tabbed_lines};

fn main() -> Result<()> {
    CompleteEnv::with_factory(Cli::command)
        .var("ARCHP_AS_COMPLETE")
        .complete();

    env_logger::init();

    let cli = Cli::parse();

    let settings = AssemblerSettings {
        disable_macro: cli.disable_macro,
    };

    let file_content = read_to_string(&cli.src_file)
        .map_err(|e| anyhow!("Can't read source file '{}': {}", cli.src_file, e))?;

    let asmblr = Assembler::new(settings);
    let context = asmblr.assemble(&file_content)?;

    let mut out = BufWriter::new(if cli.stdout {
        Box::new(stdout()) as Box<dyn Write>
    } else {
        Box::new(File::create(cli.out_file)?) as Box<dyn Write>
    });

    if cli.hex {
        let labels = context
            .labels
            .iter()
            .map(|(k, v)| (v, k))
            .collect::<HashMap<_, _>>();

        let displays = context
            .instrs
            .into_iter()
            .enumerate()
            .map(|(idx, instr)| {
                let mut display = context.source_map[idx].1.to_string();

                if let Some((name, ops)) = instr {
                    let line = fmt_line(name, &ops);
                    if line != display {
                        display = format!("{line}\t[{display}]");
                    } else {
                        display += "\t"
                    }
                } else {
                    display = format!("\t[{display}]");
                }

                if let Some(label) = labels.get(&(idx * 4)) {
                    display = format!("{display}\t<label: {label}>");
                } else {
                    display += "\t";
                }

                display
            })
            .collect::<Vec<_>>();

        for (code, display) in context.codes.into_iter().zip(align_tabbed_lines(&displays)) {
            writeln!(out, "{:#010X} # {}", code, display)?;
        }
    } else {
        for code in context.codes {
            out.write_all(&code.to_le_bytes())?;
        }
    }

    Ok(())
}
