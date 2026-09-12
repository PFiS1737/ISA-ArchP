use crate::{
    directives::{DirectiveOperand::*, directive},
    expression::Expr::*,
};

directive! {
    pub Equ {
        name: ".equ",
        matches: [Expr(Ident(name)), Expr(expr)],
        handler: |ctx| {
            let (is_relative, value) = expr.cast_absolute_or_relative(ctx)?;

            ctx.equates.insert(name, value);

            if is_relative {
                ctx.labels.insert(name, value as usize);
            }
        },
    }
}

directive! {
    pub Set {
        name: ".set",
        matches: [Expr(Ident(name)), Expr(expr)],
        handler: |ctx| {
            let (is_relative, value) = expr.cast_absolute_or_relative(ctx)?;

            ctx.equates.insert(name, value);

            if is_relative {
                ctx.labels.insert(name, value as usize);
            }
        },
    }
}
