pub fn sign_extend(v: u32, bits: u8) -> u32 {
    let shift = 32 - bits;
    if bits == 32 {
        v
    } else {
        ((v << shift) as i32 >> shift) as u32
    }
}
