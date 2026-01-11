use anyhow::{Result, bail};

use crate::{assembler::Context, operand::OperandValue};

pub fn parse_address(ctx: &Context, op: &OperandValue) -> Result<Address> {
    match op {
        OperandValue::StringSlice(s) => {
            let label = ctx.constants.get(s).unwrap_or(s);
            if let Some(&addr) = ctx.labels.get(label) {
                Ok(Address(addr as u32))
            } else {
                bail!("Undefined label: {}", label)
            }
        }
        OperandValue::Unsigned(_) | OperandValue::Signed(_) => {
            bail!("Expected address label, got numeric literal: {}", op)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address(pub u32);

impl Address {
    pub fn as_i12(&self, base: u32) -> Result<u32> {
        let v = ((self.0 as i32) - (base as i32)) >> 1;

        if !(-2048..=2047).contains(&v) {
            bail!("Address offset {} out of range for i12 ( -2048..=2047 )", v);
        }

        Ok((v as u32) & 0xFFF)
    }

    pub fn as_i20(&self, base: u32) -> Result<u32> {
        let v = ((self.0 as i32) - (base as i32)) >> 1;

        if !(-524288..=524287).contains(&v) {
            bail!(
                "Address offset {} out of range for i20 ( -524288..=524287 )",
                v
            );
        }

        Ok((v as u32) & 0xFFFFF)
    }

    #[cfg(test)]
    pub fn try_as_i12(&self, base: u32) -> (u32, u32) {
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
        match bits {
            12 => self.as_i12(base),
            20 => self.as_i20(base),
            _ => panic!("Internal Error: Unsupported address field size: {}", bits),
        }
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
        assert_snapshot!(f("start"), @"(0, 0xFFFFFFFA)");
        assert_snapshot!(f("loop"), @"(0, 0xFFFFFFFC)");
        assert_snapshot!(f("end"), @"(0, 0x7F9)");
        assert_snapshot!(f("over"), @"(0x123, 0x44A)");
        assert_snapshot!(f("123"), @"Error: Undefined label: 123");
    }

    fn test_addr(addr: u32, base: u32) -> String {
        let addr = Address(addr);
        let (hi, lo) = addr.try_as_i12(base);
        format!("({}, {})", fmt_hex(hi), fmt_hex(lo))
    }

    #[test]
    fn addr_as_i12() {
        assert_snapshot!(test_addr(0x1000, 0), @"(1, 0)");
        assert_snapshot!(test_addr(0x1FFE, 0), @"(2, 0xFFFFFFFE)");
        assert_snapshot!(test_addr(0x1000, 0x1000), @"(0, 0)");
        assert_snapshot!(test_addr(0x1000, 0xFFE), @"(0, 1)");
    }
}
