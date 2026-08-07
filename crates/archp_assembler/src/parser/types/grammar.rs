use smallvec::SmallVec;

use crate::parser::types::expression::Expr;

#[derive(Debug)]
pub struct Source<'src> {
    pub lines: Vec<Line<'src>>,
}

#[derive(Debug)]
pub enum Line<'src> {
    Instr {
        name: &'src str,
        operands: SmallVec<[Operand<'src>; 3]>,
        line: (usize, &'src str),
    },
    Label(&'src str),
}

#[derive(Debug)]
pub enum Operand<'src> {
    Num(i64),
    Ident(&'src str),
    #[allow(unused)] // FIXME: unused
    String(&'src str),
    #[allow(unused)] // FIXME: unused
    Expr(Expr<'src>),
}
