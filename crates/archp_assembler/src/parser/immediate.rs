use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};

use crate::{
    assembler::Context,
    operand::Operand,
    parser::{expression::parse_expr, types::immediate::Immediate},
};

pub fn parse_imm(ctx: &Context, imm: &Operand) -> Result<Immediate> {
    match imm {
        Operand::Num(n) => Ok(Immediate(*n)),
        Operand::Ident(s) => {
            let s = ctx.constants.get(s).unwrap_or(s);

            let (remain, expr) =
                parse_expr(s).map_err(|e| anyhow!("Failed to parse immediate '{}': {}", s, e))?;

            if !remain.trim().is_empty() {
                bail!("Invalid immediate: {}", s);
            }

            // TODO: evaluate the expression with constants
            let imm = expr
                .eval(&HashMap::new())
                .map_err(|e| anyhow!("Failed to evaluate immediate '{}': {}", s, e))?;

            Ok(Immediate(imm))
        },

        // FIXME: workaround for something like '-1'
        Operand::Expr(e) => Ok(Immediate(
            e.eval_with(&|_| None).map_err(|e| anyhow!("{}", e))?,
        )),
        // TODO: impl
        _ => unimplemented!("parse_imm: {}", imm),
    }
}

pub fn parse_imm_as(ctx: &Context, imm: &Operand, bits: u8, signed: bool) -> Result<u32> {
    let (low, hi) = parse_imm(ctx, imm)?.split(bits, signed);

    if hi != 0 {
        bail!(
            "Immediate '{}' out of range for {}{} ({} ..= {})",
            imm,
            if signed { "i" } else { "u" },
            bits,
            if signed { i32::MIN >> (32 - bits) } else { 0 },
            if signed {
                (i32::MAX >> (32 - bits)) as u32
            } else {
                u32::MAX >> (32 - bits)
            }
        );
    }

    Ok(low)
}
