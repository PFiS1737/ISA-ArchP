use crate::instructions::instruction;

instruction! {
    name: "add",
    opcode: 0b_0000_000,
    itype: R,
}

instruction! {
    name: "sub",
    opcode: 0b_0000_001,
    itype: R,
}

instruction! {
    name: "mulh",
    opcode: 0b_0000_010,
    itype: R,
}

instruction! {
    name: "mull",
    opcode: 0b_0000_011,
    itype: R,
}

instruction! {
    name: "mod",
    opcode: 0b_0000_100,
    itype: R,
}

instruction! {
    name: "div",
    opcode: 0b_0000_101,
    itype: R,
}

instruction! {
    name: "addi",
    opcode: 0b_0100_000,
    itype: I,
}

instruction! {
    name: "subi",
    opcode: 0b_0100_001,
    itype: I,
}

instruction! {
    name: "mulhi",
    opcode: 0b_0100_010,
    itype: I,
}

instruction! {
    name: "mulli",
    opcode: 0b_0100_011,
    itype: I,
}

instruction! {
    name: "modi",
    opcode: 0b_0100_100,
    itype: I,
}

instruction! {
    name: "divi",
    opcode: 0b_0100_101,
    itype: I,
}
