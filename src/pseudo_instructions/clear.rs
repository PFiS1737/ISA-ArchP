use crate::pseudo_instructions::pseudo_instruction;

// clr rd  =>  li rd 0
pseudo_instruction! {
    name: "clr",
    operand_types: [ RegD ],
    expander: |_, operands| {
        vec![(
            "li",
            vec![
                operands[0].to_string(),
                "0".to_string(),
            ],
        )]
    },
}
