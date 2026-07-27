use std::fmt::Debug;

use crate::devices::{Device, FrameBuffer, Ram};

#[derive(Debug)]
pub struct Memory {
    regions: Vec<Region>,
}

#[derive(Debug)]
pub struct Region {
    pub start: usize,
    pub size: usize,
    pub dev: Device,
}

pub trait MemDevice: Debug + Send + Sync {
    fn reset(&mut self);
    fn load(&self, addr: usize, width: usize) -> u32;
    fn store(&mut self, addr: usize, width: usize, value: u32);
}

impl Memory {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    pub fn with_config(ram_size: usize, (fb_width, fb_height): (usize, usize)) -> Self {
        let mut mem = Memory::new();

        if ram_size > /* 2G */ 2 * 1024 * 1024 * 1024 {
            panic!(
                "RAM size too large: {} bytes. Maximum allowed is 2GB.",
                ram_size
            );
        }

        mem.add_region(Region {
            start: 0x0000_0000,
            size: ram_size,
            dev: Device::Ram(Ram::new(ram_size)),
        });

        let fb_size = fb_width * fb_height * 4;
        if fb_size > /* 16M */ 16 * 1024 * 1024 {
            panic!(
                "Framebuffer size too large: {} bytes. Maximum allowed is 16MB.",
                fb_size
            );
        }

        mem.add_region(Region {
            start: 0x8000_0000,
            size: fb_size,
            dev: Device::FrameBuffer(FrameBuffer::new(fb_width, fb_height)),
        });

        mem
    }

    pub fn add_region(&mut self, region: Region) {
        self.regions.push(region);
    }

    fn find_region(&self, addr: usize) -> &Region {
        self.regions
            .iter()
            .find(|r| r.contains(addr))
            .unwrap_or_else(|| panic!("Invalid addr: 0x{:08X}", addr))
    }

    fn find_region_mut(&mut self, addr: usize) -> &mut Region {
        self.regions
            .iter_mut()
            .find(|r| r.contains(addr))
            .unwrap_or_else(|| panic!("Invalid addr: 0x{:08X}", addr))
    }

    #[inline]
    pub fn reset(&mut self) {
        for r in &mut self.regions {
            r.dev.reset();
        }
    }

    #[inline]
    pub fn load(&self, addr: usize, width: usize) -> u32 {
        let r = self.find_region(addr);
        r.dev.load(r.offset(addr), width)
    }

    #[inline]
    pub fn store(&mut self, addr: usize, width: usize, value: u32) {
        let r = self.find_region_mut(addr);
        r.dev.store(r.offset(addr), width, value);
    }

    pub fn get_fb(&mut self) -> &mut FrameBuffer {
        let r = self
            .regions
            .iter_mut()
            .find(|r| matches!(r.dev, Device::FrameBuffer(..)))
            .unwrap_or_else(|| panic!("Framebuffer region not found"));

        match &mut r.dev {
            Device::FrameBuffer(fb) => fb,
            _ => unreachable!(),
        }
    }
}

impl Region {
    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.size - 1
    }

    #[inline]
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr <= self.end()
    }

    #[inline]
    pub fn offset(&self, addr: usize) -> usize {
        addr - self.start
    }
}
