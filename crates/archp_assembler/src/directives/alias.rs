use crate::{directives::directive, expression::Expr::*, operand::DirectiveOperand::*};

directive! {
    pub Alias {
        name: ".alias",
        matches: [Expr(Ident(op1)), Expr(Ident(op2))],
        handler: |ctx| ctx.aliases.insert(op1, op2),
    }
}
