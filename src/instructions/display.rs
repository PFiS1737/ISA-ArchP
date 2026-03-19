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
        operands_format: [ _, RegS, RegS ],
    }
}

instruction! {
    pub Seg {
        name: "seg",
        opcode: 0b_1101_010,
        itype: R,
        operands_format: [ _, _, RegS ],
    }
}

instruction! {
    pub Segi {
        name: "segi",
        opcode: 0b_1101_011,
        itype: I,
        operands_format: [ _, _, Imm(12, i) ],
    }
}
