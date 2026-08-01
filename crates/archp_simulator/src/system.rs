use std::sync::mpsc;

use anyhow::Result;

use crate::{command::Cli, memory::Memory};

pub struct System<'a> {
    memory: Memory<'a>,
}

impl<'a> System<'a> {
    pub fn with_config(tx: mpsc::Sender<bool>, config: &Cli) -> Result<Self> {
        Ok(System {
            memory: Memory::with_config(tx, config)?,
        })
    }

    pub fn get_memory(&self) -> &Memory<'a> {
        &self.memory
    }
}
