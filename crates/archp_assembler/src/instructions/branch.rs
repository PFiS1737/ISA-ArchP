use crate::instructions::instruction;

instruction! {
    pub Beq {
        name: "beq",
        opcode: 0b_1001_001,
        itype: B,
    }
}

instruction! {
    pub Bne {
        name: "bne",
        opcode: 0b_1001_010,
        itype: B,
    }
}

instruction! {
    pub Blt {
        name: "blt",
        opcode: 0b_1001_011,
        itype: B,
    }
}

instruction! {
    pub Ble {
        name: "bge",
        opcode: 0b_1001_100,
        itype: B,
    }
}

instruction! {
    pub Bgt {
        name: "bltu",
        opcode: 0b_1001_101,
        itype: B,
    }
}

instruction! {
    pub Bge {
        name: "bgeu",
        opcode: 0b_1001_110,
        itype: B,
    }
}
