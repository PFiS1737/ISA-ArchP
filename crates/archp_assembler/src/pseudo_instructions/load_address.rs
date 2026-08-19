use crate::{
    instructions::INSTRUCTIONS,
    operand::ops,
    pseudo_instructions::{ExpandFn, pseudo_instruction},
};

// TODO: change this
pseudo_instruction! {
    pub La {
        name: "la",
        format: [ RegD, Addr(12) ],
        expander: F,
    }
}

pseudo_instruction! {
    pub Lla {
        name: "lla",
        format: [ RegD, Addr(12) ],
        expander: F,
    }
}

const F: ExpandFn = |ctx, ops| {
    let instr = INSTRUCTIONS.get("addi").unwrap();
    let offset = ctx.text.len();
    // INFO: Safe to unwrap because we have checked the operands
    //       by [crate::pseudo_instructions::Entry::assert_operand_format]
    ctx.add_relocation(instr, offset, &ops[1]).unwrap();

    smallvec::smallvec![("addi", ops![ops[0], ops[0], 0])]
};
