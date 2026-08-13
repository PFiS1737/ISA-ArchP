use std::{collections::HashMap, fmt::Display};

use thiserror::Error;

use crate::operand::Operand;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'src> {
    Num(i64),
    Ident(&'src str),
    Unary {
        op: UnaryOp,
        rhs: Box<Expr<'src>>,
    },
    Binary {
        lhs: Box<Expr<'src>>,
        op: BinaryOp,
        rhs: Box<Expr<'src>>,
    },
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Ident(s) => write!(f, "{}", s),
            Expr::Unary { op, rhs } => write!(f, "{}{}", op, rhs),
            Expr::Binary { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EvalError {
    #[error("shift amount out of range: {}", .0)]
    ShiftOutOfRange(i64),

    #[error("division by zero")]
    DivByZero,

    #[error("expression is not absolute")]
    NotAbsolute,
}

impl<'ctx, 'src: 'ctx> Expr<'src> {
    fn eval_to_operand<F>(&self, resolve: &F) -> Result<Operand<'src>, EvalError>
    where
        F: Fn(&'src str) -> Option<i64>,
    {
        match self {
            Expr::Num(n) => Ok(Operand::Num(*n)),

            Expr::Ident(name) => {
                if let Some(num) = resolve(name) {
                    Ok(Operand::Num(num))
                } else {
                    Ok(Operand::Ident(name))
                }
            },

            Expr::Unary { op, rhs } => {
                let r = rhs.eval_to_operand(resolve)?;
                eval_unary_to_operand(*op, r)
            },

            Expr::Binary { lhs, op, rhs } => {
                let l = lhs.eval_to_operand(resolve)?;
                let r = rhs.eval_to_operand(resolve)?;
                eval_binary_to_operand(l, *op, r)
            },
        }
    }

    pub fn eval_to_operand_with(
        &self,
        env: &'ctx HashMap<&'src str, i64>,
    ) -> Result<Operand<'src>, EvalError> {
        let op = self.eval_to_operand(&|s| env.get(s).copied())?;
        Ok(match op {
            Operand::Addition(ident, 0) => Operand::Ident(ident),
            _ => op,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg, // -
    Not, // ~
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "~",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Mul, // *
    Div, // /
    Mod, // %
    Shl, // <<
    Shr, // >>

    And,   // &
    Xor,   // ^
    Or,    // |
    OrNot, // !

    Add, // +
    Sub, // -

    Eq, // ==
    Ne, // !=, <>
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=

    LogicalAnd, // &&

    LogicalOr, // ||
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",

            BinaryOp::And => "&",
            BinaryOp::Xor => "^",
            BinaryOp::Or => "|",
            BinaryOp::OrNot => "!",

            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",

            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",

            BinaryOp::LogicalAnd => "&&",

            BinaryOp::LogicalOr => "||",
        })
    }
}

fn eval_unary(op: UnaryOp, rhs: i64) -> Result<i64, EvalError> {
    match op {
        UnaryOp::Neg => Ok(rhs.wrapping_neg()),
        UnaryOp::Not => Ok(!rhs),
    }
}

fn eval_unary_to_operand<'a>(op: UnaryOp, rhs: Operand<'a>) -> Result<Operand<'a>, EvalError> {
    if let Operand::Num(r) = rhs {
        return Ok(Operand::Num(eval_unary(op, r)?));
    }

    Err(EvalError::NotAbsolute)
}

fn eval_binary(lhs: i64, op: BinaryOp, rhs: i64) -> Result<i64, EvalError> {
    match op {
        BinaryOp::Mul => Ok(lhs.wrapping_mul(rhs)),
        BinaryOp::Div => {
            if rhs == 0 {
                return Err(EvalError::DivByZero);
            }
            Ok(lhs.wrapping_div(rhs))
        },
        BinaryOp::Mod => {
            if rhs == 0 {
                return Err(EvalError::DivByZero);
            }
            Ok(lhs.wrapping_rem(rhs))
        },
        BinaryOp::Shl => {
            if !(0..32).contains(&rhs) {
                return Err(EvalError::ShiftOutOfRange(rhs));
            }
            Ok(lhs.wrapping_shl(rhs as u32))
        },
        BinaryOp::Shr => {
            if !(0..32).contains(&rhs) {
                return Err(EvalError::ShiftOutOfRange(rhs));
            }
            Ok(lhs.wrapping_shr(rhs as u32))
        },

        BinaryOp::And => Ok(lhs & rhs),
        BinaryOp::Xor => Ok(lhs ^ rhs),
        BinaryOp::Or => Ok(lhs | rhs),
        BinaryOp::OrNot => Ok(lhs | !rhs),

        BinaryOp::Add => Ok(lhs.wrapping_add(rhs)),
        BinaryOp::Sub => Ok(lhs.wrapping_sub(rhs)),

        BinaryOp::Eq => Ok(if lhs == rhs { -1 } else { 0 }),
        BinaryOp::Ne => Ok(if lhs != rhs { -1 } else { 0 }),
        BinaryOp::Lt => Ok(if lhs < rhs { -1 } else { 0 }),
        BinaryOp::Le => Ok(if lhs <= rhs { -1 } else { 0 }),
        BinaryOp::Gt => Ok(if lhs > rhs { -1 } else { 0 }),
        BinaryOp::Ge => Ok(if lhs >= rhs { -1 } else { 0 }),

        BinaryOp::LogicalAnd => Ok(if lhs != 0 && rhs != 0 { 1 } else { 0 }),

        BinaryOp::LogicalOr => Ok(if lhs == 0 && rhs == 0 { 0 } else { 1 }),
    }
}

fn eval_binary_to_operand<'a>(
    lhs: Operand<'a>,
    op: BinaryOp,
    rhs: Operand<'a>,
) -> Result<Operand<'a>, EvalError> {
    if let Operand::Num(l) = lhs
        && let Operand::Num(r) = rhs
    {
        return Ok(Operand::Num(eval_binary(l, op, r)?));
    }

    if let Operand::Ident(l) = lhs
        && let Operand::Num(r) = rhs
    {
        return match op {
            BinaryOp::Add => Ok(Operand::Addition(l, r)),
            BinaryOp::Sub => Ok(Operand::Addition(l, -r)),
            _ => Err(EvalError::NotAbsolute),
        };
    }

    if let Operand::Addition(l, add) = lhs
        && let Operand::Num(r) = rhs
    {
        return match op {
            BinaryOp::Add => Ok(Operand::Addition(l, add + r)),
            BinaryOp::Sub => Ok(Operand::Addition(l, add - r)),
            _ => Err(EvalError::NotAbsolute),
        };
    }

    if let Operand::Num(l) = lhs
        && let Operand::Ident(r) = rhs
        && matches!(op, BinaryOp::Add)
    {
        return Ok(Operand::Addition(r, l));
    }

    if let Operand::Num(l) = lhs
        && let Operand::Addition(r, add) = rhs
        && matches!(op, BinaryOp::Add)
    {
        return Ok(Operand::Addition(r, add + l));
    }

    Err(EvalError::NotAbsolute)
}
