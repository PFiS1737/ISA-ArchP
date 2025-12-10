use anyhow::bail;

use crate::{
    instructions::parse_reg_s,
    macro_instructions::{ExpandFn, load_imm, macro_instruction},
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

    parse_reg_s(ops[0])?;

    let (up20, low12) = load_imm(ops[1])?;

    // INFO: We don't check the branch target (ops[2]) here, as it can be a label.

    if let Some(up20) = up20 {
        if cond.is_some() {
            bail!("Conditional '{name}' is not supported for 32-bit immediates");
        }

        Ok(Some(vec![
            ("lui", None, vec!["tmp".to_string(), up20]),
            (
                "ori",
                None,
                vec!["tmp".to_string(), "tmp".to_string(), low12],
            ),
            (
                inst,
                None,
                vec![ops[0].to_string(), "tmp".to_string(), ops[2].to_string()],
            ),
        ]))
    } else if low12 == "0x0" {
        Ok(Some(vec![(
            inst,
            cond,
            vec![ops[0].to_string(), "r0".to_string(), ops[2].to_string()],
        )]))
    } else {
        Ok(Some(vec![
            ("li", cond, vec!["tmp".to_string(), ops[1].to_string()]),
            (
                inst,
                cond,
                vec![ops[0].to_string(), "tmp".to_string(), ops[2].to_string()],
            ),
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
        assert_snapshot!(beqi("", &["r1", "0x1234", "0"]), @"lui tmp 0x1; ori tmp tmp 0x234; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0x12345678", "0"]), @"lui tmp 0x12345; ori tmp tmp 0x678; beq r1 tmp 0");
        assert_snapshot!(beqi("", &["r1", "0", "0"]), @"beq r1 r0 0");

        assert_snapshot!(beqi("eq", &["r1", "0x1234", "0"]), @"Error: Conditional 'beqi' is not supported for 32-bit immediates");
    }
}
