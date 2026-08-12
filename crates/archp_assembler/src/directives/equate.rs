use anyhow::bail;

use crate::{
    directives::{HandlerFn, directive},
    operand::Operand,
};

directive! {
    pub Equ {
        name: ".equ",
        operand_count: 2,
        handler: F,
    }
}

const F: HandlerFn = |ctx, ops| {
    let Operand::Ident(name) = &ops[0] else {
        bail!("expected identifier, got {}", ops[0]);
    };

    let Operand::Num(value) = &ops[1] else {
        bail!("expected number, got {}", ops[1]);
    };

    ctx.equates.insert(name, *value);

    Ok(())
};
