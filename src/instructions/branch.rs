use crate::instruction;

instruction! {
    name: "jmp",
    opcode: 0b_1001_000,
    itype: B,
}

instruction! {
    name: "beq",
    opcode: 0b_1001_001,
    itype: B,
}

instruction! {
    name: "bne",
    opcode: 0b_1001_010,
    itype: B,
}

instruction! {
    name: "blt",
    opcode: 0b_1001_011,
    itype: B,
}

instruction! {
    name: "ble",
    opcode: 0b_1001_100,
    itype: B,
}

instruction! {
    name: "bgt",
    opcode: 0b_1001_101,
    itype: B,
}

instruction! {
    name: "bge",
    opcode: 0b_1001_110,
    itype: B,
}
