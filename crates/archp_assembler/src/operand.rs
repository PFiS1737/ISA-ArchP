use std::fmt::Display;

use crate::parser::types::{expression::Expr, immediate::Immediate};

#[derive(Debug, Clone, PartialEq)]
pub enum Operand<'src> {
    Num(i64),
    Ident(&'src str),
    String(&'src str),
    Expr(Expr<'src>),
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

impl From<Immediate> for Operand<'_> {
    fn from(n: Immediate) -> Self {
        Operand::Num(n.0)
    }
}

impl Display for Operand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Num(n) => write!(f, "{}", n),
            Operand::Ident(s) => write!(f, "{}", s),
            Operand::String(s) => write!(f, "\"{}\"", s),
            Operand::Expr(e) => write!(f, "{}", e),
        }
    }
}

pub macro ops {
    ( $( $value:expr ),* $(,)? ) => {
        smallvec::smallvec![
            $(
                $crate::operand::Operand::from($value.clone()) // TODO: can we optimize this '.clone()' ?
            ),*
        ]
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandType {
    RegD,
    RegS,
    Imm(u8, bool),
    Addr(u8),
}

pub macro op_fmt {
    ( $( $type:tt $(( $v:literal $( , $s:tt )? ))? ),* ) => {
        &[
            $(
                $crate::operand::op_fmt!(@one $type $(( $v $( , $s )? ))?)
            ),*
        ]
    },

    (@one _) => { None },

    (@one $type:tt $(( $v:literal $( , $s:tt )? ))?) => {
        Some(
            $crate::operand::OperandType::$type $(( $v $( , $crate::operand::_sig!($s) )? ))?
        )
    },
}

pub macro op_types {
    ( $( $type:ident $(( $v:literal $( , $s:tt )? ))? ),* ) => {
        &[
            $(
                $crate::operand::OperandType::$type $(( $v $( , $crate::operand::_sig!($s) )? ))?
            ),*
        ]
    },
}

pub macro _sig {
    (i) => {
        true
    },
    (u) => {
        false
    },
}
