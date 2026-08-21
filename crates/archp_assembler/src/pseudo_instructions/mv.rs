use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Mv "mv" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("addi", ops![ops[0], ops[1], 0])
        ];
    }
}
