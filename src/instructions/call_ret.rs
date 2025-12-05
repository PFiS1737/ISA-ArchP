use crate::{code, instruction};

instruction! {
    name: "ret",
    opcode: 0b_1010_100,
    itype: I,
    operand_types: [],
    encoder: |opcode, cond, _| {
        code!(opcode, cond, 0, 0, 0)
    }
}

instruction! {
    name: "call",
    opcode: 0b_1010_101,
    itype: I,
    operand_types: [ Imm(12) ],
    encoder: |opcode, cond, operands| {
        let imm12 = operands[0];

        code!(opcode, cond, 0, 0, imm12)
    }
}
