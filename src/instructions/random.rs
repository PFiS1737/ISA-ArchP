use crate::{code, instruction};

instruction! {
    name: "rnd",
    opcode: 0b_1100_000,
    itype: I,
    operand_types: [ Reg ],
    encoder: |opcode, cond, operands| {
        let rd = operands[0];
        code!(opcode, cond, rd, 0, 0)
    }
}
