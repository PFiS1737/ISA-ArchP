mod branch;
mod inc_dec;
mod jump;
mod load_address;
mod load_imm;
mod mv;
mod negate;
mod nop;
mod not;
mod set;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Result, bail};
use smallvec::SmallVec;

use crate::{
    assembler::Instr,
    context::Context,
    operand::{Operand, OperandType},
};

type ExpandFn = for<'a> fn(&mut Context<'a>, &[Operand<'a>]) -> SmallVec<[Instr<'a>; 2]>;

inventory::collect!(Entry);

pub static PSEUDO_INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    format: &'static [OperandType],
    expander: ExpandFn,
}

trait PseudoInstruction: Send + Sync {
    const NAME: &'static str;
    const FORMAT: &'static [OperandType];
    const EXPANDER: ExpandFn;
}

impl Entry {
    const fn of<T: PseudoInstruction>() -> Self {
        Self {
            name: T::NAME,
            format: T::FORMAT,
            expander: T::EXPANDER,
        }
    }

    pub fn expand<'a>(
        &self,
        ctx: &mut Context<'a>,
        operands: &[Operand<'a>],
    ) -> Result<SmallVec<[Instr<'a>; 2]>> {
        self.assert_operand_format(operands)?;

        Ok((self.expander)(ctx, operands))
    }

    fn assert_operand_format(&self, operands: &[Operand]) -> Result<()> {
        if operands.len() != self.format.len() {
            bail!(
                "Pseudo-instruction '{}' requires {} operands, got {}",
                self.name,
                self.format.len(),
                operands.len()
            );
        }

        // TODO: change this
        for (i, op) in operands.iter().enumerate() {
            match self.format[i] {
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
                OperandType::None => {
                    panic!(
                        "Internal error: Pseudo-instruction '{}' has an operand type of None",
                        self.name
                    );
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
            format: $types:tt,
            expander: $expander:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::pseudo_instructions::PseudoInstruction for $id {
            const NAME: &'static str = $name;
            const FORMAT: &'static [$crate::operand::OperandType] = $crate::operand::op_types! $types;
            const EXPANDER: $crate::pseudo_instructions::ExpandFn = $expander;
        }

        inventory::submit! {
            $crate::pseudo_instructions::Entry::of::<$id>()
        }
    },
}

#[cfg(test)]
macro ps_instr( @($ctx:expr) $name:ident $($ops:expr),* $(;)? ) {{
    let name = <$name as $crate::pseudo_instructions::PseudoInstruction>::NAME;
    let ps_instr = $crate::pseudo_instructions::PSEUDO_INSTRUCTIONS.get(name).unwrap();
    ps_instr.expand($ctx, &$crate::operand::ops![$($ops),*])
}}

#[cfg(test)]
macro test_ps_instr( $name:ident $($ops:expr),* $(;)? ) {{
    use $crate::pseudo_instructions::ps_instr;
    use $crate::utils::fmt::fmt_line;
    use $crate::context::Context;
    match ps_instr!{ @(&mut Context::test()) $name $($ops),* } {
        Ok(expanded) => expanded
            .into_iter()
            .map(|(name, ops)| fmt_line(name, &ops))
            .collect::<Vec<_>>()
            .join("; "),
        Err(e) => format!("Error: {}", e),
    }
}}
