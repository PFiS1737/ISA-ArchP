use crate::instructions::instruction;

instruction! {
    pub Sll {
        name: "sll",
        opcode: 0b_0010_000,
        itype: R,
    }
}

instruction! {
    pub Srl {
        name: "srl",
        opcode: 0b_0010_001,
        itype: R,
    }
}

instruction! {
    pub Rol {
        name: "rol",
        opcode: 0b_0010_010,
        itype: R,
    }
}

instruction! {
    pub Ror {
        name: "ror",
        opcode: 0b_0010_011,
        itype: R,
    }
}

instruction! {
    pub Sra {
        name: "sra",
        opcode: 0b_0010_100,
        itype: R,
    }
}

instruction! {
    pub Slli {
        name: "slli",
        opcode: 0b_0110_000,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Srli {
        name: "srli",
        opcode: 0b_0110_001,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Roli {
        name: "roli",
        opcode: 0b_0110_010,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Rori {
        name: "rori",
        opcode: 0b_0110_011,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Srai {
        name: "srai",
        opcode: 0b_0110_100,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}
