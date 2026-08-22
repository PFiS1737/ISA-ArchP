use std::fmt::Display;

use anyhow::{Result, bail};

use crate::{context::Context, expression::Expr, operand::Operand};

#[derive(Debug, Clone)]
pub enum DirectiveOperand<'src> {
    Empty,
    Expr(Expr<'src>),
    String(&'src str),
}

impl Display for DirectiveOperand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectiveOperand::Empty => write!(f, ""),
            DirectiveOperand::Expr(expr) => write!(f, "{}", expr),
            DirectiveOperand::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EvaluatedDirectiveOperand<'src> {
    Empty,
    Operand(Operand<'src>),
    String(&'src str),
}

impl Display for EvaluatedDirectiveOperand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluatedDirectiveOperand::Empty => write!(f, ""),
            EvaluatedDirectiveOperand::Operand(op) => write!(f, "{}", op),
            EvaluatedDirectiveOperand::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl<'src> DirectiveOperand<'src> {
    pub fn as_evaluated(&self, ctx: &Context<'src>) -> Result<EvaluatedDirectiveOperand<'src>> {
        Ok(match self {
            DirectiveOperand::Expr(expr) => {
                EvaluatedDirectiveOperand::Operand(expr.eval_to_operand_with(&ctx.equates)?)
            },
            DirectiveOperand::String(s) => EvaluatedDirectiveOperand::String(s),
            DirectiveOperand::Empty => EvaluatedDirectiveOperand::Empty,
        })
    }

    pub fn cast_absolute(&self, ctx: &Context<'src>) -> Result<i64> {
        match self {
            DirectiveOperand::Expr(expr) => expr.cast_absolute(ctx),
            _ => bail!("Expected absolute expression, got: {}", self),
        }
    }
}
