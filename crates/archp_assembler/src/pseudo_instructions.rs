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
    encoder::{address::encode_address, immediate::encode_immediate_as, register::encode_register},
    operand::{Operand, OperandType},
};

type ExpandFn = for<'a> fn(&[Operand<'a>]) -> Instr<'a>;

inventory::collect!(Entry);

pub static PSEUDO_INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    operand_types: &'static [OperandType],
    expander: ExpandFn,
}

trait PseudoInstruction: Send + Sync {
    const NAME: &'static str;
    const OPERAND_TYPES: &'static [OperandType];
    const EXPANDER: ExpandFn;
}

impl Entry {
    const fn of<T: PseudoInstruction>() -> Self {
        Self {
            name: T::NAME,
            operand_types: T::OPERAND_TYPES,
            expander: T::EXPANDER,
        }
    }

    pub fn expand<'a>(
        &self,
        ctx: &Context,
        pc: u32,
        operands: &[Operand<'a>],
    ) -> Result<Instr<'a>> {
        self.assert_operand_format(ctx, pc, operands)?;

        Ok((self.expander)(operands))
    }

    fn assert_operand_format(&self, ctx: &Context, pc: u32, operands: &[Operand]) -> Result<()> {
        if operands.len() != self.operand_types.len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name,
                self.operand_types.len(),
                operands.len()
            );
        }

        for (i, op) in operands.iter().enumerate() {
            match self.operand_types[i] {
                OperandType::RegD | OperandType::RegS => {
                    encode_register(ctx, op)?;
                },
                OperandType::Imm(bits, signed) => {
                    encode_immediate_as(ctx, op, bits, signed)?;
                },
                OperandType::Addr(bits) => {
                    encode_address(ctx, op)?.as_field(bits, pc)?;
                },
            };
        }

        Ok(())
    }
}

macro pseudo_instruction {
    (@impl
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
            const NAME: &'static str = $name;
            const OPERAND_TYPES: &'static [ $crate::operand::OperandType] = $crate::operand::op_types! $types;
            const EXPANDER: $crate::pseudo_instructions::ExpandFn = $expander;
        }

        inventory::submit! {
            $crate::pseudo_instructions::Entry::of::<$id>()
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            operand_types: $types:tt,
            expander: $expander:expr,
        }
    ) => {
        pseudo_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                operand_types: $types,
                expander: $expander,
            }
        }
    },
}
