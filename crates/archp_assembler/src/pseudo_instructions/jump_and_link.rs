use smallvec::smallvec;

use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::{ExpandFn, pseudo_instruction},
};

pseudo_instruction! {
    pub J "j" |ops| {
        [ Ident(..) | Addition(..) ] => [
            ("jal", ops!["r0", ops[0]])
        ];
    }
}

// TODO: relax to 'jal'
pseudo_instruction! {
    pub Jump "jump" {
        [ Ident(..) | Addition(..), Ident(..) ] => F;
    }
}

const F: ExpandFn = |ctx, ops| {
    ctx.add_auipc_relocation("jalr", &ops[0]).unwrap();

    Ok(smallvec![
        ("auipc", ops![ops[1], 0]),
        ("jalr", ops!["r0", ops[1], 0])
    ])
};

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
