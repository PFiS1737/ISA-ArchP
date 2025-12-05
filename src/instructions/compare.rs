use crate::{code, instruction};

instruction! {
    name: "cmp",
    opcode: 0b_0011_000,
    itype: R,
    operand_types: [ Reg, Reg ],
    encoder: |opcode, cond, operands| {
        let rs1 = operands[0];
        let rs2 = operands[1];

        code!(opcode, cond, 0, rs1, rs2)
    }
}

instruction! {
    name: "cmpi",
    opcode: 0b_0111_000,
    itype: I,
    operand_types: [ Reg, Imm(12) ],
    encoder: |opcode, cond, operands| {
        let rs1 = operands[0];
        let imm12 = operands[1];

        code!(opcode, cond, 0, rs1, imm12)
    }
}
