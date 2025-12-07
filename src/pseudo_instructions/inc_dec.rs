use crate::pseudo_instructions::pseudo_instruction;

// inc rd  =>  addi rd rd 1
// dec rd  =>  subi rd rd 1
pseudo_instruction! {
    name: [ "inc", "dec" ],
    operand_types: [ RegD ],
    expander: |name, operands| {
        let inst = match name {
            "inc" => "addi",
            "dec" => "subi",
            _ => unreachable!(),
        };

        Ok(vec![(
            inst,
            vec![
                operands[0].to_string(),
                operands[0].to_string(),
                "1".to_string(),
            ],
        )])
    },
}
