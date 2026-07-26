#[derive(Debug)]
pub struct Memory {
    // Max: 2G [0x0000_0000 - 0x7FFF_FFFF]
    pub data: Vec<u8>,
    pub size: usize,

    // Max: 16M [0x8000_0000 - 0x80FF_FFFF]
    pub fb_data: Vec<u8>,
    pub fb_width: usize,
    pub fb_height: usize,
}

impl Memory {
    pub fn new(memory_size: usize, framebuffer_size: (usize, usize)) -> Self {
        if memory_size > /* 2G */ 2 * 1024 * 1024 * 1024 {
            panic!(
                "Memory size too large: {} bytes. Maximum allowed is 2GB.",
                memory_size
            );
        }

        let fb_size = framebuffer_size.0 * framebuffer_size.1 * 4;
        if fb_size > /* 16M */ 16 * 1024 * 1024 {
            panic!(
                "Framebuffer size too large: {} bytes. Maximum allowed is 16MB.",
                fb_size
            );
        }

        let mut memory = Self {
            size: memory_size,
            data: Vec::new(),

            fb_data: Vec::new(),
            fb_width: framebuffer_size.0,
            fb_height: framebuffer_size.1,
        };

        memory.reset();

        memory
    }

    pub fn reset(&mut self) {
        self.data = vec![0; self.size];

        let v: Vec<u32> = vec![0x404040FF; self.fb_width * self.fb_height];
        self.fb_data = {
            let len = v.len() * 4;
            let cap = v.capacity() * 4;
            let ptr = v.as_ptr() as *mut u8;
            std::mem::forget(v);
            unsafe { Vec::from_raw_parts(ptr, len, cap) }
        };
    }

    pub fn load(&self, addr: usize, width: usize) -> u32 {
        let src = get_src!(self, addr);
        let addr = get_addr!(self, addr);

        match width {
            1 => src[addr] as u32,
            2 => u16::from_le_bytes(src[addr..addr + 2].try_into().unwrap()) as u32,
            4 => u32::from_le_bytes(src[addr..addr + 4].try_into().unwrap()),
            _ => panic!("Invalid width: {}", width),
        }
    }

    pub fn store(&mut self, addr: usize, width: usize, value: u32) {
        let dst = get_dst!(self, addr);
        let addr = get_addr!(self, addr);

        match width {
            1 => dst[addr] = value as u8,
            2 => dst[addr..addr + 2].copy_from_slice(&(value as u16).to_le_bytes()),
            4 => dst[addr..addr + 4].copy_from_slice(&value.to_le_bytes()),
            _ => panic!("Invalid width: {}", width),
        }
    }

    pub fn to_fb_addr(&self, x: usize, y: usize) -> usize {
        if x >= self.fb_width || y >= self.fb_height {
            panic!("Framebuffer coordinates out of bounds: ({}, {})", x, y);
        }
        0x8000_0000 + (y * self.fb_width + x) * 4
    }
}

macro get_src($self:expr, $addr:expr) {
    switch!($self, $addr, [&$self.data, &$self.fb_data])
}

macro get_dst($self:expr, $addr:expr) {
    switch!($self, $addr, [&mut $self.data, &mut $self.fb_data])
}

macro get_addr($self:expr, $addr:expr) {
    switch!($self, $addr, [$addr, $addr - 0x8000_0000])
}

// TODO: make memory layout structural
macro switch($self:expr, $addr:expr,[$ram:expr, $fb:expr]) {
    match $addr {
        0x0000_0000..=0x7FFF_FFFF => $ram,
        0x8000_0000..=0x80FF_FFFF => $fb,
        _ => panic!("Address out of bounds: 0x{:08X}", $addr),
    }
}
