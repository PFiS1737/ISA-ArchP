use crate::instructions::instruction;

// func3[2]: immediate
// func3[1]: unsigned
// func3[0]: high-bit + signed

instruction! {
    pub Mul {
        name: "mul",
        opcode: 0b0000010,
        funct3: 0b000,
        itype: R,
    }
}

instruction! {
    pub Mulh {
        name: "mulh",
        opcode: 0b0000010,
        funct3: 0b001,
        itype: R,
    }
}

instruction! {
    pub Mulhu {
        name: "mulhu",
        opcode: 0b0000010,
        funct3: 0b010,
        itype: R,
    }
}

instruction! {
    pub Mulhsu {
        name: "mulhsu",
        opcode: 0b0000010,
        funct3: 0b011,
        itype: R,
    }
}

instruction! {
    pub Muli {
        name: "muli",
        opcode: 0b0000010,
        funct3: 0b100,
        itype: I,
    }
}

instruction! {
    pub Mulhi {
        name: "mulhi",
        opcode: 0b0000010,
        funct3: 0b101,
        itype: I,
    }
}

instruction! {
    pub Mulhiu {
        name: "mulhiu",
        opcode: 0b0000010,
        funct3: 0b110,
        itype: I,
    }
}

instruction! {
    pub Mulhisu {
        name: "mulhisu",
        opcode: 0b0000010,
        funct3: 0b111,
        itype: I,
    }
}

// func3[2]: immediate
// func3[1]: unsigned
// func3[0]: div/rem

instruction! {
    pub Div {
        name: "div",
        opcode: 0b0000011,
        funct3: 0b000,
        itype: R,
    }
}

instruction! {
    pub Rem {
        name: "rem",
        opcode: 0b0000011,
        funct3: 0b001,
        itype: R,
    }
}

instruction! {
    pub Divi {
        name: "divi",
        opcode: 0b0000011,
        funct3: 0b100,
        itype: I,
    }
}

instruction! {
    pub Remi {
        name: "remi",
        opcode: 0b0000011,
        funct3: 0b101,
        itype: I,
    }
}
