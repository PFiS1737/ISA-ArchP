use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub J {
        name: "j",
        operand_types: [ Addr(20) ],
        expander: |_, ops| smallvec::smallvec![("jal", ops!["r0", ops[0]])],
    }
}

pseudo_instruction! {
    pub Jr {
        name: "jr",
        operand_types: [ RegS, Imm(12, i) ],
        expander: |_, ops| smallvec::smallvec![("jalr", ops!["r0", ops[0], ops[1]])],
    }
}
