use crate::pseudo_instructions::pseudo_instruction;

// clr rd  =>  li rd 0
pseudo_instruction! {
    name: "clr",
    format: [ RegD ],
    expander: |_, operands| {
        Ok(vec![(
            "li",
            vec![
                operands[0].to_string(),
                "0".to_string(),
            ],
        )])
    },
}
