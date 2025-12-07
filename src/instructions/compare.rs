use crate::instructions::instruction;

instruction! {
    name: "cmp",
    opcode: 0b_0011_000,
    itype: R,
    operand_types: [ RegS, RegS ],
    encode_format: [ None, Some, Some ],
}

instruction! {
    name: "cmpi",
    opcode: 0b_0111_000,
    itype: I,
    operand_types: [ RegS, Imm(12) ],
    encode_format: [ None, Some, Some ],
}
