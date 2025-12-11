use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// clr rd  =>  li rd 0
pseudo_instruction! {
    name: "clr",
    operand_types: [ RegD ],
    expander: |_, ops| (
        "li",
        op_values![
            ops[0],
            "0",
        ],
    ),
}
