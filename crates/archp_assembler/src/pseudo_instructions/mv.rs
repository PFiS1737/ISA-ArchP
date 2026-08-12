use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Mv {
        name: "mv",
        operand_types: [ RegD, RegS ],
        expander: |_, ops| smallvec::smallvec![("addi", ops![ops[0], ops[1], 0])],
    }
}
