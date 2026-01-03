use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// neg rd rs  =>  sub rd r0 rs
pseudo_instruction! {
    name: "neg",
    operand_types: [ RegD, RegS ],
    expander: |_, ops| (
        "sub",
        op_values![
            ops[0],
            "r0",
            ops[1],
        ],
    ),
}
