use crate::instructions::instruction;

// func3[2]  : unsigned
// func3[1:0]: encoding

instruction! {
    pub Seq {
        name: "seq",
        opcode: 0b0000110,
        funct3: 0b000,
        itype: R,
    }
}

instruction! {
    pub Sne {
        name: "sne",
        opcode: 0b0000110,
        funct3: 0b001,
        itype: R,
    }
}

instruction! {
    pub Slt {
        name: "slt",
        opcode: 0b0000110,
        funct3: 0b010,
        itype: R,
    }
}

instruction! {
    pub Sge {
        name: "sge",
        opcode: 0b0000110,
        funct3: 0b011,
        itype: R,
    }
}

instruction! {
    pub Sltu {
        name: "sltu",
        opcode: 0b0000110,
        funct3: 0b100,
        itype: R,
    }
}

instruction! {
    pub Sgeu {
        name: "sgeu",
        opcode: 0b0000110,
        funct3: 0b101,
        itype: R,
    }
}

instruction! {
    pub Seqi {
        name: "seqi",
        opcode: 0b0000111,
        funct3: 0b000,
        itype: I,
    }
}

instruction! {
    pub Snei {
        name: "snei",
        opcode: 0b0000111,
        funct3: 0b001,
        itype: I,
    }
}

instruction! {
    pub Slti {
        name: "slti",
        opcode: 0b0000111,
        funct3: 0b010,
        itype: I,
    }
}

instruction! {
    pub Sgei {
        name: "sgei",
        opcode: 0b0000111,
        funct3: 0b011,
        itype: I,
    }
}

instruction! {
    pub Sltiu {
        name: "sltiu",
        opcode: 0b0000111,
        funct3: 0b100,
        itype: I,
    }
}

instruction! {
    pub Sgeiu {
        name: "sgeiu",
        opcode: 0b0000111,
        funct3: 0b101,
        itype: I,
    }
}
