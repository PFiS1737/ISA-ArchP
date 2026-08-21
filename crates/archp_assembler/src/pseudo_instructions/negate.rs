use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Neg "neg" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("sub", ops![ops[0], "r0", ops[1]])
        ];
    }
}
