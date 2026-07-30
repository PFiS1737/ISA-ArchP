mod framebuffer;
mod keyboard;
mod ram;

use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

pub use crate::devices::{framebuffer::FrameBuffer, keyboard::Keyboard, ram::Ram};
use crate::memory::MemDevice;

pub enum Device {
    Ram(Ram),
    FrameBuffer(FrameBuffer),
    Keyboard(Keyboard),
}

impl MemDevice for Device {
    fn load(&self, addr: usize, width: usize) -> u32 {
        match self {
            Device::Ram(dev) => dev.data.load(addr, width),
            Device::FrameBuffer(dev) => dev.data.load(addr, width),
            Device::Keyboard(dev) => Load::load(&dev.data, addr, width),
        }
    }

    fn store(&mut self, addr: usize, width: usize, value: u32) {
        match self {
            Device::Ram(dev) => dev.data.store(addr, width, value),
            Device::FrameBuffer(dev) => dev.data.store(addr, width, value),
            Device::Keyboard(dev) => Store::store(&mut dev.data, addr, width, value),
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

impl Load for Arc<AtomicU64> {
    fn load(&self, addr: usize, width: usize) -> u32 {
        if addr + width > 8 {
            panic!("out of bounds");
        }

        let v = AtomicU64::load(self, Ordering::Relaxed);

        let shift = (addr * 8) as u32;

        match width {
            1 => ((v >> shift) & 0xFF) as u32,
            2 => ((v >> shift) & 0xFFFF) as u32,
            4 => ((v >> shift) & 0xFFFF_FFFF) as u32,
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

// TODO: remove this, can't write to it
impl Store for Arc<AtomicU64> {
    fn store(&mut self, addr: usize, width: usize, value: u32) {
        if addr + width > 8 {
            panic!("out of bounds");
        }

        let shift = (addr * 8) as u32;

        let mask: u64 = match width {
            1 => 0xFF,
            2 => 0xFFFF,
            4 => 0xFFFF_FFFF,
            _ => panic!("invalid width"),
        } << shift;

        let value = (value as u64) << shift;

        // CAS loop
        let mut old = AtomicU64::load(self, Ordering::Relaxed);

        loop {
            let new = (old & !mask) | (value & mask);

            match self.compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(v) => old = v,
            }
        }
    }
}
