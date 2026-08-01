use std::sync::RwLock;

pub struct RegisterFile {
    regs: RwLock<[u32; 32]>,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            regs: RwLock::new([0; 32]),
        }
    }

    pub fn read(&self, index: usize) -> u32 {
        self.regs.read().unwrap()[index]
    }

    pub fn write(&self, index: usize, value: u32) {
        self.regs.write().unwrap()[index] = value;
    }
}
