use crate::{
    instructions::{parse_reg_d, parse_reg_s},
    macro_instructions::{ExpandFn, load_imm32, macro_instruction},
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

const F: ExpandFn = |name, cond, ops| {
    let inst = match name {
        "addi" => "add",
        "subi" => "sub",
        "mulhi" => "mulh",
        "mulli" => "mul",
        "modi" => "mod",
        "divi" => "div",
        "andi" => "and",
        "nandi" => "nand",
        "ori" => "or",
        "nori" => "nor",
        "xori" => "xor",
        "xnori" => "xnor",
        "shli" => "shl",
        "shri" => "shr",
        "roli" => "rol",
        "rori" => "ror",
        "ashri" => "ashr",
        _ => unreachable!(),
    };

    parse_reg_d(&ops[0])?;
    parse_reg_s(&ops[1])?;

    #[allow(clippy::useless_vec)]
    if let Some(mut ret) = load_imm32::F(name, cond, &op_values!("tmp", ops[2]))? {
        ret.push((inst, None, op_values![ops[0], ops[1], "tmp"]));
        Ok(Some(ret))
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
        assert_snapshot!(addi("", &["r0", "r2", "123"]), @"Error: Register 'r0' is raed-only");
        assert_snapshot!(addi("", &["r1", "r2", "r3"]), @"Error: Invalid immediate: r3");
        assert_snapshot!(addi("", &["123", "r1", "456"]), @"Error: Expected register, found immediate: 123");

        assert_snapshot!(addi("", &["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi("", &["r1", "r2", "0x1234"]), @"lui tmp 1; ori tmp tmp 0x234; add r1 r2 tmp");
        assert_snapshot!(addi("", &["r1", "r2", "0x12345678"]), @"lui tmp 0x12345; ori tmp tmp 0x678; add r1 r2 tmp");

        assert_snapshot!(addi("eq", &["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi("eq", &["r1", "r2", "0x1234"]), @"Error: Conditional 'addi' is not supported for 32-bit immediates");
    }
}
