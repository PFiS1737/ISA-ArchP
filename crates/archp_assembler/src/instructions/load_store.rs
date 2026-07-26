use crate::instructions::instruction;

// func3[2:0]: encoding

instruction! {
    pub Lw {
        name: "lw",
        opcode: 0b0001000,
        funct3: 0b000,
        itype: I,
    }
}

instruction! {
    pub Lh {
        name: "lh",
        opcode: 0b0001000,
        funct3: 0b001,
        itype: I,
    }
}

instruction! {
    pub Lhu {
        name: "lhu",
        opcode: 0b0001000,
        funct3: 0b010,
        itype: I,
    }
}

instruction! {
    pub Lb {
        name: "lb",
        opcode: 0b0001000,
        funct3: 0b011,
        itype: I,
    }
}

instruction! {
    pub Lbu {
        name: "lbu",
        opcode: 0b0001000,
        funct3: 0b100,
        itype: I,
    }
}

instruction! {
    pub Sw {
        name: "sw",
        opcode: 0b0001000,
        funct3: 0b101,
        itype: S,
    }
}

instruction! {
    pub Sh {
        name: "sh",
        opcode: 0b0001000,
        funct3: 0b110,
        itype: S,
    }
}

instruction! {
    pub Sb {
        name: "sb",
        opcode: 0b0001000,
        funct3: 0b111,
        itype: S,
    }
}
