use crate::instructions::instruction;

instruction! {
    name: "lw",
    opcode: 0b_1000_000,
    itype: I,
}

instruction! {
    name: "sw",
    opcode: 0b_1000_001,
    itype: B,
}

instruction! {
    name: "li",
    opcode: 0b_1000_010,
    itype: I,
    operand_types: [ RegD, Imm(12) ],
    encode_format: [ Some, None, Some ],
}

instruction! {
    name: "lui",
    opcode: 0b_1000_011,
    itype: U,
}
