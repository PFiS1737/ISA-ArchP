use std::num::{IntErrorKind, ParseIntError};

use anyhow::{Result, anyhow, bail};

use crate::{assembler::Context, operand::OperandValue, utils::sig_ext::sign_extend};

pub fn parse_imm(ctx: &Context, imm: &OperandValue) -> Result<Immediate> {
    match imm {
        OperandValue::StringSlice(s) => {
            let imm = ctx.constants.get(s).unwrap_or(s);

            (|| {
                if let Some(hex) = imm.strip_prefix("0x") {
                    Ok(Immediate {
                        raw: u64::from_str_radix(hex, 16)? as u32,
                        bits: hex.len() as u8 * 4,
                    })
                } else if let Some(bin) = imm.strip_prefix("0b") {
                    Ok(Immediate {
                        raw: u64::from_str_radix(bin, 2)? as u32,
                        bits: bin.len() as u8,
                    })
                } else if let Some(num) = imm.strip_prefix("-") {
                    Ok(Immediate {
                        raw: -num.parse::<i64>()? as u32,
                        bits: 32,
                    })
                } else {
                    Ok(Immediate {
                        raw: imm.parse::<i64>()? as u32,
                        bits: 32,
                    })
                }
            })()
            .map_err(|err: ParseIntError| {
                if matches!(
                    err.kind(),
                    IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
                ) {
                    anyhow!("Immediate '{}' out of range of 64-bit integer.", imm)
                } else {
                    anyhow!("Invalid immediate: {}", imm)
                }
            })
        },
        OperandValue::Integer(n, bits) => Ok(Immediate {
            raw: *n,
            bits: *bits,
        }),
    }
}

pub fn parse_imm_as(ctx: &Context, imm: &OperandValue, bits: u8, signed: bool) -> Result<u32> {
    let (low, hi) = parse_imm(ctx, imm)?.split(bits, signed);

    if hi != 0 {
        bail!(
            "Immediate '{}' out of range for {}{} ({} ..= {})",
            imm,
            if signed { "i" } else { "u" },
            bits,
            if signed { i32::MIN >> (32 - bits) } else { 0 },
            if signed {
                (i32::MAX >> (32 - bits)) as u32
            } else {
                u32::MAX >> (32 - bits)
            }
        );
    }

    Ok(low)
}

#[derive(Debug, Clone, Copy)]
pub struct Immediate {
    pub raw: u32,
    pub bits: u8,
}

impl Immediate {
    pub fn is_zero(&self) -> bool {
        self.raw == 0
    }

    /// 'signed' 为 'true' 时，保证输出满足 'self.raw == (hi << bits) + sig_ext(low, bits)'
    /// 'signed' 为 'false' 时，保证输出满足 'self.raw == (hi << bits) + low'
    pub fn split(&self, bits: u8, signed: bool) -> (u32, u32) {
        assert!(bits > 0 && bits <= 32);

        let mask = if bits == 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };

        if signed {
            let raw = sign_extend(self.raw, self.bits);

            let low = raw & mask;

            if raw == sign_extend(low, bits) {
                (low, 0)
            } else {
                let mut hi = if bits == 32 { 0 } else { raw >> bits };

                if bits < 32 && low >= (1u32 << (bits - 1)) {
                    hi = hi.wrapping_add(1);
                }
                (low, hi)
            }
        } else {
            let low = self.raw & mask;

            let hi = if bits == 32 { 0 } else { self.raw >> bits };

            (low, hi)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{testkit::assert_snapshot, utils::fmt::fmt_hex};

    fn test_and_fmt(imm: Immediate, bits: u8, signed: bool) -> String {
        let (low, hi) = imm.split(bits, signed);

        if signed {
            assert_eq!(
                sign_extend(imm.raw, imm.bits) as i32,
                ((hi << bits) as i32).wrapping_add(sign_extend(low, bits) as i32)
            )
        } else {
            assert_eq!(imm.raw, (hi << bits) + low)
        }

        format!(
            "({}, {}) - {}{}",
            fmt_hex(low),
            fmt_hex(hi),
            if signed { "i" } else { "u" },
            bits
        )
    }

    fn test<'a, T>(s: T) -> String
    where
        OperandValue<'a>: From<T>,
    {
        let imm = parse_imm(&Context::default(), &OperandValue::from(s)).unwrap();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            fmt_hex(imm.raw),
            test_and_fmt(imm, 5, true),
            test_and_fmt(imm, 5, false),
            test_and_fmt(imm, 12, true),
            test_and_fmt(imm, 12, false),
            test_and_fmt(imm, 20, true),
            test_and_fmt(imm, 20, false),
        )
    }

