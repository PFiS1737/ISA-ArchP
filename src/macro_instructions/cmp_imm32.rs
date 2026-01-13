use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_imm,
};

macro_instruction! {
    pub CmpImm32 {
        name: "cmpi",
        operand_count: 2,
        expander: F,
    }
}

const F: ExpandFn = |ctx, _, name, cond, ops| {
    let inst = &name[..3]; // remove the trailing 'i'

    if let Ok(imm) = parse_imm(ctx, &ops[1])
        && imm.as_i12().is_err()
    {
        Some(vec![
            ("li", cond, op_values!["tmp", ops[1]]),
            (inst, cond, op_values![ops[0], "tmp"]),
        ])
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn cmp_imm32() {
        let cmpi = mc_instr("cmpi");

        assert_snapshot!(cmpi("", &["r1"]), @"Error: Macro-instruction 'cmpi' requires 2 operands, got 1");
        assert_snapshot!(cmpi("", &["r1", "r2", "r3"]), @"Error: Macro-instruction 'cmpi' requires 2 operands, got 3");
        assert_snapshot!(cmpi("", &["123", "456"]), @"");
        assert_snapshot!(cmpi("", &["r1", "r2"]), @"");

        assert_snapshot!(cmpi("", &["r1", "0x123"]), @"");
        assert_snapshot!(cmpi("", &["r1", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; cmp r1 tmp");
        assert_snapshot!(cmpi("", &["r1", "0x12345678"]), @"lui tmp 0x12345; addi tmp tmp 0x678; cmp r1 tmp");

        assert_snapshot!(cmpi("", &["r1", "123"]), @"");
        assert_snapshot!(cmpi("", &["r1", "3000"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; cmp r1 tmp");
        assert_snapshot!(cmpi("", &["r1", "-123"]), @"");
        assert_snapshot!(cmpi("", &["r1", "-3000"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; cmp r1 tmp");

        assert_snapshot!(cmpi("eq", &["r1", "0x123"]), @"");
        assert_snapshot!(cmpi("eq", &["r1", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; cmp.eq r1 tmp");
    }
}
