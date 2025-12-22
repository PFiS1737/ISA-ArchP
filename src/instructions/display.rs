use crate::instructions::instruction;

instruction! {
    pub Col {
        name: "col",
        opcode: 0b_1101_000,
        itype: C,
    }
}

instruction! {
    pub Spx {
        name: "spx",
        opcode: 0b_1101_001,
        itype: R,
        operand_types: [ RegS, RegS ],
        encode_format: [ None, Some, Some ],
    }
}

instruction! {
    pub Seg {
        name: "seg",
        opcode: 0b_1101_010,
        itype: R,
        operand_types: [ RegS ],
        encode_format: [ None, None, Some ],
    }
}

instruction! {
    pub Segi {
        name: "segi",
        opcode: 0b_1101_011,
        itype: I,
        operand_types: [ Imm(8, u) ],
        encode_format: [ None, None, Some ],
    }
}
