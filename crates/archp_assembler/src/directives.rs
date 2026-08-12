mod alias;
mod equate;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Result, bail};

use crate::{assembler::Context, operand::Operand};

type HandlerFn = for<'a> fn(&mut Context<'a>, &[Operand<'a>]) -> Result<()>;

inventory::collect!(Entry);

pub static DIRECTIVES: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    operand_count: Option<usize>,
    handler: HandlerFn,
}

trait Directive: Send + Sync {
    const NAME: &'static str;
    const OPERAND_COUNT: Option<usize>;
    const HANDLER: HandlerFn;
}

impl Entry {
    const fn of<T: Directive>() -> Self {
        Self {
            name: T::NAME,
            operand_count: T::OPERAND_COUNT,
            handler: T::HANDLER,
        }
    }

    pub fn handle<'a>(&self, ctx: &mut Context<'a>, operands: &[Operand<'a>]) -> Result<()> {
        self.assert_operand_count(operands)?;

        (self.handler)(ctx, operands)
    }

    fn assert_operand_count(&self, operands: &[Operand]) -> Result<()> {
        if let Some(count) = self.operand_count
            && operands.len() != count
        {
            bail!(
                "Directive '{}' requires {} operands, got {}",
                self.name,
                count,
                operands.len()
            );
        }

        Ok(())
    }
}

macro directive {
    (@impl
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            operand_count: $count:ident $( ($value:literal) )? ,
            handler: $handler:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::directives::Directive for $id {
            const NAME: &'static str = $name;
            const OPERAND_COUNT: Option<usize> = $count $( ( $value ) )?;
            const HANDLER: $crate::directives::HandlerFn = $handler;
        }

        inventory::submit! {
            $crate::directives::Entry::of::<$id>()
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            operand_count: $count:literal,
            handler: $handler:expr,
        }
    ) => {
        directive! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                operand_count: Some($count),
                handler: $handler,
            }
        }
    },

(
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            handler: $handler:expr,
        }
    ) => {
        directive! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                operand_count: None,
                handler: $handler,
            }
        }
    }
}
