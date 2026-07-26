use crate::instructions::instruction;

instruction! {
    pub Jal {
        name: "jal",
        opcode: 0b0001010,
        itype: J,
    }
}

instruction! {
    pub Jalr {
        name: "jalr",
        opcode: 0b0001001,
        funct3: 0b111,
        itype: I,
    }
}
