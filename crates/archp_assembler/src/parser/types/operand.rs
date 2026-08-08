use crate::parser::types::expression::Expr;

#[derive(Debug)]
pub enum Operand<'src> {
    Num(i64),
    Ident(&'src str),
    #[allow(unused)] // FIXME: unused
    String(&'src str),
    Expr(Expr<'src>),
}
