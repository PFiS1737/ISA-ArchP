use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_imm,
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

const F1: ExpandFn = |ctx, this, cond, ops| {
    let inst = this._may_be_name_with_i;

    if let Ok(imm) = parse_imm(ctx, &ops[2]).and_then(|imm| imm.as_i32()) {
        Some(vec![(inst, cond, op_values![ops[0], ops[1], imm])])
    } else {
        None
    }
};

macro_instruction! {
    name: "cmp",
    operand_count: 2,
    expander: F2,
}

const F2: ExpandFn = |ctx, this, cond, ops| {
    let inst = this._may_be_name_with_i;

    if let Ok(imm) = parse_imm(ctx, &ops[1]).and_then(|imm| imm.as_i32()) {
        Some(vec![(inst, cond, op_values![ops[0], imm])])
    } else {
        None
    }
};

macro_instruction! {
    name: [ "beq", "bne", "blt", "ble", "bgt", "bge" ],
    operand_count: 3,
    expander: F3,
}

const F3: ExpandFn = |ctx, this, cond, ops| {
    let inst = this._may_be_name_with_i;

    if let Ok(imm) = parse_imm(ctx, &ops[1]).and_then(|imm| imm.as_i32()) {
        Some(vec![(inst, cond, op_values![ops[0], imm, ops[2]])])
    } else {
        None
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
        assert_snapshot!(add("", &["r1", "r2", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; add r1 r2 tmp");
        assert_snapshot!(add("", &["r1", "r2", "0x12345678"]), @"lui tmp 0x12345; addi tmp tmp 0x678; add r1 r2 tmp");

        assert_snapshot!(add("", &["r1", "r2", "123"]), @"addi r1 r2 123");
        assert_snapshot!(add("", &["r1", "r2", "3000"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; add r1 r2 tmp");
        assert_snapshot!(add("", &["r1", "r2", "-123"]), @"addi r1 r2 -123");
        assert_snapshot!(add("", &["r1", "r2", "-3000"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; add r1 r2 tmp");

        assert_snapshot!(add("eq", &["r1", "r2", "r3"]), @"");
        assert_snapshot!(add("eq", &["r1", "r2", "0x123"]), @"addi.eq r1 r2 0x123");
        assert_snapshot!(add("eq", &["r1", "r2", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; add.eq r1 r2 tmp");
    }

    #[test]
    fn auto_imm_cmp() {
        let cmp = mc_instr("cmp");

        assert_snapshot!(cmp("", &["r1", "r2"]), @"");
        assert_snapshot!(cmp("", &["r1", "0x123"]), @"cmpi r1 0x123");
        assert_snapshot!(cmp("", &["r1", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; cmp r1 tmp");
        assert_snapshot!(cmp("", &["r1", "0x12345678"]), @"lui tmp 0x12345; addi tmp tmp 0x678; cmp r1 tmp");

        assert_snapshot!(cmp("", &["r1", "123"]), @"cmpi r1 123");
        assert_snapshot!(cmp("", &["r1", "3000"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; cmp r1 tmp");
        assert_snapshot!(cmp("", &["r1", "-123"]), @"cmpi r1 -123");
        assert_snapshot!(cmp("", &["r1", "-3000"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; cmp r1 tmp");

        assert_snapshot!(cmp("eq", &["r1", "0x123"]), @"cmpi.eq r1 0x123");
        assert_snapshot!(cmp("eq", &["r1", "0x1234"]), @"lui tmp 1; addi tmp tmp 0x234; cmp.eq r1 tmp");
    }

    #[test]
    fn auto_imm_branch() {
        let beq = mc_instr("beq");

        assert_snapshot!(beq("", &["r1", "r2", "0"]), @"");
        assert_snapshot!(beq("", &["r1", "0x123", "0"]), @"li tmp 0x123; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "0x1234", "0"]), @"lui tmp 1; addi tmp tmp 0x234; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "0x12345678", "0"]), @"lui tmp 0x12345; addi tmp tmp 0x678; beq r1 tmp 0");

        assert_snapshot!(beq("", &["r1", "123", "0"]), @"li tmp 123; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "3000", "0"]), @"lui tmp 1; addi tmp tmp 0xFFFFFBB8; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "-123", "0"]), @"li tmp -123; beq r1 tmp 0");
        assert_snapshot!(beq("", &["r1", "-3000", "0"]), @"lui tmp 0xFFFFF; addi tmp tmp 0x448; beq r1 tmp 0");

        assert_snapshot!(beq("eq", &["r1", "r2", "0"]), @"");
        assert_snapshot!(beq("eq", &["r1", "0x123", "0"]), @"li.eq tmp 0x123; beq.eq r1 tmp 0");
        assert_snapshot!(beq("eq", &["r1", "0x1234", "0"]), @"lui tmp 1; addi tmp tmp 0x234; beq.eq r1 tmp 0");
    }
}
