use smallvec::smallvec;

use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::{ExpandFn, pseudo_instruction},
};

// TODO: relax to 'jal'
pseudo_instruction! {
    pub Call "call" {
        [ Ident(..) | Addition(..) ] => F1;
        [ Ident(..), Ident(..) | Addition(..) ] => F2;
    }
}

pseudo_instruction! {
    pub Tail "tail" {
        [ Ident(..) | Addition(..) ] => F3;
    }
}

pseudo_instruction! {
    pub Ret "ret" |ops| {
        [] => [
            ("jalr", ops!["r0", "ra", 0])
        ];
    }
}

const F1: ExpandFn = |ctx, ops| {
    ctx.add_auipc_relocation("jalr", &ops[0]).unwrap();

    Ok(smallvec![
        ("auipc", ops!["ra", 0]),
        ("jalr", ops!["ra", "ra", 0])
    ])
};

const F2: ExpandFn = |ctx, ops| {
    ctx.add_auipc_relocation("jalr", &ops[1]).unwrap();

    Ok(smallvec![
        ("auipc", ops![ops[0], 0]),
        ("jalr", ops![ops[0], ops[0], 0])
    ])
};

const F3: ExpandFn = |ctx, ops| {
    ctx.add_auipc_relocation("jalr", &ops[0]).unwrap();

    Ok(smallvec![
        ("auipc", ops!["r6", 0]),
        ("jalr", ops!["r0", "r6", 0])
    ])
};
