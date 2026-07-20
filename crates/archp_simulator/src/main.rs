mod cpu;
mod dpi;

use std::{
    env,
    process::ExitCode,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};

fn main() -> ExitCode {
    let stopped = Arc::new(AtomicBool::new(false));

    {
        let stopped = Arc::clone(&stopped);
        ctrlc::set_handler(move || {
            stopped.store(true, atomic::Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    let Some(file) = env::args().find_map(|arg| arg.strip_prefix("+FILE=").map(|s| s.to_string()))
    else {
        eprintln!(
            "Error: No input file specified. Use +FILE=<path_to_file> to specify the input file."
        );
        return ExitCode::FAILURE;
    };

    unsafe {
        use dpi::*;

        if let Err(err) = program.open(&file) {
            eprintln!("{}", err.what());
            return ExitCode::FAILURE;
        }

        mem.init(1024 * 1024 * 1024);

        if !pd.init(128, 96, 6) {
            pd.destroy();
            return ExitCode::FAILURE;
        }
    }

    let cpu_top = cpu::ffi::create_cpu();

    while !cpu_top.got_finish() {
        cpu_top.increase_time(1);

        if cpu_top.time() > 2 {
            cpu_top.set_rst(false);
        }

        cpu_top.flip_clk();

        if cpu_top.posedge_clk() {
            stopped.store(unsafe { !dpi::pd.handle_event() }, atomic::Ordering::SeqCst);
        }

        if stopped.load(atomic::Ordering::SeqCst) {
            eprintln!("Simulation interrupted by user.");
            break;
        }

        cpu_top.eval();
    }

    unsafe {
        dpi::pd.destroy();
    }

    ExitCode::SUCCESS
}
