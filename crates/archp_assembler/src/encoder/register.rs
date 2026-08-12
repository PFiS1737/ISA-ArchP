use anyhow::{Result, bail};

use crate::{assembler::Context, operand::Operand};

pub fn encode_register(ctx: &Context, op: &Operand) -> Result<u32> {
    let Operand::Ident(reg) = op else {
        bail!("Invalid register: {}", op)
    };

    let reg = ctx.aliases.get(reg).unwrap_or(reg);

    Ok(match *reg {
        "r0" | "zero" => 0,
        "r1" | "ra" => 1,
        "r2" | "sp" => 2,
        "r3" | "gp" => 3,
        "r4" | "tp" => 4,
        "r5" | "t0" => 5,
        "r6" | "t1" => 6,
        "r7" | "t2" => 7,
        "r8" | "s0" | "fp" => 8,
        "r9" | "s1" => 9,
        "r10" | "a0" => 10,
        "r11" | "a1" => 11,
        "r12" | "a2" => 12,
        "r13" | "a3" => 13,
        "r14" | "a4" => 14,
        "r15" | "a5" => 15,
        "r16" | "a6" => 16,
        "r17" | "a7" => 17,
        "r18" | "s2" => 18,
        "r19" | "s3" => 19,
        "r20" | "s4" => 20,
        "r21" | "s5" => 21,
        "r22" | "s6" => 22,
        "r23" | "s7" => 23,
        "r24" | "s8" => 24,
        "r25" | "s9" => 25,
        "r26" | "s10" => 26,
        "r27" | "s11" => 27,
        "r28" | "t3" => 28,
        "r29" | "t4" => 29,
        "r30" | "t5" => 30,
        "r31" | "t6" => 31,

        _ => bail!("Invalid register: {}", reg),
    })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use insta::assert_snapshot;

    use crate::{assembler::Context, operand::Operand};

    fn test(func: fn(&Context, &Operand) -> Result<u32>) -> impl Fn(&str) -> String {
        move |s| match func(&Context::test(), &Operand::from(s)) {
            Ok(n) => format!("{n}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[test]
    fn parse_reg() {
        let f = test(super::encode_register);
        assert_snapshot!(f("r0"), @"0");
        assert_snapshot!(f("r9"), @"9");
        assert_snapshot!(f("r27"), @"27");
        assert_snapshot!(f("invalid"), @"Error: Invalid register: invalid");
        assert_snapshot!(f("FOO"), @"Error: Invalid register: FOO");
        assert_snapshot!(f("BAR"), @"Error: Invalid register: BAR");
        assert_snapshot!(f("R0"), @"0");
        assert_snapshot!(f("R1"), @"1");
    }
}
