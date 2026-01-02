use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// not rd rs  =>  xori rd rs -1
pseudo_instruction! {
    name: "not",
    operand_types: [ RegD, RegS ],
    expander: |_, ops| (
        "xori",
        op_values![
            ops[0],
            ops[1],
            -1,
        ],
    ),
}
