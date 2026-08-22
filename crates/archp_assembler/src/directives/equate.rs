use crate::{
    directives::{DirectiveOperand::*, directive},
    expression::Expr::*,
};

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
