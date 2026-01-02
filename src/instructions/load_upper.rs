use crate::instructions::instruction;

instruction! {
    pub Lui {
        name: "lui",
        opcode: 0b_1011_000,
        itype: U,
    }
}
