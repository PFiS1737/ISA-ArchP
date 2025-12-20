use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_imm,
};

macro_instruction! {
    name: "pushi",
    operand_count: 1,
    expander: F,
}

const F: ExpandFn = |ctx, this, cond, ops| {
    let inst = &this.name[..4]; // remove the trailing 'i'

    if let Ok(imm) = parse_imm(ctx, &ops[0])
        && imm.as_i12().is_err()
    {
        Some(vec![
            ("li", cond, op_values!["tmp", ops[0]]),
            (inst, cond, op_values!["tmp"]),
        ])
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn push_imm32() {
        let pushi = mc_instr("pushi");

        assert_snapshot!(pushi("", &["0x123"]), @"");
        assert_snapshot!(pushi("", &["0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; push tmp");
        assert_snapshot!(pushi("", &["0x12345678"]), @"lui tmp 0x12345; addi tmp tmp 0x678; push tmp");

        assert_snapshot!(pushi("", &["123"]), @"");
        assert_snapshot!(pushi("", &["3000"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; push tmp");
        assert_snapshot!(pushi("", &["-123"]), @"");
        assert_snapshot!(pushi("", &["-3000"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; push tmp");

        assert_snapshot!(pushi("eq", &["0x123"]), @"");
        assert_snapshot!(pushi("eq", &["0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; push.eq tmp");
    }
}
