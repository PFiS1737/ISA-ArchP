use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Set {
        names: [ "sgt", "sle", "sgtu", "sleu" ],
        operand_types: [ RegD, RegS, RegS ],
        expander: |name, ops| {
            let inst = match name {
                "sgt" => "slt",
                "sle" => "sge",
                "sgtu" => "sltu",
                "sleu" => "sgeu",
                _ => unreachable!(),
            };

            (
                inst,
                op_values![
                    ops[0],
                    ops[2],
                    ops[1],
                ],
            )
        },
    }
}
