use std::{
    fs::File,
    io::Read,
    sync::{LazyLock, Mutex},
};

use anyhow::{Result, anyhow};

pub struct Program {
    data: Vec<u8>,
}

impl Program {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn open(&mut self, path: &str) -> Result<()> {
        let mut file =
            File::open(path).map_err(|err| anyhow!("Failed to open file '{}': {}", path, err))?;

        self.data.clear();
        file.read_to_end(&mut self.data)
            .map_err(|err| anyhow!("Failed to read file '{}': {}", path, err))?;

        Ok(())
    }

    pub fn get_instruction(&self, pc: usize) -> u32 {
        u32::from_le_bytes(self.data[pc..pc + 4].try_into().unwrap())
    }
}

pub static PROGRAM: LazyLock<Mutex<Program>> = LazyLock::new(|| Mutex::new(Program::new()));

#[unsafe(no_mangle)]
extern "C" fn get_instruction(pc: u32) -> u32 {
    PROGRAM.lock().unwrap().get_instruction(pc as usize)
}
