use crate::instructions::instruction;

// func3[2:0]: encoding

instruction! {
    pub Beq {
        name: "beq",
        opcode: 0b0001001,
        funct3: 0b000,
        itype: B,
    }
}

instruction! {
    pub Bne {
        name: "bne",
        opcode: 0b0001001,
        funct3: 0b001,
        itype: B,
    }
}

instruction! {
    pub Blt {
        name: "blt",
        opcode: 0b0001001,
        funct3: 0b010,
        itype: B,
    }
}

instruction! {
    pub Bge {
        name: "bge",
        opcode: 0b0001001,
        funct3: 0b011,
        itype: B,
    }
}

instruction! {
    pub Bltu {
        name: "bltu",
        opcode: 0b0001001,
        funct3: 0b100,
        itype: B,
    }
}

instruction! {
    pub Bgeu {
        name: "bgeu",
        opcode: 0b0001001,
        funct3: 0b101,
        itype: B,
    }
}
