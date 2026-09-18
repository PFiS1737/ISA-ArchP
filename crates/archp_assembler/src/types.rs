#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    R,
    I,
    B,
    S,
    U,
    J,
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
                $crate::types::op_types!(@one $type $(( $v $( , $s )? ))?)
            ),*
        ]
    },

    (@one _) => {
        $crate::types::OperandType::None
    },

    (@one $type:tt $(( $v:literal $( , $s:tt )? ))?) => {
        $crate::types::OperandType::$type $(( $v $( , crate::types::op_types!(@sig $s) )? ))?
    },

    (@sig i) => { true },
    (@sig u) => { false },
}
