use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Not "not" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("xori", ops![rd, rs, -1])
        ];
    }
}
