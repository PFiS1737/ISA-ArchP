use clap::{Parser, ValueHint::FilePath};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
pub struct Cli {
    /// File path to the binary machine code file to be simulated.
    #[arg(value_hint = FilePath)]
    pub file: String,
}
