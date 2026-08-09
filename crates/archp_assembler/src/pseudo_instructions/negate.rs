use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Neg {
        name: "neg",
        operand_types: [ RegD, RegS ],
        expander: |ops| ("sub", ops![ops[0], "r0", ops[1]]),
    }
}
