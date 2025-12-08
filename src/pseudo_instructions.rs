mod branch_imm;
mod clear;
mod inc_dec;
mod load_imm32;
mod mv;

use std::collections::HashMap;

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{
    instructions::{parse_imm, parse_reg_d, parse_reg_s},
    operand_types::OperandType,
};

type ExpandRet<'a> = Vec<(&'static str, Vec<String>)>;
type ExpandFn = for<'a> fn(&'static str, &[&'a str]) -> ExpandRet<'a>;

#[derive(Debug, Clone, Copy)]
pub struct PseudoInstruction {
    name: &'static str,
    operand_types: &'static [OperandType],
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
    pub fn expand<'a>(&self, operands: &[&'a str]) -> Result<ExpandRet<'a>> {
        if self.assert_operand_format(operands)? {
            Ok((self.expander)(self.name, operands))
        } else {
            Ok(vec![(
                self.name,
                operands.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
            )])
        }
    }

    fn assert_operand_format(&self, operands: &[&str]) -> Result<bool> {
        if operands.len() != self.operand_types.len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name,
                self.operand_types.len(),
                operands.len()
            );
        }

        for (i, operand) in operands.iter().enumerate() {
            match &self.operand_types[i] {
                OperandType::RegD => {
                    parse_reg_d(operand)?;
                }
                OperandType::RegS => {
                    parse_reg_s(operand)?;
                }
                OperandType::Imm(range) => {
                    let num = parse_imm(operand)?;
                    if !range.contains(&num) {
                        return Ok(false);
                    }
                }
            };
        }

        Ok(true)
    }
}

macro pseudo_instruction {
    (
        name: [ $($name:literal),+ ],
        operand_types: $types:tt,
        expander: $expander:expr,
    ) => {
        $(
            $crate::pseudo_instructions::pseudo_instruction! {
                name: $name,
                operand_types: $types,
                expander: $expander,
            }
        )+
    },

    (
        name: $name:literal,
        operand_types: $types:tt,
        expander: $expander:expr,
    ) => {
        inventory::submit! {
            $crate::pseudo_instructions::PseudoInstruction {
                name: $name,
                operand_types: $crate::operand_types::op_types! $types,
                expander: $expander,
            }
        }
    },
}

pub fn load_upper_imm(s: &str) -> (String, String) {
    let num = parse_imm(s).unwrap(); // INFO: Safe to unwrap
    (format!("0x{:X}", num >> 12), format!("0x{:X}", num & 0xFFF))
}
