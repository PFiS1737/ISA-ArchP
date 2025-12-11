mod clear;
mod inc_dec;
mod mv;

use std::collections::HashMap;

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{
    instructions::{parse_reg_d, parse_reg_s},
    operand::{OperandType, OperandValue},
};

type ExpandRet<'a> = (&'static str, Vec<OperandValue<'a>>);
type ExpandFn = for<'a> fn(&'static str, &[OperandValue<'a>]) -> ExpandRet<'a>;

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
    pub fn expand<'a>(&self, operands: &[OperandValue<'a>]) -> Result<ExpandRet<'a>> {
        self.assert_operand_format(operands)?;

        Ok((self.expander)(self.name, operands))
    }

    fn assert_operand_format(&self, operands: &[OperandValue]) -> Result<()> {
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
                OperandType::RegD => parse_reg_d(operand)?,
                OperandType::RegS => parse_reg_s(operand)?,
                OperandType::Imm(_) => unimplemented!(),
            };
        }

        Ok(())
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
                operand_types: $crate::operand::op_types! $types,
                expander: $expander,
            }
        }
    },
}
