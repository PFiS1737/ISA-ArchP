use crate::instructions::instruction;

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

instruction! {
    name: "seg",
    opcode: 0b_1101_010,
    itype: R,
    operand_types: [ RegS ],
    encode_format: [ None, None, Some ],
}

instruction! {
    name: "segi",
    opcode: 0b_1101_011,
    itype: I,
    operand_types: [ Imm(8) ],
    encode_format: [ None, None, Some ],
}
