use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// li rd imm12  =>  addi rd r0 imm12
pseudo_instruction! {
    name: "li",
    operand_types: [ RegD, Imm(12, i) ],
    expander: |_, ops| {
        (
            "addi",
            op_values![
                ops[0],
                "r0",
                ops[1],
            ],
        )
    },
}
