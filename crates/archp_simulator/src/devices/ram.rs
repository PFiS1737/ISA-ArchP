use std::{fs::File, io::Read};

use anyhow::{Result, anyhow, bail};

#[derive(Debug)]
pub struct Ram {
    pub data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize, program_path: &str) -> Result<Self> {
        let mut file = File::open(program_path)
            .map_err(|err| anyhow!("Failed to open file '{}': {}", program_path, err))?;

        let metadata = file.metadata().map_err(|err| {
            anyhow!(
                "Failed to get metadata for file '{}': {}",
                program_path,
                err
            )
        })?;
        let file_len = metadata.len() as usize;

        if size < file_len {
            bail!(
                "RAM size ({}) is too small for program '{}' (requires {} bytes)",
                size,
                program_path,
                file_len
            );
        }

        let mut ram = Self {
            data: vec![0; size],
        };

        file.read_exact(&mut ram.data[..file_len])
            .map_err(|err| anyhow!("Failed to read file '{}': {}", program_path, err))?;

        Ok(ram)
    }
}
