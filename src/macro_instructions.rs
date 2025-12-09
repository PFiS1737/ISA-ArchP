mod load_imm32;

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
pub struct MacroInstruction {
    name: &'static str,
    operand_types: &'static [OperandType],
    expander: ExpandFn,
}

inventory::collect!(MacroInstruction);

pub static MACRO_INSTRUCTIONS: Lazy<HashMap<&'static str, MacroInstruction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for entry in inventory::iter::<MacroInstruction> {
        map.insert(entry.name, *entry);
    }
    map
});

impl MacroInstruction {
    pub fn expand<'a>(&self, operands: &[&'a str]) -> Result<Option<ExpandRet<'a>>> {
        if self.assert_operand_format(operands)? {
            Ok(Some((self.expander)(self.name, operands)))
        } else {
            Ok(None)
        }
    }

    fn assert_operand_format(&self, operands: &[&str]) -> Result<bool> {
        if operands.len() != self.operand_types.len() {
            bail!(
                "Macro-instruction '{}' requires {} operands, got {}",
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

macro macro_instruction {
    (
        name: [ $($name:literal),+ ],
        operand_types: $types:tt,
        expander: $expander:expr,
    ) => {
        $(
            $crate::macro_instructions::macro_instruction! {
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
            $crate::macro_instructions::MacroInstruction {
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
