use crate::instructions::instruction;

instruction! {
    pub Ret {
        name: "ret",
        opcode: 0b_1010_100,
        itype: I,
        operand_types: [],
        encode_format: [ None, None, None ],
    }
}

instruction! {
    pub Call {
        name: "call",
        opcode: 0b_1010_101,
        itype: I,
        operand_types: [ Addr ],
        encode_format: [ None, None, Some ],
    }
}
