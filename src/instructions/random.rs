use crate::{code, instruction, instructions::parse_reg};

instruction! {
    name: "rnd",
    opcode: 0b_1100_000,
    itype: R,
    operand_count: 1,
    encoder: |opcode, cond, operands| {
        let rd = parse_reg(operands[0])?;
        code!(opcode, cond, rd, 0, 0)
    }
}
