use anyhow::{Result, bail};

use crate::{assembler::Context, operand::OperandValue, parser::immediate::parse_imm};

pub fn parse_reg(ctx: &Context, op: &OperandValue) -> Result<u32> {
    let reg = match op {
        OperandValue::StringSlice(s) => ctx.constants.get(s).unwrap_or(s),
        OperandValue::Unsigned(n) => err_expect_reg!(n),
        OperandValue::Signed(n) => err_expect_reg!(n),
    };

    match *reg {
        r if let Some(n) = r.strip_prefix("r")
            && let Ok(n) = n.parse::<u32>() =>
        {
            if n > 31 {
                err_reg_out_of_range!(reg);
            }
            Ok(n)
        },

        _ => {
            if parse_imm(ctx, op).is_ok() {
                err_expect_reg!(reg)
            } else {
                err_inval_reg!(reg)
            }
        },
    }
}

macro err_expect_reg($e:expr) {
    bail!("Expected register, found immediate: {}", $e)
}
macro err_inval_reg($e:expr) {
    bail!("Invalid register: {}", $e)
}
macro err_reg_out_of_range($e:expr) {
    bail!("Register number out of range (0..=31): {}", $e)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{assembler::Context, operand::OperandValue, testkit::*};

    fn test(func: fn(&Context, &OperandValue) -> Result<u32>) -> impl Fn(&str) -> String {
        move |s| match func(&Context::test(), &OperandValue::from(s)) {
            Ok(n) => format!("{n}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[test]
    fn parse_reg() {
        let f = test(super::parse_reg);
        assert_snapshot!(f("r0"), @"0");
        assert_snapshot!(f("r9"), @"9");
        assert_snapshot!(f("r27"), @"27");
        assert_snapshot!(f("invalid"), @"Error: Invalid register: invalid");
        assert_snapshot!(f("FOO"), @"Error: Expected register, found immediate: 42");
        assert_snapshot!(f("BAR"), @"Error: Invalid register: BAR");
        assert_snapshot!(f("R0"), @"0");
        assert_snapshot!(f("R1"), @"1");
    }
}
