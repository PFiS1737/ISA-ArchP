use crate::instructions::instruction;

instruction! {
    pub Lw {
        name: "lw",
        opcode: 0b_1000_000,
        itype: I,
    }
}

instruction! {
    pub Sw {
        name: "sw",
        opcode: 0b_1000_001,
        itype: B,
        operands_format: [RegS, RegS, Imm(12, i)],
    }
}

instruction! {
    pub Li {
        name: "li",
        opcode: 0b_1000_010,
        itype: I,
        operands_format: [ RegD, _, Imm(12, i) ],
    }
}

instruction! {
    pub Lui {
        name: "lui",
        opcode: 0b_1000_011,
        itype: U,
    }
}
