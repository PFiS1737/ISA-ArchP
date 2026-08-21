use smallvec::smallvec;

use crate::{
    encoder::immediate::split_hi_lo,
    operand::{Operand::*, ops},
    pseudo_instructions::{ExpandFn, pseudo_instruction},
    utils::sig_ext::sign_extend,
};

pseudo_instruction! {
    pub Li "li" {
        [ Ident(..), Num(..) ] => F;
    }
}

const F: ExpandFn = |_, ops| {
    if let Ok(n) = ops[1].cast_immediate()
        && let (lo, hi) = split_hi_lo(n, 12, true)
        && hi != 0
    {
        let mut ret = smallvec![("lui", ops![ops[0], hi])];

        if lo != 0 {
            ret.push(("addi", ops![ops[0], ops[0], sign_extend(lo, 12)]))
        }

        Ok(ret)
    } else {
        Ok(smallvec![("addi", ops![ops[0], "r0", ops[1]])])
    }
};

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::{super::test_ps_instr, *};

    #[test]
    fn load_imm32() {
        assert_snapshot!(test_ps_instr!(Li "r1"), @"Error: operands mismatch");
        assert_snapshot!(test_ps_instr!(Li "r1", "r2"), @"Error: operands mismatch");
        assert_snapshot!(test_ps_instr!(Li 123, "r2"), @"Error: operands mismatch");

        assert_snapshot!(test_ps_instr!(Li "r1", 0x123), @"addi r1 r0 291");
        assert_snapshot!(test_ps_instr!(Li "r1", 0x1234), @"lui r1 1; addi r1 r1 564");
        assert_snapshot!(test_ps_instr!(Li "r1", 0x12345678), @"lui r1 0x12345; addi r1 r1 0x678");
        assert_snapshot!(test_ps_instr!(Li "r1", 0x10000000), @"lui r1 0x10000");
        assert_snapshot!(test_ps_instr!(Li "r1", 0xFFFFFFF), @"lui r1 0x10000; addi r1 r1 0xFFFFFFFF");
        assert_snapshot!(test_ps_instr!(Li "r1", 0xFFFFFFFF_i64), @"addi r1 r0 0xFFFFFFFF");

        assert_snapshot!(test_ps_instr!(Li "r1", 123), @"addi r1 r0 123");
        assert_snapshot!(test_ps_instr!(Li "r1", 3000), @"lui r1 1; addi r1 r1 0xFFFFFBB8");
        assert_snapshot!(test_ps_instr!(Li "r1", -123), @"addi r1 r0 -123");
        assert_snapshot!(test_ps_instr!(Li "r1", -3000), @"lui r1 0xFFFFF; addi r1 r1 0x448");
    }
}
