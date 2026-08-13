use crate::instructions::instruction;

// func3[2]: shift/rotate
// func3[1]: logical/arithmetic
// func3[0]: left/right

instruction! {
    pub Sll {
        name: "sll",
        opcode: 0b0000100,
        funct3: 0b000,
        itype: R,
    }
}

instruction! {
    pub Srl {
        name: "srl",
        opcode: 0b0000100,
        funct3: 0b001,
        itype: R,
    }
}

instruction! {
    pub Sra {
        name: "sra",
        opcode: 0b0000100,
        funct3: 0b011,
        itype: R,
    }
}

instruction! {
    pub Rol {
        name: "rol",
        opcode: 0b0000100,
        funct3: 0b100,
        itype: R,
    }
}

instruction! {
    pub Ror {
        name: "ror",
        opcode: 0b0000100,
        funct3: 0b101,
        itype: R,
    }
}

instruction! {
    pub Slli {
        name: "slli",
        opcode: 0b0000101,
        funct3: 0b000,
        itype: I,
        format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Srli {
        name: "srli",
        opcode: 0b0000101,
        funct3: 0b001,
        itype: I,
        format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Srai {
        name: "srai",
        opcode: 0b0000101,
        funct3: 0b011,
        itype: I,
        format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Roli {
        name: "roli",
        opcode: 0b0000101,
        funct3: 0b100,
        itype: I,
        format: [ RegD, RegS, Imm(5, u) ],
    }
}

instruction! {
    pub Rori {
        name: "rori",
        opcode: 0b0000101,
        funct3: 0b101,
        itype: I,
        format: [ RegD, RegS, Imm(5, u) ],
    }
}
