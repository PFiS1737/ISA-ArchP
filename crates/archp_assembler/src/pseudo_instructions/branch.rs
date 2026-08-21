use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Bgt "bgt" |ops| {
        [ Ident(..), Ident(..), Ident(..) | Addition(..) ] => [
            ("blt", ops![ops[1], ops[0], ops[2]])
        ];
    }
}

pseudo_instruction! {
    pub Ble "ble" |ops| {
        [ Ident(..), Ident(..), Ident(..) | Addition(..) ] => [
            ("bge", ops![ops[1], ops[0], ops[2]])
        ];
    }
}

pseudo_instruction! {
    pub Bgtu "bgtu" |ops| {
        [ Ident(..), Ident(..), Ident(..) | Addition(..) ] => [
            ("bltu", ops![ops[1], ops[0], ops[2]])
        ];
    }
}

pseudo_instruction! {
    pub Bleu "bleu" |ops| {
        [ Ident(..), Ident(..), Ident(..) | Addition(..) ] => [
            ("bgeu", ops![ops[1], ops[0], ops[2]])
        ];
    }
}

pseudo_instruction! {
    pub Beqz "beqz" |ops| {
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("beq", ops![ops[0], "r0", ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Bnez "bnez" |ops| {
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("bne", ops![ops[0], "r0", ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Bltz "bltz" |ops| {
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("blt", ops![ops[0], "r0", ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Bgez "bgez" |ops| {
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("bge", ops![ops[0], "r0", ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Blez "blez" |ops| {
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("bge", ops!["r0", ops[0], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Bgtz "bgtz" |ops| {
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("blt", ops!["r0", ops[0], ops[1]])
        ];
    }
}
