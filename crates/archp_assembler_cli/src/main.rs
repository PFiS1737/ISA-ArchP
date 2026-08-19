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

use crate::{
    command::Cli,
    utils::{align_tabbed_lines, merge_maps},
};

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
        let lines = merge_maps(
            context.instrs,
            context.labels.into_iter().map(|(k, v)| (v, k)),
        );

        let displays = align_tabbed_lines(
            lines
                .into_iter()
                .map(|(offset, (instr, label))| {
                    let mut display = instr
                        .flatten()
                        .map(|(name, ops)| fmt_line(name, &ops))
                        .unwrap_or("".to_string());

                    if let Some(label) = label {
                        display = format!("{display}\t<label: {label}>");
                    } else {
                        display += "\t";
                    }

                    (offset, display)
                })
                .collect::<HashMap<_, _>>(),
        );

        for (idx, code) in context
            .text
            .as_chunks::<4>()
            .0
            .iter()
            .map(|x| u32::from_le_bytes(*x))
            .enumerate()
        {
            writeln!(
                out,
                "{:#010X} # {}",
                code,
                displays.get(&(idx * 4)).unwrap_or(&"".to_string())
            )?;
        }
    } else {
        out.write_all(&context.text)?;
    }

    Ok(())
}
