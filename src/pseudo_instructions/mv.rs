use crate::pseudo_instructions::pseudo_instruction;

// mv rd rs  =>  addi rd rs 0
pseudo_instruction! {
    name: "mv",
    operand_types: [ RegD, RegS ],
    expander: |_, operands| {
        vec![(
            "addi",
            vec![
                operands[0].to_string(),
                operands[1].to_string(),
                "0".to_string(),
            ],
        )]
    },
}
