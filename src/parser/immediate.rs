use std::num::IntErrorKind;

use anyhow::{Result, anyhow, bail};

use crate::{assembler::Context, operand::OperandValue};

#[derive(Debug, Clone, Copy)]
pub enum ParsedImm {
    Unsigned(u64),
    Signed(i64),
}

pub fn parse_imm(ctx: &Context, imm: &OperandValue) -> Result<ParsedImm> {
    let parse_str = |s: &&str| -> Result<ParsedImm> {
        if let Some(hex) = s.strip_prefix("0x") {
            Ok(ParsedImm::Unsigned(u64::from_str_radix(hex, 16)?))
        } else if let Some(bin) = s.strip_prefix("0b") {
            Ok(ParsedImm::Unsigned(u64::from_str_radix(bin, 2)?))
        } else {
            Ok(ParsedImm::Signed(s.parse::<i64>()?))
        }
    };

    let parsed = match imm {
        OperandValue::StringSlice(s) => {
            if let Some(const_value) = ctx.constants.get(s) {
                parse_str(const_value)
            } else if let Some(&label_addr) = ctx.labels.get_by_left(s) {
                Ok(ParsedImm::Unsigned(label_addr as u64))
            } else {
                parse_str(s)
            }
        }
        OperandValue::Unsigned(n) => Ok(ParsedImm::Unsigned(*n as u64)),
        OperandValue::Signed(n) => Ok(ParsedImm::Signed(*n as i64)),
    };

    parsed.map_err(|err| {
        if let Some(int_err) = err.downcast_ref::<std::num::ParseIntError>() {
            if matches!(
                int_err.kind(),
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
            ) {
                anyhow!("Immediate '{}' out of range of 64-bit integer.", imm)
            } else {
                anyhow!("Invalid immediate: {}", imm)
            }
        } else {
            err
        }
    })
}

impl ParsedImm {
    #[cfg(test)]
    pub fn as_u12(&self) -> Result<u32> {
        self.as_field(12, false)
    }
    pub fn as_i12(&self) -> Result<u32> {
        self.as_field(12, true)
    }
    #[cfg(test)]
    pub fn as_u32(&self) -> Result<u32> {
        self.as_field(32, false)
    }
    pub fn as_i32(&self) -> Result<i32> {
        Ok(self.as_field(32, true)? as i32)
    }
    #[cfg(test)]
    pub fn try_as_u12(&self) -> Result<(u32, u32)> {
        match self.as_u12() {
            Ok(v) => Ok((0, v)),
            Err(_) => {
                let v = self.as_u32()?;
                Ok((v >> 12, v & 0xFFF))
            }
        }
    }
    pub fn try_as_i12(&self) -> Result<(u32, i32)> {
        match self.as_i12() {
            Ok(v) => Ok((0, sig_ext_12_to_32(v) as i32)),
            Err(_) => {
                let v = self.as_i32()? as u32;

                let mut hi = v >> 12;
                let lo = v & 0xFFF;

                if lo >= 0x800 {
                    hi += 1;
                }

                Ok((hi, sig_ext_12_to_32(lo) as i32))
            }
        }
    }
}

fn sig_ext_12_to_32(val: u32) -> u32 {
    if (val & 0x800) != 0 {
        val | 0xFFFFF000
    } else {
        val & 0x00000FFF
    }
}

impl ParsedImm {
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
            let v = match self {
                ParsedImm::Unsigned(v) => *v as i64,
                ParsedImm::Signed(v) => *v,
            };

            let min = -(1i64 << (bits - 1));
            let max = (1i64 << (bits - 1)) - 1;

            if v < min || v > max {
                bail!(
                    "Immediate '{}' out of range for i{} ({} ..= {})",
                    v,
                    bits,
                    min, // TODO: fmt_hex
                    max
                );
            }

