use crate::instructions::instruction;

instruction! {
    pub Col {
        name: "colr",
        opcode: 0b_1101_000,
        itype: I,
    }
}

instruction! {
    pub Spx {
        name: "spx",
        opcode: 0b_1101_001,
        itype: R,
        operands_format: [ _, RegS, RegS ],
    }
}

instruction! {
    pub In {
        name: "in",
        opcode: 0b_1110_000,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}

instruction! {
    pub Out {
        name: "out",
        opcode: 0b_1110_001,
        itype: I,
        operands_format: [ _, RegS, _ ],
    }
}

instruction! {
    pub Kbget {
        name: "kbget",
        opcode: 0b_1110_010,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}

instruction! {
    pub Rand {
        name: "rand",
        opcode: 0b_1110_011,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}
