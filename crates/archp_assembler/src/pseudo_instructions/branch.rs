use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Bgt {
        name: "bgt",
        format: [ RegS, RegS, Addr(12) ],
        expander: F1!("blt"),
    }
}

pseudo_instruction! {
    pub Ble {
        name: "ble",
        format: [ RegS, RegS, Addr(12) ],
        expander: F1!("bge"),
    }
}

pseudo_instruction! {
    pub Bgtu {
        name: "bgtu",
        format: [ RegS, RegS, Addr(12) ],
        expander: F1!("bltu"),
    }
}

pseudo_instruction! {
    pub Bleu {
        name: "bleu",
        format: [ RegS, RegS, Addr(12) ],
        expander: F1!("bgeu"),
    }
}

pseudo_instruction! {
    pub Beqz {
        name: "beqz",
        format: [ RegS, Addr(12) ],
        expander: F2!("beq"),
    }
}

pseudo_instruction! {
    pub Bnez {
        name: "bnez",
        format: [ RegS, Addr(12) ],
        expander: F2!("bne"),
    }
}

pseudo_instruction! {
    pub Bltz {
        name: "bltz",
        format: [ RegS, Addr(12) ],
        expander: F2!("blt"),
    }
}

pseudo_instruction! {
    pub Bgez {
        name: "bgez",
        format: [ RegS, Addr(12) ],
        expander: F2!("bge"),
    }
}

pseudo_instruction! {
    pub Blez {
        name: "blez",
        format: [ RegS, Addr(12) ],
        expander: F3!("bge"),
    }
}

pseudo_instruction! {
    pub Bgtz {
        name: "bgtz",
        format: [ RegS, Addr(12) ],
        expander: F3!("blt"),
    }
}

macro F1($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops![ops[1], ops[0], ops[2]])]
}

macro F2($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops![ops[0], "r0", ops[1]])]
}

macro F3($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops!["r0", ops[0], ops[1]])]
}
