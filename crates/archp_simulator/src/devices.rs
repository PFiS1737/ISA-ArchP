mod framebuffer;
mod ram;

pub use crate::devices::{framebuffer::FrameBuffer, ram::Ram};
use crate::memory::MemDevice;

#[derive(Debug)]
pub enum Device {
    Ram(Ram),
    FrameBuffer(FrameBuffer),
}

macro match_self($self:expr, $( $field:ident ),+ ()) {
    match $self {
        Device::Ram(dev) => dev$(.$field)+(),
        Device::FrameBuffer(dev) => dev$(.$field)+(),
    }
}

impl MemDevice for Device {
    fn load(&self, addr: usize, width: usize) -> u32 {
        let data = match_self!(self, data, as_slice());

        match width {
            1 => data[addr] as u32,
            2 => u16::from_le_bytes(data[addr..addr + 2].try_into().unwrap()) as u32,
            4 => u32::from_le_bytes(data[addr..addr + 4].try_into().unwrap()),
            _ => panic!("invalid width"),
        }
    }

    fn store(&mut self, addr: usize, width: usize, value: u32) {
        let data = match_self!(self, data, as_mut_slice());

        match width {
            1 => data[addr] = value as u8,
            2 => data[addr..addr + 2].copy_from_slice(&(value as u16).to_le_bytes()),
            4 => data[addr..addr + 4].copy_from_slice(&value.to_le_bytes()),
            _ => panic!("invalid width"),
        }
    }
}
