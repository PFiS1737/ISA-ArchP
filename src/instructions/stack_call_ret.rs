use crate::instructions::instruction;

instruction! {
    pub Pop {
        name: "pop",
        opcode: 0b_1010_000,
        itype: I,
        operand_types: [ RegD ],
        encode_format: [ Some, None, None ],
    }
}

instruction! {
    pub Push {
        name: "push",
        opcode: 0b_1010_001,
        itype: I,
        operand_types: [ RegS ],
        encode_format: [ None, Some, None ],
    }
}
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
