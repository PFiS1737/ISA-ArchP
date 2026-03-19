use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// b*z rs1 offset12  =>  b* rs1 r0 offset12
pseudo_instruction! {
    pub BranchZero {
        names: [ "beqz", "bnez", "bltz", "blez", "bgtz", "bgez" ],
        operand_types: [ RegS, Addr(12) ],
        expander: |name, ops| {
            let inst = &name[..3];

            (
                inst,
                op_values![
                    ops[0],
                    "r0",
                    ops[1],
                ],
            )
        },
    }
}
