use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Sgt "sgt" |ops| {
        [ Ident(..), Ident(..), Ident(..) ] => [
            ("slt", ops![ops[0], ops[2], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Sle "sle" |ops| {
        [ Ident(..), Ident(..), Ident(..) ] => [
            ("sge", ops![ops[0], ops[2], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Sgtu "sgtu" |ops| {
        [ Ident(..), Ident(..), Ident(..) ] => [
            ("sltu", ops![ops[0], ops[2], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Sleu "sleu" |ops| {
        [ Ident(..), Ident(..), Ident(..) ] => [
            ("sgeu", ops![ops[0], ops[2], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Seqz "seqz" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("seq", ops![ops[0], ops[1], "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Snez "snez" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("sne", ops![ops[0], ops[1], "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Sltz "sltz" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("slt", ops![ops[0], ops[1], "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Sgez "sgez" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("sge", ops![ops[0], ops[1], "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Slez "slez" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("sge", ops![ops[0], "r0", ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Sgtz "sgtz" |ops| {
        [ Ident(..), Ident(..) ] => [
            ("slt", ops![ops[0], "r0", ops[1]])
        ];
    }
}
