use anyhow::Result;
use clap::{Parser, ValueHint::FilePath};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
pub struct Cli {
    /// File path to the binary machine code file to be simulated.
    #[arg(value_hint = FilePath)]
    pub file: String,

    /// Memory size in byte for the simulator, optional unit is supported.
    #[arg(short, long, value_parser = parse_united_size, default_value = "64M")]
    pub memory_size: u32,
}

fn parse_united_size(size_str: &str) -> Result<u32> {
    let (num_str, scale): (&str, u32) = if let Some(num) = size_str.strip_suffix(['k', 'K']) {
        (num, 1024)
    } else if let Some(num) = size_str.strip_suffix(['M', 'm']) {
        (num, 1024 * 1024)
    } else if let Some(num) = size_str.strip_suffix(['G', 'g']) {
        (num, 1024 * 1024 * 1024)
    } else {
        (size_str, 1)
    };

    let num: u32 = num_str.parse()?;

    Ok(num * scale)
}
