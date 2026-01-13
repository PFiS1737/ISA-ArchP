use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_imm,
};

macro_instruction! {
    pub BranchImm {
        name: [ "beqi", "bnei", "blti", "blei", "bgti", "bgei" ],
        operand_count: 3,
        expander: F,
    }
}

const F: ExpandFn = |ctx, _, name, cond, ops| {
    let inst = &name[..3]; // remove the trailing 'i'

    let Ok(imm) = parse_imm(ctx, &ops[1]).and_then(|imm| imm.as_i32()) else {
        return None;
    };

    if imm == 0 {
        Some(vec![(inst, cond, op_values![ops[0], "r0", ops[2]])])
    } else {
        Some(vec![
            ("li", cond, op_values!["tmp", imm]),
            (inst, cond, op_values![ops[0], "tmp", ops[2]]),
        ])
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn branch_imm() {
        let beqi = mc_instr("beqi");

        assert_snapshot!(beqi("", &["r1", "0x123"]), @"Error: Macro-instruction 'beqi' requires 3 operands, got 2");
        assert_snapshot!(beqi("", &["r1", "r2", "0"]), @"");
        assert_snapshot!(beqi("", &["123", "123", "0"]), @"li tmp 123; beq 123 tmp 0");

        assert_snapshot!(beqi("", &["r1", "0x123", "0"]), @"li tmp 0x123; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0x1234", "0"]), @"lui tmp 1; addi tmp tmp 0x234; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0x12345678", "0"]), @"lui tmp 0x12345; addi tmp tmp 0x678; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0", "0"]), @"beq r1 r0 0");

        assert_snapshot!(beqi("", &["r1", "123", "0"]), @"li tmp 123; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "3000", "0"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "-123", "0"]), @"li tmp -123; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "-3000", "0"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; beq r1 tmp 0");

        assert_snapshot!(beqi("eq", &["r1", "0x123", "0"]), @"li.eq tmp 0x123; beq.eq r1 tmp 0");
        assert_snapshot!(beqi("eq", &["r1", "0x1234", "0"]), @"lui tmp 1; addi tmp tmp 0x234; beq.eq r1 tmp 0");
    }
}
