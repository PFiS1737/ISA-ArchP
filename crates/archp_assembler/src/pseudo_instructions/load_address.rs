use smallvec::smallvec;

use crate::{
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
    let instr = INSTRUCTIONS.get("addi").unwrap();

    let offset = ctx.text.len();

    // NOTE: Safe to unwrap because we have checked the operands
    //       by [crate::pseudo_instructions::Entry::assert_operand_format]
    ctx.add_relocation(instr, offset + 4, offset, &ops[1])
        .unwrap();

    Ok(smallvec![
        ("auipc", ops![ops[0], 0]),
        ("addi", ops![ops[0], ops[0], 0])
    ])
};
