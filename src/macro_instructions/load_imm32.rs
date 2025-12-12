use crate::{
    instructions::parse_reg_d,
    macro_instructions::{ExpandFn, err_cond_not_supported, load_imm, macro_instruction},
    operand::op_values,
};

// li rd imm32  => lui rd imm32[31:12]; ori rd rd imm32[11:0]
macro_instruction! {
    name: "li",
    operand_count: 2,
    expander: F,
}

pub(super) const F: ExpandFn = |name, cond, ops| {
    parse_reg_d(&ops[0])?;

    let (up20, low12) = load_imm(&ops[1])?;

    if let Some(up20) = up20 {
        if cond.is_some() {
            err_cond_not_supported!(name);
        }

        Ok(Some(vec![
            ("lui", None, op_values![ops[0], up20]),
            ("ori", None, op_values![ops[0], ops[0], low12]),
        ]))
    } else {
        Ok(None)
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn li_imm32() {
        let li = mc_instr("li");

        assert_snapshot!(li("", &["r1"]), @"Error: Macro-instruction 'li' requires 2 operands, got 1");
        assert_snapshot!(li("", &["r1", "r2"]), @"Error: Invalid immediate: r2");
        assert_snapshot!(li("", &["123", "123"]), @"Error: Expected register, found immediate: 123");
        assert_snapshot!(li("", &["kb", "123"]), @"Error: Register 'kb' is raed-only");

        assert_snapshot!(li("", &["r1", "0x123"]), @"");
        assert_snapshot!(li("", &["r1", "0x1234"]), @"lui r1 1; ori r1 r1 0x234");
        assert_snapshot!(li("", &["r1", "0x12345678"]), @"lui r1 0x12345; ori r1 r1 0x678");

        assert_snapshot!(li("eq", &["r1", "0x123"]), @"");
        assert_snapshot!(li("eq", &["r1", "0x1234"]), @"Error: Conditional 'li' is not supported for 32-bit immediates");
    }
}
