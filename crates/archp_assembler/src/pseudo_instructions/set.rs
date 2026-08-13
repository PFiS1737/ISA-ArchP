use crate::{operand::ops, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Sgt {
        name: "sgt",
        format: [ RegD, RegS, RegS ],
        expander: F1!("slt"),
    }
}

pseudo_instruction! {
    pub Sle {
        name: "sle",
        format: [ RegD, RegS, RegS ],
        expander: F1!("sge"),
    }
}

pseudo_instruction! {
    pub Sgtu {
        name: "sgtu",
        format: [ RegD, RegS, RegS ],
        expander: F1!("sltu"),
    }
}

pseudo_instruction! {
    pub Sleu {
        name: "sleu",
        format: [ RegD, RegS, RegS ],
        expander: F1!("sgeu"),
    }
}

pseudo_instruction! {
    pub Seqz {
        name: "seqz",
        format: [ RegD, RegS ],
        expander: F2!("seq"),
    }
}

pseudo_instruction! {
    pub Snez {
        name: "snez",
        format: [ RegD, RegS ],
        expander: F2!("sne"),
    }
}

pseudo_instruction! {
    pub Sltz {
        name: "sltz",
        format: [ RegD, RegS ],
        expander: F2!("slt"),
    }
}

pseudo_instruction! {
    pub Sgez {
        name: "sgez",
        format: [ RegD, RegS ],
        expander: F2!("sge"),
    }
}

pseudo_instruction! {
    pub Slez {
        name: "slez",
        format: [ RegD, RegS ],
        expander: F3!("sge"),
    }
}

pseudo_instruction! {
    pub Sgtz {
        name: "sgtz",
        format: [ RegD, RegS ],
        expander: F3!("slt"),
    }
}

macro F1($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops![ops[0], ops[2], ops[1]])]
}

macro F2($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops![ops[0], ops[1], "r0"])]
}

macro F3($instr:literal) {
    |_, ops| smallvec::smallvec![($instr, ops![ops[0], "r0", ops[1]])]
}
