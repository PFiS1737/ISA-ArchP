use std::{collections::HashSet, fs, path::PathBuf};

use anyhow::Result;
use clap::ValueEnum;

#[derive(Clone, PartialEq, Eq, Hash, ValueEnum)]
pub enum Codegens {
    Instructions,
}

pub fn codegen(targets: Vec<Codegens>) -> Result<()> {
    let targets = HashSet::<Codegens>::from_iter(targets);

    let dest_dir = PathBuf::from("crates/archp_assembler/src/_generated");

    fs::create_dir_all(&dest_dir)?;

    for target in targets {
        match target {
            Codegens::Instructions => {
                archp_codegen::instructions::generate(
                    "crates/archp_assembler/src/instructions.toml",
                    &dest_dir,
                )?;
            },
        }
    }

    Ok(())
}
