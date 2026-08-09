use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::ops,
    parser::immediate::parse_imm,
    utils::sig_ext::sign_extend,
};

macro_instruction! {
    pub LoadImm32 {
        name: "li",
        operand_count: 2,
        expander: F,
    }
}

const F: ExpandFn = |ctx, _, _, ops| {
    if let Ok((lo, hi)) = parse_imm(ctx, &ops[1]).map(|imm| imm.split(12, true))
        && hi != 0
    {
        let mut ret = vec![("lui", ops![ops[0], hi])];

        if lo != 0 {
            ret.push(("addi", ops![ops[0], ops[0], sign_extend(lo, 12)]))
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
    fn load_imm32() {
        let li = mc_instr("li");

        assert_snapshot!(li(&["r1"]), @"Error: Macro-instruction 'li' requires 2 operands, got 1");
        assert_snapshot!(li(&["r1", "r2"]), @"");
        assert_snapshot!(li(&["123", "123"]), @"");

        assert_snapshot!(li(&["r1", "0x123"]), @"");
        assert_snapshot!(li(&["r1", "0x1234"]), @"lui r1 1; addi r1 r1 0x234");
        assert_snapshot!(li(&["r1", "0x12345678"]), @"lui r1 0x12345; addi r1 r1 0x678");
        assert_snapshot!(li(&["r1", "0x10000000"]), @"lui r1 0x10000");
        assert_snapshot!(li(&["r1", "0xFFFFFFFF"]), @"");

        assert_snapshot!(li(&["r1", "123"]), @"");
        assert_snapshot!(li(&["r1", "3000"]), @"lui r1 1; addi r1 r1 0xFFFFFBB8");
        assert_snapshot!(li(&["r1", "-123"]), @"");
        assert_snapshot!(li(&["r1", "-3000"]), @"lui r1 0xFFFFF; addi r1 r1 0x448");
    }
}
