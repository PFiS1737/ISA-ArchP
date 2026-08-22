mod devices;
mod memory;
mod register;
mod syscalls;

use std::sync::mpsc;

use anyhow::Result;

use crate::{
    command::Cli,
    system::{memory::Memory, register::RegisterFile},
};

pub struct System<'a> {
    tx: mpsc::Sender<u8>,
    memory: Memory<'a>,
    register_file: RegisterFile,
}

impl<'a> System<'a> {
    pub fn with_config(tx: mpsc::Sender<u8>, config: &Cli) -> Result<Self> {
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
}
