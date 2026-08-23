use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run a program in the simulator
    Run {
        /// File path to run
        file: PathBuf,

        /// <FILE> is an assembly file, assemble it before running
        #[arg(short = 's', long)]
        asm: bool,

        /// Enable tracing and open the trace viewer after running
        #[arg(short, long)]
        trace: bool,

        /// Start simulation in a new tty, nessesary if you used the framebuffer
        #[arg(short, long)]
        console: bool,

        /// Arguments to the simulator
        #[arg(raw = true)]
        sim_args: Vec<String>,
    },

    /// Open a waveform file in the trace viewer
    Trace {
        /// Waveform file to open in the trace viewer
        file: PathBuf,
    },
}
