use crate::instruction;

instruction! {
    name: "rnd",
    opcode: 0b_1100_000,
    itype: I,
    operand_types: [ Reg ],
    encode_format: [ Some, None, None ],
}
