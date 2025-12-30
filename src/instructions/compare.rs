use crate::instructions::instruction;

instruction! {
    pub Cmp {
        name: "cmp",
        opcode: 0b_0011_000,
        itype: R,
        operands_format: [ _, RegS, RegS ],
    }
}

instruction! {
    pub Cmpi {
        name: "cmpi",
        opcode: 0b_0111_000,
        itype: I,
        operands_format: [ _, RegS, Imm(12, i) ],
    }
}
