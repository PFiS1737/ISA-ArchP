use crate::{
    encoder::immediate::{encode_immediate, split_hi_lo},
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

const F: ExpandFn = |ctx, _, name, ops| {
    let inst = match name {
        "mulhiu" => "mulhu",
        "mulhisu" => "mulhsu",
        "sltiu" => "sltu",
        "sgeiu" => "sgeu",
        _ => &name[..name.len() - 1],
    };

    if let Ok(n) = encode_immediate(ctx, &ops[2])
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

    use crate::testkit::*;

    #[test]
    fn als_imm32() {
        let addi = mc_instr("addi");

        assert_snapshot!(addi(&["r1", "r2"]), @"Error: Macro-instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(addi(&["r1", "r2", "123", "r4"]), @"Error: Macro-instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(addi(&["r0", "r2", "123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "r3"]), @"");
        assert_snapshot!(addi(&["123", "r1", "456"]), @"");

        assert_snapshot!(addi(&["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "0x1234"]), @"li r31 0x1234; add r1 r2 r31");
        assert_snapshot!(addi(&["r1", "r2", "0x12345678"]), @"li r31 0x12345678; add r1 r2 r31");
        assert_snapshot!(addi(&["r1", "r2", "0xFFFFFFFF"]), @"");

        assert_snapshot!(addi(&["r1", "r2", "123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "3000"]), @"li r31 0xBB8; add r1 r2 r31");
        assert_snapshot!(addi(&["r1", "r2", "-123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "-3000"]), @"li r31 -3000; add r1 r2 r31");

        assert_snapshot!(addi(&["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "0x1234"]), @"li r31 0x1234; add r1 r2 r31");
    }
}
