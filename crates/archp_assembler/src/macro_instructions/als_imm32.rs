use crate::{
    encoder::immediate::split_hi_lo,
    macro_instructions::{ExpandFn, macro_instruction},
    operand::ops,
};

macro_instruction! {
    pub AlsImm32 {
        names: [
            "addi", "subi", "muli", "mulhi", "mulhiu", "mulhisu", "remi", "divi",
            "andi", "nandi", "ori", "nori", "xori", "xnori",
            "slli", "srli", "roli", "rori", "srai",
            "seqi", "snei", "slti", "sgei", "sltiu", "sgeiu",
        ],
        operand_count: 3,
        expander: F,
    }
}

const F: ExpandFn = |_, name, ops| {
    let inst = match name {
        "mulhiu" => "mulhu",
        "mulhisu" => "mulhsu",
        "sltiu" => "sltu",
        "sgeiu" => "sgeu",
        _ => &name[..name.len() - 1],
    };

    if let Ok(n) = ops[2].cast_immediate()
        && let (_, hi) = split_hi_lo(n, 12, true)
        && hi != 0
    {
        Some(vec![
            ("li", ops!["r31", ops[2]]),
            (inst, ops![ops[0], ops[1], "r31"]),
        ])
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::super::test_mc_instr;

    #[test]
    fn als_imm32() {
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2"), @"Error: Macro-instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 123, "r4"), @"Error: Macro-instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(test_mc_instr!("addi" "r0", "r2", 123), @"");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", "r3"), @"");
        assert_snapshot!(test_mc_instr!("addi" 123, "r1", 456), @"");

        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 0x123), @"");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 0x1234), @"li r31 0x1234; add r1 r2 r31");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 0x12345678), @"li r31 0x12345678; add r1 r2 r31");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 0xFFFFFFFF_i64), @"");

        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 123), @"");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 3000), @"li r31 0xBB8; add r1 r2 r31");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", -123), @"");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", -3000), @"li r31 -3000; add r1 r2 r31");

        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 0x123), @"");
        assert_snapshot!(test_mc_instr!("addi" "r1", "r2", 0x1234), @"li r31 0x1234; add r1 r2 r31");
    }
}
