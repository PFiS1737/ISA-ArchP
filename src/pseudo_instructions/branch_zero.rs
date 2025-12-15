use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// b*z rs1 offset12  =>  b* rs1 zero offset12
pseudo_instruction! {
    name: [ "beqz", "bnez", "bltz", "blez", "bgtz", "bgez" ],
    operand_types: [ RegS, Imm(12) ],
    expander: |name, ops| {
        let inst = &name[..3];

        (
            inst,
            op_values![
                ops[0],
                "zero",
                ops[1],
            ],
        )
    },
}
