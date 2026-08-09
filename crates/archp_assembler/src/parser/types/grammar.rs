use smallvec::SmallVec;

use crate::operand::Operand;

#[derive(Debug)]
pub struct Source<'src> {
    pub lines: Vec<Line<'src>>,
}

#[derive(Debug)]
pub enum Line<'src> {
    Label(&'src str),
    Instr {
        name: &'src str,
        operands: SmallVec<[Operand<'src>; 3]>,
        line: (usize, &'src str),
    },
}
