mod branch;
mod branch_zero;
mod clear;
mod inc_dec;
mod jump;
mod load_imm;
mod mv;
mod negate;
mod not;

use std::{collections::HashMap, iter::successors};

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{
    assembler::Context,
    operand::{OperandType, OperandValue},
    parser::{parse_address, parse_imm, parse_reg_d, parse_reg_s},
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
    pub fn expand<'a>(
        &self,
        ctx: &Context,
        pc: u32,
        operands: &[OperandValue<'a>],
    ) -> Result<ExpandRet<'a>> {
        self.assert_operand_format(ctx, pc, operands)?;

        Ok(
            successors(Some((self.expander)(self.name, operands)), |(name, ops)| {
                let ps_instr = PSEUDO_INSTRUCTIONS.get(*name)?;

                ps_instr.assert_operand_format(ctx, pc, ops).ok()?;

                Some((ps_instr.expander)(name, ops))
            })
            .last()
            .unwrap(), // INFO: Safe because at least the first expansion exists
        )
    }

    fn assert_operand_format(
        &self,
        ctx: &Context,
        pc: u32,
        operands: &[OperandValue],
    ) -> Result<()> {
        if operands.len() != self.operand_types.len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name,
                self.operand_types.len(),
                operands.len()
            );
        }

        for (i, operand) in operands.iter().enumerate() {
            match self.operand_types[i] {
                OperandType::RegD => {
                    parse_reg_d(ctx, operand)?;
                }
                OperandType::RegS => {
                    parse_reg_s(ctx, operand)?;
                }
                OperandType::Imm(bits, signed) => {
                    parse_imm(ctx, operand)?.as_field(bits, signed)?;
                }
                OperandType::Addr(bits) => {
                    parse_address(ctx, operand)?.as_field(bits, pc)?;
                }
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
