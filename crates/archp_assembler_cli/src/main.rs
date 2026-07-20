mod command;
mod utils;

use std::{
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
    let (codes, lines) = asmblr.assemble(file_content.lines())?;

    let mut out = BufWriter::new(if cli.stdout {
        Box::new(stdout()) as Box<dyn Write>
    } else {
        Box::new(File::create(cli.out_file)?) as Box<dyn Write>
    });

    if cli.hex {
        for (code, display) in codes.into_iter().zip(align_tabbed_lines(
            &lines
                .into_iter()
                .map(|((name, ops), instr_info)| {
                    let mut display = fmt_line(name, ops);

                    let (_, original_line) = instr_info.original_line;

                    if display != original_line {
                        display = format!("{display}\t[{original_line}]");
                    } else {
                        display += "\t";
                    }

                    if let Some(label_name) = instr_info.label_name {
                        display = format!("{display}\t<label: {label_name}>");
                    } else {
                        display += "\t";
                    }

                    display
                })
                .collect::<Vec<String>>(),
        )) {
            writeln!(out, "0x{:08X} # {}", code, display)?;
        }
    } else {
        for code in codes {
            out.write_all(&code.to_le_bytes())?;
        }
    }

    Ok(())
}
