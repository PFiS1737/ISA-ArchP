use anyhow::{Result, bail};

use crate::utils::split::split_hi_lo;

pub fn encode_immediate(n: i64, bits: u8, signed: bool) -> Result<u32> {
    let (lo, hi) = split_hi_lo(n, bits, signed);

    if hi != 0 {
        bail!(
            "Immediate '{}' out of range for {}{} ({} ..= {})",
            n,
            if signed { "i" } else { "u" },
            bits,
            if signed { i32::MIN >> (32 - bits) } else { 0 },
            if signed {
                (i32::MAX >> (32 - bits)) as u32
            } else {
                u32::MAX >> (32 - bits)
            },
        );
    }

    Ok(lo)
}
