#![feature(decl_macro)]

mod command;
mod cpu;
mod devices;
mod dpi;
mod memory;

use std::sync::{
    Arc, Mutex,
    atomic::{self, AtomicBool},
};

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser};
use clap_complete::CompleteEnv;

use crate::{
    command::Cli,
    dpi::{memory::MEMORY, pixel_display::PIXEL_DISPLAY},
    memory::Memory,
};

fn main() -> Result<()> {
    CompleteEnv::with_factory(Cli::command)
        .var("ARCHP_COMPLETE")
        .complete();

    env_logger::init();

    let cli = Cli::parse();

    let stopped = Arc::new(AtomicBool::new(false));

    {
        let stopped = Arc::clone(&stopped);
        ctrlc::set_handler(move || {
            stopped.store(true, atomic::Ordering::SeqCst);
        })?;
    }

    MEMORY
        .set(Mutex::new(Memory::with_config(
            cli.ram_size,
            cli.framebuffer_size,
            &cli.file,
        )?))
        .expect("'MEMORY' has already been initialized.");
    PIXEL_DISPLAY.with(|pd| pd.borrow_mut().init(cli.framebuffer_size, 6))?;

    let cpu_top = cpu::ffi::create_cpu();

    while !cpu_top.got_finish() {
        cpu_top.increase_time(1);

        if cpu_top.time() > 2 {
            cpu_top.set_rst(false);
        }

        cpu_top.flip_clk();

        if cpu_top.posedge_clk() {
            stopped.fetch_or(
                !PIXEL_DISPLAY.with(|pd| pd.borrow_mut().handle_event()),
                atomic::Ordering::SeqCst,
            );
        }

        if stopped.load(atomic::Ordering::SeqCst) {
            bail!("Simulation interrupted by user.");
        }

        cpu_top.eval();
    }

    Ok(())
}
