use anyhow::Result;
use clap::{Parser, ValueHint::FilePath};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
pub struct Cli {
    /// File path to the binary machine code file to be simulated.
    #[arg(value_hint = FilePath)]
    pub file: String,

    /// RAM size in byte for the simulator, optional unit is supported.
    #[arg(short = 'm', long, value_parser = parse_united_size, default_value = "64M")]
    pub ram_size: usize,

    /// Framebuffer size in WIDTHxHEIGHT format for the simulator.
    #[arg(short, long, value_parser = parse_framebuffer_size, default_value = "128x96")]
    pub framebuffer_size: (usize, usize),
}

fn parse_united_size(size_str: &str) -> Result<usize> {
    let (num_str, scale): (&str, usize) = if let Some(num) = size_str.strip_suffix(['k', 'K']) {
        (num, 1024)
    } else if let Some(num) = size_str.strip_suffix(['M', 'm']) {
        (num, 1024 * 1024)
    } else if let Some(num) = size_str.strip_suffix(['G', 'g']) {
        (num, 1024 * 1024 * 1024)
    } else {
        (size_str, 1)
    };

    let num: usize = num_str.parse()?;

    Ok(num * scale)
}

fn parse_framebuffer_size(size_str: &str) -> Result<(usize, usize)> {
    let parts: Vec<&str> = size_str.split('x').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid framebuffer size format. Expected format: WIDTHxHEIGHT");
    }

    let width: usize = parts[0].parse()?;
    let height: usize = parts[1].parse()?;

    Ok((width, height))
}
