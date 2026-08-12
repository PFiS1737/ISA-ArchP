use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Not {
        name: "not",
        operand_types: [ RegD, RegS ],
        expander: |_, ops| smallvec::smallvec![("xori", ops![ops[0], ops[1], -1])],
    }
}
