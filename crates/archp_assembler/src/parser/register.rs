use anyhow::{Result, bail};

use crate::{assembler::Context, operand::OperandValue, parser::immediate::parse_imm};

pub fn parse_reg(ctx: &Context, op: &OperandValue) -> Result<u32> {
    let reg = match op {
        OperandValue::StringSlice(s) => ctx.constants.get(s).unwrap_or(s),
        OperandValue::Integer(n, _) => err_expect_reg!(n),
    };

    Ok(match *reg {
        r if let Some(n) = r.strip_prefix("r")
            && let Ok(n) = n.parse::<u32>() =>
        {
            if n > 31 {
                err_reg_out_of_range!(reg);
            }

            n
        },

        "zero" => 0,
        "ra" => 1,
        "sp" => 2,
        "gp" => 3,
        "tp" => 4,
        "t0" => 5,
        "t1" => 6,
        "t2" => 7,
        "s0" => 8,
        "s1" => 9,
        "a0" => 10,
        "a1" => 11,
        "a2" => 12,
        "a3" => 13,
        "a4" => 14,
        "a5" => 15,
        "a6" => 16,
        "a7" => 17,
        "s2" => 18,
        "s3" => 19,
        "s4" => 20,
        "s5" => 21,
        "s6" => 22,
        "s7" => 23,
        "s8" => 24,
        "s9" => 25,
        "s10" => 26,
        "s11" => 27,
        "t3" => 28,
        "t4" => 29,
        "t5" => 30,
        "t6" => 31,

        "fp" => 8,

        _ => {
            if parse_imm(ctx, op).is_ok() {
                err_expect_reg!(reg)
            } else {
                err_inval_reg!(reg)
            }
        },
    })
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
