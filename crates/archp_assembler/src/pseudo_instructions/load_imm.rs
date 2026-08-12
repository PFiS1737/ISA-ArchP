use crate::{
    encoder::immediate::{encode_immediate, split_hi_lo},
    operand::ops,
    pseudo_instructions::{ExpandFn, pseudo_instruction},
    utils::sig_ext::sign_extend,
};

pseudo_instruction! {
    pub Li {
        name: "li",
        operand_types: [ RegD, Imm(12, i) ],
        expander: F,
    }
}

const F: ExpandFn = |ctx, ops| {
    if let Ok(n) = encode_immediate(ctx, &ops[1])
        && let (lo, hi) = split_hi_lo(n, 12, true)
        && hi != 0
    {
        let mut ret = smallvec::smallvec![("lui", ops![ops[0], hi])];

        if lo != 0 {
            ret.push(("addi", ops![ops[0], ops[0], sign_extend(lo, 12)]))
        }

        ret
    } else {
        smallvec::smallvec![("addi", ops![ops[0], "r0", ops[1]])]
    }
};

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::testkit::*;

    #[test]
    fn load_imm32() {
        let li = ps_instr("li");

        assert_snapshot!(li(&["r1"]), @"Error: Pseudo-instruction 'li' requires 2 operands, got 1");
        assert_snapshot!(li(&["r1", "r2"]), @"Error: Pseudo-instruction 'li' requires operand 2 to be an immediate, got r2");
        assert_snapshot!(li(&["123", "456"]), @"Error: Pseudo-instruction 'li' requires operand 1 to be a register, got 123");

        assert_snapshot!(li(&["r1", "0x123"]), @"addi r1 r0 0x123");
        assert_snapshot!(li(&["r1", "0x1234"]), @"lui r1 1; addi r1 r1 0x234");
        assert_snapshot!(li(&["r1", "0x12345678"]), @"lui r1 0x12345; addi r1 r1 0x678");
        assert_snapshot!(li(&["r1", "0x10000000"]), @"lui r1 0x10000");
        assert_snapshot!(li(&["r1", "0xFFFFFFF"]), @"lui r1 0x10000; addi r1 r1 0xFFFFFFFF");
        assert_snapshot!(li(&["r1", "0xFFFFFFFF"]), @"addi r1 r0 0xFFFFFFFF");

        assert_snapshot!(li(&["r1", "123"]), @"addi r1 r0 123");
        assert_snapshot!(li(&["r1", "3000"]), @"lui r1 1; addi r1 r1 0xFFFFFBB8");
        assert_snapshot!(li(&["r1", "-123"]), @"addi r1 r0 -123");
        assert_snapshot!(li(&["r1", "-3000"]), @"lui r1 0xFFFFF; addi r1 r1 0x448");
    }
}
