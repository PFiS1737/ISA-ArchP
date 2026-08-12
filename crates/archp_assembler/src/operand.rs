use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operand<'src> {
    Num(i64),
    Ident(&'src str),
    String(&'src str),
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
            Operand::Num(n) => write!(f, "{}", n),
            Operand::Ident(s) => write!(f, "{}", s),
            Operand::String(s) => write!(f, "\"{}\"", s),
            Operand::Addition(s, n) => write!(f, "{}{:+}", s, n),
        }
    }
}

pub macro ops {
    ( $( $value:expr ),* $(,)? ) => {
        smallvec::smallvec![
            $(
                $crate::operand::Operand::from($value)
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
            $crate::operand::OperandType::$type $(( $v $( , crate::operand::op_fmt!(@sig $s) )? ))?
        )
    },

    (@sig i) => { true },
    (@sig u) => { false },
}

pub macro op_types {
    ( $( $type:ident $(( $v:literal $( , $s:tt )? ))? ),* ) => {
        &[
            $(
                $crate::operand::OperandType::$type $(( $v $( , crate::operand::op_types!(@sig $s) )? ))?
            ),*
        ]
    },

    (@sig i) => { true },
    (@sig u) => { false },
}
