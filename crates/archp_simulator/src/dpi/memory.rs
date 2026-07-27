use std::sync::{Mutex, OnceLock};

use crate::memory::Memory;

pub static MEMORY: OnceLock<Mutex<Memory>> = OnceLock::new();

#[unsafe(no_mangle)]
extern "C" fn mem_load(addr: u32, width: *const u32) -> u32 {
    MEMORY
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .load(addr as usize, unsafe { *width } as usize)
}

#[unsafe(no_mangle)]
extern "C" fn mem_store(addr: u32, width: *const u32, value: i32) {
    MEMORY.get().unwrap().lock().unwrap().store(
        addr as usize,
        unsafe { *width } as usize,
        value as u32,
    );
}
