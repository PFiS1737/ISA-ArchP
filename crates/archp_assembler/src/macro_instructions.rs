mod als_imm32;
mod auto_ecall;
mod auto_imm;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Result, bail};

use crate::{assembler::Instr, context::Context, operand::Operand};

type ExpandRet<'a> = Option<Vec<Instr<'a>>>;
type ExpandFn = for<'a> fn(&Context<'a>, &'a str, &[Operand<'a>]) -> ExpandRet<'a>;

inventory::collect!(Entry);

pub static MACRO_INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| {
        HashMap::from_iter(
            inventory::iter::<Entry>
                .into_iter()
                .flat_map(|e| e.names.iter().map(move |&name| (name, e))),
        )
    });

pub struct Entry {
    names: &'static [&'static str],
    operand_count: Option<usize>,
    expander: ExpandFn,
}

trait MacroInstruction: Send + Sync {
    const NAMES: &'static [&'static str];
    const OPERAND_COUNT: Option<usize>;
    const EXPANDER: ExpandFn;
}

impl Entry {
    const fn of<T: MacroInstruction>() -> Self {
        Self {
            names: T::NAMES,
            operand_count: T::OPERAND_COUNT,
            expander: T::EXPANDER,
        }
    }

    pub fn expand<'a>(
        &self,
        ctx: &Context<'a>,
        name: &'a str,
        operands: &[Operand<'a>],
    ) -> Result<ExpandRet<'a>> {
        self.assert_operand_count(name, operands)?;

        Ok((self.expander)(ctx, name, operands))
    }

    fn assert_operand_count(&self, name: &str, operands: &[Operand]) -> Result<()> {
        if let Some(count) = self.operand_count
            && operands.len() != count
        {
            bail!(
                "Macro-instruction '{}' requires {} operands, got {}",
                name,
                count,
                operands.len()
            );
        }

        Ok(())
    }
}

macro macro_instruction {
    (@impl
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            names: $names:tt,
            operand_count: $count:ident $( ($value:literal) )? ,
            expander: $expander:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::macro_instructions::MacroInstruction for $id {
            const NAMES: &'static [&'static str] = &$names;
            const OPERAND_COUNT: Option<usize> = $count $( ( $value ) )?;
            const EXPANDER: $crate::macro_instructions::ExpandFn = $expander;
        }

        inventory::submit! {
            $crate::macro_instructions::Entry::of::<$id>()
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            operand_count: $count:literal,
            expander: $expander:expr,
        }
    ) => {
        macro_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                names: [ $name ],
                operand_count: Some($count),
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            expander: $expander:expr,
        }
    ) => {
        macro_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                names: [ $name ],
                operand_count: None,
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            names: $names:tt,
            operand_count: $count:literal,
            expander: $expander:expr,
        }
    ) => {
        macro_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                names: $names,
                operand_count: Some($count),
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            names: $names:tt,
            expander: $expander:expr,
        }
    ) => {
        macro_instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                names: $names,
                operand_count: None,
                expander: $expander,
            }
        }
    },
}
