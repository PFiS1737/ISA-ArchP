use crate::instructions::instruction;

instruction! {
    pub Lw {
        name: "lw",
        opcode: 0b_1000_000,
        itype: I,
    }
}

instruction! {
    pub Lh {
        name: "lh",
        opcode: 0b_1000_001,
        itype: I,
    }
}

instruction! {
    pub Lhu {
        name: "lhu",
        opcode: 0b_1000_010,
        itype: I,
    }
}

instruction! {
    pub Lb {
        name: "lb",
        opcode: 0b_1000_011,
        itype: I,
    }
}

instruction! {
    pub Lbu {
        name: "lbu",
        opcode: 0b_1000_100,
        itype: I,
    }
}

instruction! {
    pub Sw {
        name: "sw",
        opcode: 0b_1000_101,
        itype: B,
        operands_format: [RegS, RegS, Imm(12, i)],
    }
}

instruction! {
    pub Sh {
        name: "sh",
        opcode: 0b_1000_110,
        itype: B,
        operands_format: [RegS, RegS, Imm(12, i)],
    }
}

instruction! {
    pub Sb {
        name: "sb",
        opcode: 0b_1000_111,
        itype: B,
        operands_format: [RegS, RegS, Imm(12, i)],
    }
}
