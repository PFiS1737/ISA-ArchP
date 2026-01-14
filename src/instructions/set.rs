use crate::instructions::instruction;

instruction! {
    pub Seq {
        name: "seq",
        opcode: 0b_0011_001,
        itype: R,
    }
}

instruction! {
    pub Sne {
        name: "sne",
        opcode: 0b_0011_010,
        itype: R,
    }
}

instruction! {
    pub Slt {
        name: "slt",
        opcode: 0b_0011_011,
        itype: R,
    }
}

instruction! {
    pub Sge {
        name: "sge",
        opcode: 0b_0011_100,
        itype: R,
    }
}

instruction! {
    pub Sltu {
        name: "sltu",
        opcode: 0b_0011_101,
        itype: R,
    }
}

instruction! {
    pub Sgeu {
        name: "sgeu",
        opcode: 0b_0011_110,
        itype: R,
    }
}

instruction! {
    pub Seqi {
        name: "seqi",
        opcode: 0b_0111_001,
        itype: I,
    }
}

instruction! {
    pub Snei {
        name: "snei",
        opcode: 0b_0111_010,
        itype: I,
    }
}

instruction! {
    pub Slti {
        name: "slti",
        opcode: 0b_0111_011,
        itype: I,
    }
}

instruction! {
    pub Sgei {
        name: "sgei",
        opcode: 0b_0111_100,
        itype: I,
    }
}

instruction! {
    pub Sltiu {
        name: "sltiu",
        opcode: 0b_0111_101,
        itype: I,
    }
}

instruction! {
    pub Sgeiu {
        name: "sgeiu",
        opcode: 0b_0111_110,
        itype: I,
    }
}
