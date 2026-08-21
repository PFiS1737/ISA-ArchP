use anyhow::bail;

use crate::{
    directives::directive,
    operand::{DirectiveOperand::*, Operand},
};

directive! {
    pub Zero {
        name: ".zero",
        matches: [Expr(expr)],
        handler: |ctx| {
            let bytes = match expr.eval_to_operand_with(&ctx.equates)? {
                Operand::Num(value) => value,
                _ => bail!("expected absolute expression, got {}", expr),
            };

            for _ in 0..bytes {
                ctx.add_byte(0);
            }
        },
    }
}
