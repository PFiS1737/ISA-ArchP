mod command;
mod utils;

use std::{
    collections::HashMap,
    fs::{File, read_to_string},
    io::{BufWriter, Write, stdout},
};

use anyhow::{Result, anyhow};
use archp_assembler::{
    assembler::{Assembler, Instr},
    instructions::Instruction,
    utils::fmt::fmt_line,
};
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

    let file_content = read_to_string(&cli.src_file)
        .map_err(|e| anyhow!("Can't read source file '{}': {}", cli.src_file, e))?;

    let context = Assembler::assemble(&file_content)?;

    let mut out = BufWriter::new(if cli.stdout {
        Box::new(stdout()) as Box<dyn Write>
    } else {
        Box::new(File::create(cli.out_file)?) as Box<dyn Write>
    });

    if cli.hex {
        let lines = merge_maps(
            disassemble(&context.text)?,
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

        let (codes, rem) = context.text.as_chunks::<4>();

        for (idx, code) in codes.iter().map(|x| u32::from_le_bytes(*x)).enumerate() {
            writeln!(
                out,
                "{:#010X} # {}",
                code,
                displays.get(&(idx * 4)).unwrap_or(&"".to_string())
            )?;
        }

        if !rem.is_empty() {
            let mut code = [0u8; 4];
            code[..rem.len()].copy_from_slice(rem);
            writeln!(out, "{:#010X} # ", u32::from_le_bytes(code))?;
        }
    } else {
        out.write_all(&context.text)?;
    }

    Ok(())
}

// TODO: Remove this after we implement the ELF and Disassembler
// TODO: Can we have more metadata so we can differentiate between instructions and data,
//       and about where the instruction jumps to?
fn disassemble(codes: &[u8]) -> Result<HashMap<usize, Option<Instr<'static>>>> {
    codes
        .as_chunks::<4>()
        .0
        .iter()
        .enumerate()
        .map(|(idx, code)| -> Result<(usize, Option<Instr<'static>>)> {
            let code = u32::from_le_bytes(*code);

            let instr =
                Instruction::get_by_code(code).map(|instr| (instr.name, instr.decode(code)));

            Ok((idx * 4, instr))
        })
        .collect()
}
