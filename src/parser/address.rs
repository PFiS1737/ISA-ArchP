use anyhow::{Result, bail};

use crate::{assembler::Context, operand::OperandValue};

pub fn parse_address(ctx: &Context, op: &OperandValue) -> Result<u32> {
    match op {
        OperandValue::StringSlice(s) => {
            let label = ctx.constants.get(s).unwrap_or(s);
            if let Some(&addr) = ctx.labels.get_by_left(label) {
                if addr < 0b111111111111 {
                    Ok(addr as u32)
                } else {
                    bail!("Address out of range for label: '{}' ( = {} )", label, addr)
                }
            } else {
                bail!("Undefined label: {}", label)
            }
        }
        OperandValue::Unsigned(_) | OperandValue::Signed(_) => {
            bail!("Expected address label, got numeric literal: {}", op)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assembler::Context, operand::OperandValue, testkit::*};
    use anyhow::Result;

    fn test(func: fn(&Context, &OperandValue) -> Result<u32>) -> impl Fn(&str) -> String {
        move |s| match func(&Context::test(), &OperandValue::from(s)) {
            Ok(n) => format!("{n}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[test]
    fn parse_reg_d() {
        let f = test(super::parse_address);
        assert_snapshot!(f("start"), @"0");
        assert_snapshot!(f("loop"), @"4");
        assert_snapshot!(f("end"), @"16");
        assert_snapshot!(f("over"), @"Error: Address out of range for label: 'over' ( = 4096 )");
        assert_snapshot!(f("123"), @"Error: Undefined label: 123");
    }
}
