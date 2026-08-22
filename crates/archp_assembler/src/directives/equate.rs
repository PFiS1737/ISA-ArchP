use crate::{directives::directive, expression::Expr::*, operand::DirectiveOperand::*};

directive! {
    pub Equ {
        name: ".equ",
        matches: [Expr(Ident(name)), Expr(expr)],
        handler: |ctx| {
            let value = expr.cast_absolute(ctx)?;
            ctx.equates.insert(name, value);
        },
    }
}

directive! {
    pub Set {
        name: ".set",
        matches: [Expr(Ident(name)), Expr(expr)],
        handler: |ctx| {
            let value = expr.cast_absolute(ctx)?;
            ctx.equates.insert(name, value);
        },
    }
}
