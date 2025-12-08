use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum OperandType {
    RegD,
    RegS,
    Imm(ImmRange),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImmRange(pub u8, pub u8);

impl ImmRange {
    pub fn contains(&self, value: &u32) -> bool {
        if self.1 <= self.0 {
            panic!(
                "Internal Error: Invalid ImmRange: {}_bits ~ {}_bits",
                self.0, self.1
            );
        }

        *value >= self.start() && *value <= self.end()
    }
    fn start(&self) -> u32 {
        Self::ones(self.0)
    }
    fn end(&self) -> u32 {
        Self::ones(self.1)
    }
    fn ones(bits: u8) -> u32 {
        if bits > 32 {
            panic!(
                "Internal Error: ImmRange bits cannot be greater than 32, got {}",
                bits
            );
        }

        if bits == 32 {
            u32::MAX
        } else {
            (1u32 << bits) - 1
        }
    }
}

impl Display for ImmRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ~ {}", fmt_hex(self.start()), fmt_hex(self.end()))
    }
}

pub macro op_types {
    ( $( $type:ident $(($v:literal))? ),* ) => {
        &[
            $(
                $crate::operand_types::OperandType::$type $(
                    ( $crate::operand_types::ImmRange(0, $v) )
                )?
            ),*
        ]
    },

    ( $( $type:ident $(($start:tt, $end:tt))? ),* ) => {
        &[
            $(
                $crate::operand_types::OperandType::$type $(
                    ( $crate::operand_types::ImmRange($start, $end) )
                )?
            ),*
        ]
    },
}

fn fmt_hex(n: u32) -> String {
    if n < 256 {
        n.to_string()
    } else {
        format!("0x{:X}", n)
    }
}
