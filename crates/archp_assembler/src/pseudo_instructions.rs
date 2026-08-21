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

use anyhow::Result;
use smallvec::SmallVec;

use crate::{assembler::Instr, context::Context, operand::Operand};

type ExpandFn = for<'a> fn(&mut Context<'a>, &[Operand<'a>]) -> Result<SmallVec<[Instr<'a>; 2]>>;

inventory::collect!(Entry);

pub static PSEUDO_INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    expander: ExpandFn,
}

trait PseudoInstruction: Send + Sync {
    const NAME: &'static str;
    const EXPANDER: ExpandFn;
}

impl Entry {
    const fn of<T: PseudoInstruction>() -> Self {
        Self {
            name: T::NAME,
            expander: T::EXPANDER,
        }
    }

    pub fn expand<'a>(
        &self,
        ctx: &mut Context<'a>,
        operands: &[Operand<'a>],
    ) -> Result<SmallVec<[Instr<'a>; 2]>> {
        (self.expander)(ctx, operands)
    }
}

macro pseudo_instruction {
    (@impl
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident $name:literal {
            $expander:expr
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::pseudo_instructions::PseudoInstruction for $id {
            const NAME: &'static str = $name;
            const EXPANDER: $crate::pseudo_instructions::ExpandFn = $expander;
        }

        inventory::submit! {
            $crate::pseudo_instructions::Entry::of::<$id>()
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident $name:literal |$ops:tt| {
            $(
                $matches:tt => [ $( $op:expr ),* $(,)? ]
            );+ ;
        }
    ) => {
        pseudo_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id $name {
                |_, $ops| {
                    $(
                        if let $matches = $ops {
                            return Ok(smallvec::smallvec![ $( $op) ,* ])
                        }
                    )+

                    anyhow::bail!("operands mismatch")
                }
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident $name:literal {
            $(
                $matches:tt => $expander:expr
            );+ ;
        }
    ) => {
        pseudo_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id $name {
                |ctx, ops| {
                    $(
                        if let $matches = ops {
                            return ($expander)(ctx, ops)
                        }
                    )+

                    anyhow::bail!("operands mismatch")
                }
            }
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
