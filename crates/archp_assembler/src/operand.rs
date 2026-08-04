use std::fmt::Display;

use crate::parser::immediate::Immediate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandValue<'a> {
    StringSlice(&'a str),
    Integer(u32, u8),
}

impl<'a> From<&'a str> for OperandValue<'a> {
    fn from(s: &'a str) -> Self {
        OperandValue::StringSlice(s)
    }
}

impl From<u32> for OperandValue<'_> {
    fn from(n: u32) -> Self {
        OperandValue::Integer(n, 32)
    }
}

impl From<i32> for OperandValue<'_> {
    fn from(n: i32) -> Self {
        OperandValue::Integer(n as u32, 32)
    }
}

impl From<Immediate> for OperandValue<'_> {
    fn from(n: Immediate) -> Self {
        OperandValue::Integer(n.raw, n.bits)
    }
}

impl Display for OperandValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandValue::StringSlice(s) => write!(f, "{}", s),
            OperandValue::Integer(n, _) => write!(f, "{}", n),
        }
    }
}

pub macro op_values {
    ( $( $value:expr ),* $(,)? ) => {
        smallvec::smallvec![
            $(
                $crate::operand::OperandValue::from($value)
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
