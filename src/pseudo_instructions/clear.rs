use crate::pseudo_instructions::pseudo_instruction;

// clr rd  =>  li rd 0
pseudo_instruction! {
    name: "clr",
    operand_types: [ RegD ],
    expander: |_, ops| (
        "li",
        vec![
            ops[0],
            "0",
        ],
    ),
}
