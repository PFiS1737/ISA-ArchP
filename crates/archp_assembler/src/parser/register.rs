use anyhow::{Result, bail};

use crate::{assembler::Context, operand::OperandValue, parser::immediate::parse_imm};

pub fn parse_reg_d(ctx: &Context, op: &OperandValue) -> Result<u32> {
    let reg = match op {
        OperandValue::StringSlice(s) => ctx.constants.get(s).unwrap_or(s),
        OperandValue::Unsigned(n) => err_expect_reg!(n),
        OperandValue::Signed(n) => err_expect_reg!(n),
    };

    match *reg {
        "r0" => Ok(0),

        "io" => Ok(26),
        "tmp" => Ok(31),

        "kb" | "rng" => err_read_only_reg!(reg),

        r if let Some(n) = r.strip_prefix("r")
            && let Ok(n) = n.parse::<u32>() =>
        {
            if n > 24 {
                err_reg_out_of_range!(reg, "1");
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

pub fn parse_reg_s(ctx: &Context, op: &OperandValue) -> Result<u32> {
    let reg = match op {
        OperandValue::StringSlice(s) => ctx.constants.get(s).unwrap_or(s),
        OperandValue::Unsigned(n) => err_expect_reg!(n),
        OperandValue::Signed(n) => err_expect_reg!(n),
    };

    match *reg {
        "r0" => Ok(0),

        "io" => Ok(26),
        "kb" => Ok(27),
        "rng" => Ok(28),
        "tmp" => Ok(31),

        r if let Some(n) = r.strip_prefix("r")
            && let Ok(n) = n.parse::<u32>() =>
        {
            if n > 24 {
                err_reg_out_of_range!(reg, "0");
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
macro err_reg_out_of_range($e:expr, $s:expr) {
    bail!("Register number out of range ({}-24): {}", $s, $e)
}
macro err_read_only_reg($e:expr) {
    bail!("Register '{}' is raed-only", $e)
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
    fn parse_reg_d() {
        let f = test(super::parse_reg_d);
        assert_snapshot!(f("r0"), @"0");
        assert_snapshot!(f("r9"), @"9");
        assert_snapshot!(f("r27"), @"Error: Register number out of range (1-24): r27");
        assert_snapshot!(f("kb"), @"Error: Register 'kb' is raed-only");
        assert_snapshot!(f("invalid"), @"Error: Invalid register: invalid");
        assert_snapshot!(f("FOO"), @"Error: Expected register, found immediate: 42");
        assert_snapshot!(f("BAR"), @"Error: Invalid register: BAR");
        assert_snapshot!(f("R0"), @"0");
        assert_snapshot!(f("R1"), @"1");
    }

    #[test]
    fn parse_reg_s() {
        let f = test(super::parse_reg_s);
        assert_snapshot!(f("r0"), @"0");
        assert_snapshot!(f("r15"), @"15");
        assert_snapshot!(f("r30"), @"Error: Register number out of range (0-24): r30");
        assert_snapshot!(f("invalid"), @"Error: Invalid register: invalid");
        assert_snapshot!(f("FOO"), @"Error: Expected register, found immediate: 42");
        assert_snapshot!(f("BAR"), @"Error: Invalid register: BAR");
        assert_snapshot!(f("R0"), @"0");
        assert_snapshot!(f("R1"), @"1");
    }
}
