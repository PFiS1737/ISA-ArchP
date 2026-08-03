use std::{error::Error, ops::Mul, path::PathBuf, str::FromStr};

use anyhow::{Result, bail};
use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
pub struct Cli {
    /// File path to the binary machine code file to be simulated.
    pub file: PathBuf,

    /// Max simulation frequency in Hz, optional unit is supported.
    #[arg(short = 'F', long, value_parser = parse_united::<f64>)]
    pub hz: Option<f64>,

    /// RAM size in byte, optional unit is supported.
    #[arg(long, value_name = "SIZE", value_parser = parse_united::<u32>, default_value = "64M")]
    pub ram_size: u32,

    /// Add a XRGB8888 framebuffer to the simulation.
    #[arg(short, long)]
    pub framebuffer: bool,

    /// Start address of the framebuffer device.
    #[arg(long, value_name = "START", value_parser = parse_addr, default_value = "0x80000000", requires = "framebuffer")]
    pub framebuffer_start: u32,

    /// Framebuffer size in WIDTHxHEIGHT format, your device must support the specified size.
    #[arg(long, value_name = "SIZE", value_parser = parse_framebuffer_size, default_value = "640x480", requires = "framebuffer")]
    pub framebuffer_size: (u32, u32),

    /// Specify the framebuffer device path.
    #[arg(
        long,
        value_name = "DEVICE",
        default_value = "/dev/dri/card1", // use integrated GPU by default
        requires = "framebuffer"
    )]
    pub framebuffer_device: PathBuf,

    /// Add a keyboard device to the simulation.
    #[arg(short, long)]
    pub keyboard: bool,

    /// Start address of the keyboard device.
    #[arg(long, value_name = "START", value_parser = parse_addr, default_value = "0x90000000", requires = "keyboard")]
    pub keyboard_start: u32,

    /// Whether to grab the keyboard input.
    #[arg(long, requires = "keyboard")]
    pub keyboard_grab: bool,
}

fn parse_addr(str: &str) -> Result<u32> {
    Ok(if let Some(hex) = str.strip_prefix("0x") {
        u32::from_str_radix(hex, 16)?
    } else {
        str.parse()?
    })
}

fn parse_united<T>(str: &str) -> Result<T>
where
    T: FromStr + Mul<Output = T> + From<u32>,
    T::Err: Error + Send + Sync + 'static,
{
    let (num_str, scale) = if let Some(num) = str.strip_suffix(['k', 'K']) {
        (num, T::from(1024))
    } else if let Some(num) = str.strip_suffix(['M', 'm']) {
        (num, T::from(1024 * 1024))
    } else if let Some(num) = str.strip_suffix(['G', 'g']) {
        (num, T::from(1024 * 1024 * 1024))
    } else {
        (str, T::from(1))
    };

    let num: T = num_str.parse()?;

    Ok(num * scale)
}

fn parse_framebuffer_size(str: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = str.split('x').collect();
    if parts.len() != 2 {
        bail!("Invalid framebuffer size format. Expected format: <WIDTH>x<HEIGHT>");
    }

    let width = parts[0].parse()?;
    let height = parts[1].parse()?;

    Ok((width, height))
}
