use anyhow::{Result, bail};

use crate::{assembler::Context, operand::OperandValue};

pub fn parse_address(ctx: &Context, op: &OperandValue) -> Result<Address> {
    match op {
        OperandValue::StringSlice(s) => {
            let label = ctx.constants.get(s).unwrap_or(s);
            if let Some(&addr) = ctx.labels.get(label) {
                Ok(Address(addr as u64))
            } else {
                bail!("Undefined label: {}", label)
            }
        }
        OperandValue::Unsigned(n) => Ok(Address(*n as u64)),
        OperandValue::Signed(n) => Ok(Address(*n as u64)),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Address(pub u64);

impl Address {
    pub fn as_i12(&self, base: u32) -> Result<u32> {
        self.as_field(12, base)
    }

    pub fn try_as_i12(&self, base: u32) -> (u32, i32) {
        use crate::utils::sig_ext_12_to_32;

        match self.as_i12(base) {
            Ok(v) => (0, sig_ext_12_to_32(v)),
            Err(_) => {
                let v = ((self.0 as i32) - (base as i32)) as u32;

                let mut hi = v >> 12;
                let lo = v & 0xFFF;

                if lo >= 0x800 {
                    hi += 1;
                }

                (hi, sig_ext_12_to_32(lo))
            }
        }
    }
}

impl Address {
    pub fn as_field(&self, bits: u8, base: u32) -> Result<u32> {
        if bits == 0 || bits > 32 {
            bail!("Invalid address offset field width: {}", bits);
        }

        let mask = if bits == 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        };

        let v = ((self.0 as i64) - (base as i64)) >> 1;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{testkit::assert_snapshot, utils::fmt_hex};

    fn test_parser(
        func: fn(&Context, &OperandValue) -> Result<Address>,
    ) -> impl Fn(&str) -> String {
        move |s| match func(&Context::test(), &OperandValue::from(s)) {
            Ok(n) => {
                let (hi, lo) = n.try_as_i12(12);
                format!("({}, {})", fmt_hex(hi), fmt_hex(lo))
            }
            Err(e) => format!("Error: {e}"),
        }
    }

    #[test]
    fn parse_addr() {
        let f = test_parser(super::parse_address);
        assert_snapshot!(f("start"), @"(0, -6)");
        assert_snapshot!(f("loop"), @"(0, -4)");
        assert_snapshot!(f("end"), @"(0, 0x7F9)");
        assert_snapshot!(f("over"), @"(0x123, 0x44A)");
        assert_snapshot!(f("123"), @"Error: Undefined label: 123");
    }

    fn test_addr(addr: u32, base: u32) -> String {
        let addr = Address(addr as u64);
        let (hi, lo) = addr.try_as_i12(base);
        format!("({}, {})", fmt_hex(hi), fmt_hex(lo))
    }

    #[test]
    fn addr_as_i12() {
        assert_snapshot!(test_addr(0x1000, 0), @"(1, 0)");
        assert_snapshot!(test_addr(0x1FFE, 0), @"(2, -2)");
        assert_snapshot!(test_addr(0x1000, 0x1000), @"(0, 0)");
        assert_snapshot!(test_addr(0x1000, 0xFFE), @"(0, 1)");
    }
}
