mod als_imm32;
mod auto_imm;
mod branch_imm;
mod cmp_imm32;
mod jal_addr32;
mod load_imm32;
mod riscv_offset;

use std::collections::{HashMap, VecDeque};

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{
    assembler::{Context, Line},
    operand::OperandValue,
};

type ExpandRet<'a> = Option<Vec<Line<'a>>>;
type ExpandFn =
    for<'a> fn(&Context<'a>, u32, &'a str, Option<&'a str>, &[OperandValue<'a>]) -> ExpandRet<'a>;

inventory::collect!(&'static dyn MacroInstruction);

pub static MACRO_INSTRUCTIONS: Lazy<HashMap<&'static str, &'static dyn MacroInstruction>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        for entry in inventory::iter::<&'static dyn MacroInstruction> {
            for name in entry.names() {
                map.insert(*name, *entry);
            }
        }
        map
    });

pub trait MacroInstruction: Send + Sync {
    fn names(&self) -> &'static [&'static str];
    fn operand_count(&self) -> Option<usize>;
    fn expander(&self) -> ExpandFn;

    fn expand<'a>(
        &self,
        ctx: &Context<'a>,
        pc: u32,
        name: &'a str,
        cond: Option<&'a str>,
        operands: &[OperandValue<'a>],
    ) -> Result<ExpandRet<'a>> {
        self.assert_operand_count(name, operands)?;

        let mut deq: VecDeque<_> = match (self.expander())(ctx, pc, name, cond, operands) {
            None => return Ok(None),
            Some(v) => v.into(),
        };

        let mut ret = Vec::new();

        while let Some((name, cond, ops)) = deq.pop_front() {
            if let Some(mc) = MACRO_INSTRUCTIONS.get(name) {
                mc.assert_operand_count(name, &ops)?;

                match (mc.expander())(ctx, pc, name, cond, &ops) {
                    None => {
                        ret.push((name, cond, ops));
                    }
                    Some(v) => {
                        let mut q: VecDeque<_> = v.into();
                        q.append(&mut deq);
                        deq = q;
                    }
                }
            } else {
                ret.push((name, cond, ops));
            }
        }

        Ok(Some(ret))
    }

    fn assert_operand_count(&self, name: &str, operands: &[OperandValue]) -> Result<()> {
        if let Some(count) = self.operand_count()
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

macro impl_macro_instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $names:tt,
            operand_count: $count:ident $( ($value:literal) )? ,
            expander: $expander:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::macro_instructions::MacroInstruction for $id {
            fn names(&self) -> &'static [&'static str] {
                &$names
            }
            fn operand_count(&self) -> Option<usize> {
                $count $( ( $value ) )?
            }
            fn expander(&self) -> $crate::macro_instructions::ExpandFn {
                $expander
            }
        }

        inventory::submit! {
            &$id as &'static dyn $crate::macro_instructions::MacroInstruction
        }
    },
}

macro macro_instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $names:literal,
            operand_count: $count:literal,
            expander: $expander:expr,
        }
    ) => {
        $crate::macro_instructions::impl_macro_instruction! {
            $( #[doc = $doc] )*
            $vis $id {
                name: [ $names ],
                operand_count: Some($count),
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $names:literal,
            expander: $expander:expr,
        }
    ) => {
        $crate::macro_instructions::impl_macro_instruction! {
            $( #[doc = $doc] )*
            $vis $id {
                name: [ $names ],
                operand_count: None,
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $names:tt,
            operand_count: $count:literal,
            expander: $expander:expr,
        }
    ) => {
        $crate::macro_instructions::impl_macro_instruction! {
            $( #[doc = $doc] )*
            $vis $id {
                name: $names,
                operand_count: Some($count),
                expander: $expander,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $names:tt,
            expander: $expander:expr,
        }
    ) => {
        $crate::macro_instructions::impl_macro_instruction! {
            $( #[doc = $doc] )*
            $vis $id {
                name: $names,
                operand_count: None,
                expander: $expander,
            }
        }
    },
}
