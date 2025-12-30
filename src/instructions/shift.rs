use crate::instructions::instruction;

instruction! {
    pub Shl {
        name: "shl",
        opcode: 0b_0010_000,
        itype: R,
    }
}

instruction! {
    pub Shr {
        name: "shr",
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
    pub Ashr {
        name: "ashr",
        opcode: 0b_0010_100,
        itype: R,
    }
}

instruction! {
    pub Shli {
        name: "shli",
        opcode: 0b_0110_000,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Shri {
        name: "shri",
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
    pub Ashri {
        name: "ashri",
        opcode: 0b_0110_100,
        itype: I,
        operands_format: [ RegD, RegS, Imm(5, u) ],
    }
}
