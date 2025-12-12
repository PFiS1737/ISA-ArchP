use crate::{
    instructions::{parse_imm, parse_reg_d, parse_reg_s},
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
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

const F: ExpandFn = |_, this, cond, ops| {
    let inst = &this.name[..this.name.len() - 1]; // remove the trailing 'i'

    parse_reg_d(&ops[0])?;
    parse_reg_s(&ops[1])?;

    let imm = parse_imm(&ops[2])?;

    if imm > 0xFFF {
        Ok(Some(vec![
            ("li", cond, op_values!["tmp", imm]),
            (inst, cond, op_values![ops[0], ops[1], "tmp"]),
        ]))
    } else {
        Ok(None)
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
        assert_snapshot!(addi("", &["zero", "r2", "123"]), @"Error: Register 'zero' is raed-only");
        assert_snapshot!(addi("", &["r1", "r2", "r3"]), @"Error: Invalid immediate: r3");
        assert_snapshot!(addi("", &["123", "r1", "456"]), @"Error: Expected register, found immediate: 123");

        assert_snapshot!(addi("", &["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi("", &["r1", "r2", "0x1234"]), @"lui tmp 1; ori tmp tmp 0x234; add r1 r2 tmp");
        assert_snapshot!(addi("", &["r1", "r2", "0x12345678"]), @"lui tmp 0x12345; ori tmp tmp 0x678; add r1 r2 tmp");

        assert_snapshot!(addi("eq", &["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi("eq", &["r1", "r2", "0x1234"]), @"Error: Conditional 'addi' is not supported for 32-bit immediates");
    }
}
