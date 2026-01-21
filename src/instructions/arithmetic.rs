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
    pub Mulh {
        name: "mulh",
        opcode: 0b_0000_011,
        itype: R,
    }
}

instruction! {
    pub Mulhu {
        name: "mulhu",
        opcode: 0b_0000_100,
        itype: R,
    }
}

instruction! {
    pub Mulhsu {
        name: "mulhsu",
        opcode: 0b_0000_101,
        itype: R,
    }
}

instruction! {
    pub Rem {
        name: "rem",
        opcode: 0b_0000_110,
        itype: R,
    }
}

instruction! {
    pub Div {
        name: "div",
        opcode: 0b_0000_111,
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
    pub Mulhi {
        name: "mulhi",
        opcode: 0b_0100_011,
        itype: I,
    }
}

instruction! {
    pub Mulhiu {
        name: "mulhiu",
        opcode: 0b_0100_100,
        itype: I,
    }
}

instruction! {
    pub Mulhisu {
        name: "mulhisu",
        opcode: 0b_0100_101,
        itype: I,
    }
}

instruction! {
    pub Remi {
        name: "remi",
        opcode: 0b_0100_110,
        itype: I,
    }
}

instruction! {
    pub Divi {
        name: "divi",
        opcode: 0b_0100_111,
        itype: I,
    }
}
