use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Nop {
        name: "nop",
        format: [],
        expander: |_, _| smallvec::smallvec![("addi", ops!["r0", "r0", 0])],
    }
}
