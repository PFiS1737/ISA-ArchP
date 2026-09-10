use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Inc "inc" |ops| {
        [ Ident(rd) ] => [
            ("addi", ops![rd, rd, 1])
        ];
    }
}

pseudo_instruction! {
    pub Dec "dec" |ops| {
        [ Ident(rd) ] => [
            ("subi", ops![rd, rd, 1])
        ];
    }
}
