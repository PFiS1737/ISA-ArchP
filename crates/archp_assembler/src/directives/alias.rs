use anyhow::bail;

use crate::{
    directives::{HandlerFn, directive},
    operand::Operand,
};

directive! {
    pub Alias {
        name: ".alias",
        operand_count: 2,
        handler: F,
    }
}

const F: HandlerFn = |ctx, ops| {
    let Operand::Ident(op1) = ops[0] else {
        bail!("expected identifier, got {}", ops[0]);
    };

    let Operand::Ident(op2) = ops[1] else {
        bail!("expected identifier, got {}", ops[1]);
    };

    ctx.aliases.insert(op1, op2);

    Ok(())
};
