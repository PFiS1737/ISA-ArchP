use crate::macro_instructions::{load_upper_imm, macro_instruction};

// li rd imm32  => lui rd imm32[31:12]; ori rd rd imm32[11:0]
macro_instruction! {
    name: "li",
    operand_types : [ RegD, Imm(13, 32) ],
    expander: |_, operands| {
        let (up20, low12) = load_upper_imm(operands[1]);

        // FIXME: in this case, the 'li' instruction can't work with conditions
        vec![
            (
                "lui",
                vec![
                    operands[0].to_string(),
                    up20,
                ],
            ),
            (
                "ori",
                vec![
                    operands[0].to_string(),
                    operands[0].to_string(),
                    low12,
                ],
            ),
        ]
    },
}

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn li_imm32() {
        let li = mc_instr("li");

        assert_snapshot!(li(&["r1"]), @"Error: Macro-instruction 'li' requires 2 operands, got 1");
        assert_snapshot!(li(&["r1", "r2"]), @"Error: Invalid immediate: r2");
        assert_snapshot!(li(&["123", "123"]), @"Error: Expected register, found immediate: 123");

        assert_snapshot!(li(&["r1", "0x123"]), @"");
        assert_snapshot!(li(&["r1", "0x1234"]), @"lui r1 0x1; ori r1 r1 0x234");
        assert_snapshot!(li(&["r1", "0x12345678"]), @"lui r1 0x12345; ori r1 r1 0x678");
    }
}
