use super::SYSTEM;

#[unsafe(no_mangle)]
extern "C" fn mem_load(addr: u32, width: *const u32) -> u32 {
    SYSTEM
        .get()
        .unwrap()
        .get_memory()
        .load(addr as usize, unsafe { *width } as usize)
}

#[unsafe(no_mangle)]
extern "C" fn mem_store(addr: u32, width: *const u32, value: i32) {
    SYSTEM.get().unwrap().get_memory().store(
        addr as usize,
        unsafe { *width } as usize,
        value as u32,
    );
}

#[unsafe(no_mangle)]
extern "C" fn pixel_display_set(x: u32, y: u32, color: u32) {
    SYSTEM
        .get()
        .unwrap()
        .get_memory()
        .get_fb()
        .set_pixel(x as usize, y as usize, color);
}
