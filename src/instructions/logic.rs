use crate::instruction;

instruction! {
    name: "and",
    opcode: 0b_0001_000,
    itype: R,
}

instruction! {
    name: "nand",
    opcode: 0b_0001_001,
    itype: R,
}

instruction! {
    name: "or",
    opcode: 0b_0001_010,
    itype: R,
}

instruction! {
    name: "nor",
    opcode: 0b_0001_011,
    itype: R,
}

instruction! {
    name: "xor",
    opcode: 0b_0001_100,
    itype: R,
}

instruction! {
    name: "xnor",
    opcode: 0b_0001_101,
    itype: R,
}

instruction! {
    name: "not",
    opcode: 0b_0001_110,
    itype: R,
    operand_types: [ Reg, Reg ],
    encode_format: [ Some, Some, None ],
}

instruction! {
    name: "andi",
    opcode: 0b_0101_000,
    itype: I,
}

instruction! {
    name: "nandi",
    opcode: 0b_0101_001,
    itype: I,
}

instruction! {
    name: "ori",
    opcode: 0b_0101_010,
    itype: I,
}

instruction! {
    name: "nori",
    opcode: 0b_0101_011,
    itype: I,
}

instruction! {
    name: "xori",
    opcode: 0b_0101_100,
    itype: I,
}

instruction! {
    name: "xnori",
    opcode: 0b_0101_101,
    itype: I,
}
