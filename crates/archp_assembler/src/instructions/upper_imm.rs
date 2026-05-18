use crate::instructions::instruction;

instruction! {
    pub Lui {
        name: "lui",
        opcode: 0b_1011_000,
        itype: U,
    }
}

instruction! {
    pub Auipc {
        name: "auipc",
        opcode: 0b_1011_001,
        itype: U,
    }
}
