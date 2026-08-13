use anyhow::bail;

use crate::{
    directives::{HandlerFn, directive},
    expression::Expr,
    operand::{DirectiveOperand, Operand},
};

directive! {
    pub Equ {
        name: ".equ",
        operand_count: 2,
        handler: F,
    }
}

const F: HandlerFn = |ctx, ops| {
    let DirectiveOperand::Expr(Expr::Ident(name)) = ops[0] else {
        bail!("expected identifier, got {}", ops[0]);
    };

    let DirectiveOperand::Expr(ref expr) = ops[1] else {
        bail!("expected expression, got {}", ops[1]);
    };

    let value = match expr.eval_to_operand_with(&ctx.equates)? {
        Operand::Num(value) => value,
        _ => bail!("expected absolute expression, got {}", expr),
    };

    ctx.equates.insert(name, value);

    Ok(())
};
