use crate::instruction;

instruction! {
    name: "ret",
    opcode: 0b_1010_100,
    itype: I,
    operand_types: [],
    encode_format: [ None, None, None ],
}

instruction! {
    name: "call",
    opcode: 0b_1010_101,
    itype: I,
    operand_types: [ Imm(12) ],
    encode_format: [ None, None, Some ],
}
