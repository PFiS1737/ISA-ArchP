use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Mv "mv" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("addi", ops![rd, rs, 0])
        ];
    }
}
