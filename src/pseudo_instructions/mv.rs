use crate::pseudo_instruction;

// mv rd rs  =>  addi rd rs 0
pseudo_instruction! {
    name: "mv",
    format: [ Reg, Reg ],
    expander: |_, operands| {
        Ok(vec![(
            "addi",
            vec![
                operands[0].to_string(),
                operands[1].to_string(),
                "0".to_string(),
            ],
        )])
    },
}
