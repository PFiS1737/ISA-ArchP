use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_imm,
};

macro_instruction! {
    name: [
        "addi", "subi", "mulhi", "mulli", "modi", "divi",
        "andi", "nandi", "ori", "nori", "xori", "xnori",
        "shli", "shri", "roli", "rori", "ashri",
    ],
    operand_count: 3,
    expander: F,
}

const F: ExpandFn = |ctx, this, cond, ops| {
    let inst = &this.name[..this.name.len() - 1]; // remove the trailing 'i'

    if let Ok(imm) = parse_imm(ctx, &ops[2])
        && imm.as_i12().is_err()
    {
        Some(vec![
            ("li", cond, op_values!["tmp", ops[2]]),
            (inst, cond, op_values![ops[0], ops[1], "tmp"]),
        ])
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn als_imm32() {
        let addi = mc_instr("addi");

        assert_snapshot!(addi("", &["r1", "r2"]), @"Error: Macro-instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(addi("", &["r1", "r2", "123", "r4"]), @"Error: Macro-instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(addi("", &["zero", "r2", "123"]), @"");
        assert_snapshot!(addi("", &["r1", "r2", "r3"]), @"");
        assert_snapshot!(addi("", &["123", "r1", "456"]), @"");

        assert_snapshot!(addi("", &["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi("", &["r1", "r2", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; add r1 r2 tmp");
        assert_snapshot!(addi("", &["r1", "r2", "0x12345678"]), @"lui tmp 0x12345; addi tmp tmp 0x678; add r1 r2 tmp");

        assert_snapshot!(addi("", &["r1", "r2", "123"]), @"");
        assert_snapshot!(addi("", &["r1", "r2", "3000"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; add r1 r2 tmp");
        assert_snapshot!(addi("", &["r1", "r2", "-123"]), @"");
        assert_snapshot!(addi("", &["r1", "r2", "-3000"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; add r1 r2 tmp");

        assert_snapshot!(addi("eq", &["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi("eq", &["r1", "r2", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; add.eq r1 r2 tmp");
    }
}
