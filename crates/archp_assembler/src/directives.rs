mod alias;
mod equate;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;

use crate::{context::Context, operand::DirectiveOperand};

type HandlerFn = for<'a> fn(&mut Context<'a>, &[DirectiveOperand<'a>]) -> Result<()>;

inventory::collect!(Entry);

pub static DIRECTIVES: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    handler: HandlerFn,
}

trait Directive: Send + Sync {
    const NAME: &'static str;
    const HANDLER: HandlerFn;
}

impl Entry {
    const fn of<T: Directive>() -> Self {
        Self {
            name: T::NAME,
            handler: T::HANDLER,
        }
    }

    pub fn handle<'a>(
        &self,
        ctx: &mut Context<'a>,
        operands: &[DirectiveOperand<'a>],
    ) -> Result<()> {
        (self.handler)(ctx, operands)
    }
}

macro directive {
    (@impl
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            handler: $handler:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::directives::Directive for $id {
            const NAME: &'static str = $name;
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
            matches: $matches:tt,
            handler: |$ctx:ident| $handler:expr,
        }
    ) => {
        directive! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                handler: |$ctx, ops| {
                    let $matches = ops else { anyhow::bail!("operands mismatch"); };
                    $handler;
                    Ok(())
                },
            }
        }
    },
}
