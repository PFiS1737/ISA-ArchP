mod devices;
mod memory;
mod register;
mod syscalls;

use std::sync::mpsc;

use anyhow::Result;

use crate::{
    command::Cli,
    system::{memory::Memory, register::RegisterFile, syscalls::*},
};

pub struct System<'a> {
    tx: mpsc::Sender<bool>,
    memory: Memory<'a>,
    register_file: RegisterFile,
}

impl<'a> System<'a> {
    pub fn with_config(tx: mpsc::Sender<bool>, config: &Cli) -> Result<Self> {
        Ok(Self {
            tx: tx.clone(),
            memory: Memory::with_config(tx, config)?,
            register_file: RegisterFile::new(),
        })
    }

    pub fn get_memory(&self) -> &Memory<'a> {
        &self.memory
    }

    pub fn get_register_file(&self) -> &RegisterFile {
        &self.register_file
    }

    pub fn system_call(&self) {
        let regs = self.register_file.rw_args();

        match regs[17] {
            1 => print_int(regs),
            4 => print_string(regs, self.memory.get_ram()),
            5 => read_int(regs),
            10 => self.tx.send(true).unwrap(),
            11 => print_char(regs),
            12 => read_char(regs),
            41 => random_int(regs),
            42 => random_int_range(regs),
            63 => read(regs, self.memory.get_ram()),
            64 => write(regs, self.memory.get_ram()),
            0x1000_0000 => set_pixel(regs, self.memory.get_framebuffer()),
            _ => {
                // TODO: 93: support exit code
                panic!("Unsupported system call number: {}", regs[17]);
            },
        }
    }
}
