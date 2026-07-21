use crate::{
    macro_instructions::{ExpandFn, macro_instruction},
    operand::op_values,
    parser::immediate::parse_imm,
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

    if let Ok((_, hi)) = parse_imm(ctx, &ops[2]).map(|imm| imm.split(12, true))
        && hi != 0
    {
        Some(vec![
            ("li", op_values!["r31", ops[2]]),
            (inst, op_values![ops[0], ops[1], "r31"]),
        ])
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
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
        assert_snapshot!(addi(&["r1", "r2", "0x1234"]), @"lui r31 1; addi r31 r31 0x234; add r1 r2 r31");
        assert_snapshot!(addi(&["r1", "r2", "0x12345678"]), @"lui r31 0x12345; addi r31 r31 0x678; add r1 r2 r31");
        assert_snapshot!(addi(&["r1", "r2", "0xFFFFFFFF"]), @"");

        assert_snapshot!(addi(&["r1", "r2", "123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "3000"]), @"lui r31 1; addi r31 r31 0xFFFFFBB8; add r1 r2 r31");
        assert_snapshot!(addi(&["r1", "r2", "-123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "-3000"]), @"lui r31 0xFFFFF; addi r31 r31 0x448; add r1 r2 r31");

        assert_snapshot!(addi(&["r1", "r2", "0x123"]), @"");
        assert_snapshot!(addi(&["r1", "r2", "0x1234"]), @"lui r31 1; addi r31 r31 0x234; add r1 r2 r31");
    }
}
