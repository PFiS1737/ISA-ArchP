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
    pub Jal "jal" |ops| {
        [ Ident(..) | Addition(..) ] => [
            ("jal", ops!["ra", ops[0]])
        ];
        [ Ident(..), Ident(..) | Addition(..) ] => [
            ("jal", ops![ops[0], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Jr "jr" |ops| {
        [ Ident(..) ] => [
            ("jalr", ops!["r0", ops[0], 0])
        ];
        [ Ident(..), Num(..) ] => [
            ("jalr", ops!["r0", ops[0], ops[1]])
        ];
    }
}

pseudo_instruction! {
    pub Jalr "jalr" |ops| {
        [ Ident(..) ] => [
            ("jalr", ops!["ra", ops[0], 0])
        ];
        [ Ident(..), Num(..) ] => [
            ("jalr", ops!["ra", ops[0], ops[1]])
        ];
        [ Ident(..), Ident(..) ] => [
            ("jalr", ops![ops[0], ops[1], 0])
        ];
        [ Ident(..), Ident(..), Num(..) ] => [
            ("jalr", ops![ops[0], ops[1], ops[2]])
        ];
    }
}
