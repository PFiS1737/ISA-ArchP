#[derive(Debug)]
pub struct Memory {
    size: usize,
    data: Vec<u8>,
}

impl Memory {
    pub fn new(size_in_bytes: usize) -> Self {
        Self {
            size: size_in_bytes,
            data: vec![0; size_in_bytes],
        }
    }

    pub fn reset(&mut self) {
        self.data = vec![0; self.size];
    }

    fn assert_addr(&self, addr: usize, width: usize) {
        if addr + width > self.size {
            panic!(
                "Memory access out of bounds: addr={} width={} size={}",
                addr,
                width,
                self.data.len()
            );
        }
    }

    pub fn load(&self, addr: usize, width: usize) -> u32 {
        self.assert_addr(addr, width);

        match width {
            1 => self.data[addr] as u32,
            2 => u16::from_le_bytes(self.data[addr..addr + 2].try_into().unwrap()) as u32,
            4 => u32::from_le_bytes(self.data[addr..addr + 4].try_into().unwrap()),
            _ => panic!("Invalid width: {}", width),
        }
    }

    pub fn store(&mut self, addr: usize, width: usize, value: u32) {
        self.assert_addr(addr, width);

        match width {
            1 => self.data[addr] = value as u8,
            2 => {
                let bytes = (value as u16).to_le_bytes();
                self.data[addr..addr + 2].copy_from_slice(&bytes);
            },
            4 => {
                let bytes = value.to_le_bytes();
                self.data[addr..addr + 4].copy_from_slice(&bytes);
            },
            _ => panic!("Invalid width: {}", width),
        }
    }
}
