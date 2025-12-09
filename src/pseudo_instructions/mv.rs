use crate::pseudo_instructions::pseudo_instruction;

// mv rd rs  =>  addi rd rs 0
pseudo_instruction! {
    name: "mv",
    operand_types: [ RegD, RegS ],
    expander: |_, ops| (
        "addi",
        vec![
            ops[0],
            ops[1],
            "0",
        ],
    ),
}
