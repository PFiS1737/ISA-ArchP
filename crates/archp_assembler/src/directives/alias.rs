use crate::{
    directives::{DirectiveOperand::*, directive},
    expression::Expr::*,
};

directive! {
    pub Alias {
        name: ".alias",
        matches: [Expr(Ident(op1)), Expr(Ident(op2))],
        handler: |ctx| ctx.aliases.insert(op1, op2),
    }
}
