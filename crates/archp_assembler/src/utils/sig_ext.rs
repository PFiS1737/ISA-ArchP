pub fn sig_ext_12_to_32(val: u32) -> i32 {
    (if (val & 0x800) != 0 {
        val | 0xFFFFF000
    } else {
        val & 0x00000FFF
    }) as i32
}
