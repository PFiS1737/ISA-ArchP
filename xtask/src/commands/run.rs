use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
    process::Command,
};

use anyhow::{Result, bail};
use tempfile::TempDir;

use crate::utils::print_h1;

pub fn run(
    file: PathBuf,
    asm: bool,
    trace: bool,
    console: bool,
    mut sim_args: Vec<String>,
) -> Result<()> {
    if !file.exists() {
        bail!("file '{}' not found", file.display());
    }

    let tmp_dir = TempDir::new()?;
    let tmp_path = tmp_dir.path();

    let output_bin = tmp_path.join("output.bin");
    let trace_file = tmp_path.join("dump.fst");
    let surfer_dir = tmp_path.join(".surfer").join("mappings");
    create_dir_all(&surfer_dir)?;
    let mapping_file = surfer_dir.join("mapping");

    print_h1("Building...");
    let status = Command::new("cargo")
        .arg("build")
        .args(if trace {
            &["--features", "trace"][..]
        } else {
            &[]
        })
        .status()?;
    if !status.success() {
        bail!("cargo build failed");
    }
    println!();

    let file_to_sim = if asm {
        print_h1("Assembling...");

        let status = Command::new("./target/debug/archp-as")
            .arg(&file)
            .arg("-o")
            .arg(&output_bin)
            .status()?;
        if !status.success() {
            bail!("assemble failed");
        }

        println!("Done");

        if trace {
            let mut mapping_file = File::create(&mapping_file)?;
            writeln!(mapping_file, "Name = Archp Instruction")?;
            writeln!(mapping_file, "Bits = 32")?;

            Command::new("./target/debug/archp-as")
                .arg(&file)
                .arg("--hex")
                .arg("--stdout")
                .stdout(mapping_file)
                .status()?;
        }

        println!();

        output_bin
    } else {
        file
    };

    if trace {
        sim_args.push("-T".to_string());
        sim_args.push(trace_file.to_string_lossy().to_string());
    }

    print_h1("Running...");
    let status = if console {
        Command::new("sudo")
            .args(["openvt", "-sw", "--"])
            .arg("./target/debug/archp")
            .arg(&file_to_sim)
            .arg("-fk")
            .args(&sim_args)
            .status()?
    } else {
        Command::new("./target/debug/archp")
            .arg(&file_to_sim)
            .args(&sim_args)
            .status()?
    };
    if !status.success() {
        bail!("simulator failed");
    }

    if trace {
        println!();
        print_h1("Opening trace viewer...");
        crate::commands::trace::trace(trace_file, tmp_path.into())?;
    }

    Ok(())
}
