use anyhow::{Result, bail};

pub fn encode_address_as(addr: i64, bits: u8, base: u32) -> Result<u32> {
    if bits == 0 || bits > 32 {
        bail!("Invalid address offset field width: {}", bits);
    }

    let mask = if bits == 32 {
        u32::MAX
    } else {
        (1u32 << bits) - 1
    };

    let v = (addr - (base as i64)) >> 1;

    let min = -(1i64 << (bits - 1));
    let max = (1i64 << (bits - 1)) - 1;

    if v < min || v > max {
        bail!(
            "Address offset '{}' out of range for i{} ({} ..= {})",
            v,
            bits,
            min,
            max
        );
    }

    Ok((v as u32) & mask)
}
