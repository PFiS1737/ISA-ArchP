mod branch;
mod inc_dec;
mod jump;
mod load_imm;
mod mv;
mod negate;
mod not;
mod set;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Result, bail};

use crate::{
    assembler::{Context, Instr},
    operand::{Operand, OperandType},
    parser::{address::parse_address, immediate::parse_imm_as, register::parse_reg},
};

type ExpandFn = for<'a> fn(&[Operand<'a>]) -> Instr<'a>;

inventory::collect!(&'static dyn PseudoInstruction);

pub static PSEUDO_INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static dyn PseudoInstruction>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        for entry in inventory::iter::<&'static dyn PseudoInstruction> {
            map.insert(entry.name(), *entry);
        }
        map
    });

pub trait PseudoInstruction: Send + Sync {
    fn name(&self) -> &'static str;
    fn operand_types(&self) -> &'static [OperandType];
    fn expander(&self) -> ExpandFn;

    fn expand<'a>(&self, ctx: &Context, pc: u32, operands: &[Operand<'a>]) -> Result<Instr<'a>> {
        self.assert_operand_format(ctx, pc, operands)?;

        Ok((self.expander())(operands))
    }

    fn assert_operand_format(&self, ctx: &Context, pc: u32, operands: &[Operand]) -> Result<()> {
        if operands.len() != self.operand_types().len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name(),
                self.operand_types().len(),
                operands.len()
            );
        }

        for (i, op) in operands.iter().enumerate() {
            match self.operand_types()[i] {
                OperandType::RegD | OperandType::RegS => {
                    parse_reg(ctx, op)?;
                },
                OperandType::Imm(bits, signed) => {
                    parse_imm_as(ctx, op, bits, signed)?;
                },
                OperandType::Addr(bits) => {
                    parse_address(ctx, op)?.as_field(bits, pc)?;
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
            name: $name:literal,
            operand_types: $types:tt,
            expander: $expander:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::pseudo_instructions::PseudoInstruction for $id {
            fn name(&self) -> &'static str {
                $name
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
                name: $name,
                operand_types: $types,
                expander: $expander,
            }
        }
    },
}
