use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
};

// TODO: remove this

macro_instruction! {
    pub RiscvLoad {
        names: [ "lw", "lh", "lhu", "lb", "lbu" ],
        expander: F1,
    }
}

macro_instruction! {
    pub RiscvJalr {
        names: [ "jalr" ],
        expander: F1,
    }
}

macro_instruction! {
    pub RiscvJr {
        names: [ "jr" ],
        expander: F2,
    }
}

macro_instruction! {
    pub RiscvSave {
        names: [ "sw", "sh", "sb" ],
        expander: F3,
    }
}

const F1: ExpandFn = |_, _, name, ops| {
    if ops.len() != 2 {
        return None;
    }

    Some(vec![(name, op_values!(ops[0], ops[1], 0))])
};

const F2: ExpandFn = |_, _, name, ops| {
    if ops.len() != 1 {
        return None;
    }

    Some(vec![(name, op_values!(ops[0], 0))])
};

const F3: ExpandFn = |_, _, name, ops| {
    if ops.len() != 2 {
        return None;
    }

    Some(vec![(name, op_values!(ops[0], ops[1], 0))])
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn load() {
        let lw = mc_instr("lw");

        assert_snapshot!(lw(&["r1"]), @"");
        assert_snapshot!(lw(&["r1", "r2", "123"]), @"");

        assert_snapshot!(lw(&["r1", "100"]), @"lw r1 100 0");
        assert_snapshot!(lw(&["r1", "base"]), @"lw r1 base 0");
    }

    #[test]
    fn save() {
        let sw = mc_instr("sw");

        assert_snapshot!(sw(&["r1"]), @"");
        assert_snapshot!(sw(&["r1", "r2", "123"]), @"");

        assert_snapshot!(sw(&["r1", "100"]), @"sw r1 100 0");
        assert_snapshot!(sw(&["r1", "base"]), @"sw r1 base 0");
    }
}
