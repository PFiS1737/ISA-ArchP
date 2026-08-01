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

impl<'a> Memory<'a> {
    pub fn with_config(tx: mpsc::Sender<bool>, config: &Cli) -> Result<Self> {
        let mut regions = Vec::new();

        let &Cli {
            file: ref program_path,
            ram_size,
            framebuffer,
            framebuffer_start,
            framebuffer_size: (fb_width, fb_height),
            framebuffer_device: ref fb_device,
            keyboard,
            keyboard_start,
            keyboard_grab: grab_keyboard,
            hz: _,
        } = config;

        regions.push(Region {
            start: 0,
            dev: Device::Ram(Ram::new(ram_size as usize, program_path)?),
        });

        if framebuffer {
            regions.push(Region {
                start: framebuffer_start as usize,
                dev: Device::FrameBuffer(FrameBuffer::new(
                    fb_device,
                    fb_width as usize,
                    fb_height as usize,
                )?),
            });
        }

        if keyboard {
            regions.push(Region {
                start: keyboard_start as usize,
                dev: Device::Keyboard(Keyboard::new(tx, grab_keyboard)),
            });
        }

        let memory = Memory { regions };

        memory.validate_regions()?;

        Ok(memory)
    }

    fn validate_regions(&self) -> Result<()> {
        self.regions.windows(2).try_for_each(|pair| {
            let (a, b) = (&pair[0], &pair[1]);
            if a.end() >= b.start {
                bail!(
                    "Memory regions overlap: {:#010X}-{:#010X} and {:#010X}-{:#010X}",
                    a.start,
                    a.end(),
                    b.start,
                    b.end()
                );
            }
            Ok(())
        })
    }

    pub fn load(&self, addr: usize, width: usize) -> u32 {
        self.regions
            .iter()
            .find(|r| r.contains(addr))
            .unwrap_or_else(|| panic!("Invalid addr: {:#010X}", addr))
            .load(addr, width)
    }

    pub fn store(&self, addr: usize, width: usize, value: u32) {
        self.regions
            .iter()
            .find(|r| r.contains(addr))
            .unwrap_or_else(|| panic!("Invalid addr: {:#010X}", addr))
            .store(addr, width, value);
    }

    pub fn get_fb(&self) -> &FrameBuffer<'a> {
        let r = self
            .regions
            .iter()
            .find(|r| matches!(r.dev, Device::FrameBuffer(..)))
            .unwrap_or_else(|| panic!("Framebuffer region not found"));

        match &r.dev {
            Device::FrameBuffer(fb) => fb,
            _ => unreachable!(),
        }
    }
}

impl Region<'_> {
    fn load(&self, addr: usize, width: usize) -> u32 {
        self.dev.load(self.offset(addr), width)
    }

    fn store(&self, addr: usize, width: usize, value: u32) {
        self.dev.store(self.offset(addr), width, value);
    }

    fn offset(&self, addr: usize) -> usize {
        addr - self.start
    }

    fn size(&self) -> usize {
        self.dev.size()
    }

    fn end(&self) -> usize {
        self.start + self.size() - 1
    }

    fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr <= self.end()
    }
}
