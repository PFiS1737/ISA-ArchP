use crate::instruction;

instruction! {
    name: "peek",
    opcode: 0b_1010_000,
    itype: I,
    operand_types: [ RegD ],
    encode_format: [ Some, None, None ],
}

instruction! {
    name: "pop",
    opcode: 0b_1010_001,
    itype: I,
    operand_types: [ RegD ],
    encode_format: [ Some, None, None ],
}

instruction! {
    name: "push",
    opcode: 0b_1010_010,
    itype: I,
    operand_types: [ RegS ],
    encode_format: [ None, Some, None ],
}

instruction! {
    name: "pushi",
    opcode: 0b_1010_011,
    itype: I,
    operand_types: [ Imm(12) ],
    encode_format: [ None, None, Some ],
}
