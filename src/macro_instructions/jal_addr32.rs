use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::parse_address,
};

macro_instruction! {
    /// jal rd addr32  =>  auipc tmp addr20; jalr rd tmp addr12
    pub JalAddr32 {
        name: "jal",
        operand_count: 2,
        expander: F1,
    }
}

macro_instruction! {
    /// j addr32  =>  auipc tmp addr20; jr tmp addr12
    pub JAddr32 {
        name: "j",
        operand_count: 1,
        expander: F2,
    }
}

// FIXME: 有端倪，在这里展开，有可能因为找不到 label 导致失败

const F1: ExpandFn = |ctx, pc, _, ops| {
    if let Ok(addr) = parse_address(ctx, &ops[1])
        && let (hi, lo) = addr.try_as_i12(pc)
        && hi != 0
    {
        Some(vec![
            ("auipc", op_values!["tmp", hi]),
            ("jalr", op_values![ops[0], "tmp", lo]),
        ])
    } else {
        None
    }
};

const F2: ExpandFn = |ctx, pc, _, ops| {
    if let Ok(addr) = parse_address(ctx, &ops[0])
        && let (hi, lo) = addr.try_as_i12(pc)
        && hi != 0
    {
        Some(vec![
            ("auipc", op_values!["tmp", hi]),
            ("jr", op_values!["tmp", lo]),
        ])
    } else {
        None
    }
};

// TODO: test
