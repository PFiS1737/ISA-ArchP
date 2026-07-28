mod framebuffer;
mod ram;

pub use crate::devices::{framebuffer::FrameBuffer, ram::Ram};
use crate::memory::MemDevice;

#[derive(Debug)]
pub enum Device {
    Ram(Ram),
    FrameBuffer(FrameBuffer),
}

impl MemDevice for Device {
    fn load(&self, addr: usize, width: usize) -> u32 {
        match self {
            Device::Ram(dev) => dev.data.load(addr, width),
            Device::FrameBuffer(dev) => dev.data.load(addr, width),
        }
    }

    fn store(&mut self, addr: usize, width: usize, value: u32) {
        match self {
            Device::Ram(dev) => dev.data.store(addr, width, value),
            Device::FrameBuffer(dev) => dev.data.store(addr, width, value),
        };
    }
}

trait Load {
    fn load(&self, addr: usize, width: usize) -> u32;
}

impl Load for Vec<u8> {
    fn load(&self, addr: usize, width: usize) -> u32 {
        match width {
            1 => self[addr] as u32,
            2 => u16::from_le_bytes(self[addr..addr + 2].try_into().unwrap()) as u32,
            4 => u32::from_le_bytes(self[addr..addr + 4].try_into().unwrap()),
            _ => panic!("invalid width"),
        }
    }
}

trait Store {
    fn store(&mut self, addr: usize, width: usize, value: u32);
}

impl Store for Vec<u8> {
    fn store(&mut self, addr: usize, width: usize, value: u32) {
        match width {
            1 => self[addr] = value as u8,
            2 => self[addr..addr + 2].copy_from_slice(&(value as u16).to_le_bytes()),
            4 => self[addr..addr + 4].copy_from_slice(&value.to_le_bytes()),
            _ => panic!("invalid width"),
        }
    }
}
