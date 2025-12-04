use crate::{code, instruction, instructions::parse_reg};

instruction! {
    name: "col",
    opcode: 0b_1101_000,
    itype: C,
}

instruction! {
    name: "spx",
    opcode: 0b_1101_001,
    itype: R,
    operand_count: 2,
    encoder: |opcode, cond, operands| {
        let rs1 = parse_reg(operands[0])?;
        let rs2 = parse_reg(operands[1])?;
        code!(opcode, cond, 0, rs1, rs2)
    }
}
