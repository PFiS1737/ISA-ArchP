use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub J "j" |ops| {
        [ Ident(..) | Addition(..) ] => [
            ("jal", ops!["r0", ops[0]])
        ];
    }
}

pseudo_instruction! {
    pub Jr "jr" |ops| {
        [ Ident(..), Num(..) ] => [
            ("jalr", ops!["r0", ops[0], ops[1]])
        ];
    }
}
