use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Li {
        name: "li",
        operand_types: [ RegD, Imm(12, i) ],
        expander: |ops| ("addi", ops![ops[0], "r0", ops[1]]),
    }
}
