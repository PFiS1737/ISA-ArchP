use smallvec::SmallVec;

use crate::parser::types::operand::Operand;

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
