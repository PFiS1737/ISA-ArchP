use crate::instructions::instruction;

// func3[2]  : immediate
// func3[1:0]: encoding

instruction! {
    pub Add {
        name: "add",
        opcode: 0b0000000,
        funct3: 0b000,
        itype: R,
    }
}

instruction! {
    pub Sub {
        name: "sub",
        opcode: 0b0000000,
        funct3: 0b001,
        itype: R,
    }
}

instruction! {
    pub And {
        name: "and",
        opcode: 0b0000000,
        funct3: 0b010,
        itype: R,
    }
}

instruction! {
    pub Or {
        name: "or",
        opcode: 0b0000000,
        funct3: 0b011,
        itype: R,
    }
}

instruction! {
    pub Addi {
        name: "addi",
        opcode: 0b0000000,
        funct3: 0b100,
        itype: I,
    }
}

instruction! {
    pub Subi {
        name: "subi",
        opcode: 0b0000000,
        funct3: 0b101,
        itype: I,
    }
}

instruction! {
    pub Andi {
        name: "andi",
        opcode: 0b0000000,
        funct3: 0b110,
        itype: I,
    }
}

instruction! {
    pub Ori {
        name: "ori",
        opcode: 0b0000000,
        funct3: 0b111,
        itype: I,
    }
}

instruction! {
    pub Xor {
        name: "xor",
        opcode: 0b0000001,
        funct3: 0b000,
        itype: R,
    }
}

instruction! {
    pub Xnor {
        name: "xnor",
        opcode: 0b0000001,
        funct3: 0b001,
        itype: R,
    }
}

instruction! {
    pub Nand {
        name: "nand",
        opcode: 0b0000001,
        funct3: 0b010,
        itype: R,
    }
}

instruction! {
    pub Nor {
        name: "nor",
        opcode: 0b0000001,
        funct3: 0b011,
        itype: R,
    }
}

instruction! {
    pub Xori {
        name: "xori",
        opcode: 0b0000001,
        funct3: 0b100,
        itype: I,
    }
}

instruction! {
    pub Xnori {
        name: "xnori",
        opcode: 0b0000001,
        funct3: 0b101,
        itype: I,
    }
}

instruction! {
    pub Nandi {
        name: "nandi",
        opcode: 0b0000001,
        funct3: 0b110,
        itype: I,
    }
}

instruction! {
    pub Nori {
        name: "nori",
        opcode: 0b0000001,
        funct3: 0b111,
        itype: I,
    }
}
