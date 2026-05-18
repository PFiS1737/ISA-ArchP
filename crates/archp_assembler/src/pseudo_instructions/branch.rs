use crate::{operand::op_values, pseudo_instructions::pseudo_instruction};

pseudo_instruction! {
    pub Branch {
        names: [ "bgt", "ble", "bgtu", "bleu" ],
        operand_types: [ RegS, RegS, Addr(12) ],
        expander: |name, ops| {
            let inst = match name {
                "bgt" => "blt",
                "ble" => "bge",
                "bgtu" => "bltu",
                "bleu" => "bgeu",
                _ => unreachable!(),
            };

            (
                inst,
                op_values![
                    ops[1],
                    ops[0],
                    ops[2],
                ],
            )
        },
    }
}
