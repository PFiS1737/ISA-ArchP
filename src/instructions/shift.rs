use crate::instruction;

instruction! {
    name: "shl",
    opcode: 0b_0010_000,
    itype: R,
}

instruction! {
    name: "shr",
    opcode: 0b_0010_001,
    itype: R,
}

instruction! {
    name: "rol",
    opcode: 0b_0010_010,
    itype: R,
}

instruction! {
    name: "ror",
    opcode: 0b_0010_011,
    itype: R,
}

instruction! {
    name: "ashr",
    opcode: 0b_0010_100,
    itype: R,
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

instruction! {
    name: "shli",
    opcode: 0b_0110_000,
    itype: I,
}

instruction! {
    name: "shri",
    opcode: 0b_0110_001,
    itype: I,
}

instruction! {
    name: "roli",
    opcode: 0b_0110_010,
    itype: I,
}

instruction! {
    name: "rori",
    opcode: 0b_0110_011,
    itype: I,
}

instruction! {
    name: "ashri",
    opcode: 0b_0110_100,
    itype: I,
}
