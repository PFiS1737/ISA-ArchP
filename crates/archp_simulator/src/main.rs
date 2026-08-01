mod command;
mod cpu;
mod devices;
mod dpi;
mod memory;
mod register;
mod system;

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

use crate::{command::Cli, dpi::SYSTEM, system::System};

fn main() -> Result<()> {
    CompleteEnv::with_factory(Cli::command)
        .var("ARCHP_COMPLETE")
        .complete();

    env_logger::init();

    let cli = Cli::parse();

    let (tx, rx) = mpsc::channel::<bool>();

    let _ = SYSTEM.set(System::with_config(tx, &cli)?);

    let cpu_top = cpu::ffi::create_cpu();

    let mut last_time = Instant::now();

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

        if let Some(hz) = cli.hz {
            let elapsed = last_time.elapsed();
            let duration = Duration::from_secs_f64(1.0 / hz);
            if elapsed < duration {
                thread::sleep(duration - elapsed);
            }
            last_time = Instant::now();
        }
    }

    Ok(())
}
