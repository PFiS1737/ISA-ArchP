use crate::instructions::instruction;

instruction! {
    pub Pop {
        name: "pop",
        opcode: 0b_1010_000,
        itype: I,
        operands_format: [ RegD, _, _ ],
    }
}

instruction! {
    pub Push {
        name: "push",
        opcode: 0b_1010_001,
        itype: I,
        operands_format: [ _, RegS, _ ],
    }
}

instruction! {
    pub Ret {
        name: "ret",
        opcode: 0b_1010_100,
        itype: I,
        operands_format: [ _, _, _ ],
    }
}

instruction! {
    pub Call {
        name: "call",
        opcode: 0b_1010_101,
        itype: J,
        operands_format: [ _, Addr(20) ],
    }
}

instruction! {
    pub Callr {
        name: "callr",
        opcode: 0b_1010_110,
        itype: I,
        operands_format: [ _, RegS, _ ],
    }
}
