use anyhow::bail;

use crate::{
    directives::directive,
    expression::Expr::*,
    operand::{DirectiveOperand::*, Operand},
};

directive! {
    pub [
        Equ ".equ";
        Set ".set";
    ] {
        matches: [Expr(Ident(name)), Expr(expr)],
        handler: |ctx| {
            let value = match expr.eval_to_operand_with(&ctx.equates)? {
                Operand::Num(value) => value,
                _ => bail!("expected absolute expression, got {}", expr),
            };

            ctx.equates.insert(name, value);
        },
    }
}
