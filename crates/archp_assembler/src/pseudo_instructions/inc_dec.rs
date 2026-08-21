use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Inc "inc" |ops| {
        [ Ident(..) ] => [
            ("addi", ops![ops[0], ops[0], 1])
        ];
    }
}

pseudo_instruction! {
    pub Dec "dec" |ops| {
        [ Ident(..) ] => [
            ("subi", ops![ops[0], ops[0], 1])
        ];
    }
}
