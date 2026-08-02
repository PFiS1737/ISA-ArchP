use crate::instructions::instruction;

instruction! {
    pub Ecall {
        name: "ecall",
        opcode: 0b0100000,
        funct3: 0b000,
        itype: R, // TODO: new itype: None
        operands_format: [ _, _, _ ],
    }
}
