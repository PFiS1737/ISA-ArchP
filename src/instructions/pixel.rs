use crate::{code, instruction};

instruction! {
    name: "col",
    opcode: 0b_1101_000,
    itype: C,
}

instruction! {
    name: "spx",
    opcode: 0b_1101_001,
    itype: R,
    operand_types: [ Reg, Reg ],
    encoder: |opcode, cond, operands| {
        let rs1 = operands[0];
        let rs2 = operands[1];

        code!(opcode, cond, 0, rs1, rs2)
    }
}
