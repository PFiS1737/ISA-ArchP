use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub ColImm {
        name: "col",
        operand_types: [ Imm(12, i) ],
        expander: |_, ops| {
            (
                "colr",
                op_values![
                    "r0",
                    "r0",
                    ops[0],
                ],
            )
        },
    }
}
