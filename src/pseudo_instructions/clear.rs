use crate::{instructions::parse_reg, pseudo_instruction, pseudo_instructions::ExpandFn};

pseudo_instruction! {
    name: [ "clr" ],
    operand_count: 2,
    expander: F1,
}

// clr rd  =>  li rd 0
static F1: ExpandFn = |_, operands| {
    parse_reg(operands[0])?;

    Ok(vec![("li", vec![operands[0].to_string(), "0".to_string()])])
};
