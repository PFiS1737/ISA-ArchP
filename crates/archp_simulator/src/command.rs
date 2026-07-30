use std::{error::Error, ops::Mul, str::FromStr};

use anyhow::{Result, bail};
use clap::{Parser, ValueHint::FilePath};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
pub struct Cli {
    /// File path to the binary machine code file to be simulated.
    #[arg(value_hint = FilePath)]
    pub file: String,

    /// RAM size in byte for the simulator, optional unit is supported.
    #[arg(short = 'm', long, value_parser = parse_united::<u32>, default_value = "64M")]
    pub ram_size: u32,

    /// DRI device path for the framebuffer output, e.g. /dev/dri/card0.
    #[arg(short = 'f', long, value_hint = FilePath)]
    pub dri_device: Option<String>,

    /// Framebuffer size in WIDTHxHEIGHT format for the simulator.
    #[arg(short, long, value_parser = parse_framebuffer_size, default_value = "128x96")]
    pub resolution: (usize, usize),

    /// Whether to grab the keyboard input for the simulator.
    #[arg(short, long, default_value_t = false)]
    pub grab_keyboard: bool,

    // Max simulation frequency in Hz, optional unit is supported.
    #[arg(long, value_parser = parse_united::<f64>)]
    pub hz: Option<f64>,
}

fn parse_united<T>(size_str: &str) -> Result<T>
where
    T: FromStr + Mul<Output = T> + From<u32>,
    T::Err: Error + Send + Sync + 'static,
{
    let (num_str, scale) = if let Some(num) = size_str.strip_suffix(['k', 'K']) {
        (num, T::from(1024))
    } else if let Some(num) = size_str.strip_suffix(['M', 'm']) {
        (num, T::from(1024 * 1024))
    } else if let Some(num) = size_str.strip_suffix(['G', 'g']) {
        (num, T::from(1024 * 1024 * 1024))
    } else {
        (size_str, T::from(1))
    };

    let num: T = num_str.parse()?;

    Ok(num * scale)
}

fn parse_framebuffer_size(size_str: &str) -> Result<(usize, usize)> {
    let parts: Vec<&str> = size_str.split('x').collect();
    if parts.len() != 2 {
        bail!("Invalid framebuffer size format. Expected format: WIDTHxHEIGHT");
    }

    let width: usize = parts[0].parse()?;
    let height: usize = parts[1].parse()?;

    Ok((width, height))
}
