use crate::instruction;

instruction! {
    name: "cmp",
    opcode: 0b_0011_000,
    itype: R,
    operand_types: [ Reg, Reg ],
    encode_format: [ None, Some, Some ],
}

instruction! {
    name: "cmpi",
    opcode: 0b_0111_000,
    itype: I,
    operand_types: [ Reg, Imm(12) ],
    encode_format: [ None, Some, Some ],
}
