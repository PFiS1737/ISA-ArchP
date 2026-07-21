use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::immediate::parse_imm,
    utils::sig_ext::sign_extend,
};

macro_instruction! {
    pub ColImm32 {
        name: "col",
        operand_count: 1,
        expander: F,
    }
}

const F: ExpandFn = |ctx, _, _, ops| {
    if let Ok((lo, hi)) = parse_imm(ctx, &ops[0]).map(|imm| imm.split(12, true))
        && hi != 0
    {
        Some(vec![
            ("lui", op_values!["r31", hi]),
            ("colr", op_values!["r0", "r31", sign_extend(lo, 12)]),
        ])
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn col_imm32() {
        let col = mc_instr("col");

        assert_snapshot!(col(&["0x123"]), @"");
        assert_snapshot!(col(&["0x1234"]), @"lui r31 1; colr r0 r31 0x234");
        assert_snapshot!(col(&["0x12345678"]), @"lui r31 0x12345; colr r0 r31 0x678");
        assert_snapshot!(col(&["0xFFFFFFFF"]), @"");
        assert_snapshot!(col(&["0x181A1B00"]), @"lui r31 0x181A2; colr r0 r31 0xFFFFFB00");

        assert_snapshot!(col(&["123"]), @"");
        assert_snapshot!(col(&["3000"]), @"lui r31 1; colr r0 r31 0xFFFFFBB8");
        assert_snapshot!(col(&["-123"]), @"");
        assert_snapshot!(col(&["-3000"]), @"lui r31 0xFFFFF; colr r0 r31 0x448");
    }
}
