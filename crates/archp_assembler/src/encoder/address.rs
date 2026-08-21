use anyhow::{Result, bail};

use crate::utils::split::split_hi_lo;

pub fn encode_address(addr: i64, base: u32, shift: bool) -> i64 {
    let v = addr - (base as i64);

    if shift { v >> 1 } else { v }
}

pub fn encode_address_check(addr: i64, base: u32, shift: bool, bits: u8) -> Result<u32> {
    let v = encode_address(addr, base, shift);

    let (lo, hi) = split_hi_lo(v, bits, true);

    if hi != 0 {
        bail!(
            "Address offset '{}' out of range for i{} ({} ..= {})",
            v,
            bits,
            i32::MIN >> (32 - bits),
            i32::MAX >> (32 - bits),
        );
    }

    Ok(lo)
}
