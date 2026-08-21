use smallvec::smallvec;

use crate::{
    context::RelocationType,
    instructions::INSTRUCTIONS,
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
    let auipc = INSTRUCTIONS.get("auipc").unwrap();
    let addi = INSTRUCTIONS.get("addi").unwrap();

    let offset = ctx.text.len();

    ctx.add_relocation(auipc, RelocationType::High, offset, offset, &ops[1])
        .unwrap();

    ctx.add_relocation(addi, RelocationType::Low, offset + 4, offset, &ops[1])
        .unwrap();

    Ok(smallvec![
        ("auipc", ops![ops[0], 0]),
        ("addi", ops![ops[0], ops[0], 0])
    ])
};
