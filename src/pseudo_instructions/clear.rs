use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// clr rd  =>  addi rd r0 0
pseudo_instruction! {
    pub Clear {
        name: "clr",
        operand_types: [ RegD ],
        expander: |_, ops| (
            "addi",
            op_values![
                ops[0],
                "r0",
                0,
            ],
        ),
    }
}
