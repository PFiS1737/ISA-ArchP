use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Inc {
        name: "inc",
        format: [ RegD ],
        expander: F!("addi"),
    }
}

pseudo_instruction! {
    pub Dec {
        name: "dec",
        format: [ RegD ],
        expander: F!("subi"),
    }
}

macro F($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops![ops[0], ops[0], 1])]
}
