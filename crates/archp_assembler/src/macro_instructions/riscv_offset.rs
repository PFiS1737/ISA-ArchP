use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::{OperandValue, op_values},
};

// INFO: We only do replacement here, the validity of the operands will
//       be checked during encoding.

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

    let (imm, base) = parse_offset(&ops[1])?;

    Some(vec![(name, op_values!(ops[0], base, imm))])
};

const F2: ExpandFn = |_, _, name, ops| {
    if ops.len() != 1 {
        return None;
    }

    let (imm, base) = parse_offset(&ops[0])?;

    Some(vec![(name, op_values!(base, imm))])
};

const F3: ExpandFn = |_, _, name, ops| {
    if ops.len() != 2 {
        return None;
    }

    let (imm, base) = parse_offset(&ops[1])?;

    Some(vec![(name, op_values!(base, ops[0], imm))])
};

fn parse_offset<'a>(op: &OperandValue<'a>) -> Option<(&'a str, &'a str)> {
    let s = match op {
        OperandValue::StringSlice(s) => s,
        OperandValue::Unsigned(_) | OperandValue::Signed(_) => return None,
    };

    if let Some(s) = s.strip_suffix(')') {
        s.split_once('(')
    } else if s.find('(').is_none() {
        Some(("0", s))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn parse_offset() {
        use super::{OperandValue, parse_offset as f};

        assert!(f(&OperandValue::StringSlice("100sp)")).is_none());
        assert!(f(&OperandValue::StringSlice("100(sp")).is_none());
        assert!(f(&OperandValue::Unsigned(123)).is_none());

        assert_eq!(f(&OperandValue::StringSlice("100")).unwrap(), ("0", "100"));
        assert_eq!(f(&OperandValue::StringSlice("r1")).unwrap(), ("0", "r1"));
        assert_eq!(
            f(&OperandValue::StringSlice("100(sp)")).unwrap(),
            ("100", "sp")
        );
    }

    #[test]
    fn load() {
        let lw = mc_instr("lw");

        assert_snapshot!(lw(&["r1"]), @"");
        assert_snapshot!(lw(&["r1", "r2", "123"]), @"");

        assert_snapshot!(lw(&["r1", "100sp)"]), @"");
        assert_snapshot!(lw(&["r1", "100(sp"]), @"");

        assert_snapshot!(lw(&["r1", "100"]), @"lw r1 100 0");
        assert_snapshot!(lw(&["r1", "base"]), @"lw r1 base 0");

        assert_snapshot!(lw(&["r0", "10(r1)"]), @"lw r0 r1 10");
        assert_snapshot!(lw(&["r1", "100(sp)"]), @"lw r1 sp 100");
    }

    #[test]
    fn save() {
        let sw = mc_instr("sw");

        assert_snapshot!(sw(&["r1"]), @"");
        assert_snapshot!(sw(&["r1", "r2", "123"]), @"");

        assert_snapshot!(sw(&["r1", "100sp)"]), @"");
        assert_snapshot!(sw(&["r1", "100(sp"]), @"");

        assert_snapshot!(sw(&["r1", "100"]), @"sw 100 r1 0");
        assert_snapshot!(sw(&["r1", "base"]), @"sw base r1 0");

        assert_snapshot!(sw(&["r2", "10(r1)"]), @"sw r1 r2 10");
        assert_snapshot!(sw(&["r3", "100(sp)"]), @"sw sp r3 100");
    }
}
