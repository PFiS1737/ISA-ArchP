use std::num::{IntErrorKind, ParseIntError};

use anyhow::{Result, anyhow, bail};

use crate::{assembler::Context, operand::OperandValue, utils::sig_ext_12_to_32};

pub fn parse_imm(ctx: &Context, imm: &OperandValue) -> Result<Immediate> {
    match imm {
        OperandValue::StringSlice(s) => {
            let imm = ctx.constants.get(s).unwrap_or(s);

            (|| {
                if let Some(hex) = imm.strip_prefix("0x") {
                    Ok(Immediate(u64::from_str_radix(hex, 16)?))
                } else if let Some(bin) = imm.strip_prefix("0b") {
                    Ok(Immediate(u64::from_str_radix(bin, 2)?))
                } else {
                    Ok(Immediate(imm.parse::<i64>()? as u64))
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
        }
        OperandValue::Unsigned(n) => Ok(Immediate(*n as u64)),
        OperandValue::Signed(n) => Ok(Immediate(*n as u64)),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Immediate(pub u64);

impl Immediate {
    pub fn as_i12(&self) -> Result<u32> {
        self.as_field(12, true)
    }
    pub fn as_i32(&self) -> Result<i32> {
        self.as_field(32, true).map(|n| n as i32)
    }
    pub fn try_as_i12(&self) -> Result<(u32, i32)> {
        match self.as_i12() {
            Ok(v) => Ok((0, sig_ext_12_to_32(v))),
            Err(_) => {
                let v = self.as_i32()? as u32;

                let mut hi = v >> 12;
                let lo = v & 0xFFF;

                if lo >= 0x800 {
                    hi += 1;
                }

                Ok((hi, sig_ext_12_to_32(lo)))
            }
        }
    }
}

impl Immediate {
    pub fn as_field(&self, bits: u8, signed: bool) -> Result<u32> {
        if bits == 0 || bits > 32 {
            bail!("Invalid immediate field width: {}", bits);
        }

        let mask = if bits == 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };

        if signed {
            let v = self.0 as i64;

            let min = -(1i64 << (bits - 1));
            let max = (1i64 << (bits - 1)) - 1;

            if v < min || v > max {
                bail!(
                    "Immediate '{}' out of range for i{} ({} ..= {})",
                    v,
                    bits,
                    min,
                    max
                );
            }

            Ok((v as u32) & mask)
        } else {
            let v = self.0;

            let max = (1u64 << bits) - 1;

            if v > max {
                bail!(
                    "Immediate '{}' out of range for u{} (0 ..= {})",
                    v,
                    bits,
                    max
                );
            }

            Ok((v as u32) & mask)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{testkit::assert_snapshot, utils::fmt_hex};

    trait TestFormat {
        fn test_fmt(&self) -> String;
    }

    impl TestFormat for u32 {
        fn test_fmt(&self) -> String {
            fmt_hex(*self)
        }
    }

    impl TestFormat for i32 {
        fn test_fmt(&self) -> String {
            fmt_hex(*self)
        }
    }

    impl TestFormat for (u32, u32) {
        fn test_fmt(&self) -> String {
            format!("({}, {})", fmt_hex(self.0), fmt_hex(self.1))
        }
    }

    impl TestFormat for (u32, i32) {
        fn test_fmt(&self) -> String {
            format!("({}, {})", fmt_hex(self.0), fmt_hex(self.1))
        }
    }

    fn unwrap<T: TestFormat>(res: Result<T>) -> String {
        match res {
            Ok(v) => v.test_fmt(),
            Err(e) => format!("Error: {}", e),
        }
    }

    fn test<'a, T>(s: T) -> String
    where
        OperandValue<'a>: From<T>,
    {
        let imm = parse_imm(&Context::default(), &OperandValue::from(s)).unwrap();
        format!(
            "{}\n{}\n{}",
            unwrap(imm.as_i32()),
            unwrap(imm.as_i12()),
            unwrap(imm.try_as_i12())
        )
    }

    #[test]
    fn signed_operand_value() {
        assert_snapshot!(test(0_i32), @r"
        0
        0
        (0, 0)
        ");

        assert_snapshot!(test(123_i32), @r"
        123
        123
        (0, 123)
        ");

        assert_snapshot!(test(-123_i32), @r"
        -123
        0xF85
        (0, -123)
        ");

        assert_snapshot!(test(2047_i32), @r"
        0x7FF
        0x7FF
        (0, 0x7FF)
        ");

        assert_snapshot!(test(2048_i32), @r"
        0x800
        Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)
        (1, 0xFFFFF800)
        ");

        assert_snapshot!(test(-2048_i32), @r"
        0xFFFFF800
        0x800
        (0, 0xFFFFF800)
        ");

        assert_snapshot!(test(-2049_i32), @r"
        0xFFFFF7FF
        Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)
        (0xFFFFF, 0x7FF)
        ");

        assert_snapshot!(test(123456_i32), @r"
        0x1E240
        Error: Immediate '123456' out of range for i12 (-2048 ..= 2047)
        (30, 0x240)
        ");

        assert_snapshot!(test(-123456_i32), @r"
        0xFFFE1DC0
        Error: Immediate '-123456' out of range for i12 (-2048 ..= 2047)
        (0xFFFE2, 0xFFFFFDC0)
        ");

        assert_snapshot!(test(2147483647_i32), @r"
        0x7FFFFFFF
        Error: Immediate '2147483647' out of range for i12 (-2048 ..= 2047)
        (0x80000, -1)
        ");

        assert_snapshot!(test(-2147483648_i32), @r"
        0x80000000
        Error: Immediate '-2147483648' out of range for i12 (-2048 ..= 2047)
        (0x80000, 0)
        ");
    }

    #[test]
    fn unsigned_operand_value() {
        assert_snapshot!(test(0_u32), @r"
        0
        0
        (0, 0)
        ");

        assert_snapshot!(test(123_u32), @r"
        123
        123
        (0, 123)
        ");

        assert_snapshot!(test(2047_u32), @r"
        0x7FF
        0x7FF
        (0, 0x7FF)
        ");

        assert_snapshot!(test(2048_u32), @r"
        0x800
        Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)
        (1, 0xFFFFF800)
        ");

        assert_snapshot!(test(123456_u32), @r"
        0x1E240
        Error: Immediate '123456' out of range for i12 (-2048 ..= 2047)
        (30, 0x240)
        ");

        assert_snapshot!(test(2147483647_u32), @r"
        0x7FFFFFFF
        Error: Immediate '2147483647' out of range for i12 (-2048 ..= 2047)
        (0x80000, -1)
        ");

        assert_snapshot!(test(2147483648_u32), @r"
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '2147483648' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test(4294967295_u32), @r"
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '4294967295' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        ");
    }

    #[test]
    fn str_operand_value() {
        assert_snapshot!(test("0"), @r"
        0
        0
        (0, 0)
        ");

        assert_snapshot!(test("123"), @r"
        123
        123
        (0, 123)
        ");

        assert_snapshot!(test("-123"), @r"
        -123
        0xF85
        (0, -123)
        ");

        assert_snapshot!(test("2047"), @r"
        0x7FF
        0x7FF
        (0, 0x7FF)
        ");

        assert_snapshot!(test("2048"), @r"
        0x800
        Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)
        (1, 0xFFFFF800)
        ");

        assert_snapshot!(test("-2048"), @r"
        0xFFFFF800
        0x800
        (0, 0xFFFFF800)
        ");

        assert_snapshot!(test("-2049"), @r"
        0xFFFFF7FF
        Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)
        (0xFFFFF, 0x7FF)
        ");

        assert_snapshot!(test("123456"), @r"
        0x1E240
        Error: Immediate '123456' out of range for i12 (-2048 ..= 2047)
        (30, 0x240)
        ");

        assert_snapshot!(test("-123456"), @r"
        0xFFFE1DC0
        Error: Immediate '-123456' out of range for i12 (-2048 ..= 2047)
        (0xFFFE2, 0xFFFFFDC0)
        ");

        assert_snapshot!(test("2147483647"), @r"
        0x7FFFFFFF
        Error: Immediate '2147483647' out of range for i12 (-2048 ..= 2047)
        (0x80000, -1)
        ");

        assert_snapshot!(test("2147483648"), @r"
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '2147483648' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test("-2147483648"), @r"
        0x80000000
        Error: Immediate '-2147483648' out of range for i12 (-2048 ..= 2047)
        (0x80000, 0)
        ");

        assert_snapshot!(test("-2147483649"), @r"
        Error: Immediate '-2147483649' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '-2147483649' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-2147483649' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test("4294967295"), @r"
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '4294967295' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test("4294967296"), @r"
        Error: Immediate '4294967296' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '4294967296' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '4294967296' out of range for i32 (-2147483648 ..= 2147483647)
        ");
    }
}
