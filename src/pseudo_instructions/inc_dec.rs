use crate::{instructions::parse_reg, pseudo_instruction, pseudo_instructions::ExpandFn};

pseudo_instruction! {
    name: [ "inc", "dec" ],
    operand_count: 1,
    expander: F1,
}

// inc rd  =>  addi rd rd 1
// dec rd  =>  subi rd rd 1
static F1: ExpandFn = |name, operands| {
    parse_reg(operands[0])?;

    let inst = match name {
        "inc" => "addi",
        "dec" => "subi",
        _ => unreachable!(),
    };

    Ok(vec![(
        inst,
        vec![
            operands[0].to_string(),
            operands[0].to_string(),
            "1".to_string(),
        ],
    )])
};
