mod framebuffer;
mod keyboard;
mod ram;

use std::sync::{
    Arc, Mutex, RwLock,
    atomic::{AtomicU64, Ordering},
};

pub use crate::system::devices::{framebuffer::FrameBuffer, keyboard::Keyboard, ram::Ram};

pub enum Device<'a> {
    Ram(Ram),
    FrameBuffer(FrameBuffer<'a>),
    Keyboard(Keyboard),
}

impl Device<'_> {
    pub fn size(&self) -> usize {
        match self {
            Device::Ram(dev) => dev.size,
            Device::FrameBuffer(dev) => dev.size(),
            Device::Keyboard(dev) => dev.size,
        }
    }

    pub fn load(&self, addr: usize, width: usize) -> u32 {
        match self {
            Device::Ram(dev) => dev.data.load(addr, width),
            Device::FrameBuffer(dev) => dev.data.load(addr, width),
            Device::Keyboard(dev) => Load::load(&dev.data, addr, width),
        }
    }

    pub fn store(&self, addr: usize, width: usize, value: u32) {
        match self {
            Device::Ram(dev) => dev.data.store(addr, width, value),
            Device::FrameBuffer(dev) => dev.data.store(addr, width, value),
            Device::Keyboard(_) => panic!("{:#010X} is read-only for keyboard device", addr),
        };
    }
}

trait LoadU8 {
    fn load_u8(&self, addr: usize, width: usize) -> u32;
}

impl<T: AsRef<[u8]>> LoadU8 for T {
    fn load_u8(&self, addr: usize, width: usize) -> u32 {
        let data = self.as_ref();
        match width {
            1 => data[addr] as u32,
            2 => u16::from_le_bytes(data[addr..addr + 2].try_into().unwrap()) as u32,
            4 => u32::from_le_bytes(data[addr..addr + 4].try_into().unwrap()),
            _ => panic!("invalid width"),
        }
    }
}

trait Load {
    fn load(&self, addr: usize, width: usize) -> u32;
}

impl<T: LoadU8> Load for Mutex<T> {
    fn load(&self, addr: usize, width: usize) -> u32 {
        self.lock().unwrap().load_u8(addr, width)
    }
}

impl<T: LoadU8> Load for RwLock<T> {
    fn load(&self, addr: usize, width: usize) -> u32 {
        self.read().unwrap().load_u8(addr, width)
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

trait StoreU8 {
    fn store_u8(&mut self, addr: usize, width: usize, value: u32);
}

impl<T: AsMut<[u8]>> StoreU8 for T {
    fn store_u8(&mut self, addr: usize, width: usize, value: u32) {
        let data = self.as_mut();
        match width {
            1 => data[addr] = value as u8,
            2 => data[addr..addr + 2].copy_from_slice(&(value as u16).to_le_bytes()),
            4 => data[addr..addr + 4].copy_from_slice(&value.to_le_bytes()),
            _ => panic!("invalid width"),
        }
    }
}

trait Store {
    fn store(&self, addr: usize, width: usize, value: u32);
}

impl<T: StoreU8> Store for Mutex<T> {
    fn store(&self, addr: usize, width: usize, value: u32) {
        self.lock().unwrap().store_u8(addr, width, value)
    }
}

impl<T: StoreU8> Store for RwLock<T> {
    fn store(&self, addr: usize, width: usize, value: u32) {
        self.write().unwrap().store_u8(addr, width, value)
    }
}
