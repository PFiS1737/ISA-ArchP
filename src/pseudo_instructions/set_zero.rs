use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub SetZero {
        names: [ "seqz", "snez", "sltz", "sgez", "sgtz", "slez" ],
        operand_types: [ RegD, RegS ],
        expander: |name, ops| {
            let inst = &name[..3];

            (
                inst,
                op_values![
                    ops[0],
                    ops[1],
                    "r0",
                ],
            )
        },
    }
}
