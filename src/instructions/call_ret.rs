use crate::{code, instruction, instructions::parse_imm};

instruction! {
    name: "ret",
    opcode: 0b_1010_100,
    itype: I,
    operand_count: 0,
    encoder: |opcode, cond, _| {
        code!(opcode, cond, 0, 0, 0)
    }
}

instruction! {
    name: "call",
    opcode: 0b_1010_101,
    itype: I,
    operand_count: 1,
    encoder: |opcode, cond, operands| {
        let imm12 = parse_imm(operands[0])?;
        code!(opcode, cond, 0, 0, imm12)
    }
}
