use crate::instructions::instruction;

instruction! {
    name: "rnd",
    opcode: 0b_1100_000,
    itype: I,
    operand_types: [ RegD ],
    encode_format: [ Some, None, None ],
}
