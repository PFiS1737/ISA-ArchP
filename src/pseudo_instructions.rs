mod clear;
mod inc_dec;
mod mv;

use std::collections::HashMap;

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::instructions::{OperandType, parse_imm, parse_reg_d, parse_reg_s};

type ExpandRet<'a> = Result<Vec<(&'static str, Vec<String>)>>;
type ExpandFn = for<'a> fn(&'static str, &[&'a str]) -> ExpandRet<'a>;

#[derive(Debug, Clone, Copy)]
pub struct PseudoInstruction {
    name: &'static str,
    format: &'static [OperandType],
    expander: ExpandFn,
}

inventory::collect!(PseudoInstruction);

pub static PSEUDO_INSTRUCTIONS: Lazy<HashMap<&'static str, PseudoInstruction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for entry in inventory::iter::<PseudoInstruction> {
        map.insert(entry.name, *entry);
    }
    map
});

impl PseudoInstruction {
    pub fn expand<'a>(&self, operands: &[&'a str]) -> ExpandRet<'a> {
        self.assert_operand_format(operands)?;

        (self.expander)(self.name, operands)
    }

    fn assert_operand_format(&self, operands: &[&str]) -> Result<()> {
        if operands.len() != self.format.len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name,
                self.format.len(),
                operands.len()
            );
        }

        for (i, operand) in operands.iter().enumerate() {
            match self.format[i] {
                OperandType::RegD => parse_reg_d(operand)?,
                OperandType::RegS => parse_reg_s(operand)?,
                OperandType::Imm(_) => parse_imm(operand)?,
            };
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! pseudo_instruction {
    (
        name: [ $($name:literal),+ ],
        format: $format:tt,
        expander: $expander:expr,
    ) => {
        $(
            $crate::pseudo_instruction! {
                name: $name,
                format: $format,
                expander: $expander,
            }
        )+
    };

    (
        name: $name:literal,
        format: [ $( $type:ident $(($v:expr))? ),* ],
        expander: $expander:expr,
    ) => {
        inventory::submit! {
            $crate::pseudo_instructions::PseudoInstruction {
                name: $name,
                format: &[ $( $crate::instructions::OperandType::$type $(($v))? ),* ],
                expander: $expander,
            }
        }
    };

    // (
    //     name: [ $($name:expr),+ ],
    //     expander: $expander:expr,
    // ) => {
    //     $(
    //         inventory::submit! {
    //             $crate::pseudo_instructions::PseudoInstruction {
    //                 name: $name,
    //                 operand_count: 0,
    //                 check_operand_count: false,
    //                 expander: $expander,
    //             }
    //         }
    //     )+
    // };
}

// pub fn load_imm(num: u32) -> (Option<String>, String) {
//     if num > 0xFFF {
//         (Some((num >> 12).to_string()), (num & 0xFFF).to_string())
//     } else {
//         (None, num.to_string())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_load_imm() {
//         let (up20, low12) = load_imm(0x123456);
//         assert_eq!(up20, Some(0x123.to_string()));
//         assert_eq!(low12, 0x456.to_string());
//
//         let (up20, low12) = load_imm(0xABC);
//         assert_eq!(up20, None);
//         assert_eq!(low12, 0xABC.to_string());
//     }
// }
