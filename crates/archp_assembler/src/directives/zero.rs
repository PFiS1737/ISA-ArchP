use crate::{directives::directive, operand::DirectiveOperand::*};

directive! {
    pub Zero {
        name: ".zero",
        matches: [Expr(expr)],
        handler: |ctx| {
            let bytes = expr.cast_absolute(ctx)?;

            for _ in 0..bytes {
                ctx.add_byte(0);
            }
        },
    }
}
