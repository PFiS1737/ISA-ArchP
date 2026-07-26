use crate::instructions::instruction;

instruction! {
    pub Pop {
        name: "pop",
        opcode: 0b1111110,
        funct3: 0b000,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}

instruction! {
    pub Push {
        name: "push",
        opcode: 0b1111110,
        funct3: 0b001,
        itype: I,
        operands_format: [ _, RegS, _ ],
    }
}

instruction! {
    pub Ret {
        name: "ret",
        opcode: 0b1111110,
        funct3: 0b010,
        itype: I,
        operands_format: [ _, _, _ ],
    }
}

instruction! {
    pub Callr {
        name: "callr",
        opcode: 0b1111110,
        funct3: 0b011,
        itype: I,
        operands_format: [ _, RegS, Imm(12, i) ],
    }
}

instruction! {
    pub Call {
        name: "call",
        opcode: 0b1111101,
        itype: J,
        operands_format: [ _, Addr(20) ],
    }
}
