use crate::instructions::instruction;

instruction! {
    pub Add {
        name: "add",
        opcode: 0b_0000_000,
        itype: R,
    }
}

instruction! {
    pub Sub {
        name: "sub",
        opcode: 0b_0000_001,
        itype: R,
    }
}

instruction! {
    pub Mul {
        name: "mul",
        opcode: 0b_0000_010,
        itype: R,
    }
}

instruction! {
    pub Mod {
        name: "mod",
        opcode: 0b_0000_011,
        itype: R,
    }
}

instruction! {
    pub Div {
        name: "div",
        opcode: 0b_0000_100,
        itype: R,
    }
}

instruction! {
    pub Addi {
        name: "addi",
        opcode: 0b_0100_000,
        itype: I,
    }
}

instruction! {
    pub Subi {
        name: "subi",
        opcode: 0b_0100_001,
        itype: I,
    }
}

instruction! {
    pub Muli {
        name: "muli",
        opcode: 0b_0100_010,
        itype: I,
    }
}

instruction! {
    pub Modi {
        name: "modi",
        opcode: 0b_0100_011,
        itype: I,
    }
}

instruction! {
    pub Divi {
        name: "divi",
        opcode: 0b_0100_100,
        itype: I,
    }
}
