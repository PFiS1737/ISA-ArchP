use crate::pseudo_instructions::pseudo_instruction;

// inc rd  =>  addi rd rd 1
// dec rd  =>  subi rd rd 1
pseudo_instruction! {
    name: [ "inc", "dec" ],
    operand_types: [ RegD ],
    expander: |name, ops| {
        let inst = match name {
            "inc" => "addi",
            "dec" => "subi",
            _ => unreachable!(),
        };

        (
            inst,
            vec![
                ops[0],
                ops[0],
                "1",
            ],
        )
    },
}
