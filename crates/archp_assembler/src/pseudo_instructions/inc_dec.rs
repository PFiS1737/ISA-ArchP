use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Inc {
        name: "inc",
        operand_types: [ RegD ],
        expander: F!("addi"),
    }
}

pseudo_instruction! {
    pub Dec {
        name: "dec",
        operand_types: [ RegD ],
        expander: F!("subi"),
    }
}

macro F($instr:literal) {
    |ops| ($instr, ops![ops[0], ops[0], 1])
}
