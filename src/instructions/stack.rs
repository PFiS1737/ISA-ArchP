use crate::{
    code, instruction,
    instructions::{parse_imm, parse_reg},
};

instruction! {
    name: "peek",
    opcode: 0b_1010_000,
    itype: I,
    operand_count: 1,
    encoder: |opcode, cond, operands| {
        let rd = parse_reg(operands[0])?;
        code!(opcode, cond, rd, 0, 0)
    }
}

instruction! {
    name: "pop",
    opcode: 0b_1010_001,
    itype: I,
    operand_count: 1,
    encoder: |opcode, cond, operands| {
        let rd = parse_reg(operands[0])?;
        code!(opcode, cond, rd, 0, 0)
    }
}

instruction! {
    name: "push",
    opcode: 0b_1010_010,
    itype: I,
    operand_count: 1,
    encoder: |opcode, cond, operands| {
        let rs1 = parse_reg(operands[0])?;
        code!(opcode, cond, 0, rs1, 0)
    }
}

instruction! {
    name: "pushi",
    opcode: 0b_1010_011,
    itype: I,
    operand_count: 1,
    encoder: |opcode, cond, operands| {
        let imm12 = parse_imm(operands[0])?;
        code!(opcode, cond, 0, 0, imm12)
    }
}
