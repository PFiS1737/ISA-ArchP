use crate::{instructions::parse_reg, pseudo_instruction, pseudo_instructions::ExpandFn};

pseudo_instruction! {
    name: [ "mv" ],
    operand_count: 2,
    expander: F1,
}

// mv rs rd  =>  addi rd rs 0
static F1: ExpandFn = |_, operands| {
    parse_reg(operands[0])?;
    parse_reg(operands[1])?;

    Ok(vec![(
        "addi",
        vec![
            operands[1].to_string(),
            operands[0].to_string(),
            "0".to_string(),
        ],
    )])
};
