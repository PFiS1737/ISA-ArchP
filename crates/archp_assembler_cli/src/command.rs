use std::{
    fmt::Display,
    fs::File,
    io::{Write, stdout},
};

use anyhow::Result;
use clap::{
    Parser,
    ValueHint::FilePath,
    builder::{Styles, styling::AnsiColor},
};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
#[command(styles = get_styles())]
pub struct Cli {
    /// File path to the source assembly file.
    #[arg(value_hint = FilePath)]
    pub src_file: String,

    /// The output file path.
    #[arg(short, long, value_hint = FilePath, default_value_t = Output::Stdout)]
    pub output: Output,

    /// Output binary machine code instead of formatted hex.
    #[arg(long)]
    pub bin: bool,

    /// Disable the macro-instructions.
    #[arg(long)]
    pub disable_macro: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Stdout,
    File(String),
}

impl Output {
    pub fn get(&self) -> Result<Box<dyn Write>> {
        Ok(match self {
            Self::Stdout => Box::new(stdout()),
            Self::File(path) => Box::new(File::create(path)?),
        })
    }
}

const STDOUT: &str = "<stdout>";

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stdout => write!(f, "{}", STDOUT),
            Self::File(path) => write!(f, "{}", path),
        }
    }
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        if value == STDOUT {
            Self::Stdout
        } else {
            Self::File(value.to_owned())
        }
    }
}

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default().bold().underline())
        .usage(AnsiColor::Yellow.on_default().bold().underline())
        .valid(AnsiColor::Green.on_default().bold().underline())
        .invalid(AnsiColor::Red.on_default().bold())
        .placeholder(AnsiColor::White.on_default())
        .error(AnsiColor::Red.on_default().bold())
        .literal(AnsiColor::Green.on_default())
        .context(AnsiColor::Cyan.on_default())
        .context_value(AnsiColor::Magenta.on_default())
}
