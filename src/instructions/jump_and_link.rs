use crate::instructions::instruction;

instruction! {
    pub Jal {
        name: "jal",
        opcode: 0b_1001_000,
        itype: I,
        operands_format: [ RegD, _, Addr ],
    }
}

instruction! {
    pub Jalr {
        name: "jalr",
        opcode: 0b_1001_111,
        itype: I,
        operands_format: [ RegD, RegS, _ ],
    }
}
