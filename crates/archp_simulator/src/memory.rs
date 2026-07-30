use std::sync::mpsc;

use anyhow::{Result, bail};

use crate::{
    command::Cli,
    devices::{Device, FrameBuffer, Keyboard, Ram},
};

pub struct Memory<'a> {
    regions: Vec<Region<'a>>,
}

pub struct Region<'a> {
    pub start: usize,
    pub dev: Device<'a>,
}

pub trait MemDevice: Send + Sync {
    fn size(&self) -> usize;
    fn load(&self, addr: usize, width: usize) -> u32;
    fn store(&mut self, addr: usize, width: usize, value: u32);
}

impl<'a> Memory<'a> {
    pub fn with_config(tx: mpsc::Sender<bool>, config: &Cli) -> Result<Self> {
        let mut regions = Vec::new();

        let &Cli {
            ram_size,
            file: ref program_path,
            dri_device: ref fb_device,
            resolution: (fb_width, fb_height),
            grab_keyboard,
            ..
        } = config;

        if ram_size > /* 2G */ 2 * 1024 * 1024 * 1024 {
            bail!(
                "RAM size too large: {} bytes. Maximum allowed is 2GB.",
                ram_size
            );
        }

        regions.push(Region {
            start: 0x0000_0000,
            dev: Device::Ram(Ram::new(ram_size as usize, program_path)?),
        });

        if let Some(fb_device) = fb_device {
            regions.push(Region {
                start: 0x8000_0000,
                dev: Device::FrameBuffer(FrameBuffer::new(fb_device, fb_width, fb_height)?),
            });
        }

        regions.push(Region {
            start: 0x9000_0000,
            dev: Device::Keyboard(Keyboard::new(tx, grab_keyboard)),
        });

        Ok(Memory { regions })
    }

    fn find_region(&self, addr: usize) -> &Region<'a> {
        self.regions
            .iter()
            .find(|r| r.contains(addr))
            .unwrap_or_else(|| panic!("Invalid addr: 0x{:08X}", addr))
    }

    fn find_region_mut(&mut self, addr: usize) -> &mut Region<'a> {
        self.regions
            .iter_mut()
            .find(|r| r.contains(addr))
            .unwrap_or_else(|| panic!("Invalid addr: 0x{:08X}", addr))
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

    pub fn get_fb(&mut self) -> &mut FrameBuffer<'a> {
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

impl Region<'_> {
    #[inline]
    pub fn size(&self) -> usize {
        self.dev.size()
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.start + self.size() - 1
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
