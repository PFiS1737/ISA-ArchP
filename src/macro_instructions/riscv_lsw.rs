use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::{OperandValue, op_values},
};

// INFO: We only do replacement here, the validity of the operands will
//       be checked during encoding.

// lw rd imm(rs)  =>  lw rd rs imm12  (base=rs, offset=imm12)
macro_instruction! {
    name: "lw",
    expander: F1,
}

const F1: ExpandFn = |_, _, cond, ops| {
    if ops.len() != 2 {
        return None;
    }

    let (imm, base) = parse_offset(&ops[1])?;

    Some(vec![("lw", cond, op_values!(ops[0], base, imm))])
};

// sw rs2 imm(rs1)  =>  sw rs1 rs2 imm12  (base=rs1, offset=imm12)
macro_instruction! {
    name: "sw",
    expander: F2,
}

const F2: ExpandFn = |_, _, cond, ops| {
    if ops.len() != 2 {
        return None;
    }

    let (imm, base) = parse_offset(&ops[1])?;

    Some(vec![("sw", cond, op_values!(base, ops[0], imm))])
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
    fn lw() {
        let lw = mc_instr("lw");

        assert_snapshot!(lw("", &["r1"]), @"");
        assert_snapshot!(lw("", &["r1", "r2", "123"]), @"");

        assert_snapshot!(lw("", &["r1", "100sp)"]), @"");
        assert_snapshot!(lw("", &["r1", "100(sp"]), @"");

        assert_snapshot!(lw("", &["r1", "100"]), @"lw r1 100 0");
        assert_snapshot!(lw("", &["r1", "base"]), @"lw r1 base 0");

        assert_snapshot!(lw("", &["r0", "10(r1)"]), @"lw r0 r1 10");
        assert_snapshot!(lw("", &["r1", "100(sp)"]), @"lw r1 sp 100");
        assert_snapshot!(lw("eq", &["r1", "200(r5)"]), @"lw.eq r1 r5 200");
    }

    #[test]
    fn sw() {
        let sw = mc_instr("sw");

        assert_snapshot!(sw("", &["r1"]), @"");
        assert_snapshot!(sw("", &["r1", "r2", "123"]), @"");

        assert_snapshot!(sw("", &["r1", "100sp)"]), @"");
        assert_snapshot!(sw("", &["r1", "100(sp"]), @"");

        assert_snapshot!(sw("", &["r1", "100"]), @"sw 100 r1 0");
        assert_snapshot!(sw("", &["r1", "base"]), @"sw base r1 0");

        assert_snapshot!(sw("", &["r2", "10(r1)"]), @"sw r1 r2 10");
        assert_snapshot!(sw("", &["r3", "100(sp)"]), @"sw sp r3 100");
        assert_snapshot!(sw("ne", &["r4", "200(r5)"]), @"sw.ne r5 r4 200");
    }
}
