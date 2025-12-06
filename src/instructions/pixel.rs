use crate::instruction;

instruction! {
    name: "col",
    opcode: 0b_1101_000,
    itype: C,
}

instruction! {
    name: "spx",
    opcode: 0b_1101_001,
    itype: R,
    operand_types: [ RegS, RegS ],
    encode_format: [ None, Some, Some ],
}
