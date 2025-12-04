use crate::{
    code, instruction,
    instructions::{parse_imm, parse_reg},
};

instruction! {
    name: "cmp",
    opcode: 0b_0011_000,
    itype: R,
    operand_count: 2,
    encoder: |opcode, cond, operands| {
        let rs1 = parse_reg(operands[0])?;
        let rs2 = parse_reg(operands[1])?;
        code!(opcode, cond, 0, rs1, rs2)
    }
}

instruction! {
    name: "cmpi",
    opcode: 0b_0111_000,
    itype: I,
    operand_count: 2,
    encoder: |opcode, cond, operands| {
        let rs1 = parse_reg(operands[0])?;
        let imm12 = parse_imm(operands[1])?;
        code!(opcode, cond, 0, rs1, imm12)
    }
}
