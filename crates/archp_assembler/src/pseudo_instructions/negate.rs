use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Neg "neg" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("sub", ops![rd, "r0", rs])
        ];
    }
}
