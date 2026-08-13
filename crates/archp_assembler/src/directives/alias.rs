use anyhow::bail;

use crate::{
    directives::{HandlerFn, directive},
    expression::Expr,
    operand::DirectiveOperand,
};

directive! {
    pub Alias {
        name: ".alias",
        operand_count: 2,
        handler: F,
    }
}

const F: HandlerFn = |ctx, ops| {
    let DirectiveOperand::Expr(Expr::Ident(op1)) = ops[0] else {
        bail!("expected identifier, got {}", ops[0]);
    };

    let DirectiveOperand::Expr(Expr::Ident(op2)) = ops[1] else {
        bail!("expected identifier, got {}", ops[1]);
    };

    ctx.aliases.insert(op1, op2);

    Ok(())
};
