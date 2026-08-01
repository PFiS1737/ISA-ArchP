use std::sync::mpsc;

use anyhow::Result;

use crate::{command::Cli, memory::Memory, register::RegisterFile};

pub struct System<'a> {
    memory: Memory<'a>,
    register_file: RegisterFile,
}

impl<'a> System<'a> {
    pub fn with_config(tx: mpsc::Sender<bool>, config: &Cli) -> Result<Self> {
        Ok(Self {
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
