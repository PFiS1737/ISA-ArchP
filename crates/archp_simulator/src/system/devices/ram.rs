use std::{fs::File, io::Read, path::PathBuf, sync::RwLock};

use anyhow::{Result, anyhow, bail};

pub struct Ram {
    pub data: RwLock<Vec<u8>>,
    pub size: usize,
}

impl Ram {
    pub fn new(size: usize, program_path: &PathBuf) -> Result<Self> {
        let mut file = File::open(program_path)
            .map_err(|err| anyhow!("Failed to open file '{}': {}", program_path.display(), err))?;

        let metadata = file.metadata().map_err(|err| {
            anyhow!(
                "Failed to get metadata for file '{}': {}",
                program_path.display(),
                err
            )
        })?;
        let file_len = metadata.len() as usize;

        if size < file_len {
            bail!(
                "RAM size ({}) is too small for program '{}' (requires {} bytes)",
                size,
                program_path.display(),
                file_len
            );
        }

        let mut data = vec![0; size];

        file.read_exact(&mut data[..file_len])
            .map_err(|err| anyhow!("Failed to read file '{}': {}", program_path.display(), err))?;

        Ok(Self {
            data: RwLock::new(data),
            size,
        })
    }
}
