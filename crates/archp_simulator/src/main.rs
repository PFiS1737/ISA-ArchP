mod command;
mod cpu;
mod dpi;
mod system;

use std::{
    process::ExitCode,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

use crate::{command::Cli, dpi::SYSTEM, system::System};

fn main() -> Result<ExitCode> {
    CompleteEnv::with_factory(Cli::command)
        .var("ARCHP_COMPLETE")
        .complete();

    env_logger::init();

    let cli = Cli::parse();

    let (tx, rx) = mpsc::channel::<u8>();

    let _ = SYSTEM.set(System::with_config(tx, &cli)?);

    let cpu_top = cpu::ffi::create_cpu();

    let mut exit_code = 0;
    let mut last_time = Instant::now();

    while !cpu_top.got_finish() {
        cpu_top.increase_time(1);

        if cpu_top.time() > 2 {
            cpu_top.set_rst(false);
        }

        cpu_top.flip_clk();

        if let Ok(code) = rx.try_recv() {
            exit_code = code;
            break;
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

    Ok(exit_code.into())
}
