use std::fmt::Display;

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand<'src> {
    Num(i64),
    Ident(&'src str),
    Addition(&'src str, i64),
}

impl<'src> Operand<'src> {
    pub fn cast_register(&self) -> Result<&'src str> {
        match self {
            Operand::Ident(s) => Ok(s),
            _ => bail!("Expected register, got: {}", self),
        }
    }

    pub fn cast_immediate(&self) -> Result<i64> {
        match self {
            Operand::Num(n) => Ok(*n),
            _ => bail!("Expected immediate, got: {}", self),
        }
    }

    pub fn cast_address(&self) -> Result<(&'src str, i64)> {
        match self {
            Operand::Ident(s) => Ok((s, 0)),
            Operand::Addition(s, n) => Ok((s, *n)),
            _ => bail!("Expected address label, got: {}", self),
        }
    }
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
