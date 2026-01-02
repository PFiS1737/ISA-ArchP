use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

// j addr12  =>  jal r0 addr12
pseudo_instruction! {
    name: "j",
    operand_types: [ Addr ],
    expander: |_, ops| {
        (
            "jal",
            op_values![
                "r0",
                ops[0],
            ],
        )
    },
}

// jr rs1  =>  jalr r0 rs1
pseudo_instruction! {
    name: "jr",
    operand_types: [ RegS ],
    expander: |_, ops| {
        (
            "jalr",
            op_values![
                "r0",
                ops[0],
            ],
        )
    },
}
