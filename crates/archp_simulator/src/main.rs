mod command;
mod cpu;
mod devices;
mod dpi;
mod memory;

use std::sync::{Mutex, mpsc};

use anyhow::{Result, anyhow, bail};
use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

use crate::{command::Cli, dpi::memory::MEMORY, memory::Memory};

fn main() -> Result<()> {
    CompleteEnv::with_factory(Cli::command)
        .var("ARCHP_COMPLETE")
        .complete();

    env_logger::init();

    let cli = Cli::parse();

    let (tx, rx) = mpsc::channel::<bool>();

    MEMORY
        .set(Mutex::new(Memory::with_config(tx, &cli)?))
        .map_err(|_| anyhow!("'MEMORY' has already been initialized."))?;

    let cpu_top = cpu::ffi::create_cpu();

    while !cpu_top.got_finish() {
        cpu_top.increase_time(1);

        if cpu_top.time() > 2 {
            cpu_top.set_rst(false);
        }

        cpu_top.flip_clk();

        if let Ok(stopped) = rx.try_recv()
            && stopped
        {
            bail!("Simulation interrupted by user.");
        }

        cpu_top.eval();
    }

    Ok(())
}
