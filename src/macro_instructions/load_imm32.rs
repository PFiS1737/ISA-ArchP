use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_imm,
};

macro_instruction! {
    name: "li",
    operand_count: 2,
    expander: F,
}

pub const F: ExpandFn = |ctx, _, cond, ops| {
    if let Ok((hi, lo)) = parse_imm(ctx, &ops[1]).and_then(|imm| imm.try_as_i12())
        && hi != 0
    {
        if ops[0] != "tmp".into() && cond.is_none() {
            return Some(vec![
                ("lui", None, op_values![ops[0], hi]),
                ("addi", None, op_values![ops[0], ops[0], lo]),
            ]);
        }

        let mut ret = vec![
            ("lui", None, op_values!["tmp", hi]),
            ("addi", None, op_values!["tmp", "tmp", lo]),
        ];

        if ops[0] == "tmp".into() {
            return Some(ret);
        }

        if cond.is_some() {
            ret.push(("mv", cond, op_values![ops[0], "tmp"]))
        }

        Some(ret)
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn li_imm32() {
        let li = mc_instr("li");

        assert_snapshot!(li("", &["r1"]), @"Error: Macro-instruction 'li' requires 2 operands, got 1");
        assert_snapshot!(li("", &["r1", "r2"]), @"");
        assert_snapshot!(li("", &["123", "123"]), @"");
        assert_snapshot!(li("", &["kb", "123"]), @"");

        assert_snapshot!(li("", &["r1", "0x123"]), @"");
        assert_snapshot!(li("", &["r1", "0x1234"]), @"lui r1 1; addi r1 r1 0x234");
        assert_snapshot!(li("", &["r1", "0x12345678"]), @"lui r1 0x12345; addi r1 r1 0x678");

        assert_snapshot!(li("", &["r1", "123"]), @"");
        assert_snapshot!(li("", &["r1", "3000"]), @"lui r1 1; addi r1 r1 0xFFFFFBB8");
        assert_snapshot!(li("", &["r1", "-123"]), @"");
        assert_snapshot!(li("", &["r1", "-3000"]), @"lui r1 0xFFFFF; addi r1 r1 0x448");

        assert_snapshot!(li("eq", &["r1", "0x123"]), @"");
        assert_snapshot!(li("eq", &["r1", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; mv.eq r1 tmp");
    }
}
