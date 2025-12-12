use crate::{
    instructions::{parse_imm, parse_reg_d, parse_reg_s},
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
};

macro_instruction! {
    name: [
        "add", "sub", "mulh", "mull", "mod", "div",
        "and", "nand", "or", "nor", "xor", "xnor",
        "shl", "shr", "rol", "ror", "ashr",
    ],
    operand_count: 3,
    expander: F1,
}

const F1: ExpandFn = |_, this, cond, ops| {
    let inst = this._may_be_name_with_i;

    parse_reg_d(&ops[0])?;
    parse_reg_s(&ops[1])?;

    if let Ok(imm) = parse_imm(&ops[2]) {
        Ok(Some(vec![(inst, cond, op_values![ops[0], ops[1], imm])]))
    } else {
        Ok(None)
    }
};

macro_instruction! {
    name: [ "beq", "bne", "blt", "ble", "bgt", "bge" ],
    operand_count: 3,
    expander: F2,
}

const F2: ExpandFn = |_, this, cond, ops| {
    let inst = this._may_be_name_with_i;

    parse_reg_s(&ops[0])?;

    // INFO: 'ops[2]' is also not checked here, see [[./branch_imm.rs]].

    if let Ok(imm) = parse_imm(&ops[1]) {
        Ok(Some(vec![(inst, cond, op_values![ops[0], imm, ops[2]])]))
    } else {
        Ok(None)
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn auto_imm_als() {
        let add = mc_instr("add");
        assert_snapshot!(add("", &["r1", "r2", "r3"]), @"");
        assert_snapshot!(add("", &["r1", "r2", "0x123"]), @"addi r1 r2 0x123");
        assert_snapshot!(add("", &["r1", "r2", "0x1234"]), @"lui tmp 1; ori tmp tmp 0x234; add r1 r2 tmp");
        assert_snapshot!(add("", &["r1", "r2", "0x12345678"]), @"lui tmp 0x12345; ori tmp tmp 0x678; add r1 r2 tmp");

        assert_snapshot!(add("eq", &["r1", "r2", "r3"]), @"");
        assert_snapshot!(add("eq", &["r1", "r2", "0x123"]), @"addi.eq r1 r2 0x123");
        assert_snapshot!(add("eq", &["r1", "r2", "0x1234"]), @"Error: Conditional 'add' is not supported for 32-bit immediates");
    }

    #[test]
    fn auto_imm_branch() {
        let beq = mc_instr("beq");

        assert_snapshot!(beq("", &["r1", "r2", "0"]), @"");
        assert_snapshot!(beq("", &["r1", "0x123", "0"]), @"li tmp 0x123; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "0x1234", "0"]), @"lui tmp 1; ori tmp tmp 0x234; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "0x12345678", "0"]), @"lui tmp 0x12345; ori tmp tmp 0x678; beq r1 tmp 0");

        assert_snapshot!(beq("eq", &["r1", "r2", "0"]), @"");
        assert_snapshot!(beq("eq", &["r1", "0x123", "0"]), @"li.eq tmp 0x123; beq.eq r1 tmp 0");
        assert_snapshot!(beq("eq", &["r1", "0x1234", "0"]), @"Error: Conditional 'beq' is not supported for 32-bit immediates");
    }
}
