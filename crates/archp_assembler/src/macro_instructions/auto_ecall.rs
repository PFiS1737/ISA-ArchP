use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::ops,
};

macro_instruction! {
    pub AutoEcall {
        name: "ecall",
        expander: F,
    }
}

const F: ExpandFn = |_, _, _, ops| {
    if ops.len() != 1 {
        return None;
    }
    Some(vec![("li", ops!["r17", ops[0]]), ("ecall", ops![])])
};
