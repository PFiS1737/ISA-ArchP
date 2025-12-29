use crate::instructions::instruction;

instruction! {
    pub Jal {
        name: "jal",
        opcode: 0b_1001_000,
        itype: I,
        operand_types: [ RegD, Addr ],
        encode_format: [ Some, None, Some ],
    }
}

instruction! {
    pub Jalr {
        name: "jalr",
        opcode: 0b_1001_111,
        itype: I,
        operand_types: [ RegD, RegS ],
        encode_format: [ Some, Some, None ],
    }
}