    #[test]
    fn signed_operand_value() {
        assert_snapshot!(test(0), @"
        0
        (0, 0) - i5
        (0, 0) - u5
        (0, 0) - i12
        (0, 0) - u12
        (0, 0) - i20
        (0, 0) - u20
        ");

        assert_snapshot!(test(31), @"
        31
        (31, 1) - i5
        (31, 0) - u5
        (31, 0) - i12
        (31, 0) - u12
        (31, 0) - i20
        (31, 0) - u20
        ");

        assert_snapshot!(test(32), @"
        32
        (0, 1) - i5
        (0, 1) - u5
        (32, 0) - i12
        (32, 0) - u12
        (32, 0) - i20
        (32, 0) - u20
        ");

        assert_snapshot!(test(123), @"
        123
        (27, 4) - i5
        (27, 3) - u5
        (123, 0) - i12
        (123, 0) - u12
        (123, 0) - i20
        (123, 0) - u20
        ");

        assert_snapshot!(test(-123), @"
        0xFFFFFF85
        (5, 0x7FFFFFC) - i5
        (5, 0x7FFFFFC) - u5
        (0xF85, 0) - i12
        (0xF85, 0xFFFFF) - u12
        (0xFFF85, 0) - i20
        (0xFFF85, 0xFFF) - u20
        ");

        assert_snapshot!(test(2047), @"
        0x7FF
        (31, 64) - i5
        (31, 63) - u5
        (0x7FF, 0) - i12
        (0x7FF, 0) - u12
        (0x7FF, 0) - i20
        (0x7FF, 0) - u20
        ");

        assert_snapshot!(test(2048), @"
        0x800
        (0, 64) - i5
        (0, 64) - u5
        (0x800, 1) - i12
        (0x800, 0) - u12
        (0x800, 0) - i20
        (0x800, 0) - u20
        ");

        assert_snapshot!(test(-2048), @"
        0xFFFFF800
        (0, 0x7FFFFC0) - i5
        (0, 0x7FFFFC0) - u5
        (0x800, 0) - i12
        (0x800, 0xFFFFF) - u12
        (0xFF800, 0) - i20
        (0xFF800, 0xFFF) - u20
        ");

        assert_snapshot!(test(-2049), @"
        0xFFFFF7FF
        (31, 0x7FFFFC0) - i5
        (31, 0x7FFFFBF) - u5
        (0x7FF, 0xFFFFF) - i12
        (0x7FF, 0xFFFFF) - u12
        (0xFF7FF, 0) - i20
        (0xFF7FF, 0xFFF) - u20
        ");

        assert_snapshot!(test(123456), @"
        0x1E240
        (0, 0xF12) - i5
        (0, 0xF12) - u5
        (0x240, 30) - i12
        (0x240, 30) - u12
        (0x1E240, 0) - i20
        (0x1E240, 0) - u20
        ");

        assert_snapshot!(test(-123456), @"
        0xFFFE1DC0
        (0, 0x7FFF0EE) - i5
        (0, 0x7FFF0EE) - u5
        (0xDC0, 0xFFFE2) - i12
        (0xDC0, 0xFFFE1) - u12
        (0xE1DC0, 0) - i20
        (0xE1DC0, 0xFFF) - u20
        ");

        assert_snapshot!(test(2147483647), @"
        0x7FFFFFFF
        (31, 0x4000000) - i5
        (31, 0x3FFFFFF) - u5
        (0xFFF, 0x80000) - i12
        (0xFFF, 0x7FFFF) - u12
        (0xFFFFF, 0x800) - i20
        (0xFFFFF, 0x7FF) - u20
        ");

        assert_snapshot!(test(-2147483648), @"
        0x80000000
        (0, 0x4000000) - i5
        (0, 0x4000000) - u5
        (0, 0x80000) - i12
        (0, 0x80000) - u12
        (0, 0x800) - i20
        (0, 0x800) - u20
        ");

        assert_snapshot!(test(0xFFFFFFFF_u32), @"
        0xFFFFFFFF
        (31, 0) - i5
        (31, 0x7FFFFFF) - u5
        (0xFFF, 0) - i12
        (0xFFF, 0xFFFFF) - u12
        (0xFFFFF, 0) - i20
        (0xFFFFF, 0xFFF) - u20
        ")
    }

    #[test]
    fn str_operand_value() {
        assert_snapshot!(test("0"), @"
        0
        (0, 0) - i5
        (0, 0) - u5
        (0, 0) - i12
        (0, 0) - u12
        (0, 0) - i20
        (0, 0) - u20
        ");

        assert_snapshot!(test("123"), @"
        123
        (27, 4) - i5
        (27, 3) - u5
        (123, 0) - i12
        (123, 0) - u12
        (123, 0) - i20
        (123, 0) - u20
        ");

        assert_snapshot!(test("-123"), @"
        0xFFFFFF85
        (5, 0x7FFFFFC) - i5
        (5, 0x7FFFFFC) - u5
        (0xF85, 0) - i12
        (0xF85, 0xFFFFF) - u12
        (0xFFF85, 0) - i20
        (0xFFF85, 0xFFF) - u20
        ");

        assert_snapshot!(test("2047"), @"
        0x7FF
        (31, 64) - i5
        (31, 63) - u5
        (0x7FF, 0) - i12
        (0x7FF, 0) - u12
        (0x7FF, 0) - i20
        (0x7FF, 0) - u20
        ");

        assert_snapshot!(test("2048"), @"
        0x800
        (0, 64) - i5
        (0, 64) - u5
        (0x800, 1) - i12
        (0x800, 0) - u12
        (0x800, 0) - i20
        (0x800, 0) - u20
        ");

        assert_snapshot!(test("-2048"), @"
        0xFFFFF800
        (0, 0x7FFFFC0) - i5
        (0, 0x7FFFFC0) - u5
        (0x800, 0) - i12
        (0x800, 0xFFFFF) - u12
        (0xFF800, 0) - i20
        (0xFF800, 0xFFF) - u20
        ");

        assert_snapshot!(test("-2049"), @"
        0xFFFFF7FF
        (31, 0x7FFFFC0) - i5
        (31, 0x7FFFFBF) - u5
        (0x7FF, 0xFFFFF) - i12
        (0x7FF, 0xFFFFF) - u12
        (0xFF7FF, 0) - i20
        (0xFF7FF, 0xFFF) - u20
        ");

        assert_snapshot!(test("123456"), @"
        0x1E240
        (0, 0xF12) - i5
        (0, 0xF12) - u5
        (0x240, 30) - i12
        (0x240, 30) - u12
        (0x1E240, 0) - i20
        (0x1E240, 0) - u20
        ");

        assert_snapshot!(test("-123456"), @"
        0xFFFE1DC0
        (0, 0x7FFF0EE) - i5
        (0, 0x7FFF0EE) - u5
        (0xDC0, 0xFFFE2) - i12
        (0xDC0, 0xFFFE1) - u12
        (0xE1DC0, 0) - i20
        (0xE1DC0, 0xFFF) - u20
        ");

        assert_snapshot!(test("2147483647"), @"
        0x7FFFFFFF
        (31, 0x4000000) - i5
        (31, 0x3FFFFFF) - u5
        (0xFFF, 0x80000) - i12
        (0xFFF, 0x7FFFF) - u12
        (0xFFFFF, 0x800) - i20
        (0xFFFFF, 0x7FF) - u20
        ");

        assert_snapshot!(test("-2147483647"), @"
        0x80000001
        (1, 0x4000000) - i5
        (1, 0x4000000) - u5
        (1, 0x80000) - i12
        (1, 0x80000) - u12
        (1, 0x800) - i20
        (1, 0x800) - u20
        ");

        assert_snapshot!(test("2147483648"), @"
        0x80000000
        (0, 0x4000000) - i5
        (0, 0x4000000) - u5
        (0, 0x80000) - i12
        (0, 0x80000) - u12
        (0, 0x800) - i20
        (0, 0x800) - u20
        ");

        assert_snapshot!(test("-2147483648"), @"
        0x80000000
        (0, 0x4000000) - i5
        (0, 0x4000000) - u5
        (0, 0x80000) - i12
        (0, 0x80000) - u12
        (0, 0x800) - i20
        (0, 0x800) - u20
        ");

        assert_snapshot!(test("2147483649"), @"
        0x80000001
        (1, 0x4000000) - i5
        (1, 0x4000000) - u5
        (1, 0x80000) - i12
        (1, 0x80000) - u12
        (1, 0x800) - i20
        (1, 0x800) - u20
        ");

        assert_snapshot!(test("-2147483649"), @"
        0x7FFFFFFF
        (31, 0x4000000) - i5
        (31, 0x3FFFFFF) - u5
        (0xFFF, 0x80000) - i12
        (0xFFF, 0x7FFFF) - u12
        (0xFFFFF, 0x800) - i20
        (0xFFFFF, 0x7FF) - u20
        ");

        assert_snapshot!(test("4294967295"), @"
        0xFFFFFFFF
        (31, 0) - i5
        (31, 0x7FFFFFF) - u5
        (0xFFF, 0) - i12
        (0xFFF, 0xFFFFF) - u12
        (0xFFFFF, 0) - i20
        (0xFFFFF, 0xFFF) - u20
        ");

        assert_snapshot!(test("-4294967295"), @"
        1
        (1, 0) - i5
        (1, 0) - u5
        (1, 0) - i12
        (1, 0) - u12
        (1, 0) - i20
        (1, 0) - u20
        ");

        assert_snapshot!(test("4294967296"), @"
        0
        (0, 0) - i5
        (0, 0) - u5
        (0, 0) - i12
        (0, 0) - u12
        (0, 0) - i20
        (0, 0) - u20
        ");

        assert_snapshot!(test("-4294967296"), @"
        0
        (0, 0) - i5
        (0, 0) - u5
        (0, 0) - i12
        (0, 0) - u12
        (0, 0) - i20
        (0, 0) - u20
        ");

        assert_snapshot!(test("4294967297"), @"
        1
        (1, 0) - i5
        (1, 0) - u5
        (1, 0) - i12
        (1, 0) - u12
        (1, 0) - i20
        (1, 0) - u20
        ");

        assert_snapshot!(test("-4294967297"), @"
        0xFFFFFFFF
        (31, 0) - i5
        (31, 0x7FFFFFF) - u5
        (0xFFF, 0) - i12
        (0xFFF, 0xFFFFF) - u12
        (0xFFFFF, 0) - i20
        (0xFFFFF, 0xFFF) - u20
        ");

        assert_snapshot!(test("0xFFF"), @"
        0xFFF
        (31, 0) - i5
        (31, 127) - u5
        (0xFFF, 0) - i12
        (0xFFF, 0) - u12
        (0xFFFFF, 0) - i20
        (0xFFF, 0) - u20
        "
        );

        assert_snapshot!(test("0xFFFF"), @"
        0xFFFF
        (31, 0) - i5
        (31, 0x7FF) - u5
        (0xFFF, 0) - i12
        (0xFFF, 15) - u12
        (0xFFFFF, 0) - i20
        (0xFFFF, 0) - u20
        ");

        assert_snapshot!(test("0x7FFF"), @"
        0x7FFF
        (31, 0x400) - i5
        (31, 0x3FF) - u5
        (0xFFF, 8) - i12
        (0xFFF, 7) - u12
        (0x7FFF, 0) - i20
        (0x7FFF, 0) - u20
        ");

        assert_snapshot!(test("0xFFFFFFFF"), @"
        0xFFFFFFFF
        (31, 0) - i5
        (31, 0x7FFFFFF) - u5
        (0xFFF, 0) - i12
        (0xFFF, 0xFFFFF) - u12
        (0xFFFFF, 0) - i20
        (0xFFFFF, 0xFFF) - u20
        ");

        assert_snapshot!(test("0x1234"), @"
        0x1234
        (20, 146) - i5
        (20, 145) - u5
        (0x234, 1) - i12
        (0x234, 1) - u12
        (0x1234, 0) - i20
        (0x1234, 0) - u20
        ");

        assert_snapshot!(test("0xABC"), @"
        0xABC
        (28, 0x7FFFFD6) - i5
        (28, 85) - u5
        (0xABC, 0) - i12
        (0xABC, 0) - u12
        (0xFFABC, 0) - i20
        (0xABC, 0) - u20
        ");

        assert_snapshot!(test("0xABCDE"), @"
        0xABCDE
        (30, 0x7FFD5E7) - i5
        (30, 0x55E6) - u5
        (0xCDE, 0xFFFAC) - i12
        (0xCDE, 171) - u12
        (0xABCDE, 0) - i20
        (0xABCDE, 0) - u20
        ");

        assert_snapshot!(test("0b1000001"), @"
        65
        (1, 0x7FFFFFE) - i5
        (1, 2) - u5
        (0xFC1, 0) - i12
        (65, 0) - u12
        (0xFFFC1, 0) - i20
        (65, 0) - u20
        ");

        assert_snapshot!(test("0b0100001"), @"
        33
        (1, 1) - i5
        (1, 1) - u5
        (33, 0) - i12
        (33, 0) - u12
        (33, 0) - i20
        (33, 0) - u20
        ");
    }
}
