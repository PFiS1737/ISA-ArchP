use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperandValue<'a> {
    StringSlice(&'a str),
    Unsigned(u32),
    Signed(i32),
}

impl<'a> From<&'a str> for OperandValue<'a> {
    fn from(s: &'a str) -> Self {
        OperandValue::StringSlice(s)
    }
}

impl From<u32> for OperandValue<'_> {
    fn from(n: u32) -> Self {
        OperandValue::Unsigned(n)
    }
}

impl From<i32> for OperandValue<'_> {
    fn from(n: i32) -> Self {
        OperandValue::Signed(n)
    }
}

impl Display for OperandValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandValue::StringSlice(s) => write!(f, "{}", s),
            OperandValue::Unsigned(n) => write!(f, "{}", n),
            OperandValue::Signed(n) => write!(f, "{}", n),
        }
    }
}

pub macro op_values {
    ( $( $value:expr ),* $(,)? ) => {
        vec![
            $(
                $crate::operand::OperandValue::from($value)
            ),*
        ]
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperandType {
    RegD,
    RegS,
    Imm(u8, bool),
    Addr,
}

pub macro op_types {
    ( $( $type:ident $(($v:literal, $s:tt))? ),* ) => {
        &[
            $(
                $crate::operand::OperandType::$type $(($v, $crate::operand::_sig!($s)))?
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
