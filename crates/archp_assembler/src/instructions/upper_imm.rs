use crate::instructions::instruction;

instruction! {
    pub Lui {
        name: "lui",
        opcode: 0b0001011,
        itype: U,
    }
}

instruction! {
    pub Auipc {
        name: "auipc",
        opcode: 0b0001100,
        itype: U,
    }
}
