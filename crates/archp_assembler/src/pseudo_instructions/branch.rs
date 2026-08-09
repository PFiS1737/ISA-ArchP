use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Bgt {
        name: "bgt",
        operand_types: [ RegS, RegS, Addr(12) ],
        expander: F1!("blt"),
    }
}

pseudo_instruction! {
    pub Ble {
        name: "ble",
        operand_types: [ RegS, RegS, Addr(12) ],
        expander: F1!("bge"),
    }
}

pseudo_instruction! {
    pub Bgtu {
        name: "bgtu",
        operand_types: [ RegS, RegS, Addr(12) ],
        expander: F1!("bltu"),
    }
}

pseudo_instruction! {
    pub Bleu {
        name: "bleu",
        operand_types: [ RegS, RegS, Addr(12) ],
        expander: F1!("bgeu"),
    }
}

pseudo_instruction! {
    pub Beqz {
        name: "beqz",
        operand_types: [ RegS, Addr(12) ],
        expander: F2!("beq"),
    }
}

pseudo_instruction! {
    pub Bnez {
        name: "bnez",
        operand_types: [ RegS, Addr(12) ],
        expander: F2!("bne"),
    }
}

pseudo_instruction! {
    pub Bltz {
        name: "bltz",
        operand_types: [ RegS, Addr(12) ],
        expander: F2!("blt"),
    }
}

pseudo_instruction! {
    pub Bgez {
        name: "bgez",
        operand_types: [ RegS, Addr(12) ],
        expander: F2!("bge"),
    }
}

pseudo_instruction! {
    pub Blez {
        name: "blez",
        operand_types: [ RegS, Addr(12) ],
        expander: F3!("bge"),
    }
}

pseudo_instruction! {
    pub Bgtz {
        name: "bgtz",
        operand_types: [ RegS, Addr(12) ],
        expander: F3!("blt"),
    }
}

macro F1($instr:literal) {
    |ops| ($instr, ops![ops[1], ops[0], ops[2]])
}

macro F2($instr:literal) {
    |ops| ($instr, ops![ops[0], "r0", ops[1]])
}

macro F3($instr:literal) {
    |ops| ($instr, ops!["r0", ops[0], ops[1]])
}
