use crate::{
    instructions::parse_reg_s,
    macro_instructions::{ExpandFn, err_cond_not_supported, load_imm, macro_instruction},
    operand::op_values,
};

macro_instruction! {
    name: [ "beqi", "bnei", "blti", "blei", "bgti", "bgei" ],
    operand_count: 3,
    expander: F,
}

const F: ExpandFn = |name, cond, ops| {
    let inst = match name {
        "beqi" => "beq",
        "bnei" => "bne",
        "blti" => "blt",
        "blei" => "ble",
        "bgti" => "bgt",
        "bgei" => "bge",
        _ => unreachable!(),
    };

    parse_reg_s(&ops[0])?;

    let (up20, low12) = load_imm(&ops[1])?;

    // INFO: We don't check the branch target (ops[2]) here, as it can be a label.

    if let Some(up20) = up20 {
        if cond.is_some() {
            err_cond_not_supported!(name);
        }

        Ok(Some(vec![
            ("lui", None, op_values!["tmp", up20]),
            ("ori", None, op_values!["tmp", "tmp", low12]),
            (inst, None, op_values![ops[0], "tmp", ops[2]]),
        ]))
    } else if low12 == 0 {
        Ok(Some(vec![(inst, cond, op_values![ops[0], "zero", ops[2]])]))
    } else {
        Ok(Some(vec![
            ("li", cond, op_values!["tmp", ops[1]]),
            (inst, cond, op_values![ops[0], "tmp", ops[2]]),
        ]))
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn li_imm32() {
        let beqi = mc_instr("beqi");

        assert_snapshot!(beqi("", &["r1", "0x123"]), @"Error: Macro-instruction 'beqi' requires 3 operands, got 2");
        assert_snapshot!(beqi("",&["r1", "r2", "0"]), @"Error: Invalid immediate: r2");
        assert_snapshot!(beqi("", &["123", "123", "0"]), @"Error: Expected register, found immediate: 123");

        assert_snapshot!(beqi("", &["r1", "0x123", "0"]), @"li tmp 0x123; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0x1234", "0"]), @"lui tmp 1; ori tmp tmp 0x234; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0x12345678", "0"]), @"lui tmp 0x12345; ori tmp tmp 0x678; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0", "0"]), @"beq r1 zero 0");

        assert_snapshot!(beqi("eq", &["r1", "0x123", "0"]), @"li.eq tmp 0x123; beq.eq r1 tmp 0");
        assert_snapshot!(beqi("eq", &["r1", "0x1234", "0"]), @"Error: Conditional 'beqi' is not supported for 32-bit immediates");
    }
}
