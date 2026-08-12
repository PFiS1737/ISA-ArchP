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
use smallvec::SmallVec;

use crate::{
    assembler::{Context, Instr},
    operand::{Operand, OperandType},
};

type ExpandFn = for<'a> fn(&Context<'a>, &[Operand<'a>]) -> SmallVec<[Instr<'a>; 2]>;

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
        ctx: &Context<'a>,
        operands: &[Operand<'a>],
    ) -> Result<SmallVec<[Instr<'a>; 2]>> {
        self.assert_operand_format(operands)?;

        Ok((self.expander)(ctx, operands))
    }

    fn assert_operand_format(&self, operands: &[Operand]) -> Result<()> {
        if operands.len() != self.operand_types.len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name,
                self.operand_types.len(),
                operands.len()
            );
        }

        // TODO: change this
        for (i, op) in operands.iter().enumerate() {
            match self.operand_types[i] {
                OperandType::RegD | OperandType::RegS => {
                    if !matches!(op, Operand::Ident(..)) {
                        bail!(
                            "Pseudo-instruction '{}' requires operand {} to be a register, got {}",
                            self.name,
                            i + 1,
                            op
                        );
                    }
                },
                OperandType::Imm(..) => {
                    if !matches!(op, Operand::Num(..)) {
                        bail!(
                            "Pseudo-instruction '{}' requires operand {} to be an immediate, got {}",
                            self.name,
                            i + 1,
                            op
                        );
                    }
                },
                OperandType::Addr(..) => {
                    if !matches!(op, Operand::Ident(..) | Operand::Addition(..)) {
                        bail!(
                            "Pseudo-instruction '{}' requires operand {} to be an address, got {}",
                            self.name,
                            i + 1,
                            op
                        );
                    }
                },
            };
        }

        Ok(())
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
}
