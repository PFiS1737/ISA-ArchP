use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Neg {
        name: "neg",
        format: [ RegD, RegS ],
        expander: |_, ops| smallvec::smallvec![("sub", ops![ops[0], "r0", ops[1]])],
    }
}
