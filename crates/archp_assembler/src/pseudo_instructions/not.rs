use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Not "not" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("xori", ops![ops[0], ops[1], -1])
        ];
    }
}
