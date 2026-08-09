use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Sgt {
        name: "sgt",
        operand_types: [ RegD, RegS, RegS ],
        expander: F1!("slt"),
    }
}

pseudo_instruction! {
    pub Sle {
        name: "sle",
        operand_types: [ RegD, RegS, RegS ],
        expander: F1!("sge"),
    }
}

pseudo_instruction! {
    pub Sgtu {
        name: "sgtu",
        operand_types: [ RegD, RegS, RegS ],
        expander: F1!("sltu"),
    }
}

pseudo_instruction! {
    pub Sleu {
        name: "sleu",
        operand_types: [ RegD, RegS, RegS ],
        expander: F1!("sgeu"),
    }
}

pseudo_instruction! {
    pub Seqz {
        name: "seqz",
        operand_types: [ RegD, RegS ],
        expander: F2!("seq"),
    }
}

pseudo_instruction! {
    pub Snez {
        name: "snez",
        operand_types: [ RegD, RegS ],
        expander: F2!("sne"),
    }
}

pseudo_instruction! {
    pub Sltz {
        name: "sltz",
        operand_types: [ RegD, RegS ],
        expander: F2!("slt"),
    }
}

pseudo_instruction! {
    pub Sgez {
        name: "sgez",
        operand_types: [ RegD, RegS ],
        expander: F2!("sge"),
    }
}

pseudo_instruction! {
    pub Slez {
        name: "slez",
        operand_types: [ RegD, RegS ],
        expander: F3!("sge"),
    }
}

pseudo_instruction! {
    pub Sgtz {
        name: "sgtz",
        operand_types: [ RegD, RegS ],
        expander: F3!("slt"),
    }
}

macro F1($instr:literal) {
    |ops| ($instr, ops![ops[0], ops[2], ops[1]])
}

macro F2($instr:literal) {
    |ops| ($instr, ops![ops[0], ops[1], "r0"])
}

macro F3($instr:literal) {
    |ops| ($instr, ops![ops[0], "r0", ops[1]])
}
