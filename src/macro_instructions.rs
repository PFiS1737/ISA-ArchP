mod branch_imm;
mod load_imm32;

use std::collections::HashMap;

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{instructions::parse_imm, operand::OperandValue};

type ExpandRet<'a> = Result<Option<Vec<(&'static str, Option<&'a str>, Vec<OperandValue<'a>>)>>>;
type ExpandFn = for<'a> fn(&'static str, Option<&'a str>, &[OperandValue<'a>]) -> ExpandRet<'a>;

#[derive(Debug, Clone, Copy)]
pub struct MacroInstruction {
    name: &'static str,
    operand_count: usize,
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
    pub fn expand<'a>(
        &self,
        cond: Option<&'a str>,
        operands: &[OperandValue<'a>],
    ) -> ExpandRet<'a> {
        self.assert_operand_count(operands)?;

        (self.expander)(self.name, cond, operands)
    }

    fn assert_operand_count(&self, operands: &[OperandValue]) -> Result<()> {
        if operands.len() != self.operand_count {
            bail!(
                "Macro-instruction '{}' requires {} operands, got {}",
                self.name,
                self.operand_count,
                operands.len()
            );
        }

        Ok(())
    }
}

macro macro_instruction {
    (
        name: [ $($name:literal),+ ],
        operand_count: $count:literal,
        expander: $expander:expr,
    ) => {
        $(
            $crate::macro_instructions::macro_instruction! {
                name: $name,
                operand_count: $count,
                expander: $expander,
            }
        )+
    },

    (
        name: $name:literal,
        operand_count: $count:literal,
        expander: $expander:expr,
    ) => {
        inventory::submit! {
            $crate::macro_instructions::MacroInstruction {
                name: $name,
                operand_count: $count,
                expander: $expander,
            }
        }
    },
}

fn load_imm(s: &OperandValue) -> Result<(Option<u32>, u32)> {
    let num = parse_imm(s)?;
    if num > 0xFFF {
        Ok((Some(num >> 12), num & 0xFFF))
    } else {
        Ok((None, num))
    }
}
