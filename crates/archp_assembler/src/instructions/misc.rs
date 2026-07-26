use crate::instructions::instruction;

instruction! {
    pub Col {
        name: "colr",
        opcode: 0b1111111,
        funct3: 0b000,
        itype: I,
    }
}

instruction! {
    pub Spx {
        name: "spx",
        opcode: 0b1111111,
        funct3: 0b001,
        itype: R,
        operands_format: [ _, RegS, RegS ],
    }
}

instruction! {
    pub In {
        name: "in",
        opcode: 0b1111111,
        funct3: 0b010,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}

instruction! {
    pub Out {
        name: "out",
        opcode: 0b1111111,
        funct3: 0b011,
        itype: I,
        operands_format: [ _, RegS, _ ],
    }
}

instruction! {
    pub Kbget {
        name: "kbget",
        opcode: 0b1111111,
        funct3: 0b100,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}

instruction! {
    pub Rand {
        name: "rand",
        opcode: 0b1111111,
        funct3: 0b101,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}
