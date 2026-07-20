mod branch;
mod branch_zero;
mod clear;
mod inc_dec;
mod jump;
mod load_imm;
mod mv;
mod negate;
mod not;
mod set;
mod set_zero;

use std::{collections::HashMap, iter::successors, sync::LazyLock};

use anyhow::{Result, bail};

use crate::{
    assembler::{Context, Line},
    operand::{OperandType, OperandValue},
    parser::{parse_address, parse_imm, parse_reg},
};

type ExpandRet<'a> = Line<'a>;
type ExpandFn = for<'a> fn(&'a str, &[OperandValue<'a>]) -> ExpandRet<'a>;

inventory::collect!(&'static dyn PseudoInstruction);

pub static PSEUDO_INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static dyn PseudoInstruction>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        for entry in inventory::iter::<&'static dyn PseudoInstruction> {
            for name in entry.names() {
                map.insert(*name, *entry);
            }
        }
        map
    });

pub trait PseudoInstruction: Send + Sync {
    fn names(&self) -> &'static [&'static str];
    fn operand_types(&self) -> &'static [OperandType];
    fn expander(&self) -> ExpandFn;

    fn expand<'a>(
        &self,
        ctx: &Context,
        pc: u32,
        name: &'a str,
        operands: &[OperandValue<'a>],
    ) -> Result<ExpandRet<'a>> {
        self.assert_operand_format(ctx, pc, name, operands)?;

        Ok(
            successors(Some((self.expander())(name, operands)), |(name, ops)| {
                let ps_instr = PSEUDO_INSTRUCTIONS.get(*name)?;

                ps_instr.assert_operand_format(ctx, pc, name, ops).ok()?;

                Some((ps_instr.expander())(name, ops))
            })
            .last()
            .unwrap(), // INFO: Safe because at least the first expansion exists
        )
    }

    fn assert_operand_format(
        &self,
        ctx: &Context,
        pc: u32,
        name: &str,
        operands: &[OperandValue],
    ) -> Result<()> {
        if operands.len() != self.operand_types().len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                name,
                self.operand_types().len(),
                operands.len()
            );
        }

        for (i, operand) in operands.iter().enumerate() {
            match self.operand_types()[i] {
                OperandType::RegD | OperandType::RegS => {
                    parse_reg(ctx, operand)?;
                },
                OperandType::Imm(bits, signed) => {
                    parse_imm(ctx, operand)?.as_field(bits, signed)?;
                },
                OperandType::Addr(bits) => {
                    parse_address(ctx, operand)?.as_field(bits, pc)?;
                },
            };
        }

        Ok(())
    }
}

macro impl_pseudo_instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            names: $names:tt,
            operand_types: $types:tt,
            expander: $expander:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::pseudo_instructions::PseudoInstruction for $id {
            fn names(&self) -> &'static [&'static str] {
                &$names
            }
            fn operand_types(&self) -> &'static [OperandType] {
                $crate::operand::op_types! $types
            }
            fn expander(&self) -> $crate::pseudo_instructions::ExpandFn {
                $expander
            }
        }

        inventory::submit! {
            &$id as &'static dyn $crate::pseudo_instructions::PseudoInstruction
        }
    }
}

macro pseudo_instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            operand_types: $types:tt,
            expander: $expander:expr,
        }
    ) => {
        $crate::pseudo_instructions::impl_pseudo_instruction! {
            $( #[doc = $doc] )*
            $vis $id {
                names: [ $name ],
                operand_types: $types,
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            names: $names:tt,
            operand_types: $types:tt,
            expander: $expander:expr,
        }
    ) => {
        $crate::pseudo_instructions::impl_pseudo_instruction! {
            $( #[doc = $doc] )*
            $vis $id {
                names: $names,
                operand_types: $types,
                expander: $expander,
            }
        }
    },
}
