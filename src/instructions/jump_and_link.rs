use crate::instructions::instruction;

instruction! {
    pub Jal {
        name: "jal",
        opcode: 0b_1001_000,
        itype: J,
    }
}

instruction! {
    pub Jalr {
        name: "jalr",
        opcode: 0b_1001_111,
        itype: I,
    }
}
