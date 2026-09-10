use smallvec::smallvec;

use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::{ExpandFn, pseudo_instruction},
};

pseudo_instruction! {
    pub La "la" {
        [ Ident(..), Ident(..) | Addition(..) ] => F;
    }
}

pseudo_instruction! {
    pub Lla "lla" {
        [ Ident(..), Ident(..) | Addition(..) ] => F;
    }
}

const F: ExpandFn = |ctx, ops| {
    ctx.add_auipc_relocation("addi", &ops[1]).unwrap();

    Ok(smallvec![
        ("auipc", ops![ops[0], 0]),
        ("addi", ops![ops[0], ops[0], 0])
    ])
};
