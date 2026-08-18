use std::fmt::Display;

use anyhow::Result;

use crate::{context::Context, expression::Expr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand<'src> {
    Num(i64),
    Ident(&'src str),
    Addition(&'src str, i64),
}

impl<'a> From<&'a str> for Operand<'a> {
    fn from(s: &'a str) -> Self {
        Operand::Ident(s)
    }
}

impl From<u32> for Operand<'_> {
    fn from(n: u32) -> Self {
        Operand::Num(n as i64)
    }
}

impl From<i32> for Operand<'_> {
    fn from(n: i32) -> Self {
        Operand::Num(n as i64)
    }
}

impl From<i64> for Operand<'_> {
    fn from(n: i64) -> Self {
        Operand::Num(n)
    }
}

impl Display for Operand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Num(n) => write!(f, "{}", n), // TODO: format as hex
            Operand::Ident(s) => write!(f, "{}", s),
            Operand::Addition(s, n) => write!(f, "{}{:+}", s, n),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirectiveOperand<'src> {
    Empty,
    Expr(Expr<'src>),
    Operand(Operand<'src>),
    String(&'src str),
}

impl Display for DirectiveOperand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectiveOperand::Empty => write!(f, ""),
            DirectiveOperand::Expr(expr) => write!(f, "{}", expr),
            DirectiveOperand::Operand(op) => write!(f, "{}", op),
            DirectiveOperand::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl<'src> DirectiveOperand<'src> {
    pub fn as_evaluated(&self, ctx: &Context<'src>) -> Result<Self> {
        Ok(match self {
            DirectiveOperand::Expr(expr) => Self::Operand(expr.eval_to_operand_with(&ctx.equates)?),
            _ => self.clone(),
        })
    }
}

pub macro ops {
    ( $( $value:expr ),* $(,)? ) => {{
        let v: smallvec::SmallVec<[$crate::operand::Operand<'_>; 3]> = smallvec::smallvec![
            $(
                $crate::operand::Operand::from($value)
            ),*
        ];
        v
    }},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandType {
    RegD,
    RegS,
    Imm(u8, bool),
    Addr(u8),
    None,
}

pub macro op_types {
    ( $( $type:tt $(( $v:literal $( , $s:tt )? ))? ),* ) => {
        &[
            $(
                $crate::operand::op_types!(@one $type $(( $v $( , $s )? ))?)
            ),*
        ]
    },

    (@one _) => {
        $crate::operand::OperandType::None
    },

    (@one $type:tt $(( $v:literal $( , $s:tt )? ))?) => {
        $crate::operand::OperandType::$type $(( $v $( , crate::operand::op_types!(@sig $s) )? ))?
    },

    (@sig i) => { true },
    (@sig u) => { false },
}
