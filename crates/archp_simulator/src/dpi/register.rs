use super::SYSTEM;

#[unsafe(no_mangle)]
extern "C" fn read_regfile(index: *const u32) -> u32 {
    SYSTEM
        .get()
        .unwrap()
        .get_register_file()
        .read(unsafe { *index } as usize)
}

#[unsafe(no_mangle)]
extern "C" fn write_regfile(index: *const u32, value: i32) {
    SYSTEM
        .get()
        .unwrap()
        .get_register_file()
        .write(unsafe { *index } as usize, value as u32);
}
