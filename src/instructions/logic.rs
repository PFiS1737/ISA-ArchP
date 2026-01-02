use crate::instructions::instruction;

instruction! {
    pub And {
        name: "and",
        opcode: 0b_0001_000,
        itype: R,
    }
}

instruction! {
    pub Nand {
        name: "nand",
        opcode: 0b_0001_001,
        itype: R,
    }
}

instruction! {
    pub Or {
        name: "or",
        opcode: 0b_0001_010,
        itype: R,
    }
}

instruction! {
    pub Nor {
        name: "nor",
        opcode: 0b_0001_011,
        itype: R,
    }
}

instruction! {
    pub Xor {
        name: "xor",
        opcode: 0b_0001_100,
        itype: R,
    }
}

instruction! {
    pub Xnor {
        name: "xnor",
        opcode: 0b_0001_101,
        itype: R,
    }
}

instruction! {
    pub Andi {
        name: "andi",
        opcode: 0b_0101_000,
        itype: I,
    }
}

instruction! {
    pub Nandi {
        name: "nandi",
        opcode: 0b_0101_001,
        itype: I,
    }
}

instruction! {
    pub Ori {
        name: "ori",
        opcode: 0b_0101_010,
        itype: I,
    }
}

instruction! {
    pub Nori {
        name: "nori",
        opcode: 0b_0101_011,
        itype: I,
    }
}

instruction! {
    pub Xori {
        name: "xori",
        opcode: 0b_0101_100,
        itype: I,
    }
}

instruction! {
    pub Xnori {
        name: "xnori",
        opcode: 0b_0101_101,
        itype: I,
    }
}