            Ok((v as u32) & mask)
        } else {
            let v = match self {
                ParsedImm::Unsigned(v) => *v,
                ParsedImm::Signed(v) => {
                    if *v < 0 {
                        bail!(
                            "Immediate '{}' out of range for u{} (must be >= 0)",
                            v,
                            bits
                        );
                    }
                    *v as u64
                }
            };

            let max = if bits == 32 {
                u32::MAX as u64
            } else {
                (1u64 << bits) - 1
            };

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
            fmt_hex(*self as u32)
        }
    }

    impl TestFormat for (u32, u32) {
        fn test_fmt(&self) -> String {
            format!("({}, {})", fmt_hex(self.0), fmt_hex(self.1))
        }
    }

    impl TestFormat for (u32, i32) {
        fn test_fmt(&self) -> String {
            format!("({}, {})", fmt_hex(self.0), fmt_hex(self.1 as u32))
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
        let imm = parse_imm(&Context::test(), &OperandValue::from(s)).unwrap();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            unwrap(imm.as_u32()),
            unwrap(imm.as_i32()),
            unwrap(imm.as_u12()),
            unwrap(imm.as_i12()),
            unwrap(imm.try_as_u12()),
            unwrap(imm.try_as_i12())
        )
    }

    #[test]
    fn signed_operand_value() {
        assert_snapshot!(test(0_i32), @r"
        0
        0
        0
        0
        (0, 0)
        (0, 0)
        ");

        assert_snapshot!(test(123_i32), @r"
        123
        123
        123
        123
        (0, 123)
        (0, 123)
        ");

        assert_snapshot!(test(-123_i32), @r"
        Error: Immediate '-123' out of range for u32 (must be >= 0)
        0xFFFFFF85
        Error: Immediate '-123' out of range for u12 (must be >= 0)
        0xF85
        Error: Immediate '-123' out of range for u32 (must be >= 0)
        (0, 0xFFFFFF85)
        ");

        assert_snapshot!(test(2047_i32), @r"
        0x7FF
        0x7FF
        0x7FF
        0x7FF
        (0, 0x7FF)
        (0, 0x7FF)
        ");

        assert_snapshot!(test(2048_i32), @r"
        0x800
        0x800
        0x800
        Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)
        (0, 0x800)
        (1, 0xFFFFF800)
        ");

        assert_snapshot!(test(-2048_i32), @r"
        Error: Immediate '-2048' out of range for u32 (must be >= 0)
        0xFFFFF800
        Error: Immediate '-2048' out of range for u12 (must be >= 0)
        0x800
        Error: Immediate '-2048' out of range for u32 (must be >= 0)
        (0, 0xFFFFF800)
        ");

        assert_snapshot!(test(-2049_i32), @r"
        Error: Immediate '-2049' out of range for u32 (must be >= 0)
        0xFFFFF7FF
        Error: Immediate '-2049' out of range for u12 (must be >= 0)
        Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-2049' out of range for u32 (must be >= 0)
        (0xFFFFF, 0x7FF)
        ");

        assert_snapshot!(test(123456_i32), @r"
        0x1E240
        0x1E240
        Error: Immediate '123456' out of range for u12 (0 ..= 4095)
        Error: Immediate '123456' out of range for i12 (-2048 ..= 2047)
        (30, 0x240)
        (30, 0x240)
        ");

        assert_snapshot!(test(-123456_i32), @r"
        Error: Immediate '-123456' out of range for u32 (must be >= 0)
        0xFFFE1DC0
        Error: Immediate '-123456' out of range for u12 (must be >= 0)
        Error: Immediate '-123456' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-123456' out of range for u32 (must be >= 0)
        (0xFFFE2, 0xFFFFFDC0)
        ");

        assert_snapshot!(test(2147483647_i32), @r"
        0x7FFFFFFF
        0x7FFFFFFF
        Error: Immediate '2147483647' out of range for u12 (0 ..= 4095)
        Error: Immediate '2147483647' out of range for i12 (-2048 ..= 2047)
        (0x7FFFF, 0xFFF)
        (0x80000, 0xFFFFFFFF)
        ");

        assert_snapshot!(test(-2147483648_i32), @r"
        Error: Immediate '-2147483648' out of range for u32 (must be >= 0)
        0x80000000
        Error: Immediate '-2147483648' out of range for u12 (must be >= 0)
        Error: Immediate '-2147483648' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-2147483648' out of range for u32 (must be >= 0)
        (0x80000, 0)
        ");
    }

    #[test]
    fn unsigned_operand_value() {
        assert_snapshot!(test(0_u32), @r"
        0
        0
        0
        0
        (0, 0)
        (0, 0)
        ");

        assert_snapshot!(test(123_u32), @r"
        123
        123
        123
        123
        (0, 123)
        (0, 123)
        ");

        assert_snapshot!(test(2047_u32), @r"
        0x7FF
        0x7FF
        0x7FF
        0x7FF
        (0, 0x7FF)
        (0, 0x7FF)
        ");

        assert_snapshot!(test(2048_u32), @r"
        0x800
        0x800
        0x800
        Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)
        (0, 0x800)
        (1, 0xFFFFF800)
        ");

        assert_snapshot!(test(123456_u32), @r"
        0x1E240
        0x1E240
        Error: Immediate '123456' out of range for u12 (0 ..= 4095)
        Error: Immediate '123456' out of range for i12 (-2048 ..= 2047)
        (30, 0x240)
        (30, 0x240)
        ");

        assert_snapshot!(test(2147483647_u32), @r"
        0x7FFFFFFF
        0x7FFFFFFF
        Error: Immediate '2147483647' out of range for u12 (0 ..= 4095)
        Error: Immediate '2147483647' out of range for i12 (-2048 ..= 2047)
        (0x7FFFF, 0xFFF)
        (0x80000, 0xFFFFFFFF)
        ");

        assert_snapshot!(test(2147483648_u32), @r"
        0x80000000
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '2147483648' out of range for u12 (0 ..= 4095)
        Error: Immediate '2147483648' out of range for i12 (-2048 ..= 2047)
        (0x80000, 0)
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test(4294967295_u32), @r"
        0xFFFFFFFF
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '4294967295' out of range for u12 (0 ..= 4095)
        Error: Immediate '4294967295' out of range for i12 (-2048 ..= 2047)
        (0xFFFFF, 0xFFF)
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        ");
    }

    #[test]
    fn str_operand_value() {
        assert_snapshot!(test("0"), @r"
        0
        0
        0
        0
        (0, 0)
        (0, 0)
        ");

        assert_snapshot!(test("123"), @r"
        123
        123
        123
        123
        (0, 123)
        (0, 123)
        ");

        assert_snapshot!(test("-123"), @r"
        Error: Immediate '-123' out of range for u32 (must be >= 0)
        0xFFFFFF85
        Error: Immediate '-123' out of range for u12 (must be >= 0)
        0xF85
        Error: Immediate '-123' out of range for u32 (must be >= 0)
        (0, 0xFFFFFF85)
        ");

        assert_snapshot!(test("2047"), @r"
        0x7FF
        0x7FF
        0x7FF
        0x7FF
        (0, 0x7FF)
        (0, 0x7FF)
        ");

        assert_snapshot!(test("2048"), @r"
        0x800
        0x800
        0x800
        Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)
        (0, 0x800)
        (1, 0xFFFFF800)
        ");

        assert_snapshot!(test("-2048"), @r"
        Error: Immediate '-2048' out of range for u32 (must be >= 0)
        0xFFFFF800
        Error: Immediate '-2048' out of range for u12 (must be >= 0)
        0x800
        Error: Immediate '-2048' out of range for u32 (must be >= 0)
        (0, 0xFFFFF800)
        ");

        assert_snapshot!(test("-2049"), @r"
        Error: Immediate '-2049' out of range for u32 (must be >= 0)
        0xFFFFF7FF
        Error: Immediate '-2049' out of range for u12 (must be >= 0)
        Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-2049' out of range for u32 (must be >= 0)
        (0xFFFFF, 0x7FF)
        ");

        assert_snapshot!(test("123456"), @r"
        0x1E240
        0x1E240
        Error: Immediate '123456' out of range for u12 (0 ..= 4095)
        Error: Immediate '123456' out of range for i12 (-2048 ..= 2047)
        (30, 0x240)
        (30, 0x240)
        ");

        assert_snapshot!(test("-123456"), @r"
        Error: Immediate '-123456' out of range for u32 (must be >= 0)
        0xFFFE1DC0
        Error: Immediate '-123456' out of range for u12 (must be >= 0)
        Error: Immediate '-123456' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-123456' out of range for u32 (must be >= 0)
        (0xFFFE2, 0xFFFFFDC0)
        ");

        assert_snapshot!(test("2147483647"), @r"
        0x7FFFFFFF
        0x7FFFFFFF
        Error: Immediate '2147483647' out of range for u12 (0 ..= 4095)
        Error: Immediate '2147483647' out of range for i12 (-2048 ..= 2047)
        (0x7FFFF, 0xFFF)
        (0x80000, 0xFFFFFFFF)
        ");

        assert_snapshot!(test("2147483648"), @r"
        0x80000000
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '2147483648' out of range for u12 (0 ..= 4095)
        Error: Immediate '2147483648' out of range for i12 (-2048 ..= 2047)
        (0x80000, 0)
        Error: Immediate '2147483648' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test("-2147483648"), @r"
        Error: Immediate '-2147483648' out of range for u32 (must be >= 0)
        0x80000000
        Error: Immediate '-2147483648' out of range for u12 (must be >= 0)
        Error: Immediate '-2147483648' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-2147483648' out of range for u32 (must be >= 0)
        (0x80000, 0)
        ");

        assert_snapshot!(test("-2147483649"), @r"
        Error: Immediate '-2147483649' out of range for u32 (must be >= 0)
        Error: Immediate '-2147483649' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '-2147483649' out of range for u12 (must be >= 0)
        Error: Immediate '-2147483649' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '-2147483649' out of range for u32 (must be >= 0)
        Error: Immediate '-2147483649' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test("4294967295"), @r"
        0xFFFFFFFF
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '4294967295' out of range for u12 (0 ..= 4095)
        Error: Immediate '4294967295' out of range for i12 (-2048 ..= 2047)
        (0xFFFFF, 0xFFF)
        Error: Immediate '4294967295' out of range for i32 (-2147483648 ..= 2147483647)
        ");

        assert_snapshot!(test("4294967296"), @r"
        Error: Immediate '4294967296' out of range for u32 (0 ..= 4294967295)
        Error: Immediate '4294967296' out of range for i32 (-2147483648 ..= 2147483647)
        Error: Immediate '4294967296' out of range for u12 (0 ..= 4095)
        Error: Immediate '4294967296' out of range for i12 (-2048 ..= 2047)
        Error: Immediate '4294967296' out of range for u32 (0 ..= 4294967295)
        Error: Immediate '4294967296' out of range for i32 (-2147483648 ..= 2147483647)
        ");
    }
}
